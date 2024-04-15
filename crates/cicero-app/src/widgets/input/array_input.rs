//CSS FOR Array
/*<section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
    <div class="flex flex-row mb-[20px]">
        <div class="font-bold pt-[10px]">
            Введите массив данных:
        </div>
        <div class="pl-[10px] flex flex-row p-[5px]">
            <button class="bg-[#eeeeee] w-[40px] h-[40px] border-[3px] border-[#8c7456] rounded-[50%]">
                <i class="bx bx-plus text-[#8c7456] pl-[1px] pt-[2px] text-[30px]"></i>
            </button>
        </div>
    </div>
</section> */

use cicero_dsl::types::{self, EntityType};
use leptos::*;

use crate::shared::data;
use crate::widgets::{EntityInput, HtmlRender};

#[component]
pub fn ArrayInput(
    array: EntityType,
    is_required: bool,
    data: RwSignal<data::Array>,
    recursion_level: usize,
) -> impl IntoView {
    view! {
       //<section class="flex flex-col text-[#8c7456] w-full px-[15px] pb-[15px]">
       //    <div class="flex flex-row mb-[20px]">

       //        <div class="font-bold pt-[10px]">Введите массив данных:</div>
       //        <div class="pl-[10px] flex flex-row p-[5px]">
       //            <button
       //                class="bg-[#eeeeee] w-[40px] h-[40px] border-[3px] border-[#8c7456] rounded-[50%]"
       //                on:click = move || {
       //                    <EntityInput entity = array placeholder="".to_string data = data.clone() recursion_level = recursion_level+1/>
       //                }
       //            >
       //                <i class="bx bx-plus text-[#8c7456] pl-[1px] pt-[2px] text-[30px]"></i>
       //            </button>
       //        </div>
       //    </div>
       //</section>
    }
}
