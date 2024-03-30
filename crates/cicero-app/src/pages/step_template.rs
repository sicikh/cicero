use leptos::*;
use leptos_meta::*;

use crate::widgets::*;

#[component]
pub fn StepTemplate() -> impl IntoView {
    view! {
        <LayoutNav>
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
            <section></section>
            <section></section>
        </LayoutNav>
    }
}
