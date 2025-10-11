use std::collections::HashMap;

use super::error::TargetListsError;
use crate::interpret_json::List;
use crate::scratch_expr::SValue as VariableValue;

use crate::Id;

#[derive(Debug, PartialEq)]
pub struct TargetLists {
    /// map from list id to name and saved value
    ///
    /// (the value the list had when pressing save in the editor)
    map: HashMap<Id, (List, Vec<VariableValue>)>,
}

impl TargetLists {
    pub fn iter_lists(&self) -> impl Iterator<Item = &(List, Vec<VariableValue>)> {
        self.map.values()
    }
}

impl crate::ext::FromJsonExt<Self, TargetListsError> for TargetLists {
    fn from_json_without_ctx(value: &serde_json::Value) -> Result<Self, TargetListsError> {
        let dict = value.as_object().ok_or(TargetListsError::ExpectedObject)?;
        let map: Result<_, _> = dict
            .into_iter()
            .map(|(id, def)| {
                let id: Id = id.clone().into();
                if let Some(parsed) = parse_list(id.clone(), def) {
                    Ok((id, parsed))
                } else {
                    Err(TargetListsError::AtLeastOneInvalid(id))
                }
            })
            .collect();
        Ok(Self { map: map? })
    }
}

fn parse_list(id: Id, def: &serde_json::Value) -> Option<(List, Vec<VariableValue>)> {
    let name = def[0].as_str()?.into();
    let initial = if let Some(arr) = def[1].as_array() {
        arr.iter()
            .map(|element| {
                if let Some(number) = element
                    .as_number()
                    .cloned()
                    .and_then(|x| VariableValue::try_from(x).ok())
                {
                    return number;
                }
                let text = element
                    .as_str()
                    .map(|s| s.into())
                    .unwrap_or_else(|| element.to_string().into());
                VariableValue::Text(text)
            })
            .collect()
    } else {
        return None;
    };
    Some((List::new(name, id), initial))
}
