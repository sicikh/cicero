use leptos::*;
use leptos_meta::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-[#8C7456] h-[180px]">
            <nav id="nav-bar" class="md:flex md:justify-between md:items-center w-[92%] mx-auto h-[180px]">
                <div id="logo" class="md:flex items-start text-center">
                    <a href="/">
                        <p class="text-white text-[40px] mt-[60px] mb-[60px]">Cicero</p>
                    </a>
                </div>
                <div class="text-inter text-center pl-36 font-light" id="nav-bar-container-elem">
                    <ul class="md:flex md:items-center gap-[22px] md:justify-center">
                        <a href="/" style="text-center">
                            <li
                                id="nav-bar-main"
                                class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px] "
                            >
                                Главная
                            </li>
                        </a>
                        <a href="/scenario" style="text-center">
                            <li
                                id="nav-bar-kit"
                                class="text-white text-center items-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                            >
                                Конструктор
                            </li>
                        </a>
                        <a href="/contacts" style="text-center">
                            <li
                                id="nav-bar-contact"
                                class="text-white text-center text-[24px] w-[180px] h-[40px] hover:bg-[#BFA07A] bg-[length:180px_40px] rounded-[10px]"
                            >
                                Контакты
                            </li>
                        </a>
                        <div class="text-center gap-y-[15px] grid items-end">
                            <div class="text-center" id="button_entry">
                                <a href="Login">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Войти
                                    </button>
                                </a>
                            </div>
                            <div class="text-center" id="button_entry1">
                                <a href="Register">
                                    <button
                                        id="nav-bar-but2"
                                        class="bg-[#A69286] text-white px-5 py-2 rounded-full text-[20px] hover:bg-[#261201]"
                                    >
                                        Зарегистрироваться
                                    </button>
                                </a>
                            </div>
                        </div>
                    </ul>
                </div>
            </nav>
        </header>
    }
}
