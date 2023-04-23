use std::fs::{read_dir, read_to_string};

use maud::{html, PreEscaped};

use crate::page::{Page, PageMeta, BasicMeta, PostMeta};

pub struct PageEntry {
    pub page: Option<Page>,
    pub name: String,
}

pub struct SitePages {
    pub entries: Vec<PageEntry>
}

impl SitePages {
    pub fn collect() -> std::io::Result<Self> {
        let mut pages = Vec::new();
        for entry in read_dir(".")? {
            let entry = entry?;
            if entry.metadata()?.is_dir() {
                let path = entry.path();
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                if let Ok(page) = read_to_string(path.join("index.md")) {
                    let page = Page::new(&page);
                    pages.push(PageEntry {
                        page,
                        name,
                    });
                }
            }
        }
        Ok(Self { entries: pages })
    }

    pub fn writings(&self) -> Vec<(&PageEntry, &Page, &PostMeta)> {
        let mut writings = self.entries.iter()
            .filter_map(|entry| {
                let page = entry.page.as_ref()?;
                let PageMeta::Post(post) = &page.meta else { return None };
                Some((entry, page, post))
            })
            .collect::<Vec<_>>();
        writings.sort_by_key(|(_, _, post)| (std::cmp::Reverse(&post.date), &post.title));
        writings
    }

    pub fn writing_index_page(&self) -> String {
        let content = html! {
            p {
                "Also check out my "
                a href="./feed.xml" {
                    "RSS feed"
                }
                " if you use that."
            }
            @for (entry, _, post) in self.writings() {
                h2 {
                    a href={ "../" (entry.name) "/" } {
                        (post.title)
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
        let html = crate::templates::basic(
            &BasicMeta {
                title: "Stuff I've written".to_owned(),
                is_home: false,
                desc: "Stuff I've written.".to_owned()
            },
            "writings",
            &content
        );
        html.into_string()
    }
}
