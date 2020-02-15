extern crate clap;
use clap::{Arg, App, SubCommand};

mod build;

fn main() {
    let matches = App::new("Takehome")
        .version("1.0")
        .author("Geoffrey Hayes <hayesgm@gmail.com>")
        .about("Static website builder")
        .subcommand(SubCommand::with_name("build")
            .arg(
                Arg::with_name("input")
                    .long("input")
                    .short("i")
                    .help("input file")
                    .takes_value(true)
                    .multiple(true)
                    .value_terminator(" ")
                    .required(true)
            )
            .arg(
                Arg::with_name("target")
                    .long("target")
                    .short("t")
                    .help("target file")
                    .default_value("dist/index.js")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                Arg::with_name("worker")
                    .long("worker")
                    .short("w")
                    .help("worker template")
                    .default_value("./worker.js")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                Arg::with_name("css")
                    .long("css")
                    .short("c")
                    .help("css file")
                    .takes_value(true)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let input: Vec<&str> = matches.values_of("input").unwrap().collect();
        let worker = matches.value_of("worker").unwrap();
        let target = matches.value_of("target").unwrap();
        let css = matches.value_of("css");
        println!("Building from {:?} to {} (worker={}, css={:?})", input, target, worker, css);

        build::build(input, target, worker, css);
    }
}
