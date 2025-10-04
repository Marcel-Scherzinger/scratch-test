use std::rc::Rc;

use testreports::TestReport;
use yew::prelude::*;

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
