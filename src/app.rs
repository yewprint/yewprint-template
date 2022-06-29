use yew::prelude::*;
use yewprint::{Button, IconName};

pub struct App {
    dark_theme: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ToggleLight,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            dark_theme: web_sys::window()
                .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .map(|x| x.matches())
                .unwrap_or(true),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLight => {
                self.dark_theme ^= true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!(self.dark_theme.then(|| "bp3-dark"))>
                <Button
                    onclick=self.link.callback(|_| Msg::ToggleLight)
                    icon=IconName::Flash
                >
                    {"Toggle light"}
                </Button>
            </div>
        }
    }
}
