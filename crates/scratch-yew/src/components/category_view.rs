use super::WarningsBox;
use testreports::{Category, TestCase};
use yew::prelude::*;

// TODO: WARNING: Messages of successful test are never displayed

const FIRST_N_FAILED_TESTS: usize = 3;

#[derive(Properties, PartialEq)]
pub struct CategoryViewProps {
    pub category: Category,
}

fn sinplu<'a, I>(singular: &'a str, plural: &'a str, amount: I) -> &'a str
where
    I: Copy + Into<usize>,
{
    if amount.into() == 1 { singular } else { plural }
}

#[derive(Properties, PartialEq)]
pub struct FailedTestCaseProps {
    case: TestCase,
}

#[function_component(FailedTestCase)]
pub fn failed_test_case(FailedTestCaseProps { case }: &FailedTestCaseProps) -> Html {
    let messages: Vec<_> = case.local_messages().iter().cloned().collect();
    let expected_output = if let Some(eo) = case.expected_output() {
        html!(
            <tr>
                <td>{"expected output: "}</td>
                <td>{format!("{eo:?}")}</td>
            </tr>
        )
    } else {
        html!()
    };

    let differing_lists = case.differing_list_values().map(|(name, diff)| {
        html!(<>
            <tr>
                <td>{"list."}{name}{".program"}</td>
                <td>{format!("{:?}", diff.program())}</td>
            </tr>
            <tr>
                <td>{"list."}{name}{".expected"}</td>
                <td>{format!("{:?}", diff.expected())}</td>
            </tr>
            </>)
    });

    let requested_randoms = case.out().requested_randoms();
    let requested_randoms = if requested_randoms.any_used() {
        html!(
            <tr>
                <td>{"requested randoms: "}</td>
                <td>{format!("{requested_randoms:?}")}</td>
            </tr>
        )
    } else {
        html!()
    };
    let abnormal_termination = if let Some(at) = case.program_error() {
        html!(
            <tr>
                <td>{"abnormal termination: "}</td>
                <td><b>{format!("{at}")}</b></td>
            </tr>
        )
    } else {
        html!()
    };
    let predefined_inputs = html!(
        <tr>
            <td>{"predefined answers: "}</td>
            <td>
                {format!("{:?}", case.out().predefined_answers())}
            </td>
        </tr>
    );
    let program_output = html!(
        <tr>
            <td>{"program output: "}</td>
            <td>
                {format!("{:?}", case.out().all_output_texts().collect::<Vec<_>>())}
            </td>
        </tr>
    );

    html!(
        <div class={classes!("failed-test-details")}>
            <WarningsBox<TestCase> {messages}/>
            <div class={classes!("failed-test-comparison")}>
                <table>
                    {predefined_inputs}
                    {requested_randoms}
                    {program_output}
                    {expected_output}
                    {for differing_lists}
                    {abnormal_termination}
                </table>
            </div>
        </div>
    )
}

#[function_component(CategoryView)]
pub fn category_view(CategoryViewProps { category }: &CategoryViewProps) -> Html {
    let kind = category.kind();
    let cat_messages: Vec<_> = category.category_messages().cloned().collect();

    let success_count = category.successes().count();
    let failure_count = category.failures().count();

    let status_class = if failure_count == 0 {
        if cat_messages.is_empty() {
            classes!("category-without-failures-wrapper")
        } else {
            classes!("category-with-warnings-wrapper")
        }
    } else {
        classes!("category-with-failures-wrapper")
    };

    let cat_name = if kind.trim() == "" {
        html!(<i>{"(default category)"}</i>)
    } else {
        html!(kind)
    };

    let failures = category
        .failures()
        .take(FIRST_N_FAILED_TESTS)
        .map(|tc| html!(<FailedTestCase case={tc.clone()}/>));

    let not_showing = if failure_count > FIRST_N_FAILED_TESTS {
        let missing = failure_count - FIRST_N_FAILED_TESTS;
        html!(
            <div class={classes!("category-hidden-failures-box")}>
            <i>
            {"not showing "}{missing}{" additional "}
            {sinplu("failure","failures", missing)}
            </i>
            </div>
        )
    } else {
        html!()
    };

    html!(
        <div class={classes!("category-wrapper", status_class)}>
            <div class={classes!("category-summary-line")}>
                <h4>{cat_name}</h4>
                <span class={classes!("category-success-count")}>{success_count}{" succeeded"}</span>
                <span class={classes!("category-failure-count")}>{failure_count}{" failed"}</span>
            </div>
            <WarningsBox<Category> messages={cat_messages}/>

            {for failures}

            {not_showing}
        </div>
    )
}
