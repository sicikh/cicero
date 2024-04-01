use leptos::*;
use leptos_meta::*;

pub use self::header::Header;

mod header;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <Header/>
        <main class="flex-auto">{children()}</main>
    }
}
