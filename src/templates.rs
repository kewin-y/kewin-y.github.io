use crate::frontmatter::Frontmatter;
use maud::{DOCTYPE, Markup, html};

pub fn page(page_title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta name="viewport" content="width=device-width";
            meta charset="utf-8";
            link rel="stylesheet" href="index.css";
            title { (page_title) }
        }
        body {
            (body)
        }
    }
}

pub fn blog(frontmatter: &Frontmatter, body: Markup) -> Markup {
    page(
        &frontmatter.title,
        html! {
            h1 {(&frontmatter.title)}
            div {
                span { (&frontmatter.description)}
                span { (&frontmatter.date) }
            }
            (body)
        },
    )
}
