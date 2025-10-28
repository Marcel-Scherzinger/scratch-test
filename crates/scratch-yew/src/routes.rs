use crate::components::{ExercisePage, LandingPage, NotFoundPage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    // #[at("/")]
    // Home,
    #[at("/exercise/:id")]
    Exercise { id: crate::SupportedExercise },

    #[at("/")]
    Welcome,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Exercise { id } => {
            html!(<ExercisePage exercise={id}/>)
        }
        MainRoute::NotFound => html!(<NotFoundPage/>),
        MainRoute::Welcome => html!(<LandingPage/>),
    }
}
