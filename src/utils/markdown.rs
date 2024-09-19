use markdown_it::{
  plugins::{cmark, extra},
  MarkdownIt,
};
use std::sync::LazyLock;

pub fn markdown_to_html(text: &str) -> String {
  static MARKDOWN_PARSER: LazyLock<MarkdownIt> = LazyLock::new(|| {
    let mut parser = MarkdownIt::new();

    cmark::add(&mut parser);
    extra::add(&mut parser);
    markdown_it_sup::add(&mut parser);
    markdown_it_sub::add(&mut parser);
    markdown_it_ruby::add(&mut parser);
    markdown_it_block_spoiler::add(&mut parser);
    markdown_it_footnote::add(&mut parser);

    parser
  });

  MARKDOWN_PARSER.parse(text).xrender()
}
