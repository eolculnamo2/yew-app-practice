use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use yew_router::switch::{AllowMissing, Permissive};
use yew::virtual_dom::VNode;
use crate::pages::todo::TodoPage;
use crate::pages::home::Home;


pub struct RouterComponent {
    link: ComponentLink<Self>,
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/todo"]
    PageTodo,
    #[to = "/"]
    Index,
}

impl Component for RouterComponent {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
      html! {
          <Router<AppRoute, ()>
              render = Router::render(|switch: AppRoute| {
                  match switch {
                      AppRoute::Index => html!{<Home />},
                      AppRoute::PageTodo => html!{<TodoPage />},
                  }
              })
          />
      }
    }
}




