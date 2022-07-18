use yew::prelude::*;
use yewprint::{Button, IconName};

pub struct App {
    dark_theme: bool,
}

pub enum Msg {
    ToggleLight,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            dark_theme: web_sys::window()
                .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .map(|x| x.matches())
                .unwrap_or(true),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleLight => {
                self.dark_theme ^= true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!(self.dark_theme.then(|| "bp3-dark"))}>
                <Button
                    onclick={ctx.link().callback(|_| Msg::ToggleLight)}
                    icon={IconName::Flash}
                >
                    {"Toggle light"}
                </Button>
            </div>
        }
    }
}
