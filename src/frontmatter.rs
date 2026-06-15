use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
}

pub fn parse_frontmatter(markdown: &str) -> Option<Frontmatter> {
    if markdown.starts_with("---\n") {
        let mut frontmatter = String::new();

        for line in markdown[4..].split_inclusive("\n") {
            if line == "---\n" {
                break;
            }
            frontmatter.push_str(line);
        }

        let frontmatter = serde_yaml::from_str(&frontmatter).ok();
        frontmatter
    } else {
        None
    }
}
