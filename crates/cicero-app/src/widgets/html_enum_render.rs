use cicero_dsl::types::HtmlString;
use leptos::*;

#[component]
pub fn HtmlEnumRender(html_string: HtmlString, name: String) -> impl IntoView {
    view! { <label for=name.clone()></label> }.inner_html(html_string)
}
