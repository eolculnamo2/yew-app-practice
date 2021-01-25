use yew::prelude::*;

pub struct Home {
    link: ComponentLink<Self>,
}

impl Component for Home {
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
            <div class="page-wrap">
                <h1>{"My page created with Yew Rust :)"}</h1>
            </div>
        }
    }
}
