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
                <section id="input_data" class="w-1/2 h-full bg-[#EEEEEE]">
                    <section
                        id="warning"
                        class="h-[70px] bg-[#cccccc] border-b-[3px] py-[10px] px-[15px] items-start flex flex-col border-[#8C7456]"
                    >
                        <div class="text-[15px] text-[#8C7456] items-center">
                            Осторожно! Это описание шага, которого может не быть
                        </div>
                        <div class="text-[15px] text-[#8C7456] items-center">
                            Cмотрите ст.13 Конституции РФ
                        </div>
                    </section>
                    <section class="flex flex-col scrollbar scrollbar-track-[#cccccc] scrollbar-thumb-[#8C7456] h-full overflow-y-scroll w-full ">
                    </section>
                </section>
                <section></section>
            </section>
        </Layout>
    }
}
