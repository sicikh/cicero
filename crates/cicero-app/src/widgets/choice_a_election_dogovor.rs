use leptos::*;
use leptos_meta::*;

#[component]
pub fn ChoiceAElectionDogovor() -> impl IntoView {
    view! {
        <div
            id="choice_a_election_dogovor"
            class="flex flex-col w-[280px] bg-[#8C7456] items-start text-start rounded-[10px] p-[10px] gap-[15px]"
        >
            <div class="min-h-[35px] w-full text-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Купля-продажи и мена</a>
                </button>
            </div>
            <div class="min-h-[35px] w-full text-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Договоры в сфере корпоративного плана</a>
                </button>
            </div>
            <div class="min-h-[35px] w-full text-start">
                <button class="text-[#EEEEEE] text-[20px] font-light rounded-[10px] min-h-[35px] w-full hover:bg-[#544027]">
                    <a href="#">Другие договора</a>
                </button>
            </div>
        </div>
    }
}
