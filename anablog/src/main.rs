mod page;
mod page_entry;
mod templates;

use page_entry::PageEntry;

fn main() -> std::io::Result<()> {
    let entries = page_entry::collect_page_entries()?;
    update_entries(&entries)?;
    build_writing_index_page(&entries)?;
    eprintln!("Done.");
    Ok(())
}

fn update_entries(entries: &[PageEntry]) -> std::io::Result<()> {
    eprintln!("Updating pages...");
    for entry in entries {
        if entry.needs_update {
            if let Some(page) = &entry.page {
                let html = page.to_html();
                std::fs::write(&entry.html_path, html.as_bytes())?;
                eprintln!("[UPDATED] {} - Updated.", entry.name);
            } else {
                eprintln!("[ERRORED] {} - Invalid page!", entry.name);
            }
        } else {
            eprintln!("[SKIPPED] {} - Up to date.", entry.name);
        }
    }
    Ok(())
}

fn build_writing_index_page(entries: &[PageEntry]) -> std::io::Result<()> {
    eprintln!("Building writing index page...");
    let html = crate::templates::writing_index_page(&entries).into_string();
    std::fs::write("writing/index.html", html.as_bytes())?;
    Ok(())
}
