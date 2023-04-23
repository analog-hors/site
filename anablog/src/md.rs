use maud::PreEscaped;
use comrak::{ComrakOptions, ComrakPlugins};
use comrak::plugins::syntect::SyntectAdapter;

pub fn render_md(md: &str) -> PreEscaped<String> {
    let highlighter = SyntectAdapter::new("base16-ocean.dark");
    let mut options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();
    options.render.unsafe_ = true;
    plugins.render.codefence_syntax_highlighter = Some(&highlighter);
    PreEscaped(comrak::markdown_to_html_with_plugins(md, &options, &plugins))
}
