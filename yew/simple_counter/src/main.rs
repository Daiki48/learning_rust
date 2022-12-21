use yew::prelude::*;

#[function_component]
fn Counter() -> Html {
    let count = use_state(|| 0);
    let onclick = {
        let count = count.clone();
        move |_| {
            let value = *count + 1;
            count.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{*count}</p>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <Counter />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
