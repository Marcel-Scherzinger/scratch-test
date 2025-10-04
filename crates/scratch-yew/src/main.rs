mod components;
mod routes;

pub use routes::MainRoute;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, derive_more::Display)]
pub enum SupportedExercise {
    #[display("A1a")]
    A1a,
    #[display("A1b")]
    A1b,
}

impl SupportedExercise {
    pub fn get_runner(&self) -> std::rc::Rc<dyn testdata::ExerciseTest> {
        use std::rc::Rc;
        match self {
            SupportedExercise::A1a => Rc::new(testdata::A1a),
            SupportedExercise::A1b => Rc::new(testdata::A1b),
        }
    }
}

impl std::str::FromStr for SupportedExercise {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "1a" | "a1a" => Self::A1a,
            "1b" | "a1b" => Self::A1b,
            _ => return Err(()),
        })
    }
}

#[function_component(App)]
fn app() -> Html {
    use routes::switch_main;
    use yew_router::prelude::*;
    html!(
         <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
         </BrowserRouter>
    )
}

fn main() {
    // dotenvy::dotenv().ok();

    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
