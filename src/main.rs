use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!{
        <p class= "text-3xl font-bold"> {"Hello World "}</p>
    }
}


fn main () {
    yew::Renderer::<App>::new().render();
}