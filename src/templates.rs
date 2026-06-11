use maud::{DOCTYPE, Markup, html};

pub fn page(page_title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta name="viewport" content="width=device-width";
            meta charset="utf-8";
            title { (page_title) }
        }
        body {
            (body)
        }
    }
}
