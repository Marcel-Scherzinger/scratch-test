use yew::prelude::*;

use super::supported_exercises::ListOfSupportedExercises;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html!(
        <div>
            <div>
                <div>
                <h1 style={"margin: auto; text-align: center;"}>
                    {"Welcome!"}
                </h1>
                <p style={"text-align: center"}>
                    {"Select one of the supported exercises:"}
                </p>
                <ListOfSupportedExercises/>
            </div>
            </div>
        </div>
    )
}
