use super::paths;
use pulldown_cmark::{html, Options, Parser};
use std::fs;

// Removes any previously generated output
pub fn clean() {
    paths::remove_dir(&paths::public());
}

// Generates a static HTML bundle from markdown notes
pub fn build() {
    paths::create_dir(&paths::public());
    match fs::read_dir(&paths::notes()) {
        Ok(markdown_files) => {
            for markdown_file in markdown_files {
                let markdown_file = match &markdown_file {
                    Ok(markdown_file) => markdown_file.path(),
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let html_file = format!("{}.html", &paths::file_stem(markdown_file));
                paths::create_file(
                    &paths::public().join(html_file),
                    &markdown_to_html("Some **markdown** testing from trevordmiller.com"),
                );
            }
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}

// Adds a CNAME file for the host (GitHub Pages) and registrar (Hover) to use my custom domain name (trevordmiller.com)
pub fn configure() {
    paths::create_file(&paths::cname(), "trevordmiller.com");
}

// Pushes output to my GitHub Pages repo (https://github.com/trevordmiller/trevordmiller.github.io)
pub fn deploy() {}

fn markdown_to_html(markdown: &str) -> std::string::String {
    let parser = Parser::new_ext(markdown, Options::empty());
    let mut html = String::new();
    html::push_html(&mut html, parser);
    format!(
        "
        <!DOCTYPE html>
        <html lang=\"en-US\">
            <head>
                <meta charset=\"utf-8\">
                <meta name=\"author\" content=\"Trevor D. Miller\">
                <meta name=\"description\" content=\"Personal website.\">
                <title>trevordmiller.com</title>
            </head>
            <body>
                <header>
                    <p>Header</p>
                </header>
                <main>
                    <article>
                        {}
                    </article>
                </main>
                <footer>
                    <p>Footer</p>
                </footer>
              </body>
        </html>
    ",
        &html
    )
}
