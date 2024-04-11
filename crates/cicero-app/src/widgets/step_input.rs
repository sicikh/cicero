use cicero_dsl::types::*;
use indexmap::IndexMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::{Form, A};

use crate::widgets::HtmlRender;

#[component]
pub fn StepInput(
    #[prop(into)] current_step: Signal<ScenarioStep>,
    step_index: RwSignal<usize>,
) -> impl IntoView {
    view! {
        <section id="input_data" class="flex flex-col flex-1 h-[100%] bg-[#EEEEEE] border-r-[7px] border-[#8c7456]">
            {move || {
                view! {
                    {move || {
                        current_step
                            .with(|step| step.header.clone())
                            .map(|html_string| {
                                view! {
                                    <section
                                        id="warning"
                                        class="min-h-[80px] bg-[#cccccc] border-b-[3px] py-[15px] px-[15px] pb-[25px] gap-[2px] items-start flex flex-col border-[#8C7456]"
                                    >
                                        <div class="text-[15px] text-[#8C7456] items-center">
                                            <HtmlRender html_string/>
                                        </div>
                                    </section>
                                }
                                    .into_view()
                            })
                    }}

                    <section class="flex flex-col py-[10px] w-full h-full scrollbar-thumb-[#8C7456] scrollbar-thumb-h-[40px] scrollbar-track-[#eeeeee] hover:scrollbar-thumb-[#9c805d] active:scrollbar-thumb-[#9c805d]/50 scrollbar-h-[90%] scrollbar-w-[40px] overflow-y-scroll">
                        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                            <div class="flex flex-col gap-[10px] mb-[20px]">
                                <div class="font-bold">Введите данные покупателя:</div>
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
                                <div class="font-bold pt-[10px]">Введите массив данных:</div>
                                <div class="pl-[10px] flex flex-row p-[5px]">
                                    <button class="bg-[#eeeeee] w-[40px] h-[40px] border-[3px] border-[#8c7456] rounded-[50%]">
                                        <i class="bx bx-plus text-[#8c7456] pl-[1px] pt-[2px] text-[30px]"></i>
                                    </button>
                                </div>
                            </div>
                        </section>
                        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                            <div class="flex flex-col gap-[10px] mb-[20px]">
                                <div class="font-bold">Введите адрес покупателя:</div>
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
                        <section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
                            <div class="flex flex-col gap-[10px] mb-[20px]">
                                <div class="font-bold">Вы пидорас?:</div>
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    <p>
                                        <input class="mr-[5px]" type="radio" id="var1" name="take"/>

                                        <label for="var1">Да</label>
                                        <div class="pl-[18px] flex flex-row gap-x-[5px] items-center">
                                            Возраст :
                                            <input
                                                class="bg-[#eeeeee] appearance-none border-2 border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
                                                type="text"
                                                placeholder="Возраст"
                                                required
                                            />
                                        </div>

                                    </p>
                                </div>
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    <p>
                                        <input class="mr-[5px]" type="radio" id="var2" name="take"/>
                                        <label for="var2">Нет</label>

                                    </p>
                                </div>
                                <div class="pl-[25px] flex flex-row gap-x-[5px] items-center">
                                    <p>
                                        <input class="mr-[5px]" type="radio" id="var3" name="take"/>
                                        <label for="var3">Я 100% педик</label>
                                    </p>
                                </div>
                            </div>
                        </section>
                    </section>
                }
            }}

        </section>
    }
}
