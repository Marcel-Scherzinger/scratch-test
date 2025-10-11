use std::collections::HashMap;

use super::error::TargetVariablesError;
use crate::scratch_expr::SValue as VariableValue;

use crate::{Id, interpret_json::Variable};

#[derive(Debug, PartialEq)]
pub struct TargetVariables {
    /// map from variable id to name and saved value
    ///
    /// (the value the variable had when pressing save in the editor)
    map: HashMap<Id, (Variable, VariableValue)>,
}

impl TargetVariables {
    pub fn iter_variables(&self) -> impl Iterator<Item = &(Variable, VariableValue)> {
        self.map.values()
    }
}

impl crate::ext::FromJsonExt<Self, TargetVariablesError> for TargetVariables {
    fn from_json_without_ctx(value: &serde_json::Value) -> Result<Self, TargetVariablesError> {
        let dict = value
            .as_object()
            .ok_or(TargetVariablesError::ExpectedObject)?;
        let map: Result<_, _> = dict
            .into_iter()
            .map(|(id, def)| {
                let id: Id = id.clone().into();
                if let Some(parsed) = parse_variable(id.clone(), def) {
                    Ok((id, parsed))
                } else {
                    Err(TargetVariablesError::AtLeastOneInvalid(id))
                }
            })
            .collect();
        Ok(Self { map: map? })
    }
}

fn parse_variable(id: Id, def: &serde_json::Value) -> Option<(Variable, VariableValue)> {
    let name = def[0].as_str()?.into();
    let initial = if let Some(num) = def[1].as_number() {
        // TODO: integer out of range case is ignored
        VariableValue::try_from(num.clone()).ok()?
    } else {
        VariableValue::Text(def[1].as_str()?.into())
    };
    Some((Variable::new(name, id), initial))
}
