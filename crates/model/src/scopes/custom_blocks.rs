use std::{collections::HashMap, rc::Rc};

use crate::{
    BlockWrapper, Id, TargetBlocks,
    attr::{Expression, ProcedureArgumentDef},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::From, Clone)]
pub struct ProcedureId(Rc<str>);

impl ProcedureId {
    pub fn generate_from<'a>(
        proccode: &'a str,
        argument_ids: impl Iterator<Item = &'a str>,
    ) -> Self {
        use itertools::Itertools;
        Self(
            std::iter::once(proccode)
                .chain(argument_ids.sorted())
                .join("|")
                .into(),
        )
    }
    pub fn generate_from_fields(
        proccode: &str,
        arguments: &Rc<[(Id, Option<Expression>)]>,
    ) -> Self {
        Self::generate_from(proccode, arguments.iter().map(|(a, _)| a.as_ref()))
    }
}

/// Contains all information about a defined custom/own block, also called procedure.
///
/// This information is collected by combining to categories of blocks from the target blocks:
/// - a definition block
/// - a prototype block
#[derive(Debug, PartialEq, derive_getters::Getters)]
pub struct Procedure {
    proccode: String,
    definition_block: Rc<BlockWrapper>,
    prototype_block_id: Id,
    /// generated using proccode and arguments
    procedure_id: ProcedureId,
    arguments: std::rc::Rc<[ProcedureArgumentDef]>,
}

/// This component has no equivalent in the scratch file format
#[derive(Debug, PartialEq)]
pub struct TargetProcedures(HashMap<ProcedureId, Procedure>);

impl TargetProcedures {
    pub(crate) fn new(blocks: &TargetBlocks) -> Self {
        let mut cb = HashMap::new();
        let mut proto_to_def_block = HashMap::new();

        for block in blocks.iter_blocks() {
            use crate::BlockKind as B;
            match block.inner() {
                B::ProceduresPrototype {
                    proccode,
                    arguments,
                } => {
                    let proc_id = ProcedureId::generate_from(
                        proccode,
                        arguments.iter().map(|a| a.argument_id().id().as_ref()),
                    );
                    cb.insert(
                        block.id().clone(),
                        (proc_id, proccode.clone(), arguments.clone()),
                    );
                }
                B::ProceduresDefinition { custom_block } => {
                    proto_to_def_block.insert(custom_block.o_id(), block.clone());
                }
                _ => {}
            }
        }

        let res = cb
            .into_iter()
            .filter_map(|(proto_id, (procedure_id, proccode, arguments))| {
                Some((
                    procedure_id.clone(),
                    Procedure {
                        proccode,
                        arguments,
                        definition_block: proto_to_def_block.get(&proto_id)?.clone(),
                        procedure_id,
                        prototype_block_id: proto_id,
                    },
                ))
            })
            .collect();
        Self(res)
    }
    pub fn get(&self, id: &ProcedureId) -> Option<&Procedure> {
        self.0.get(id)
    }
}
