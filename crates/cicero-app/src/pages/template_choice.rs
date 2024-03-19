use leptos::*;
use leptos_meta::*;

use crate::widgets::*;

#[component]
pub fn TemplateChoice() -> impl IntoView {
    //let (scenario, )= create_signal(vec![ScenarioMeta::new(
    //    52,
    //    "dogovor".to_string(),
    //    "This is description".to_string(),
    //    "12.12.23".to_string(),
    //    "09.02.24".to_string(),
    //    "Gleb".to_string(),
    //)])
    //let (scenario, dildo) = create_signal(vec![new(){
    //    25;
    //   "dog".to_string();
    //}]);
    // Creates a reactive value to update the button
    view! {
        <Layout>
            <div id="main_body" class="md: flex flex-row">
                <div id="left_side" class="md:flex border-r-[7px] border-[#8C7456] w-1/2 basis-1/2">
                    <section id="search" class="w-full h-[73px] relative bg-[#EEEEEE]">
                        <div class="justify-between items-center h-[37px] mt-[18px] mb-[18px] ml-[25px] mr-[25px] relative">
                            <input
                                type="search"
                                class="w-full pl-[30px] h-full absolute outline-none bg-[#261201] bg-opacity-[81%] border-solid border-[3px] rounded-[10px] border-[#8C7456] placeholder-[#A1A1A1] text-[#A1A1A1] text-[16px] font-light pl"
                                name="search-text"
                                placeholder="Поиск документов"
                            />
                            <i class="bx bx-search items-center pt-[8px] pl-[7px] text-[#8C7456] text-[25px] absolute"></i>
                        </div>
                    </section>
                    <section id="find_doc"></section>
                </div>
                // посмотрим     // <div id="balka_ebanay" class="md:flex w-[14px] h-full bg-[#8C7456]"></div>
                <div
                    id="right_side"
                    class="md:flex md:items-center border-l-[7px] border-[#8C7456] w-1/2 basis-1/2"
                >
                    <section
                        id="create_template"
                        class="grid grid-cols-1 w-full h-[220px] bg-[#EEEEEE]"
                    >
                        <div class="ml-[27px] mt-[12px] gap-[25px]">
                            <div class="text-[40px] font-light">
                                Договор поставки
                            </div>
                            <div class="text-[20px] font-light">Актуально на:</div>
                        </div>
                        <button class="bg-[#BFA07A] items-center border-[37px] border-[#BFA07A] h-[60px] text-[32px] font-extralight">
                            Создать договор
                        </button>
                    </section>
                    <section id="find_doc"></section>
                </div>
            </div>
        </Layout>
    }
}
