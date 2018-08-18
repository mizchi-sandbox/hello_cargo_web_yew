#[macro_use]
extern crate yew;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

mod components;

use components::counter::Counter;
use components::header::render_header;

pub struct Model {}

pub enum Msg {}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {}
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
      <div>
        { render_header() }
        <Counter: />
      </div>
    }
  }
}
