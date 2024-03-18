use yew::prelude::*;

use crate::Model;


#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_state(|| Model { value: 0 });

    let increment = {
        let counter = counter.clone();
        move |_| {
            counter.set(Model {
                value: counter.value + 1,
            });
        }
    };

    let decrement = {
        let counter = counter.clone();
        move |_| {
            counter.set(Model {
                value: counter.value - 1,
            })
        }
    };

    html! {
        <main>
            <button onclick={increment}>{ "Increment"}</button>
            <button onclick={decrement}>{ "Decrement"}</button>
            <p> {counter.value} </p>
        </main>
    }
}