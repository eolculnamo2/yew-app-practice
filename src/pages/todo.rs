use yew::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub struct TodoPage {
    link: ComponentLink<Self>,
    items: Vec<TodoItem>,
    new_item: String,
    new_description: String,
}

#[derive(Serialize, Deserialize)]
struct TodoItem {
    name: String,
    description: String,
}

pub enum Msg {
    Add,
    UpdateNew(String),
    UpdateDescription(String),
    None,
}

impl Component for TodoPage {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            items: Vec::new(),
            new_item: String::new(),
            new_description: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let new_item = TodoItem {
                    name: self.new_item.clone(),
                    description: self.new_description.clone(),
                };
                self.items.push(new_item);
                self.new_item = String::new();
                self.new_description = String::new();
            }
            Msg::UpdateNew(value) => {
                self.new_item = value;
            }
            Msg::UpdateDescription(value) => {
                self.new_description = value;
            }
            Msg::None => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="page-wrap">
                <h1>{"Yew Todo List"}</h1>
                <h3>{"New item"}</h3>
                <p>{"Fill out name and description then press enter!"}</p>
                <input class="new-todo"
                    placeholder="Name"
                    value=&self.new_item
                    oninput=self.link.callback(|e: InputData| Msg::UpdateNew(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Add } else { Msg::None }
                    }) />
                <input class="new-todo"
                    placeholder="Description"
                    value=&self.new_description
                    oninput=self.link.callback(|e: InputData| Msg::UpdateDescription(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Add } else { Msg::None }
                    }) />
                    <ul class="todo-list">
                        { for self.items.iter()
                            .enumerate()
                            .map(|val| self.item_view(val)) }
                    </ul>
            </div>
        }
    }
}

impl TodoPage {
    fn item_view(&self, (idx, item): (usize, &TodoItem)) -> Html {
        html! {
            <li class="item">
                <div><b>{item.name.clone()}</b></div>
                <div><i>{item.description.clone()}</i></div>
            </li>
        }
    }
}