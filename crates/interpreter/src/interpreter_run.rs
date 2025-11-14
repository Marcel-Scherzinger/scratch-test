use std::ops::Deref;

use itertools::Itertools;
use model::{BlockKind, EventBlockKind, GetOpcodeUnit, Id, ScratchExpr, attr::RefBlock};

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

        log::trace!("execute: {kind:?}");

        use BlockKind as K;
        use model::StmtBlockKind as S;
        let scope = kind.get_opcode();
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
                log::debug!("[{scope}] reacted with event block");
            }
            K::Noop(_) => {
                self.state.stack_push_opt(next)?;
                log::debug!("[{scope}] reached no-operation block, doing nothing");
            }
            K::Unsup(_) => {
                // TODO: warn or fail on unsupported
                self.state.stack_push_opt(next)?;
                log::debug!("[{scope}] reached unsupported operation, treated like no-operation");
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
                            log::debug!("[{scope}] call procedure: {proccode:?}");
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
                    log::debug!("[{scope}] say message {message:?}");
                    self.state
                        .action_write_output(crate::OutputAction::Say, message.as_text().into())?;
                    self.state.stack_push_opt(next)?;
                }
                S::LooksThink { message } => {
                    let message = self.evaluate_expr(message)?;
                    log::debug!("[{scope}] think message {message:?}");
                    self.state.action_write_output(
                        crate::OutputAction::Think,
                        message.as_text().into(),
                    )?;
                    self.state.stack_push_opt(next)?;
                }
                S::LooksThinkforsecs { message, secs } => {
                    let message = self.evaluate_expr(message)?;
                    let secs = self.evaluate_expr(secs)?;
                    log::debug!("[{scope}] think message {message:?} for {secs:?}s");
                    self.state.action_write_output(
                        crate::OutputAction::ThinkFor(secs.as_float()),
                        message.as_text().into(),
                    )?;
                    self.state.stack_push_opt(next)?;
                }
                S::LooksSayforsecs { message, secs } => {
                    let message = self.evaluate_expr(message)?;
                    let secs = self.evaluate_expr(secs)?;
                    log::debug!("[{scope}] say message {message:?} for {secs:?}s");
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
                            log::debug!("[{scope}] condition was false, run loop iteration");
                            self.state.stack_push(stack_item)?;
                            self.state.stack_push(substack.o_id())?;
                        } else {
                            Err(crate::RunError::ConditionLoopWithoutBodyNeverStops)?;
                        }
                    } else {
                        log::debug!("[{scope}] condition was true, terminate loop");
                        self.state.stack_push_opt(next)?;
                    }
                }
                S::DataSetvariableto { variable, value } => {
                    let value = self.evaluate_expr(value)?;
                    log::debug!("[{scope}] set variable {:?} to {value:?}", variable.name());
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
                    log::debug!(
                        "[{scope}] change variable {:?} from {old:?} by {value:?}, new value is {new:?}",
                        variable.name()
                    );
                    self.state.set_variable(variable, new)?;
                    self.state.stack_push_opt(next)?;
                }
                S::ControlWait { duration } => {
                    let duration = self.evaluate_expr(duration)?;
                    log::debug!("[{scope}] wait {duration:?}s");
                    self.state.action_wait(duration.as_float());
                    self.state.stack_push_opt(next)?;
                }
                S::ControlIf {
                    condition,
                    substack,
                } => {
                    let condition = self.evaluate_opt_cmp(condition)?;
                    self.state.stack_push_opt(next)?;
                    if condition {
                        log::debug!("[{scope}] condition was true, executing then-part");
                        // push body if not empty
                        self.state
                            .stack_push_opt(substack.as_ref().map(|b| b.o_id()))?;
                    } else {
                        log::debug!("[{scope}] condition was false, skipping then-part");
                    }
                }
                S::ControlIfElse {
                    condition,
                    substack,
                    substack2,
                } => {
                    let condition = self.evaluate_opt_cmp(condition)?;
                    self.state.stack_push_opt(next)?;
                    if condition {
                        log::debug!("[{scope}] condition was true, executing then-part");
                        self.state
                            .stack_push_opt(substack.as_ref().map(|d| d.o_id()))?;
                    } else {
                        log::debug!("[{scope}] condition was false, executing else-part");
                        self.state
                            .stack_push_opt(substack2.as_ref().map(|d| d.o_id()))?;
                    }
                }
                S::ControlForever { substack } => {
                    if let Some(substack) = substack {
                        log::debug!("[{scope}] iteration of forever-loop");
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
                        log::debug!("[{scope}] waited till condition");
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
                        0 => {
                            log::debug!("[{scope}] finished last iteration of loop");
                            self.state.stack_push_opt(next)?
                        }
                        1.. => {
                            log::debug!(
                                "[{scope}] start next of remaining {remaining:?} iteration(s) of loop",
                            );
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
                    log::debug!("[{scope}] delete item {index:?} of list {:?}", list.name());
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
                    log::debug!("[{scope}] delete every item of list {:?}", list.name());
                    let mut list = self.state.get_mut_list_elements(list)?;
                    list.clear();
                    self.state.stack_push_opt(next)?
                }
                S::DataInsertatlist { list, index, item } => {
                    let index = self.evaluate_expr(index)?.as_int();
                    let item = self.evaluate_expr(item)?;
                    log::debug!(
                        "[{scope}] insert item {item:?} at {index:?} into list {:?}",
                        list.name()
                    );
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
                    log::debug!("[{scope}] ask question {question:?} and wait for answer");
                    self.state
                        .action_ask_question_and_wait(question.as_text().to_string())?;
                    log::debug!(
                        "[{scope}] question {question:?} answered with {:?}",
                        self.state.read_last_answer()
                    );
                    self.state.stack_push_opt(next)?;
                }
                S::DataReplaceitemoflist { list, index, item } => {
                    let index = self.evaluate_expr(index)?.as_int();
                    let item = self.evaluate_expr(item)?;
                    log::debug!(
                        "[{scope}] replace item {index:?} of {} with {item:?}",
                        list.name()
                    );
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
                    log::debug!("[{scope}] append item {item:?} to list {:?}", list.name());
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
                    let scope = e.get_opcode();
                    // normal expression
                    Ok(match e {
                        E::OperatorRandom { from, to } => {
                            let from = self.evaluate_expr(from)?;
                            let to = self.evaluate_expr(to)?;
                            let res = self.state.request_random_number(&from, &to)?;
                            log::debug!("[{scope}] random from {from:?} to {to:?} --> {res:?}");
                            res
                        }
                        E::OperatorAdd { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?;
                            let num2 = self.evaluate_expr(num2)?;
                            use std::ops::Add;
                            let res = num1.same_numbers_wrap_op(&num2, Add::add, Add::add);
                            log::debug!("[{scope}] {num1:?} + {num2:?} --> {res:?}");
                            res
                        }
                        E::OperatorSubtract { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?;
                            let num2 = self.evaluate_expr(num2)?;
                            use std::ops::Sub;
                            let res = num1.same_numbers_wrap_op(&num2, Sub::sub, Sub::sub);
                            log::debug!("[{scope}] {num1:?} - {num2:?} --> {res:?}");
                            res
                        }
                        E::OperatorMultiply { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?;
                            let num2 = self.evaluate_expr(num2)?;
                            use std::ops::Mul;
                            let res = num1.same_numbers_wrap_op(&num2, Mul::mul, Mul::mul);
                            log::debug!("[{scope}] {num1:?} * {num2:?} --> {res:?}");
                            res
                        }
                        E::OperatorMod { num1, num2 } => {
                            // TODO: scratch version of mod also works with floats
                            let num1 = self.evaluate_expr(num1)?.as_int();
                            let num2 = self.evaluate_expr(num2)?.as_int();

                            let res = if num2 == 0 {
                                V::Float(f64::NAN)
                            } else if num1 == 0 {
                                V::Int(0)
                            } else {
                                V::Int(num1 % num2)
                            };
                            log::debug!("[{scope}] {num1:?} mod {num2:?} --> {res:?}");
                            res
                        }
                        E::OperatorDivide { num1, num2 } => {
                            let num1 = self.evaluate_expr(num1)?.as_float();
                            let num2 = self.evaluate_expr(num2)?.as_float();

                            let res = if num1 == 0.0 && num2 == 0.0 {
                                V::Float(f64::NAN)
                            } else if num1 > 0.0 && num2 == 0.0 {
                                V::Float(f64::INFINITY)
                            } else if num1 < 0.0 && num2 == 0.0 {
                                V::Float(f64::NEG_INFINITY)
                            } else {
                                V::Float(num1 / num2)
                            };
                            log::debug!("[{scope}] {num1:?} / {num2:?} --> {res:?}");
                            res
                        }
                        E::OperatorLetterOf { letter, string } => {
                            let letter = self.evaluate_expr(letter)?.as_int();
                            let string = self.evaluate_expr(string)?;
                            let res = if letter == 0 || letter as usize > string.as_text().len() {
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
                            };
                            log::debug!(
                                "[{scope}] letter {letter:?} of text {string:?} --> {res:?}"
                            );
                            res
                        }
                        E::OperatorRound { num } => {
                            let num = self.evaluate_expr(num)?.as_int();
                            let res = V::Int(num);
                            log::debug!("[{scope}] round {num:?} --> {res}");
                            res
                        }
                        E::OperatorLength { string } => {
                            let string = self.evaluate_expr(string)?;
                            let res = V::int_or_max(string.as_text().len());
                            log::debug!("[{scope}] length of {string:?} --> {res}");
                            res
                        }
                        E::OperatorJoin { string1, string2 } => {
                            let string1 = self.evaluate_expr(string1)?;
                            let string2 = self.evaluate_expr(string2)?;
                            let res = V::Text(
                                (string1.as_text().to_string() + string2.as_text().as_ref()).into(),
                            );
                            log::debug!("[{scope}] concat {string1:?} and {string2:?} --> {res}");
                            res
                        }
                        E::DataLengthoflist { list } => {
                            let res = V::int_or_max(self.state.get_list_elements(list)?.len());
                            log::debug!("[{scope}] length of list {:?} --> {res}", list.name());
                            res
                        }
                        E::DataItemnumoflist { list, item } => {
                            let item = self.evaluate_expr(item)?;
                            let name = list.name();
                            let list = self.state.get_list_elements(list)?;
                            let pos = list
                                .iter()
                                .find_position(|i| i.scratch_eq(&item))
                                .map(|(pos, _)| pos + 1)
                                .unwrap_or(0);
                            let res = V::int_or_max(pos);
                            log::debug!(
                                "[{scope}] position of {item:?} in list {name:?} --> {res}"
                            );
                            res
                        }
                        E::DataItemoflist { list, index } => {
                            let index = self.evaluate_expr(index)?.as_int();
                            let name = list.name();
                            let list = self.state.get_list_elements(list)?;
                            let res = if index > 0 {
                                list.get((index - 1) as usize)
                                    .cloned()
                                    .unwrap_or(model::SValue::Text("".into()))
                            } else {
                                model::SValue::Text("".into())
                            };

                            log::debug!("[{scope}] element {index:?} of list {name:?} --> {res:?}");
                            res
                        }
                        E::SensingAnswer => {
                            let res = self.state.read_last_answer()?.clone();
                            log::debug!("[{scope}] read last answer --> {res:?}");
                            res
                        }
                        E::RDataVar { variable } => {
                            let res = self.state.get_variable(variable)?;
                            log::debug!(
                                "[{scope}] read variable {:?} --> {res:?}",
                                variable.name()
                            );
                            res
                        }
                        E::RDataList { list } => {
                            let res = self.state.get_list_value(list)?;
                            log::debug!(
                                "[{scope}] read textual representation of list {:?} --> {res:?}",
                                list.name()
                            );
                            res
                        }
                        E::OperatorMathop { operator, num } => {
                            let num = self.evaluate_expr(num)?.as_float();
                            let operator_name = operator.as_ref();
                            let res = V::Float(match operator_name {
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
                            });
                            log::debug!("[{scope}] ({operator_name} {num:?}) --> {res:?}");
                            res
                        }

                        E::ArgumentReporterStringNumber { value } => {
                            let res = self
                                .state
                                .procedure_arguments_nearest_string_number(value)?;
                            log::debug!("[{scope}] {value:?} --> {res:?}");
                            res
                        }
                        E::ArgumentReporterBoolean { value } => {
                            let res = self.state.procedure_arguments_nearest_boolean(value)?;
                            log::debug!("[{scope}] {value:?} --> {res}");
                            res
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
            let scope = kind.get_opcode();
            Ok(match kind {
                C::ArgumentReporterBoolean { value } => {
                    let res = self
                        .state
                        .procedure_arguments_nearest_boolean(value)?
                        .as_bool();

                    log::debug!("[{scope}] {value:?} --> {res:?}");
                    res
                }
                C::OperatorOr { operand1, operand2 } => {
                    let operand1 = self.evaluate_cmp(operand1.deref().clone())?;
                    let operand2 = self.evaluate_cmp(operand2.deref().clone())?;
                    let res = operand1 || operand2;
                    log::debug!("[{scope}] {operand1} || {operand2} --> {res:?}");
                    res
                }
                C::OperatorAnd { operand1, operand2 } => {
                    let operand1 = self.evaluate_cmp(operand1.o_id())?;
                    let operand2 = self.evaluate_cmp(operand2.o_id())?;
                    let res = operand1 && operand2;
                    log::debug!("[{scope}] {operand1} && {operand2} --> {res:?}");
                    res
                }
                C::OperatorNot { operand } => {
                    let op = self.evaluate_cmp(operand.o_id())?;
                    let res = !op;
                    log::debug!("[{scope}] !{op} --> {res:?}");
                    res
                }
                C::OperatorEquals { operand1, operand2 } => {
                    let op1 = self.evaluate_expr(operand1)?;
                    let op2 = self.evaluate_expr(operand2)?;
                    let res = op1.scratch_eq(&op2);
                    log::debug!("[{scope}] {op1} =? {op2} --> {res:?}");
                    res
                }
                C::OperatorGt { operand1, operand2 } => {
                    let op1 = self.evaluate_expr(operand1)?.as_float();
                    let op2 = self.evaluate_expr(operand2)?.as_float();
                    let res = op1 > op2;
                    log::debug!("[{scope}] {op1} > {op2} --> {res:?}");
                    res
                }
                C::OperatorLt { operand1, operand2 } => {
                    let op1 = self.evaluate_expr(operand1)?.as_float();
                    let op2 = self.evaluate_expr(operand2)?.as_float();
                    let res = op1 < op2;
                    log::debug!("[{scope}] {op1} < {op2} --> {res:?}");
                    res
                }
                C::OperatorContains { string1, string2 } => {
                    let string1 = self.evaluate_expr(string1)?;
                    let string2 = self.evaluate_expr(string2)?;
                    let res = string1.as_text().contains(string2.as_text().as_ref());
                    log::debug!("[{scope}] is {string2:?} contained in {string1:?} --> {res:?}");
                    res
                }
                C::DataListcontainsitem { list, item } => {
                    let item = self.evaluate_expr(item)?;
                    let name = list.name();
                    let list = self.state.get_list_elements(list)?;
                    let res = list.iter().any(|i| i.scratch_eq(&item));
                    log::debug!("[{scope}] is {item:?} contained in list {name:?} --> {res:?}");
                    res
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
