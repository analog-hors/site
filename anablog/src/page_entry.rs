use std::fs::{File, read_dir, metadata};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::page::Page;

pub struct PageEntry {
    pub page: Option<Page>,
    pub name: String,
    pub html_path: PathBuf,
    pub needs_update: bool,
}

pub fn collect_page_entries() -> std::io::Result<Vec<PageEntry>> {
    let mut pages = Vec::new();
    for entry in read_dir(".")? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let md_path = path.join("index.md");
            let html_path = path.join("index.html");
            if let Ok(mut md_file) = File::open(md_path) {
                let mut md = String::new();
                md_file.read_to_string(&mut md)?;
                let page = Page::new(&md);

                let md_meta = md_file.metadata()?;
                let needs_update = match metadata(&html_path) {
                    Ok(html_meta) => md_meta.modified()? > html_meta.modified()?,
                    Err(_) => true,
                };

                pages.push(PageEntry {
                    page,
                    name,
                    html_path,
                    needs_update
                });
            }
        }
    }
    Ok(pages)
}
