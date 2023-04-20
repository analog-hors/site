use serde::Deserialize;
use chrono::NaiveDate;
use comrak::{ComrakOptions, ComrakPlugins};
use comrak::plugins::syntect::SyntectAdapter;
use maud::PreEscaped;

#[derive(Debug, Deserialize)]
pub struct PageMetadata {
    pub title: String,
    pub page_title: Option<String>,
    pub post: Option<PostMetadata>
}

#[derive(Debug, Deserialize)]
pub struct PostMetadata {
    pub author: String,
    pub date: NaiveDate,
    pub desc: String,
}

#[derive(Debug)]
pub struct Page {
    pub meta: PageMetadata,
    pub content: String
}

impl Page {
    pub fn new(page: &str) -> Option<Self> {
        let (meta, content) = page.split_once("\n---")?;
        let meta = serde_yaml::from_str(&meta).ok()?;
        let content = content.to_owned();
        Some(Self { meta, content })
    }

    pub fn to_html(&self) -> String {
        let content = render_md(&self.content);
        crate::templates::content_page(content, &self.meta).into_string()
    }
}

fn render_md(md: &str) -> PreEscaped<String> {
    let highlighter = SyntectAdapter::new("base16-ocean.dark");
    let mut options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();
    options.render.unsafe_ = true;
    plugins.render.codefence_syntax_highlighter = Some(&highlighter);
    PreEscaped(comrak::markdown_to_html_with_plugins(md, &options, &plugins))
}
