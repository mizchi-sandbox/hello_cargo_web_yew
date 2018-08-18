use std::result::Result::{Err, Ok};
// use yew::format::Json;
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;

// #[derive(Serialize, Deserialize)]
pub struct Counter {
  console: ConsoleService,
  value: i64,
  adding_value_text: String,
}

pub enum Msg {
  Increment,
  AddByAddingValue,
  SetAddingValue(String),
}

impl Component for Counter {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Counter {
      console: ConsoleService::new(),
      value: 0,
      adding_value_text: "".to_string(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Increment => {
        self.value = self.value + 1;
        true
      }
      Msg::AddByAddingValue => {
        match &self.adding_value_text.parse::<i64>() {
          Ok(v) => {
            self.value = self.value + v;
            self.adding_value_text = "".to_string();
          }
          Err(_) => {
            self.console.log("Parse error");
          }
        };
        true
      }
      Msg::SetAddingValue(value) => {
        self.adding_value_text = value;
        true
      }
    }
  }
}

impl Renderable<Counter> for Counter {
  fn view(&self) -> Html<Self> {
    html! {
      <>
        <div>
          <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
        </div>

        <div>
          <button onclick=|_| Msg::AddByAddingValue,>{ "Add" }</button>
          <input
            oninput=|e| Msg::SetAddingValue(e.value),
            value=&self.adding_value_text,
            placeholder={"Input number"},
          />
        </div>

        <p> { &self.value }</p>
      </>
    }
  }
}
