use std::fs::{read_dir, read_to_string};

use crate::page::{Page, PostMetadata};

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

    pub fn writings(&self) -> Vec<(&PageEntry, &Page, &PostMetadata)> {
        let mut writings = self.entries.iter()
            .filter_map(|entry| {
                let page = entry.page.as_ref()?;
                let post = page.meta.post.as_ref()?;
                Some((entry, page, post))
            })
            .collect::<Vec<_>>();
        writings.sort_by_key(|(_, page, post)| (std::cmp::Reverse(&post.date), &page.meta.title));
        writings
    }
}
