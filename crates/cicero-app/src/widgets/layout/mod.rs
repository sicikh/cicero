use leptos::*;
use leptos_meta::*;

pub use self::header::Header;

mod header;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <Header/>
        {children()}
    }
}
