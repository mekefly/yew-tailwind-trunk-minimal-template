use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class=" text-[#fff6d5] font-['sans-serif'] text-center">
            <img class="h-80" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1 class="">{ "Hello World!" }</h1>
            <span class="block -m-4">{ "from Yew with " }<i class="after:content-['❤️'] text-3xl" /></span>
        </main>
    }
}
