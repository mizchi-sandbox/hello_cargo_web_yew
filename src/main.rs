#[macro_use]
extern crate stdweb;
extern crate counter;
extern crate yew;

use counter::Model;
use stdweb::web::{document, INode, IParentNode};
use yew::prelude::*;

fn main() {
  yew::initialize();
  let body = document().query_selector("body").unwrap().unwrap();

  let root = document().create_element("div").unwrap();
  body.append_child(&root);

  App::<Model>::new().mount(root);
  yew::run_loop();

  js! {
    console.log("in js! macro");
  };
}
