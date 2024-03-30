use leptos::*;
use leptos_meta::*;

#[component]
pub fn LayoutChoiceDogovor(children: Children) -> impl IntoView {
    view! { 
        <section class="flex flex-row justify-evenly mt-[25px]">
            {children()}
        </section> 
    }
}
