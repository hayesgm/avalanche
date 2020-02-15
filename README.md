
# Avalanche

Avalanche is a static web blog builder, built *exclusively* on CloudFlare workers. All you need is a free CloudFlare account and a domain name.

## Setup

You will need a CloudFlare account, and set the following varibles:

* `TF_VAR_takehome_cloudflare_email`: Your CloudFlare email
* `TF_VAR_takehome_cloudflare_api_token`: API Token with ability to edit zones and workers
* `TF_VAR_takehome_cloudflare_account_id`: Your CloudFlare account ID
* `TF_VAR_takehome_cloudflare_zone`: The CloudFlare zone for your blog
* `TF_VAR_takehome_cloudflare_zone_id`: The zone ID
* `TF_VAR_takehome_cloudflare_domain`: The associated domain name

You will also need rust installed (see [Installing Rust](https://www.rust-lang.org/tools/install)) and [Terraform](https://www.terraform.io/downloads.html).

You will need to run `terraform init`, which should check these variables.

## Building

```
cargo build --release
```

## Add Your Blog Entries

Create a new directory and add any number of blog entries in markdown. You may also want to add a `css` file and a file named `index.md` as a homepage.

```
avalanche/
  my_site/
    index.md
    blog_entry_1.md
    my_css.css
```

## Generating and Deploy Your Site

```bash
target/release/avalanche build --input example/*.md --css example/page.css
```

This will build a `dist/worker.js` that hosts the content of your site. Next, we need to push the files to CloudFlare. See above for the necessary variables.

```
terraform apply
```

Now you can simply visit your domain, and the site should be hooked up!

## Updating Your Site

You can re-run the commands above to update your site. Put together, that's:

```bash
target/release/avalanche build --input example/*.md --css example/page.css && terraform apply -auto-approve
```

## Contributing

Please open issues or follow up with any questions!

Originally built as a code sample for [Compound](https://compound.finance).
