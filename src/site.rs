use maud::{PreEscaped, html};

use crate::templates;

pub struct BlogLink {
    pub title: String,
    pub date: String,
    pub href: String,
}

// This functions takes the entire markdown string including
// frontmatter. This is because pulldown_cmark skips
// yaml metadata.
pub fn build_blog(title: &str, markdown: &str) -> PreEscaped<String> {
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);
    let safe_body = ammonia::clean(&body);
    let page = templates::page(title, PreEscaped(safe_body));
    page
}

pub fn build_index(title: &str, blog_links: Vec<BlogLink>) -> PreEscaped<String> {
    templates::page(
        title,
        html! {
            ul {
                @for link in &blog_links {
                    li {
                        a href=(link.href) {
                            span { (link.title) }
                            span { (link.date) }
                        }
                    }
                }
            }
        },
    )
}
