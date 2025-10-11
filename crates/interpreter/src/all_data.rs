use std::{collections::HashMap, rc::Rc};

use model::{
    Id, SValue,
    attr::{List, Variable},
};

use crate::{RResult, RunError};

#[derive(Debug, PartialEq)]
pub struct AllVariables {
    is_global: HashMap<Id, bool>,
    details: HashMap<Id, model::attr::Variable>,
    values: HashMap<Id, model::SValue>,
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
    pub fn get(&self, v: &Variable) -> RResult<&SValue> {
        self.values
            .get(v.id())
            .ok_or_else(|| RunError::AccessUnknownVariable(v.id().clone()))
    }
    pub fn get_mut(&mut self, v: &Variable) -> RResult<&mut SValue> {
        self.values
            .get_mut(v.id())
            .ok_or_else(|| RunError::AccessUnknownVariable(v.id().clone()))
    }
    pub fn name_for_id(&self, l: &Variable) -> Option<&str> {
        Some(self.details.get(l.id())?.name())
    }
}

#[derive(Debug, PartialEq)]
pub struct AllLists {
    is_global: HashMap<Id, bool>,
    details: HashMap<Id, model::attr::List>,
    values: HashMap<Id, Vec<model::SValue>>,
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
    pub fn get(&self, l: &List) -> RResult<&Vec<SValue>> {
        self.values
            .get(l.id())
            .ok_or_else(|| RunError::AccessUnknownList(l.id().clone()))
    }
    pub fn get_mut(&mut self, l: &List) -> RResult<&mut Vec<SValue>> {
        self.values
            .get_mut(l.id())
            .ok_or_else(|| RunError::AccessUnknownList(l.id().clone()))
    }
    pub fn name_for_id(&self, id: &Id) -> Option<&str> {
        Some(self.details.get(id)?.name())
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = (Id, &str, bool, &[model::SValue])> {
        self.values.iter().flat_map(|(id, v)| {
            Some((
                id.clone(),
                self.name_for_id(id)?,
                self.is_global.get(id).cloned().unwrap_or_default(),
                v.as_slice(),
            ))
        })
    }
}
