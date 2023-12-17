use pulldown_cmark::{Options, Parser};

fn get_parser(md: &str) -> Parser {
    Parser::new_ext(md, Options::all())
}

pub fn to_html(md: &str) -> String {
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, get_parser(md));
    html_output
}
