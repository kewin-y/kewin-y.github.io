use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use ammonia;
use maud::PreEscaped;

mod frontmatter;
mod templates;

const CONTENT_DIR: &str = "content";
const OUTPUT_DIR: &str = "output";

fn to_output_file(path: &PathBuf) -> PathBuf {
    let remainder = path.strip_prefix(CONTENT_DIR).unwrap_or(Path::new(path));
    let mut output_file = Path::new(OUTPUT_DIR).join(remainder);
    output_file.set_extension("html");
    output_file
}

// This functions takes the entire markdown string including
// frontmatter. This is because pulldown_cmark skips
// yaml metadata.
fn build_blog(title: &str, markdown: &str) -> PreEscaped<String> {
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);
    let safe_body = ammonia::clean(&body);
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

    for md_file in md_files {
        let path = md_file.path();
        let markdown = fs::read_to_string(&path)?;
        let frontmatter = match frontmatter::parse_frontmatter(&markdown) {
            Some(fm) => fm,
            None => {
                eprintln!(
                    "Could not parse frontmatter for {:#?}. Skipping ...",
                    path.into_os_string().into_string()
                );
                continue;
            }
        };
        let blog = build_blog(&frontmatter.title, &markdown);
        let output_file = to_output_file(&path);

        fs::write(output_file, blog.into_string())?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    if Path::is_dir(Path::new(OUTPUT_DIR)) {
        eprintln!("{} already exists. Removing ...", OUTPUT_DIR);
        fs::remove_dir_all(OUTPUT_DIR)?;
    }

    fs::create_dir(OUTPUT_DIR)?;

    write_all_blogs()?;

    Ok(())
}
