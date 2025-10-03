use crate::components::{ExercisePage, NotFoundPage};

use std::str::FromStr;

use crate::MainRoute;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum SupportedExercises {
    A1a,
    A1b,
}

impl FromStr for SupportedExercises {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1a" => Self::A1a,
            "1b" => Self::A1b,
            _ => return Err(()),
        })
    }
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Exercise { id } => {
            if let Ok(ex) = SupportedExercises::from_str(&id) {
                html!(<ExercisePage exercise={ex}/>)
            } else {
                html!(<div><p>{"Not found: "}</p> <p>{id}</p></div>)
            }
        }
        MainRoute::NotFound => html!(<NotFoundPage/>),
    }
}
