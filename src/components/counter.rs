use serde_json;
use std::result::Result::{Err, Ok};
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;

#[derive(Serialize, Deserialize)]
struct State {
  value: i64,
  adding_value_text: String,
}

pub struct Counter {
  console: ConsoleService,
  state: State,
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
      state: State {
        value: 0,
        adding_value_text: "".to_string(),
      },
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    let result = match msg {
      Msg::Increment => {
        self.state.value = self.state.value + 1;
        true
      }
      Msg::AddByAddingValue => {
        match &self.state.adding_value_text.parse::<i64>() {
          Ok(v) => {
            self.state.value = self.state.value + v;
            self.state.adding_value_text = "".to_string();
          }
          Err(_) => {
            self.console.log("Parse error");
          }
        };
        true
      }
      Msg::SetAddingValue(value) => {
        self.state.adding_value_text = value;
        true
      }
    };
    let serialized = serde_json::to_string(&self.state).unwrap();
    self.console.log(&serialized);
    result
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
            value=&self.state.adding_value_text,
            placeholder={"Input number"},
          />
        </div>

        <p> { &self.state.value }</p>
      </>
    }
  }
}
