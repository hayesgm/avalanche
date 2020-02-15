extern crate base64;

use base64::{encode};
use pulldown_cmark::{Parser, Options, html};
use std::fs;

struct InputFile {
    name: String,
    path: String,
    file_type: String,
    link: String,
    title: String,
    content: String
}

fn build_input_file(file_name: &str) -> InputFile {
    let content = fs::read_to_string(file_name)
        .expect("Input file not found");
    let (name, path, file_type) = get_name(file_name);
    let title = name.clone();
    let link = match &name[..] {
        "index" => String::from(""),
        _ => name.clone()
    };

    InputFile {
        name: name.to_string(),
        path: path.to_string(),
        file_type: file_type.to_string(),
        link: link,
        title: title.to_string(),
        content
    }
}

fn parse_file(input_file: &InputFile, css: &str, links: &str) -> String {
    assert!(input_file.file_type == "md");

    let content = input_file.content.to_string().replace("{{links}}", links);

    let parser = Parser::new_ext(content.as_str(), Options::empty());

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    format!("<html><head>{}</head><body>{}</body></html>", css, html_output)
}

fn build_links<'a>(input_files: &Vec<InputFile>) -> String {
    let mut result = String::from("\n");
    for input_file in input_files {
        let item = format!(" * [{}](/{})\n", input_file.title, input_file.link);
        result.push_str(item.as_str());
    }
    result
}

fn get_name(input_file: &str) -> (String, String, String) {
    let filename_parts: Vec<&str> = input_file.rsplit('/').collect();
    let filename = filename_parts.get(0).unwrap();
    let parts: Vec<&str> = filename.split('.').collect();
    let name = parts.get(0).expect("expected file name").to_string();
    let file_type = parts.get(1).expect("expected file type").to_string();

    (name, input_file.to_string(), file_type)
}

pub fn build(inputs: Vec<&str>, target: &str, worker: &str, css_file: Option<&str>) -> () {
    let worker_template = fs::read_to_string(worker)
        .expect("Worker template not found");

    let css = match css_file {
        Some(css_file) => {
            let css = fs::read_to_string(css_file)
                .expect("CSS file not found");
            format!("<style>{}</style>", css)
        },
        None => String::from("")
    };

    let input_files: Vec<InputFile> = inputs.into_iter().map(|input| build_input_file(input)).collect();

    let links = build_links(&input_files);

    let mut pages = "const pages = {\n".to_string();

    for input_file in input_files {
        let page = parse_file(&input_file, css.as_str(), links.as_str());

        pages.push_str(format!("  \"{}\": \"{}\",\n", input_file.link, encode(&page)).as_str())
    }

    pages.push_str("};");

    let js = format!("{}\n\n{}", pages, worker_template);

    fs::write(target, js).expect("Output file not writable");
}
