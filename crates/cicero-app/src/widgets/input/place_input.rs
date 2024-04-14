//CSS FOR PLACE INPUT
/*<section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
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
</section> */

use leptos::*;
use regex::Regex;

#[component]
pub fn PlaceInput(placeholder: String, is_required: bool, recursion_level: usize) -> impl IntoView {
    let regex = Regex::new(r"<[^>]*>").unwrap();
    let placeholder = regex.replace_all(&placeholder, "").to_string();
    let placeholder = placeholder.trim();
    let placeholder = placeholder.trim_end_matches(':').to_string();

    //view! {
    //    {move || {
    //        view! {
    //            <section class="flex flex-col text-[#8c7456] w-full px-[15px]
    // pb-[15px]">                <div class="flex flex-col gap-[10px]
    // mb-[20px]">                    <div class="font-bold">Введите адрес
    // покупателя:</div>                    <div class="pl-[25px] flex
    // flex-row gap-x-[5px] items-center">                        <input
    //                            class="bg-[#eeeeee] appearance-none border-2
    // border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight
    // focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
    //                            type="text"
    //                            placeholder="Адрес"
    //                            required
    //                        />
    //                    </div>
    //                </div>
    //            </section>
    //        }
    //    }}
    //}
}
