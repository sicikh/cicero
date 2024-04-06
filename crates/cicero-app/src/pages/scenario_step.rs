use std::collections::HashMap;
use std::hash::Hash;

use cicero_dsl::data;
use cicero_dsl::types::*;
use indexmap::IndexMap;

use leptos::*;
use leptos_meta::*;
use leptos_router::A;

use crate::widgets::*;

#[server(GetSteps, "/api", "Url", "get-steps")]
pub async fn get_steps() -> Result<IndexMap<String, Vec<ScenarioStep>>, ServerFnError> {
    let metas = vec![
        ScenarioStep {
            name: "Step1".to_string(),
            header: Some("step_1".to_string()),
            variables : vec![Var{
                name : "UltraDick".to_string(),
                comment: "UltraDick1".to_string(),
                ty : Entity{ty:EntityType::String,is_required:true}}],
        },
        ScenarioStep {
            name: "Step2".to_string(),
            header: Some("step_2".to_string()),
            variables : vec![Var{
                name : "UltraDick1".to_string(),
                comment: "UltraDick2".to_string(),
                ty : Entity{ty:EntityType::String,is_required:true}}],
        },
    ];
    let map = metas.into_iter().fold(IndexMap::new(), |mut map, meta| {
        map.entry(meta.name.clone())
            .and_modify(|entry: &mut Vec<ScenarioStep>| entry.push(meta.clone()))
            .or_insert(vec![meta]);
        map
    });

    Ok(map)
}


#[component]
pub fn ScenarioStep() -> impl IntoView {
    let selected_step: RwSignal<Option<ScenarioStep>> = create_rw_signal(None);
    let steps = Resource::once(get_steps);
    view! {
        <Layout>
            <section id="all_page" class="h-full w-full flex flex-row">
                <section
                    id="step"
                    class="pl-[15px] pr-[15px] pt-[15px] border-r-[3px] border-[#8C7456] space-y-[8px] flex flex-col h-full w-[150px] items-center bg-[#BFA07A]"
                >
                    {move || {
                        steps()
                            .map(move |vari| {
                                match vari {
                                    Ok(steps) => {
                                        let (steps, set_steps) = create_signal(steps);
                                        view! {
                                            <AllSteps steps selected_step=selected_step.read_only()/>
                                        }
                                    }
                                    Err(e) => view! { <p>"Error happened"</p> }.into_view(),
                                }
                            })
                    }}

                </section>
                <section
                    id="input_data"
                    class="flex flex-col flex-1 h-[100%] bg-[#EEEEEE] border-r-[7px] border-[#8c7456]"
                >
                    <section
                        id="warning"
                        class="h-[80px] bg-[#cccccc] border-b-[3px] py-[15px] px-[15px] items-start flex flex-col border-[#8C7456]"
                    >
                        <div class="text-[15px] text-[#8C7456] items-center">
                            Осторожно! Это описание шага, которого может не быть
                        </div>
                        <div class="text-[15px] text-[#8C7456] items-center">
                            Cмотрите ст.13 Конституции РФ
                        </div>
                    </section>
                    <section class="flex flex-col w-full h-full scrollbar-thumb-[#8C7456] scrollbar-thumb-h-[40px] scrollbar-track-[#eeeeee] hover:scrollbar-thumb-[#9c805d] active:scrollbar-thumb-[#9c805d]/50 scrollbar-h-[90%] scrollbar-w-[40px] overflow-y-scroll">

                        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                            <div class="flex flex-col gap-[10px] mb-[20px]">
                                <div class="font-bold">
                                    Введите данные покупателя:
                                </div>
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    Имя:
                                    <input
                                        class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
                                        type="text"
                                        placeholder="Имя"
                                        required
                                    />
                                </div>
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    Возраст:
                                    <input
                                        class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
                                        type="text"
                                        placeholder="Возраст"
                                        required
                                    />
                                </div>
                            </div>
                        </section>
                        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                            <div class="flex flex-row mb-[20px]">
                                <div class="font-bold pt-[10px]">
                                    Введите массив данных:
                                </div>
                                <div class="pl-[10px] flex flex-row p-[5px]">
                                    <button class="bg-[#eeeeee] w-[40px] h-[40px] border-[3px] border-[#8c7456] rounded-[50%]">
                                        <i class="bx bx-plus text-[#8c7456] pl-[1px] pt-[2px] text-[30px]"></i>
                                    </button>
                                </div>

                            </div>
                        </section>
                        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                            <div class="flex flex-col gap-[10px] mb-[20px]">
                                <div class="font-bold">
                                    Введите адрес покупателя:
                                </div>
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    <input
                                        class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
                                        type="text"
                                        placeholder="Адрес"
                                        required
                                    />
                                </div>
                            </div>
                        </section>
                    </section>
                </section>
                <section
                    id="watch_template"
                    class="flex-1 h-full flex flex-col bg-[#EEEEEE] border-l-[7px] border-[#8c7456]"
                >
                    <div class="w-full h-[45px] border-b-[3px] px-[15px] py-[7px] border-[#8c7456] items-center text-[16px] text-[#8c7456] ">
                        Предварительный просмотр документа
                    </div>
                    <section></section>
                </section>
            </section>
        </Layout>
    }
}
