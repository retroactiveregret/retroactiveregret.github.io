use dioxus::prelude::*;

#[component]
pub fn Markdown(
    text: String,
    #[props(default)] sanitize: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let parser_options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&text, parser_options);

    let mut unsafe_html_output = String::new();
    pulldown_cmark::html::push_html(&mut unsafe_html_output, parser);

    let safe_html = if sanitize {
        ammonia::clean(&*unsafe_html_output)
    } else {
        unsafe_html_output
    };

    rsx! {
        article { dangerous_inner_html: safe_html, ..attributes }
    }
}
