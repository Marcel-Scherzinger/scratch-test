use std::ops::Deref;

use itertools::Itertools;
use model::{BlockKind, EventBlockKind, Id, ScratchExpr, attr::RefBlock};

use crate::{
    Interpreter, ProcedureArgumentsFrame, ProcedureId, RResult, RunError, StackItem, Starting,
};

impl Interpreter<Starting> {
    pub(crate) fn internal_start(&mut self) -> RResult<()> {
        loop {
            self.execute_stmt()?;
            if self.state.stack_top()?.is_none() {
                return Ok(());
            }
        }
    }

    fn execute_stmt(&mut self) -> RResult<()> {
        let (block, stack_item) = self.state.next_block4exec()?;
        let kind = block.inner();
        let next = block.next().clone();

        log::debug!("execute: {kind:?}");

        use BlockKind as K;
        use model::StmtBlockKind as S;
        match &kind {
            K::ProceduresDefinition { custom_block } => {
                self.state.stack_push_opt(next)?;
            }
            K::ProceduresPrototype {
                proccode,
                arguments,
            } => todo!(),
            K::Event(
                EventBlockKind::EventWhenflagclicked | EventBlockKind::EventWhenkeypressed { .. },
            ) => {
                self.state.stack_push_opt(next)?;
            }
            K::Noop(_) | K::Unsup(_) => {
                // TODO: warn or fail on unsupported
                self.state.stack_push_opt(next)?;
            }
            K::Stmt(stmt) => match &stmt {
                S::ProceduresCall {
                    proccode,
                    arguments,
                } => {
                    match stack_item {
                        // call will be initiated
                        StackItem::Normal(_) => {
                            let procedure_id =
                                ProcedureId::generate_from_fields(proccode, arguments);

                            let evaluated_arguments: Result<Vec<_>, RunError> = arguments
                                .iter()
                                .map(|(id, expr)| {
                                    Ok::<_, RunError>((
                                        id.clone(),
                                        expr.as_ref().map(|e| self.evaluate_expr(e)).transpose()?,
                                    ))
                                })
                                .collect();

                            let prototype = self.state.get_procedure(procedure_id)?;

                            let frame = ProcedureArgumentsFrame::for_procedure(
                                prototype,
                                &evaluated_arguments?,
                            )?;
                            log::trace!("Add procedure arguments frame: {frame:?}");
                            let definition_block_id = prototype.definition_block().id().clone();

                            self.state.stack_push_opt(next)?;
                            // IMPORTANT: prepare cleanup on stack
                            self.state
                                .stack_push(StackItem::PopArgumentFrame(block.id().clone()))?;
                            self.state.procedure_arguments_push_frame(frame)?;
                            self.state.stack_push(definition_block_id)?;
                        }
                        // call will be cleaned up (it is now after the exit of the call)
                        StackItem::PopArgumentFrame(_) => {
                            self.state.procedure_arguments_pop_frame()?;
                        }
                        // only possible for loops
                        StackItem::CountLoop(_, _) => unreachable!(),
                    }
                }

                S::LooksSay { message } => {
                    let message = self.evaluate_expr(message)?;
                    self.state
                        .action_write_output(crate::OutputAction::Say, message.as_text().into())?;
                    self.state.stack_push_opt(next)?;
                }
                S::LooksThink { message } => {
                    let message = self.evaluate_expr(message)?;
                    self.state.action_write_output(
                        crate::OutputAction::Think,
                        message.as_text().into(),
                    )?;
                    self.state.stack_push_opt(next)?;
                }
                S::LooksThinkforsecs { message, secs } => {
                    let message = self.evaluate_expr(message)?;
                    let secs = self.evaluate_expr(secs)?;
                    self.state.action_write_output(
                        crate::OutputAction::ThinkFor(secs.as_float()),
                        message.as_text().into(),
                    )?;
                    self.state.stack_push_opt(next)?;
                }
                S::LooksSayforsecs { message, secs } => {
                    let message = self.evaluate_expr(message)?;
                    let secs = self.evaluate_expr(secs)?;
                    self.state.action_write_output(
                        crate::OutputAction::SayFor(secs.as_float()),
                        message.as_text().into(),
                    )?;
                    self.state.stack_push_opt(next)?;
                }
                S::ControlRepeatuntil {
                    condition,
                    substack,
                } => {
                    let stop_loop = self.evaluate_opt_cmp(condition)?;
                    if !stop_loop {
                        if let Some(substack) = substack {
                            self.state.stack_push(stack_item)?;
                            self.state.stack_push(substack.o_id())?;
                        } else {
                            Err(crate::RunError::ConditionLoopWithoutBodyNeverStops)?;
                        }
                    } else {
                        self.state.stack_push_opt(next)?;
                    }
                }
                S::DataSetvariableto { variable, value } => {
                    let value = self.evaluate_expr(value)?;
                    self.state.set_variable(variable, value)?;
                    self.state.stack_push_opt(next)?;
                }
                S::DataChangevariableby { variable, value } => {
                    let value = self.evaluate_expr(value)?;

                    let old = self.state.get_variable(variable)?;

                    let new = old.same_numbers_wrap_op(
                        &value,
                        |old, value| old + value,
                        |old, value| old + value,
                    );

                    self.state.set_variable(variable, new)?;
                    self.state.stack_push_opt(next)?;
                }
                S::ControlWait { duration } => {
                    let duration = self.evaluate_expr(duration)?;
                    self.state.action_wait(duration.as_float());
                    self.state.stack_push_opt(next)?;
                }
                S::ControlIf {
                    condition,
                    substack,
                } => {
                    let condition = self.evaluate_opt_cmp(condition)?;
                    self.state.stack_push_opt(next)?;
                    // push body if not empty
                    self.state
                        .stack_push_opt(substack.as_ref().map(|b| b.o_id()))?;
                }
                S::ControlIfElse {
                    condition,
                    substack,
                    substack2,
                } => {
                    let condition = self.evaluate_opt_cmp(condition)?;
                    self.state.stack_push_opt(next)?;
                    if condition {
                        self.state
                            .stack_push_opt(substack.as_ref().map(|d| d.o_id()))?;
                    } else {
                        self.state
                            .stack_push_opt(substack2.as_ref().map(|d| d.o_id()))?;
                    }
                }
                S::ControlForever { substack } => {
                    if let Some(substack) = substack {
                        self.state.stack_push(stack_item)?;
                        self.state.stack_push(substack.deref().clone())?;
                    } else {
                        return Err(RunError::InfiniteLoopWithoutBodyNeverStops);
                    }
                }
                S::ControlStop { stop_option } => match stop_option.as_ref() {
                    "this script" | "all" => {
                        return Err(RunError::TerminateBecauseOfStop);
                    }
                    _ => {
                        // other scripts of this sprite in single threaded mode?
                        todo!()
                    }
                },
                S::ControlWaitUntil { condition } => {
                    let condition = self.evaluate_opt_cmp(condition)?;
                    if condition {
                        self.state.stack_push_opt(next)?;
                        return Ok(());
                    } else {
                        return Err(RunError::WaitTillNeverStops);
                    }
                }
                S::ControlRepeat { times, substack } => {
                    let remaining = match stack_item {
                        StackItem::Normal(_) => {
                            self.state.warn_used_counter_loop();
                            self.evaluate_expr(times)?.as_int().max(0) as usize
                        }
                        StackItem::CountLoop(_, remaining) => remaining,
                        // in this case the block has to be a procedures call
                        StackItem::PopArgumentFrame(_) => unreachable!(),
                    };

                    match remaining {
                        0 => self.state.stack_push_opt(next)?,
                        1.. => {
                            self.state.stack_push(StackItem::CountLoop(
                                block.id().clone(),
                                remaining - 1,
                            ));
                            self.state
                                .stack_push_opt(substack.as_ref().map(|d| d.o_id()));
                        }
                    }
                }
                S::DataDeleteoflist { list, index } => {
                    let index = self.evaluate_expr(index)?.as_int();
                    let mut list = self.state.get_mut_list_elements(list)?;
                    if index > 0 {
                        let index = index as usize;
                        if index <= list.len() {
                            list.remove(index - 1);
                        }
                    }
                    self.state.stack_push_opt(next)?
                }
                S::DataDeletealloflist { list } => {
                    let mut list = self.state.get_mut_list_elements(list)?;
                    list.clear();
                    self.state.stack_push_opt(next)?
                }
                S::DataInsertatlist { list, index, item } => {
                    let index = self.evaluate_expr(index)?.as_int();
                    let item = self.evaluate_expr(item)?;
                    let mut list = self.state.get_mut_list_elements(list)?;

                    if index > 0 {
                        let index = index as usize;
                        if index <= list.len() {
                            list.insert(index - 1, item);
                        } else if index == list.len() + 1 {
                            list.push(item);
                        }
                    }
                    self.state.stack_push_opt(next)?
                }
                S::SensingAskandwait { question } => {
                    let question = self.evaluate_expr(question)?;
                    self.state
                        .action_ask_question_and_wait(question.as_text().to_string())?;
                    self.state.stack_push_opt(next)?;
                }
                S::DataReplaceitemoflist { list, index, item } => {
                    let index = self.evaluate_expr(index)?.as_int();
                    let item = self.evaluate_expr(item)?;
                    let mut list = self.state.get_mut_list_elements(list)?;

                    if index > 0 {
                        let index = index as usize;
                        if index <= list.len() {
                            list[index - 1] = item;
                        }
                    }
                    self.state.stack_push_opt(next)?
                }
                S::DataAddtolist { list, item } => {
                    let item = self.evaluate_expr(item)?;
                    let mut list = self.state.get_mut_list_elements(list)?;

                    list.push(item);
                    self.state.stack_push_opt(next)?
                }
            },
            K::Cmp(_) | K::Expr(_) => {
                Err(crate::RunError::UnexpectedNestingOfBlocks)?;
            }
        }

        Ok(())
    }

    fn evaluate_expr(&mut self, expr: &model::attr::Expression) -> RResult<model::SValue> {
        use model::attr::Expression as E;
        log::trace!("evaluate expr: {expr:?}");
        match expr {
            E::Lit(val) => Ok(val.clone()),
            E::Var(var) => self.state.get_variable(var),
            E::Blo(id) => {
                // TODO: what's with unsupported expression blocks?
                let b = self.state.get_expression_block_cmp_allowed(id)?;
                use model::BlockKind as B;
                use model::ExprBlockKind as E;
                use model::SValue as V;

                // scratch allows comparisons in expressions
                if let B::Cmp(_) = b.inner() {
                    self.evaluate_cmp(b.id().clone()).map(model::SValue::Bool)
                } else if let B::Expr(e) = b.inner() {
                    // normal expression
                    Ok(match e {
                        E::OperatorRandom { from, to } => {
                            let from = self.evaluate_expr(from)?;
                            let to = self.evaluate_expr(to)?;
                            self.state.request_random_number(&from, &to)?
                        }
                        E::OperatorAdd { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?;
                            let num2 = self.evaluate_expr(num2)?;
                            use std::ops::Add;
                            num1.same_numbers_wrap_op(&num2, Add::add, Add::add)
                        }
                        E::OperatorSubtract { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?;
                            let num2 = self.evaluate_expr(num2)?;
                            use std::ops::Sub;
                            num1.same_numbers_wrap_op(&num2, Sub::sub, Sub::sub)
                        }
                        E::OperatorMultiply { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?;
                            let num2 = self.evaluate_expr(num2)?;
                            use std::ops::Mul;
                            num1.same_numbers_wrap_op(&num2, Mul::mul, Mul::mul)
                        }
                        E::OperatorMod { num1, num2 } => {
                            // TODO: scratch version of mod also works with floats
                            let num1 = self.evaluate_expr(num1)?.as_int();
                            let num2 = self.evaluate_expr(num2)?.as_int();

                            if num2 == 0 {
                                V::Float(f64::NAN)
                            } else if num1 == 0 {
                                V::Int(0)
                            } else {
                                V::Int(num1 % num2)
                            }
                        }
                        E::OperatorDivide { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?.as_float();
                            let num2 = self.evaluate_expr(num2)?.as_float();

                            if num1 == 0.0 && num2 == 0.0 {
                                V::Float(f64::NAN)
                            } else if num1 > 0.0 && num2 == 0.0 {
                                V::Float(f64::INFINITY)
                            } else if num1 < 0.0 && num2 == 0.0 {
                                V::Float(f64::NEG_INFINITY)
                            } else {
                                V::Float(num1 / num2)
                            }
                        }
                        E::OperatorLetterOf { letter, string } => {
                            let letter = self.evaluate_expr(letter)?.as_int();
                            let string = self.evaluate_expr(string)?;
                            if letter == 0 || letter as usize > string.as_text().len() {
                                V::Text("".into())
                            } else {
                                V::Text(
                                    string
                                        .as_text()
                                        .chars()
                                        .nth((letter - 1) as usize)
                                        .unwrap_or_default()
                                        .to_string()
                                        .into(),
                                )
                            }
                        }
                        E::OperatorRound { num } => {
                            let num = self.evaluate_expr(num)?.as_int();
                            V::Int(num)
                        }
                        E::OperatorLength { string } => {
                            let string = self.evaluate_expr(string)?;
                            V::int_or_max(string.as_text().len())
                        }
                        E::OperatorJoin { string1, string2 } => {
                            let string1 = self.evaluate_expr(string1)?;
                            let string2 = self.evaluate_expr(string2)?;
                            V::Text(
                                (string1.as_text().to_string() + string2.as_text().as_ref()).into(),
                            )
                        }
                        E::DataLengthoflist { list } => {
                            V::int_or_max(self.state.get_list_elements(list)?.len())
                        }
                        E::DataItemnumoflist { list, item } => {
                            let item = self.evaluate_expr(item)?;
                            let list = self.state.get_list_elements(list)?;
                            let pos = list
                                .iter()
                                .find_position(|i| i.scratch_eq(&item))
                                .map(|(pos, _)| pos + 1)
                                .unwrap_or(0);
                            V::int_or_max(pos)
                        }
                        E::DataItemoflist { list, index } => {
                            let index = self.evaluate_expr(index)?.as_int();
                            let list = self.state.get_list_elements(list)?;
                            if index > 0 {
                                list.get((index - 1) as usize)
                                    .cloned()
                                    .unwrap_or(model::SValue::Text("".into()))
                            } else {
                                model::SValue::Text("".into())
                            }
                        }
                        E::SensingAnswer => self.state.read_last_answer()?.clone(),
                        E::RDataVar { variable } => self.state.get_variable(variable)?,
                        E::RDataList { list } => self.state.get_list_value(list)?,
                        E::OperatorMathop { operator, num } => {
                            let num = self.evaluate_expr(num)?.as_float();
                            V::Float(match operator.as_ref() {
                                "e ^" => num.exp(),
                                "log" => num.log10(),
                                "ln" => num.ln(),
                                "abs" => num.abs(),
                                "floor" => num.floor(),
                                "ceil" => num.ceil(),
                                "sqrt" => num.sqrt(),
                                "sin" => num.sin(),
                                "cos" => num.cos(),
                                "tan" => num.tan(),
                                "asin" => num.asin(),
                                "acos" => num.acos(),
                                "atan" => num.atan(),
                                other => {
                                    return Err(RunError::UnsupportedMathOperator(other.into()));
                                }
                            })
                        }

                        E::ArgumentReporterStringNumber { value } => self
                            .state
                            .procedure_arguments_nearest_string_number(value)?,
                        E::ArgumentReporterBoolean { value } => {
                            self.state.procedure_arguments_nearest_boolean(value)?
                        }
                    })
                } else {
                    Err(crate::RunError::UnexpectedBlockKind(id.deref().clone()))
                }
            }
            E::Lis(list) => self.state.get_list_value(list),
        }
    }

    fn evaluate_cmp(&mut self, id: Id) -> RResult<bool> {
        // TODO: what's with unsupported cmp blocks?
        let block = self.state.get_cmp_block(&id)?;
        let kind = block.inner();
        use model::CmpBlockKind as C;
        log::trace!("evaluate cmp: {kind:?}");
        if let model::BlockKind::Cmp(kind) = kind {
            Ok(match kind {
                C::ArgumentReporterBoolean { value } => self
                    .state
                    .procedure_arguments_nearest_boolean(value)?
                    .as_bool(),
                C::OperatorOr { operand1, operand2 } => {
                    let operand1 = self.evaluate_cmp(operand1.deref().clone())?;
                    let operand2 = self.evaluate_cmp(operand2.deref().clone())?;
                    operand1 || operand2
                }
                C::OperatorAnd { operand1, operand2 } => {
                    let operand1 = self.evaluate_cmp(operand1.o_id())?;
                    let operand2 = self.evaluate_cmp(operand2.o_id())?;
                    operand1 && operand2
                }
                C::OperatorNot { operand } => self.evaluate_cmp(operand.o_id())?,
                C::OperatorEquals { operand1, operand2 } => self
                    .evaluate_expr(operand1)?
                    .scratch_eq(&self.evaluate_expr(operand2)?),
                C::OperatorGt { operand1, operand2 } => {
                    self.evaluate_expr(operand1)?.as_float()
                        > self.evaluate_expr(operand2)?.as_float()
                }
                C::OperatorLt { operand1, operand2 } => {
                    self.evaluate_expr(operand1)?.as_float()
                        < self.evaluate_expr(operand2)?.as_float()
                }
                C::OperatorContains { string1, string2 } => {
                    let string1 = self.evaluate_expr(string1)?;
                    let string2 = self.evaluate_expr(string2)?;
                    string1.as_text().contains(string2.as_text().as_ref())
                }
                C::DataListcontainsitem { list, item } => {
                    let item = self.evaluate_expr(item)?;
                    let list = self.state.get_list_elements(list)?;
                    list.iter().any(|i| i.scratch_eq(&item))
                }
            })
        } else {
            Err(RunError::UnexpectedBlockKind(id.clone()))
        }
    }
    fn evaluate_opt_cmp(&mut self, id: &Option<RefBlock>) -> RResult<bool> {
        if let Some(id) = id {
            self.evaluate_cmp(id.o_id())
        } else {
            Ok(false)
        }
    }
}
