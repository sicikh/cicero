use leptos::*;
use leptos_meta::*;

#[component]
pub fn RightSide() -> impl IntoView {
    view! {
        <div
            id="right_side"
            class="md:flex flex-col md:items-center border-l-[7px] border-[#8C7456] w-1/2 basis-1/2"
        >
            <section id="create_template" class="grid grid-cols-1 w-full h-[220px] bg-[#EEEEEE]">
                <div class="ml-[27px] mt-[12px] gap-[25px]">
                    <div class="text-[40px] font-light">Договор поставки</div>
                    <div class="text-[20px] font-light">Актуально на:</div>
                </div>
                <div class="items-center pl-[35px] pr-[35px]">
                    <button class="bg-[#BFA07A] w-full items-center rounded-[37px] text-[#EEEEEE] border-[#BFA07A] h-[60px] text-[32px] font-extralight">
                        <a href="StepTemplate">Создать договор</a>
                    </button>
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
                Мега анусятина
            </section>
        </div>
    }
}
