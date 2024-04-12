use cicero_dsl::types::HtmlString;
use leptos::*;

#[component]
pub fn HtmlEnumRender(html_string: HtmlString) -> impl IntoView {
    view! { <label></label> }.inner_html(html_string)
}
