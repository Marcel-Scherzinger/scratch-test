use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    // #[at("/")]
    // Home,
    #[at("/exercise/:id")]
    Exercise { id: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}
