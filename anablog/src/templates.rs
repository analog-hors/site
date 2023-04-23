use maud::{html, PreEscaped, DOCTYPE};

use crate::page::{BasicMeta, PostMeta};

const ANALYTICS_SCRIPT: PreEscaped<&str> = PreEscaped(include_str!("analytics.html"));

fn styles() -> PreEscaped<String> {
    html! {
        link href="../shared/base.css" rel="stylesheet";
        link rel="preconnect" href="https://fonts.googleapis.com";
        link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
        link href="https://fonts.googleapis.com/css2?family=Fira+Sans:ital@0;1&family=JetBrains+Mono&display=swap" rel="stylesheet";
        link href="../shared/fontawesome/css/fontawesome.min.css" rel="stylesheet";
        link href="../shared/fontawesome/css/regular.min.css" rel="stylesheet";
    }
}

fn header() -> PreEscaped<String> {
    html! {
        header {
            code {
                a href="../home/" { "analog-hors" }
                (PreEscaped("&nbsp;-&nbsp;"))
                a href="../projects/" { "projects" }
                (PreEscaped("&nbsp;-&nbsp;"))
                a href="../writing" { "writing" }
            }
        }
    }
}

fn end_block() -> PreEscaped<String> {
    html! {
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
    }
}

pub fn base(title: &str, is_home: bool, content: &PreEscaped<String>, metas: &PreEscaped<String>) -> PreEscaped<String> {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                // (metas)
                @match is_home {
                    true => title { "Analog Hors" },
                    false => title { "Analog Hors - " (title) },
                }
                (styles())
                link href="../shared/favicon.svg" rel="icon";
            }
            body {
                (header())
                div id="content-container" {
                    div id="content" {
                        h1 { (title) }
                        (content)
                        br;
                        br;
                        (end_block())
                        br;
                        br;
                    }
                }
                (ANALYTICS_SCRIPT)
            }
        }
    }
}

pub fn basic(meta: &BasicMeta, name: &str, content: &PreEscaped<String>) -> PreEscaped<String> {
    base(
        &meta.title,
        meta.is_home,
        content,
        &opengraph_metas(
            name,
            &meta.title,
            &meta.desc,
            "website"
        )
    )
}

pub fn post(meta: &PostMeta, name: &str, content: &PreEscaped<String>) -> PreEscaped<String> {
    let content = html! {
        p {
            i class="fa-regular fa-user" {} " " (meta.author)
            (PreEscaped("&nbsp;&nbsp;&nbsp;"))
            i class="fa-regular fa-clock" {} " " (meta.date)
        }
        (content)
    };
    base(
        &meta.title,
        false,
        &content,
        &opengraph_metas(
            name,
            &meta.title,
            &meta.desc,
            "article"
        )
    )
}

pub fn opengraph_metas(name: &str, title: &str, desc: &str, kind: &str) -> PreEscaped<String> {
    html! {
        meta property="og:type" content=(kind);
        meta property="og:title" content=(title);
        meta property="og:description" content=(desc);
        meta property="og:url" content={"https://analog-hors.github.io/" (name) "/"};
        meta property="og:site_name" content="Analog Hors";
        meta property="og:image" content="https://analog-hors.github.io/shared/favicon.svg";
        meta property="og:locale" content="en-US";
    }
}
