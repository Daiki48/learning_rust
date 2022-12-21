use yew::prelude::*;

#[function_component]
fn HelloWorld() -> Html {
    html! {
        "Hello World!!!!"
    }
}

#[function_component]
fn App() -> Html {
    html! { <HelloWorld /> }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
