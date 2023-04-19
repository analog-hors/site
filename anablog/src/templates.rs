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
                @match page_title {
                    Some(page_title) => title { (page_title) },
                    None => title { "Analog Hors - " (title) },
                }
                link href="../shared/base.css" rel="stylesheet";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=Fira+Sans:ital@0;1&family=JetBrains+Mono&display=swap" rel="stylesheet";
                link href="../shared/fontawesome/css/fontawesome.min.css" rel="stylesheet";
                link href="../shared/fontawesome/css/regular.min.css" rel="stylesheet";
                link href="../shared/favicon.svg" rel="icon";
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
                        br;
                        br;
                        p .colored-block {
                            em {
                                "Unless stated otherwise, content on this website is licensed with "
                                a href="https://creativecommons.org/licenses/by-nc/4.0/" {
                                    "CC BY-NC 4.0"
                                }
                                ". Source code is licensed under "
                                a href="https://spdx.org/licenses/MIT.html" {
                                    "the MIT license"
                                }
                                ". Cited materials belong to their respective owners."
                                br;
                                br;
                                "Like what I do? "
                                a href="https://ko-fi.com/analog_hors" {
                                    "Support me on Ko-fi!"
                                }
                            }
                        }
                        br;
                        br;
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
