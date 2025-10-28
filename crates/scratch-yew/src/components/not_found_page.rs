use yew::prelude::*;

use super::supported_exercises::ListOfSupportedExercises;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html!(
        <div>
            <div>
                <div>
                <h1 style={"margin: auto; text-align: center;"}>{"This page does not exist"}</h1>
                <p style={"text-align: center"}>
                    {"Maybe you tried a deactivated/unsupported exercise or unknown page"}
                </p>
                <p style={"text-align: center"}>
                    {"Supported exercises:"}
                </p>
                <ListOfSupportedExercises/>
            </div>
            </div>
        </div>
    )
}
