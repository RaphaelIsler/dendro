use yew::prelude::*;


#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <h1>{"Hallo Dendro"}</h1>
            <crate::project::ProjectComp id={crate::project::Id::empty()} />
        </div>
    }
}
