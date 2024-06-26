use cicero_dsl::types::ScenarioMeta;
use leptos::*;
use leptos_meta::*;
use leptos_router::A;

#[component]
pub fn ScenarioDescription(selected_scenario: ReadSignal<Option<ScenarioMeta>>) -> impl IntoView {
    view! {
        <div id="right_side" class="flex flex-col flex-1 md:items-center border-l-[7px] border-[#8C7456]">

            {move || {
                match selected_scenario() {
                    None => {
                        view! {
                            <p>
                                "Для просмотра сценария выберите его в окне слева"
                            </p>
                        }
                            .into_view()
                    }
                    Some(scenario) => {
                        view! {
                            <section id="create_template" class="grid grid-cols-1 w-full h-[220px] bg-[#EEEEEE]">
                                <div class="ml-[27px] mt-[12px] gap-[25px]">
                                    <div class="text-[40px] font-light">{scenario.name}</div>
                                    <div class="text-[20px] font-light">Актуально на:</div>
                                </div>
                                <div class="items-center pl-[35px] pr-[35px]">
                                    <A href=format!("{}/0", scenario.id.to_string())>
                                        <button class="bg-[#BFA07A] w-full items-center rounded-[37px] text-[#EEEEEE] border-[#BFA07A] h-[60px] text-[32px] font-extralight">
                                            Создать договор
                                        </button>
                                    </A>
                                </div>
                            </section>
                            <section
                                id="choice_description_or_change"
                                class="flex flex-row w-full h-[40px] relative bg-[#EEEEEE]"
                            >

                                <button class="w-1/2 rounded-tr-[10px] border-t-[3px] border-r-[3px] border-b-[3px] border-[#8C7456] items-center text-center active:border-b-none active:text-[#BFA07A]">
                                    <div class="text-[#8C7456] hover:text-[#BFA07A] ">
                                        <a href="#">Описание</a>
                                    </div>
                                </button>
                                <button class="w-1/2 rounded-tl-[10px] border-t-[3px] border-l-[3px] border-b-[3px] border-[#8C7456] items-center text-center active:border-b-none active:text-[#BFA07A]">
                                    <div class="text-[#8C7456] hover:text-[#BFA07A]">
                                        <a href="#">Изменения в документе</a>
                                    </div>
                                </button>
                            </section>
                            <section id="description" class="mt-[15px] ml-[5px] mr-[5px] text-[#8C7456]">
                                {scenario.description}
                            </section>
                        }
                            .into_view()
                    }
                }
            }}

        </div>
    }
}
