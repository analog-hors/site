use chrono::{DateTime, Utc};

use crate::site_pages::SitePages;

pub fn rss_feed(pages: &SitePages) -> String {
    let mut feed = r#"<?xml version="1.0" encoding="UTF-8" ?>"#.to_owned();

    macro_rules! xml {
        () => {};

        ($literal:literal $($tail:tt)*) => {
            feed.push_str($literal);
            xml!($($tail)*);
        };

        ([$code:expr] $($tail:tt)*) => {
            $code;
            xml!($($tail)*);
        };

        (($value:expr) $($tail:tt)*) => {
            feed.push_str($value);
            xml!($($tail)*);
        };

        ($tag:ident $($attr:ident=$attr_val:literal)* { $($inner:tt)* } $($tail:tt)*) => {
            feed.push('<');
            feed.push_str(stringify!($tag));
            $(
                feed.push(' ');
                feed.push_str(stringify!($attr));
                feed.push('=');
                feed.push_str(stringify!($attr_val));
            )*
            feed.push('>');
            xml!($($inner)*);
            feed.push_str("</");
            feed.push_str(stringify!($tag));
            feed.push('>');
            xml!($($tail)*);
        };
    }

    xml! {
        rss version="2.0" {
            channel {
                title { "Analog Hors - Writing" }
                description { "Analog's Blog" }
                link { "https://analog-hors.github.io/site/" }
                language { "en" }
        
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
                        }
                    }
                }]
            }
        }
    }

    feed
}
