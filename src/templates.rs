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

pub fn blog(frontmatter: &Frontmatter, content: Markup) -> Markup {
    page(
        &frontmatter.title,
        html! {
            div class="frontmatter" {
                div class="title"{
                    h1 {(&frontmatter.title)}
                    a href="/" { "Back to Index" }
                }
                p { (&frontmatter.date) }
                em class="description" { (&frontmatter.description)}

            }
            div { (content) }
        },
    )
}

pub fn index(title: &str, links: Markup) -> Markup {
    page(
        title,
        html! {
            div class="intro" {
                h1 { "Kevin Yu" }
                p {
                    "I am an undergraduate student studying math and computer science at the University of Toronto."
                }
                p {
                    "My interests lie in machine learning and graphics programming, and I am currently interning as a full-stack web developer at "
                    a href="https://www.dataonthespot.com" { "DOTS." }
                }
                p {
                    "In my free time, I mess around with aquariums and listen to music."
                }
            }
            div class="projects" {
                h1 { "Projects" }
                ul {
                    li {
                        a href="https://github.com/kewin-y/kewin-craft" {
                            "kewin-craft"
                        }
                        span { "Minecraft clone" }
                    }
                    li {
                        a href="https://github.com/kewin-y/unn" {
                            "unn"
                        }
                        span { "Neural networks lib" }
                    }
                }
            }
            div class="blog-links" {
                h1 { "Blogs" }
                (links)
            }
        },
    )
}
