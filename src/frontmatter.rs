use anyhow::{Context, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
}

pub fn parse_frontmatter(markdown: &str) -> anyhow::Result<Frontmatter> {
    if !markdown.starts_with("---\n") {
        bail!("Missing frontmatter block")
    }

    let mut frontmatter = String::new();
    let mut found_end = false;

    for line in markdown[4..].split_inclusive("\n") {
        if line == "---\n" {
            found_end = true;
            break;
        }
        frontmatter.push_str(line);
    }

    if !found_end {
        bail!("Could not find end of frontmatter block")
    }

    serde_yaml::from_str(&frontmatter).context("Failed to parse YAML frontmatter")
}
