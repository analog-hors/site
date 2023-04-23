use maud::PreEscaped;
use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct BasicMeta {
    pub title: String,
    #[serde(default)]
    pub is_home: bool,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub author: String, 
    pub date: NaiveDate,
    pub desc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageMeta {
    Basic(BasicMeta),
    Post(PostMeta),
}

#[derive(Debug)]
pub struct Page {
    pub meta: PageMeta,
    pub content: PreEscaped<String>
}

impl Page {
    pub fn new(page: &str) -> Option<Self> {
        let (meta, content) = page.split_once("\n---")?;
        let meta = serde_yaml::from_str(meta).unwrap();//.ok()?;
        let content = crate::md::render_md(content);
        Some(Self { meta, content })
    }

    pub fn to_html(&self, name: &str) -> String {
        let html = match &self.meta {
            PageMeta::Basic(basic) => crate::templates::basic(basic, name, &self.content),
            PageMeta::Post(post) => crate::templates::post(post, name, &self.content),
        };
        html.into_string()
    }
}
