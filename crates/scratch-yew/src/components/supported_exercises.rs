use yew::prelude::*;
use yew_router::prelude::*;

use crate::{MainRoute, SupportedExercise};

#[function_component(ListOfSupportedExercises)]
pub fn list_of_sup_exercises() -> Html {
    let navigator = use_navigator().unwrap();

    let exercises = SupportedExercise::ACTIVE.iter().map(|e| {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| {
            navigator.push(&MainRoute::Exercise { id: e.clone() });
        });

        html!(<button class={classes!("supported-exercises-button")} {onclick}>{e.to_string()}</button>)
    });

    html!(
        <div class={classes!("supported-exercises-boxes")}>
            { for exercises }
        </div>
    )
}
