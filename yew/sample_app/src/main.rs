use yew::prelude::*;

enum Msg {
    Increment,
    Decrement,
    Delete,
}

struct Model {
    value: i32,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true
            },
            Msg::Decrement => {
                self.value -= 1;
                true
            },
            Msg::Delete => {
                self.value = 0;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <h1 class="title">{"Counter - Yew"}</h1>
                <div class="btn">
                    <button onclick={link.callback(|_| Msg::Increment)}> {"+"} </button>
                    <button onclick={link.callback(|_| Msg::Decrement)}> {"-"} </button>
                    <button onclick={link.callback(|_| Msg::Delete)}> {"delete"} </button>
                </div>
                <span class="count">{&self.value}</span>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
