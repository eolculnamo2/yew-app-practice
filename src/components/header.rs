use yew::prelude::*;
use yew_router::{prelude::*};
use crate::router::AppRoute;
pub struct Header {
    link: ComponentLink<Self>,
}

impl Component for Header {
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
            <div class="header-wrap">
                <div class="rusty-rob">{"RustyRob"}</div>
                <div><RouterAnchor<AppRoute> route=AppRoute::Index> {"Home"} </RouterAnchor<AppRoute>></div>
                <div><RouterAnchor<AppRoute> route=AppRoute::PageTodo> {"Todo App"} </RouterAnchor<AppRoute>></div>
                <div>{"App 2"}</div>
                <div>{"App 3"}</div>
            </div>
        }
    }
}
