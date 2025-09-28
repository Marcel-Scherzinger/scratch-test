use std::collections::HashMap;

use model::{Id, List, Variable, VariableValue};

use crate::{RResult, RunError};

#[derive(Debug)]
pub struct AllVariables {
    is_global: HashMap<Id, bool>,
    details: HashMap<Id, model::Variable>,
    values: HashMap<Id, model::VariableValue>,
}
impl AllVariables {
    pub(crate) fn new(doc: &model::ProjectDoc, target_idx: usize) -> Self {
        let target = &doc.targets()[target_idx];
        let mut is_global = HashMap::new();
        let mut details = HashMap::new();
        let mut values = HashMap::new();
        if let Some((stage_idx, stage)) = crate::get_stage(doc) {
            for (var, initial) in stage.variables().iter_variables() {
                is_global.insert(var.id().clone(), true);
                values.insert(var.id().clone(), initial.clone());
                details.insert(var.id().clone(), var.clone());
            }
            if stage_idx != target_idx {
                for (var, initial) in target.variables().iter_variables() {
                    is_global.insert(var.id().clone(), false);
                    values.insert(var.id().clone(), initial.clone());
                    details.insert(var.id().clone(), var.clone());
                }
            }
        } else {
            for (var, initial) in target.variables().iter_variables() {
                is_global.insert(var.id().clone(), false);
                values.insert(var.id().clone(), initial.clone());
                details.insert(var.id().clone(), var.clone());
            }
        }
        Self {
            is_global,
            details,
            values,
        }
    }
}

impl AllVariables {
    pub fn get(&self, v: &Variable) -> RResult<&VariableValue> {
        self.values
            .get(v.id())
            .ok_or_else(|| RunError::AccessUnknownVariable(v.id().clone()))
    }
    pub fn get_mut(&mut self, v: &Variable) -> RResult<&mut VariableValue> {
        self.values
            .get_mut(v.id())
            .ok_or_else(|| RunError::AccessUnknownVariable(v.id().clone()))
    }
    pub fn name_for_id(&self, l: &Variable) -> Option<&str> {
        Some(self.details.get(l.id())?.name())
    }
}

#[derive(Debug)]
pub struct AllLists {
    is_global: HashMap<Id, bool>,
    details: HashMap<Id, model::List>,
    values: HashMap<Id, Vec<model::VariableValue>>,
}
impl AllLists {
    pub(crate) fn new(doc: &model::ProjectDoc, target_idx: usize) -> Self {
        let target = &doc.targets()[target_idx];
        let mut is_global = HashMap::new();
        let mut details = HashMap::new();
        let mut values = HashMap::new();
        if let Some((stage_idx, stage)) = crate::get_stage(doc) {
            for (var, initial) in stage.lists().iter_lists() {
                is_global.insert(var.id().clone(), true);
                values.insert(var.id().clone(), initial.clone());
                details.insert(var.id().clone(), var.clone());
            }
            if stage_idx != target_idx {
                for (var, initial) in target.lists().iter_lists() {
                    is_global.insert(var.id().clone(), false);
                    values.insert(var.id().clone(), initial.clone());
                    details.insert(var.id().clone(), var.clone());
                }
            }
        } else {
            for (var, initial) in target.lists().iter_lists() {
                is_global.insert(var.id().clone(), false);
                values.insert(var.id().clone(), initial.clone());
                details.insert(var.id().clone(), var.clone());
            }
        }
        Self {
            is_global,
            details,
            values,
        }
    }
}

impl AllLists {
    pub fn get(&self, l: &List) -> RResult<&Vec<VariableValue>> {
        self.values
            .get(l.id())
            .ok_or_else(|| RunError::AccessUnknownList(l.id().clone()))
    }
    pub fn get_mut(&mut self, l: &List) -> RResult<&mut Vec<VariableValue>> {
        self.values
            .get_mut(l.id())
            .ok_or_else(|| RunError::AccessUnknownList(l.id().clone()))
    }
    pub fn name_for_id(&self, l: &List) -> Option<&str> {
        Some(self.details.get(l.id())?.name())
    }
}
