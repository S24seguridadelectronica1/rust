use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hola Mundo con Rust y Yew!" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
