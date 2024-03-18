use yew::prelude::*;
use yew_router::prelude::*;

pub struct Model {
    value: i64,
}
mod components {
    pub mod home;
    pub mod secure;
}

use components::{home::Home, secure::Secure};

#[derive(Clone, Routable, PartialEq)]
pub enum MyRoutes {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: MyRoutes) -> Html {
    match routes {
        MyRoutes::Home => html! { <Home/>},
        MyRoutes::Secure => html! {
            <Secure />
        },
        MyRoutes::NotFound => html! { <h1>{"404"}</h1>},
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <header>
                <Link<MyRoutes> to={MyRoutes::Home}>{ "Home" }</Link<MyRoutes>>
                <Link<MyRoutes> to={MyRoutes::Secure}>{ "Secure" }</Link<MyRoutes>>
                <Link<MyRoutes> to={MyRoutes::NotFound}>{ "404" }</Link<MyRoutes>>
            </header>
            <main>
                <Switch<MyRoutes> render={switch} />
            </main>
        </BrowserRouter>
    }
}
fn main() {
    yew::Renderer::<Main>::new().render();
}
