use crate::{
    handler::{log_error, render, HtmlView},
    view::frontend::index::Index,
    Result,
};

pub async fn index() -> Result<HtmlView> {
    let handler_name = "frontend/index/index";
    let tmpl = Index {};
    render(tmpl).map_err(log_error(handler_name))
}
