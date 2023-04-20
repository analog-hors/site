use chrono::{DateTime, Utc};

use crate::site_pages::SitePages;

pub fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn rss_feed(pages: &SitePages) -> String {
    let mut feed = r#"<?xml version="1.0" encoding="UTF-8" ?>"#.to_owned();

    macro_rules! xml {
        () => {};

        ([$code:expr] $($tail:tt)*) => {
            $code;
            xml!($($tail)*);
        };

        ($literal:literal $($tail:tt)*) => {
            feed.push_str(&xml_escape($literal));
            xml!($($tail)*);
        };

        (($value:expr) $($tail:tt)*) => {
            feed.push_str(&xml_escape($value));
            xml!($($tail)*);
        };

        ($tag:ident$(:$tag2:ident)? $($attr:ident$(:$attr2:ident)?=$attr_val:literal)* { $($inner:tt)* } $($tail:tt)*) => {
            feed.push('<');
            feed.push_str(stringify!($tag));
            $(
                feed.push(':');
                feed.push_str(stringify!($tag2));
            )*
            $(
                feed.push(' ');
                feed.push_str(stringify!($attr));
                $(
                    feed.push(':');
                    feed.push_str(stringify!($attr2));
                )*
                feed.push('=');
                feed.push_str(stringify!($attr_val));
            )*
            feed.push('>');
            xml!($($inner)*);
            feed.push_str("</");
            feed.push_str(stringify!($tag));
            $(
                feed.push(':');
                feed.push_str(stringify!($tag2));
            )*
            feed.push('>');
            xml!($($tail)*);
        };
    }

    xml! {
        rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" {
            channel {
                title { "Analog Hors - Writing" }
                description { "Analog's Blog" }
                link { "https://analog-hors.github.io/site/" }
                language { "en" }
                atom:link href="https://analog-hors.github.io/writing/feed.rss" rel="self" type="application/rss+xml" {}

                [for (entry, page, post) in pages.writings() {
                    let datetime = post.date.and_hms_opt(0, 0, 0).unwrap();
                    let datetime = DateTime::<Utc>::from_utc(datetime, Utc);
                    let datetime = datetime.to_rfc2822();
                    xml! {
                        item {
                            title { (&page.meta.title) }
                            description { (&post.desc) }
                            link { "https://analog-hors.github.io/site/" (&entry.name) "/" }
                            pubDate { (&datetime) }
                            guid { "https://analog-hors.github.io/site/" (&entry.name) "/" }
                        }
                    }
                }]
            }
        }
    }

    feed
}
