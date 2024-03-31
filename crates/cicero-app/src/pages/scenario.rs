use cicero_dsl::types::ScenarioMeta;
use leptos::*;
use leptos_meta::*;

use crate::widgets::*;

#[component]
pub fn Scenario() -> impl IntoView {
    view! {
        <Layout>
            <section id="all_page" class="h-full w-full flex flex-row">
                <section
                    id="step"
                    class="pl-[15px] pr-[15px] pt-[15px] border-r-[3px] border-[#8C7456] space-y-[8px] flex flex-col h-full w-[150px] items-center bg-[#BFA07A]"
                >
                    <button class="hover:bg-[#8C7456] rounded-[10px] h-[40px] w-[100px]">
                        <div class="text-[24px] text-[#EEEEEE]">Шаг 1</div>
                    </button>
                    <button class="hover:bg-[#8C7456] rounded-[10px] h-[40px] w-[100px]">
                        <div class="text-[24px] text-[#EEEEEE]">Шаг 2</div>
                    </button>
                    <button class="hover:bg-[#8C7456] rounded-[10px] h-[40px] w-[100px]">
                        <div class="text-[24px] text-[#EEEEEE]">Шаг 3</div>
                    </button>
                    <button class="hover:bg-[#8C7456] rounded-[10px] h-[40px] w-[100px]">
                        <div class="text-[24px] text-[#EEEEEE]">Шаг 4</div>
                    </button>
                    <button class="hover:bg-[#8C7456] rounded-[10px] h-[40px] w-[100px]">
                        <div class="text-[24px] text-[#EEEEEE]">Шаг 5</div>
                    </button>
                </section>
                <section
                    id="input_data"
                    class="w-1/2 h-full bg-[#EEEEEE] border-r-[7px] border-[#8c7456]"
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
                    <section class="flex flex-col w-full scrollbar-thumb-[#8C7456] scrollbar-thumb-h-[40px] scrollbar-track-[#eeeeee] hover:scrollbar-thumb-[#9c805d] active:scrollbar-thumb-[#9c805d]/50 scrollbar h-[91%] scrollbar-w-[40px] overflow-y-scroll">
                        <section class="flex flex-col text-[#8c7456] w-full h-full px-[15px] py-[30px]">
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
                    </section>
                </section>
                <section
                    id="watch_template"
                    class="w-1/2 h-full flex flex-col bg-[#EEEEEE] border-l-[7px] border-[#8c7456]"
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
