use yew::prelude::*;
use yewprint::{Button, Dark, Icon};

pub struct App;

pub enum Msg {
    ToggleLight,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleLight => {
                Dark.toggle();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("root", Dark.classes())}>
                <div class={classes!("app")}>
                    <Button
                        onclick={ctx.link().callback(|_| Msg::ToggleLight)}
                        icon={Icon::Flash}
                    >
                        {"Toggle light"}
                    </Button>
                </div>
            </div>
        }
    }
}
