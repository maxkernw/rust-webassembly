use yew::prelude::*;
use yew_router::prelude::*;

use crate::MyRoutes;

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = { move |_| navigator.push(&MyRoutes::Home) };
    html! {
        <div>
            <h1>{ "Secure"}</h1>
            <button {onclick}>{ "Go Home"}</button>
        </div>
    }
}