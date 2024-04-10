use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::A;

// view! {
//     // <A href=link.clone()>
//     <button
//         class="hover:bg-[#696764] rounded-[10px] h-[40px] w-[100px]"
//         on:click=move |mv| {
//             let button: web_sys::HtmlButtonElement = event_target(&mv);
//             let name = button.inner_text();
//             let step = steps
//                 .with(|inner| {
//                     inner
//                         .get(selected_step())
//                         .unwrap()
//                         .iter()
//                         .find(|&step| step.name == name)
//                         .cloned()
//                 });
//             selected_steps.set(step);
//         }
//     >

//         <div class="text-[24px] text-[#EEEEEE]">{name_step}</div>
//     </button>
// }

#[component]
pub fn AllSteps(
    #[prop(into)] steps_names: Signal<Vec<String>>,
    step_index: RwSignal<usize>,
) -> impl IntoView {
    view! {
        {move || {
            steps_names()
                .into_iter()
                .enumerate()
                .map(|(i, name)| {
                    view! {
                        <button
                            class="hover:bg-[#696764] rounded-[10px] h-[40px] w-[100px]"
                            on:click=move |mv| {
                                let button: web_sys::HtmlButtonElement = event_target(&mv);
                                let name = button.inner_text();
                                let selected = steps_names
                                    .with(|inner| {
                                        inner.iter().position(|step| *step == name).unwrap()
                                    });
                                step_index.set(selected);
                            }
                        >

                            <div class="text-[24px] text-[#EEEEEE]">{name.clone()}</div>
                        </button>
                    }
                })
                .collect_view()
        }}
    }
}
