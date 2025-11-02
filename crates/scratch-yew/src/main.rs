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
    #[display("A2a")]
    A2a,
    #[display("A3a")]
    A3a,
    #[display("A4")]
    A4,
}

impl SupportedExercise {
    const ACTIVE: [Self; 5] = [Self::A1a, Self::A1b, Self::A2a, Self::A3a, Self::A4];

    pub fn get_runner(&self) -> std::rc::Rc<dyn testdata::ExerciseTest> {
        use std::rc::Rc;
        match self {
            SupportedExercise::A1a => Rc::new(testdata::A1a),
            SupportedExercise::A1b => Rc::new(testdata::A1b),
            SupportedExercise::A2a => Rc::new(testdata::A2a),
            SupportedExercise::A3a => Rc::new(testdata::A3a),
            SupportedExercise::A4 => Rc::new(testdata::A4),
        }
    }
}

impl std::str::FromStr for SupportedExercise {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "1a" | "a1a" => Self::A1a,
            "1b" | "a1b" => Self::A1b,
            "2a" | "a2a" => Self::A2a,
            "3a" | "a3a" => Self::A3a,
            "4" | "4a" | "a4a" | "a4" => Self::A4,
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
