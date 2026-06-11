use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use ammonia;
use maud::PreEscaped;

pub mod templates;

const CONTENT_DIR: &str = "content";
const OUTPUT_DIR: &str = "output";

fn to_output_file(path: &PathBuf) -> PathBuf {
    let remainder = path.strip_prefix(CONTENT_DIR).unwrap_or(Path::new(path));
    let mut output_file = Path::new(OUTPUT_DIR).join(remainder);
    output_file.set_extension("html");
    output_file
}

fn build_blog(title: &str, markdown: &str) -> PreEscaped<String> {
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);
    let safe_body = ammonia::clean(&body);

    // TODO: get the yaml frontmatter from markdown
    // and get the correct title page
    let page = templates::page(title, PreEscaped(safe_body));
    page
}

fn write_all_blogs() -> anyhow::Result<()> {
    let md_files: Vec<DirEntry> = fs::read_dir(CONTENT_DIR)?
        .flatten()
        .filter(|e| {
            e.path().is_file() && e.path().extension().map(|ext| ext == "md").unwrap_or(false)
        })
        .collect();

    // https://stackoverflow.com/questions/44419890/replacing-path-parts-in-rust
    for md_file in md_files {
        let path = md_file.path();
        let markdown = fs::read_to_string(&path)?;
        let blog = build_blog("Hello", &markdown);
        let output_file = to_output_file(&path);

        println!("{}", output_file.to_str().unwrap());

        fs::write(output_file, blog.into_string())?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    if Path::is_dir(Path::new(OUTPUT_DIR)) {
        fs::remove_dir(OUTPUT_DIR)?;
    }

    fs::create_dir(OUTPUT_DIR)?;

    write_all_blogs()?;

    Ok(())
}
