// FORMAT CSS FOR PHONE
/*<section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
    <form action="">
        <div class="flex flex-row gap-[10px] mb-[20px]">
            <div class="font-bold">Номер телефона:</div>
            <input
                class="bg-[#eeeeee] appearance-none border-2 w-[200px] border-gray-200 rounded py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-[#eeeeee] focus:border-[#8c7456]"
                type="tel"
                pattern="+[7-7]{1}[0-9]{10}"
                placeholder="+7925***5268"
                required
            />
        </div>
        <div class="flex gap-[10px] mb-[20px]">
            <input type="submit"/>
        </div>
    </form>
</section> */

use leptos::*;
use regex::Regex;

#[component]
pub fn PhoneNumberInput(
    placeholder: String,
    is_required: bool,
    recursion_level: usize,
) -> impl IntoView {
    let regex = Regex::new(r"<[^>]*>").unwrap();
    let placeholder = regex.replace_all(&placeholder, "").to_string();
    let placeholder = placeholder.trim();
    let placeholder = placeholder.trim_end_matches(':').to_string();

    //   view! {
    //       {{
    //           view! {
    //               <section class="flex flex-col text-[#8c7456] w-full
    // px-[15px] pb-[15px]">                   <form action="">
    //                       <div class="flex flex-row gap-[10px] mb-[20px]">
    //                           <div class="font-bold">Номер телефона:</div>
    //                           <input
    //                               class="bg-[#eeeeee] appearance-none
    // border-2 w-[200px] border-gray-200 rounded py-2 px-4 text-gray-700
    // leading-tight focus:outline-none focus:bg-[#eeeeee]
    // focus:border-[#8c7456]"                               type="tel"
    //                               pattern="+[7-7]{1}[0-9]{10}"
    //                               placeholder="+7925***5268"
    //                               required
    //                           />
    //                       </div>
    //                       <div class="flex gap-[10px] mb-[20px]">
    //                           <input type="submit"/>
    //                       </div>
    //                   </form>
    //               </section>
    //           }
    //       }}
    //   }
}
