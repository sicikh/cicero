use leptos::*;
use leptos_meta::*;

#[component]
pub fn ChoiceADogovor() -> impl IntoView {
    view! {
        <div
            id="choice_a_dogovor"
            class="flex flex-col items-start text-start w-[550px] bg-[#8C7456] rounded-[10px] p-[10px] gap-[15px]"
        >
            <div class="min-h-[35px] w-full text-left">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Договор поставки</a>
                </button>
            </div>
            <div class="min-h-[35px] w-full items-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Рамочный договор поставки</a>
                </button>
            </div>
            <div class="min-h-[35px] w-full items-start">
                <button class="items-start text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">
                        Договор купли-продажи транспортного средства
                    </a>
                </button>
            </div>
            <div class="min-h-[35px] w-full items-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">
                        Договор купли-продажи земельного участка
                    </a>
                </button>
            </div>
            <div class="min-h-[35px] w-full items-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">
                        Договор купли-продажи нежилого недвижимого участка
                    </a>
                </button>
            </div>
        </div>
    }
}
