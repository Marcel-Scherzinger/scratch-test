use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html!(
        <div>
            <div style={"display: flex; height: 70vh;"}>
            <h1 style={"margin: auto;"}>{"This page does not exist"}</h1>
            </div>
        </div>
    )
}
