use maud::{html, DOCTYPE, PreEscaped};

use crate::page::PageMetadata;
use crate::page_entry::PageEntry;

pub fn base_page(content: PreEscaped<String>, title: &str, page_title: Option<&str>) -> PreEscaped<String> {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Analog Hors - " (page_title.unwrap_or(title)) }
                link href="../shared/base.css" rel="stylesheet";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=Fira+Sans:ital@0;1&family=JetBrains+Mono&display=swap" rel="stylesheet";
                link href="../shared/fontawesome/css/fontawesome.min.css" rel="stylesheet";
                link href="../shared/fontawesome/css/regular.min.css" rel="stylesheet";
                link href="./favicon.svg" rel="icon";
            }
            body {
                header {
                    code {
                        a href="../home/" { "analog-hors" }
                        (PreEscaped("&nbsp;-&nbsp;"))
                        a href="../projects/" { "projects" }
                        (PreEscaped("&nbsp;-&nbsp;"))
                        a href="../writing" { "writing" }
                    }
                }
                div id="content-container" {
                    div id="content" {
                        h1 { (title) }
                        (content)
                    }
                }
            }
        }
    }
}

pub fn content_page(content: PreEscaped<String>, meta: &PageMetadata) -> PreEscaped<String> {
    let content = html! {
        @if let Some(post) = &meta.post {
            p {
                i class="fa-regular fa-user" {} " " (post.author)
                (PreEscaped("&nbsp;&nbsp;&nbsp;"))
                i class="fa-regular fa-clock" {} " " (post.date)
            }
        }
        (content)
    };
    let title = &meta.title;
    let page_title = meta.page_title.as_deref();
    base_page(content, title, page_title)
}

pub fn writing_index_page<'m>(page_entries: &[PageEntry]) -> PreEscaped<String> {
    let mut writings = page_entries.iter()
        .filter_map(|entry| {
            let page = entry.page.as_ref()?;
            let post = page.meta.post.as_ref()?;
            Some((entry, page, post))
        })
        .collect::<Vec<_>>();
    writings.sort_by_key(|(_, page, post)| (std::cmp::Reverse(&post.date), &page.meta.title));

    let content = html! {
        @for (entry, page, post) in writings {
            h2 {
                a href={ "../" (entry.name) "/" } {
                    (page.meta.title)
                }
            }
            p {
                i class="fa-regular fa-user" {} " " (post.author)
                (PreEscaped("&nbsp;&nbsp;&nbsp;"))
                i class="fa-regular fa-clock" {} " " (post.date)
            }
            p { (post.desc) }
        }
    };
    base_page(content, "Stuff I've written", None)
}
