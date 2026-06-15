use std::{
    fs,
    path::{Path, PathBuf},
};

mod frontmatter;
mod site;
mod templates;

const CONTENT_DIR: &str = "content";
const OUTPUT_DIR: &str = "output";

pub fn to_blog_output_file(path: &PathBuf) -> PathBuf {
    let remainder = path.strip_prefix(CONTENT_DIR).unwrap_or(Path::new(path));
    let mut output_file = Path::new(OUTPUT_DIR).join(remainder);
    output_file.set_extension("html");
    output_file
}

fn write_all_blogs() -> anyhow::Result<Vec<site::BlogLink>> {
    let md_files: Vec<PathBuf> = fs::read_dir(CONTENT_DIR)?
        .flatten()
        .filter_map(|e| {
            if e.path().is_file()
                && !e.path().file_stem().map(|s| s == "index").unwrap_or(false)
                && e.path().extension().map(|ext| ext == "md").unwrap_or(false)
            {
                Some(e.path())
            } else {
                None
            }
        })
        .collect();

    let mut blogs: Vec<site::BlogLink> = Vec::new();

    for md_file in md_files {
        let markdown = fs::read_to_string(&md_file)?;
        let frontmatter = match frontmatter::parse_frontmatter(&markdown) {
            Some(fm) => fm,
            None => {
                eprintln!(
                    "Could not parse frontmatter for {:#?}. Skipping ...",
                    md_file.into_os_string().into_string()
                );
                continue;
            }
        };
        let blog = site::build_blog(&frontmatter.title, &markdown);
        let output_file = to_blog_output_file(&md_file);

        let href = output_file
            .strip_prefix(OUTPUT_DIR)?
            .to_string_lossy()
            .into_owned();

        fs::write(output_file, blog.into_string())?;

        let frontmatter::Frontmatter { title, date, .. } = frontmatter;

        blogs.push(site::BlogLink { title, date, href });
    }

    Ok(blogs)
}

fn write_index(index: maud::PreEscaped<String>) -> anyhow::Result<()> {
    let output_file = Path::new(OUTPUT_DIR).join("index.html");
    fs::write(output_file, index.into_string())?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    if Path::is_dir(Path::new(OUTPUT_DIR)) {
        eprintln!("{} already exists. Removing ...", OUTPUT_DIR);
        fs::remove_dir_all(OUTPUT_DIR)?;
    }

    fs::create_dir(OUTPUT_DIR)?;

    let blogs = write_all_blogs()?;

    write_index(site::build_index("Kevin Yu", blogs))?;

    Ok(())
}
