use yew::prelude::*;
use super::components::header::Header;
use crate::router::RouterComponent;

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Header />
                <RouterComponent />
            </div>
        }
    }
}
