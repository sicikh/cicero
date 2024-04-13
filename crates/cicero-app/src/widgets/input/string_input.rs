use leptos::*;
use regex::Regex;

#[component]
pub fn StringInput(
    placeholder: String,
    is_required: bool,
    data: RwSignal<String>,
    recursion_level: usize,
) -> impl IntoView {
    let regex = Regex::new(r"<[^>]*>").unwrap();
    let placeholder = regex
        .replace_all(&placeholder, "")
        .trim_end_matches(':')
        .to_string();

    view! {
        {move || {
            view! {
                <input
                    class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
                    type="text"
                    placeholder=&placeholder
                    required=is_required
                    prop:value=data
                    on:input=move |ev| data.set(event_target_value(&ev))
                />
            }
        }}
    }
}
