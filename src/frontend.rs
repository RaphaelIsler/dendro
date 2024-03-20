use yew::prelude::*;


#[function_component]
pub fn App() -> Html {
    html! {
        <div>
        {"Hallo Dendro"}
            <crate::project::ProjectComp id={crate::project::Id::empty()} />
        </div>
    }
}
