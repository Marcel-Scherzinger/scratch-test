use crate::components::{ExercisePage, LandingPage, NotFoundPage};
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(not(feature = "scratch-test-scope"))]
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
#[cfg(feature = "scratch-test-scope")]
#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    // #[at("/")]
    // Home,
    #[at("/scratch-test/exercise/:id")]
    Exercise { id: crate::SupportedExercise },

    #[at("/scratch-test")]
    Welcome,

    #[not_found]
    #[at("/scratch-test/404")]
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
