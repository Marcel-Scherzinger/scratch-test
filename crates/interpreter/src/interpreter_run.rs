use itertools::Itertools;
use model::{BlockKind, Id, ScratchExpr};

use crate::{Interpreter, RResult, RunError};

impl Interpreter {
    pub fn start(&mut self) -> RResult<()> {
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

        println!("execute: {kind:?}");

        use BlockKind as K;
        use model::StmtBlockKind as S;
        match &kind {
            K::EventWhenflagclicked => {}
            K::Stmt(stmt) => match &stmt {
                S::LooksSay { message } => {
                    println!("{message:?}");
                }
                S::LooksThink { message } => {
                    println!("think {message:?}");
                }
                S::ControlRepeatuntil {
                    condition,
                    substack,
                } => {
                    let stop_loop = self.evaluate_opt_cmp(condition.clone())?;
                    if !stop_loop {
                        if let Some(substack) = substack {
                            self.state.stack_push(stack_item)?;
                            self.state.stack_push(substack.clone())?;
                        } else {
                            Err(crate::RunError::ConditionLoopWithoutBodyNeverStops)?;
                        }
                    }
                }
                S::DataSetvariableto {
                    variable_to_set,
                    value,
                } => {
                    let value = self.evaluate_expr(value)?;
                    self.state.set_variable(variable_to_set, value)?;
                }

                // TODO TODO TODO
                _ => {}
            },
            K::Cmp(_) | K::Expr(_) => {
                Err(crate::RunError::UnexpectedNestingOfBlocks)?;
            }
        }

        self.state.stack_push_opt(next)?;
        Ok(())
    }

    fn evaluate_expr(&mut self, expr: &model::Expression) -> RResult<model::VariableValue> {
        use model::Expression as E;
        match expr {
            E::Lit(val) => Ok(val.clone()),
            E::Var(var) => self.state.get_variable(var),
            E::Blo(id) => {
                let b = self.state.get_expression_block(id)?;
                use model::BlockKind as B;
                use model::ExprBlockKind as E;
                use model::VariableValue as V;
                if let B::Expr(e) = b.inner() {
                    Ok(match e {
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
                                V::Text("".to_string())
                            } else {
                                V::Text(
                                    string
                                        .as_text()
                                        .chars()
                                        .nth((letter - 1) as usize)
                                        .unwrap_or_default()
                                        .to_string(),
                                )
                            }
                        }
                        E::OperatorRound { num } => {
                            let num = self.evaluate_expr(num)?.as_int();
                            V::Int(num)
                        }
                        E::OperatorLength { string } => {
                            let string = self.evaluate_expr(string)?;
                            // TODO: check for overflow
                            V::Int(string.as_text().len() as i64)
                        }
                        E::OperatorJoin { string1, string2 } => {
                            let string1 = self.evaluate_expr(string1)?;
                            let string2 = self.evaluate_expr(string2)?;
                            V::Text(string1.as_text().to_string() + string2.as_text().as_ref())
                        }
                        E::DataLengthoflist { list } => {
                            // TODO: range check
                            V::Int(self.state.get_list_elements(list)?.len() as i64)
                        }
                        E::DataItemnumoflist { list, item } => {
                            let item = self.evaluate_expr(item)?;
                            let list = self.state.get_list_elements(list)?;
                            let pos = list
                                .iter()
                                .find_position(|i| i.scratch_eq(&item))
                                .map(|(pos, _)| pos)
                                .unwrap_or(0);
                            // TODO: range check
                            V::Int(pos as i64)
                        }
                        E::SensingAnswer => V::Text(self.state.read_last_answer()?.into()),
                        E::RDataVar { variable } => self.state.get_variable(variable)?,
                        E::RDataList { list } => self.state.get_list_value(list)?,
                        E::OperatorMathop { operator, num } => {
                            let num = self.evaluate_expr(num)?.as_float();
                            V::Float(match operator.as_str() {
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

                        E::ArgumentReporterStringNumber { value }
                        | E::ArgumentReporterBoolean { value } => todo!(),
                    })
                } else {
                    Err(crate::RunError::UnexpectedBlockKind(id.clone()))
                }
            }
            E::Lis(list) => self.state.get_list_value(list),
        }
    }

    fn evaluate_cmp(&mut self, id: Id) -> RResult<bool> {
        let block = self.state.get_cmp_block(&id)?;
        let kind = block.inner();
        use model::CmpBlockKind as C;
        if let model::BlockKind::Cmp(kind) = kind {
            Ok(match kind {
                C::OperatorOr { operand1, operand2 } => {
                    let operand1 = self.evaluate_cmp(operand1.clone())?;
                    let operand2 = self.evaluate_cmp(operand2.clone())?;
                    operand1 || operand2
                }
                C::OperatorAnd { operand1, operand2 } => {
                    let operand1 = self.evaluate_cmp(operand1.clone())?;
                    let operand2 = self.evaluate_cmp(operand2.clone())?;
                    operand1 && operand2
                }
                C::OperatorNot { operand } => self.evaluate_cmp(operand.clone())?,
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
    fn evaluate_opt_cmp(&mut self, id: Option<Id>) -> RResult<bool> {
        if let Some(id) = id {
            self.evaluate_cmp(id)
        } else {
            Ok(false)
        }
    }
}
