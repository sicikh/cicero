use leptos::*;
use leptos_meta::*;

use super::navbar::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <NavBar/>
        {children()}
    }
}
