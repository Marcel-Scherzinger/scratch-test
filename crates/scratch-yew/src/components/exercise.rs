use std::rc::Rc;

use super::category_view::CategoryView;
use super::file_upload::{FileDetails, FileUpload};
use testdata::ExerciseTest;
use testreports::TestReport;
use yew::prelude::*;

use crate::{SupportedExercises, components::WarningsBox};

#[derive(Properties, PartialEq)]
pub struct ExerciseProps {
    pub exercise: SupportedExercises,
}

#[derive(Properties, PartialEq)]
pub struct ReportOverviewProps {
    pub report: Rc<TestReport>,
}

#[function_component(ReportOverview)]
pub fn report_overview(ReportOverviewProps { report }: &ReportOverviewProps) -> Html {
    let error_count = report.overall_failures().count();
    let success_count = report.overall_successes().count();
    let success_percent =
        ((success_count * 100) as f64 / (success_count + error_count) as f64).floor() as u64;

    if error_count == 0 {
        html!(
            <div class={classes!("report-overview-box", "report-overview-all-tests-succeeded")}>
                {"All tests succeeded"}
            </div>
        )
    } else {
        html!(
            <div class={classes!("report-overview-box", "report-overview-some-tests-failed")}>
                {format!("{success_percent}% succeeded")}
            </div>
        )
    }
}

#[function_component(ExercisePage)]
pub fn exercise(ExerciseProps { exercise }: &ExerciseProps) -> Html {
    let files_handle: UseStateHandle<Option<FileDetails>> = use_state(move || None);
    let file_selected = Callback::from({
        let files_handle = files_handle.clone();
        move |file| files_handle.set(Some(file))
    });

    if let Some(file) = files_handle.as_ref() {
        let file = file.clone();
        let name = file.name;
        let content = file.data;

        let mut handle = content.as_slice();

        let doc = match model::ProjectDoc::from_sb3_stream(&mut handle) {
            Ok(doc) => doc,
            Err(err) => {
                log::error!("invalid sb3 file (model failed): {err:?}");
                return html!(
                    <>
                        <h1>{ format!("{exercise:?}") }</h1>
                        <FileUpload {file_selected}/>
                        <h3>{"Invalid file"}</h3>
                        <div class={classes!("invalid-file-msg-wrapper", "invalid-file-model-failed")}>
                            <div class={classes!("invalid-file-msg-name")}>
                                <b>{format!("{name}")}</b>
                            </div>
                            <div class={classes!("invalid-file-msg-error")}>
                                <i>{format!("{err}")}</i>
                            </div>
                        </div>
                    </>
                );
            }
        };
        let builder = match interpreter::InterpreterBuilder::new(doc.clone()) {
            Ok(builder) => builder,
            Err(err) => {
                log::error!("invalid sb3 file (interpreter builder failed): {err:?}");
                return html!(
                    <>
                        <h1>{ format!("{exercise:?}") }</h1>
                        <FileUpload {file_selected}/>
                        <h3>{"Invalid file"}</h3>
                        <div class={classes!("invalid-file-msg-wrapper", "invalid-file-builder-failed")}>
                            <div class={classes!("invalid-file-msg-name")}>
                                <b>{format!("{name}")}</b>
                            </div>
                            <div class={classes!("invalid-file-msg-error")}>
                                <i>{format!("{err}")}</i>
                            </div>
                        </div>
                    </>
                );
            }
        };

        let report = std::rc::Rc::new(match exercise {
            SupportedExercises::A1a => testdata::A1a.run(&builder),
            SupportedExercises::A1b => testdata::A1b.run(&builder),
        });

        let global_messages = report.global_messages().cloned().collect::<Vec<_>>();

        let categories = report.categories().map(|c| {
            html!(
                <CategoryView category={c.clone()}/>
            )
        });

        return html!(
            <>
                <h1>{ format!("{exercise:?}") }</h1>
                <FileUpload {file_selected}/>
                <ReportOverview report={report.clone()}/>
                <WarningsBox<TestReport> messages={global_messages}/>
                <h2>{"Categories"}</h2>
                {for categories}
            </>
        );
    }

    html!(
        <>
            <h1>{ format!("{exercise:?}") }</h1>
            <FileUpload {file_selected}/>
        </>
    )
}
