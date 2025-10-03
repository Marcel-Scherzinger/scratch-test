use testreports::Message;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WarningsBoxProps<Level: std::cmp::PartialEq> {
    pub messages: Vec<Message<Level>>,
}

#[function_component(WarningsBox)]
pub fn warnings_box<Level>(WarningsBoxProps { messages }: &WarningsBoxProps<Level>) -> Html
where
    Level: std::cmp::PartialEq,
{
    let mut messages = messages
        .iter()
        .map(|msg| {
            let kind = msg.kind().to_string();
            html!(
                <tr>
                    <td class={classes!("report-message-kind-label")} kind={kind.clone()}>
                        {kind.to_uppercase() + ":"}
                    </td>
                    <td class={classes!("report-message-entry")} {kind}>
                        {msg.msg()}
                    </td>
                </tr>
            )
        })
        .peekable();
    if messages.peek().is_none() {
        html!(
            <div class={classes!("report-empty-message-wrapper")}>
            </div>
        )
    } else {
        html!(
            <div class={classes!("report-message-wrapper")}>
                <table class={classes!("report-message-list")}>
                    {for messages}
                </table>
            </div>
        )
    }
}
