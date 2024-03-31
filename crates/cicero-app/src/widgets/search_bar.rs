use leptos::*;
use leptos_meta::*;

#[component]
pub fn SearchBar(filter: RwSignal<String>) -> impl IntoView {
    view! {
        <section id="search" class="w-full h-[73px] relative bg-[#EEEEEE]">
            <div class="justify-between items-center h-[37px] mt-[18px] mb-[18px] ml-[25px] mr-[25px] relative">
                <input
                    type="search"
                    class="w-full pl-[30px] h-full absolute outline-none bg-[#261201] bg-opacity-[81%] border-solid border-[3px] rounded-[10px] border-[#8C7456] placeholder-[#A1A1A1] text-[#A1A1A1] text-[16px] font-light pl"
                    name="search-text"
                    placeholder="Поиск договоров"
                    on:input=move |ev| { filter.set(event_target_value(&ev)) }
                    prop:value=filter
                />

                <i class="bx bx-search items-center pt-[8px] pl-[7px] text-[#8C7456] text-[25px] absolute"></i>
            </div>
        </section>
    }
}
