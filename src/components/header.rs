use yew::prelude::{Component, Html};

pub fn render_header<C>() -> Html<C>
where
  C: Component,
{
  html! {
    <nav>
      <h1>{ "Yew App" }</h1>
    </nav>
  }
}
