mod page;
mod site_pages;
mod templates;
mod rss;

use site_pages::SitePages;

fn main() -> std::io::Result<()> {
    let pages = SitePages::collect()?;
    update_site_pages(&pages)?;
    build_writing_index_page(&pages)?;
    build_rss_feed(&pages)?;
    eprintln!("Done.");
    Ok(())
}

fn update_site_pages(pages: &SitePages) -> std::io::Result<()> {
    eprintln!("Updating pages...");
    for entry in &pages.entries {
        if let Some(page) = &entry.page {
            if entry.needs_update {
                let html = page.to_html();
                std::fs::write(&entry.html_path, html.as_bytes())?;
                eprintln!("[UPDATED] {} - Updated.", entry.name);
            } else {
                eprintln!("[SKIPPED] {} - Up to date.", entry.name);
            }
        } else {
            eprintln!("[ERRORED] {} - Invalid page!", entry.name);
        }
    }
    Ok(())
}

fn build_writing_index_page(pages: &SitePages) -> std::io::Result<()> {
    eprintln!("Building writing index page...");
    let html = crate::templates::writing_index_page(pages).into_string();
    std::fs::write("writing/index.html", html.as_bytes())?;
    Ok(())
}

fn build_rss_feed(pages: &SitePages) -> std::io::Result<()> {
    eprintln!("Building RSS feed...");
    let feed = crate::rss::rss_feed(&pages);
    std::fs::write("writing/feed.xml", feed.as_bytes())?;
    Ok(())
}
