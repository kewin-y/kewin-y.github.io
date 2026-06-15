use crate::frontmatter::Frontmatter;
use crate::templates;
use maud::{PreEscaped, html};

pub struct Blog {
    pub frontmatter: Frontmatter,
    pub href: String,
}

// This functions takes the entire markdown string including
// frontmatter. This is because pulldown_cmark skips
// yaml metadata.
pub fn build_blog(frontmatter: &Frontmatter, markdown: &str) -> PreEscaped<String> {
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);
    let safe_body = ammonia::clean(&body);
    let page = templates::blog(frontmatter, PreEscaped(safe_body));
    page
}

pub fn build_index(title: &str, blogs: Vec<Blog>) -> PreEscaped<String> {
    templates::index(
        title,
        html! {
            ul {
                @for blog in &blogs {
                    @let date = blog.frontmatter.date.format("%Y-%m-%d").to_string();
                    li {
                        a href=(&blog.href) {
                            (blog.frontmatter.title)
                        }
                        span { (date) }
                    }
                }
            }
        },
    )
}
