// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::collections::HashMap;
use crate::il::{BinOp,Region,Term};
use crate::ll::{Bytecode,Instruction};
use crate::util::*;

type Result = std::result::Result<(),CompilerError>;

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum CompilerError {
    /// An integer (or hex) literal is too large (i.e. exceeds `2^256`).
    LiteralOverflow,
    /// Attempt to read from an invalid memory region.
    InvalidMemoryAccess,
    /// Attempt to write something which doesn't exist, or is not an lval.
    InvalidLVal
}

// ============================================================================
// Compiler
// ============================================================================

pub struct Compiler<'a> {
    /// Access to the bytecode stream being constructed.
    bytecode: &'a mut Bytecode,
    /// Mapping from label names to their allocated labels in the
    /// underlying bytecode.
    labels: HashMap<String, usize>
}

impl<'a> Compiler<'a> {
    pub fn new(bytecode: &'a mut Bytecode) -> Self {
        Self{bytecode, labels: HashMap::new()}
    }

    /// Get the underlying bytecode label for a given label
    /// identifier.  If necessary, this allocates that label in the
    /// `Bytecode` object.
    pub fn label(&mut self, l: &str) -> usize {
        match self.labels.get(l) {
            Some(idx) => *idx,
            None => {
                // Allocate underlying index
                let idx = self.bytecode.fresh_label();
                // Cache it for later
                self.labels.insert(l.to_string(),idx);
                // Done
                idx
            }
        }
    }

    pub fn translate(&mut self, term: &Term) -> Result {
        match term {
            // Statements
            Term::Assert(e) => self.translate_assert(e),
            Term::Assignment(e1,e2) => self.translate_assignment(e1,e2),
            Term::Fail => self.translate_fail(),
            Term::Goto(l) => self.translate_goto(l),
            Term::IfGoto(e,l) => self.translate_ifgoto(e,l),
            Term::Label(l) => self.translate_label(l),
            Term::Revert(es) => self.translate_revert(es),
            Term::Succeed(es) => self.translate_succeed(es),
            Term::Stop => self.translate_stop(),
            // Expressions
            Term::Binary(bop,e1,e2) => self.translate_binary(*bop,e1,e2),
            Term::ArrayAccess(src,index) => self.translate_array_access(src,index),
            Term::MemoryAccess(_) => Err(CompilerError::InvalidMemoryAccess),
            // Values
            Term::Int(bytes) => self.translate_literal(bytes,10),
            Term::Hex(bytes) => self.translate_literal(bytes,16),
            //
        }
    }

    // ============================================================================
    // Statements
    // ============================================================================

    fn translate_assert(&mut self, expr: &Term) -> Result {
        // Allocate labels for true/false outcomes
        let lab = self.bytecode.fresh_label();
        // Translate conditional branch
        self.translate_conditional(expr,Some(lab),None)?;
        // False branch
        self.bytecode.push(Instruction::INVALID);
        // True branch
        self.bytecode.push(Instruction::JUMPDEST(lab));
        //
        Ok(())
    }

    fn translate_assignment(&mut self, lhs: &Term, rhs: &Term) -> Result {
        // Translate value being assigned
        self.translate(rhs)?;
        // Translate assignent itself
        match lhs {
            Term::ArrayAccess(src,idx) => {
                self.translate_assignment_array(&src,&idx)?;
            }
            _ => {
                return Err(CompilerError::InvalidLVal);
            }
        }
        //
        Ok(())
    }

    fn translate_assignment_array(&mut self, src: &Term, index: &Term) -> Result {
        match src {
            Term::MemoryAccess(r) => {
                self.translate_assignment_memory(*r,index)
            }
            _ => {
                Err(CompilerError::InvalidMemoryAccess)
            }
        }
    }

    fn translate_assignment_memory(&mut self, region: Region, address: &Term) -> Result {
        // Translate index expression
        self.translate(address)?;
        // Dispatch based on region
        match region {
            Region::Memory => self.bytecode.push(Instruction::MSTORE),
            Region::Storage => self.bytecode.push(Instruction::SSTORE),
            _ => {
                return Err(CompilerError::InvalidMemoryAccess);
            }
        };
        //
        Ok(())
    }

    fn translate_fail(&mut self) -> Result {
        self.bytecode.push(Instruction::INVALID);
        Ok(())
    }

    fn translate_goto(&mut self, label: &str) -> Result {
        // Allocate labels branch target
        let lab = self.label(label);
        // Translate unconditional branch
        self.bytecode.push(Instruction::PUSHL(lab));
        self.bytecode.push(Instruction::JUMP);
        //
        Ok(())
    }

    fn translate_ifgoto(&mut self, expr: &Term, label: &str) -> Result {
        // Allocate labels for true/false outcomes
        let lab = self.label(label);
        // Translate conditional branch
        self.translate_conditional(expr,Some(lab),None)
    }

    fn translate_label(&mut self, label: &str) -> Result {
        // Determine underlying index of label
        let lab = self.label(label);
        // Construct corresponding JumpDest
        self.bytecode.push(Instruction::JUMPDEST(lab));
        // Done
        Ok(())
    }

    fn translate_revert(&mut self, exprs: &[Term]) -> Result {
        self.translate_succeed_revert(Instruction::REVERT,exprs)
    }

    fn translate_succeed(&mut self, exprs: &[Term]) -> Result {
        if exprs.len() == 0 {
            self.bytecode.push(Instruction::STOP);
            Ok(())
        } else {
            self.translate_succeed_revert(Instruction::RETURN,exprs)
        }
    }

    fn translate_succeed_revert(&mut self, insn: Instruction, exprs: &[Term]) -> Result {
        if exprs.len() == 0 {
            self.bytecode.push(Instruction::PUSH(vec![0]));
            self.bytecode.push(Instruction::PUSH(vec![0]));
        } else {
            for i in 0 .. exprs.len() {
                let addr = (i * 0x20) as u128;
                self.translate(&exprs[i])?;
                self.bytecode.push(make_push(addr)?);
                self.bytecode.push(Instruction::MSTORE);
            }
            let len = (exprs.len() * 0x20) as u128;
            self.bytecode.push(Instruction::PUSH(vec![0]));
            self.bytecode.push(make_push(len)?);
        }
        self.bytecode.push(insn);
        Ok(())
    }

    fn translate_stop(&mut self) -> Result {
        self.bytecode.push(Instruction::STOP);
        Ok(())
    }

    // ============================================================================
    // Conditional Expressions
    // ============================================================================

    /// Translate a conditional expression using _short circuiting_
    /// semantics.  Since implementing short circuiting requires
    /// branching, we can exploit this to optimise other translations
    /// and reduce the number of branches required.  To do that, this
    /// method requires either a `true` target or a `false` target.
    /// If a `true` target is provided then, if the condition
    /// evaluates to something other than `0` (i.e. is `true`),
    /// control is transfered to this target.  Likewise, if the
    /// condition evaluates to `0` (i.e. `false`) the control is
    /// transfered to the `false` target.
    fn translate_conditional(&mut self, expr: &Term, true_lab: Option<usize>, false_lab: Option<usize>) -> Result {
	match expr {
	    Term::Binary(BinOp::LogicalAnd,l,r) => self.translate_conditional_conjunct(l,r,true_lab,false_lab),
	    Term::Binary(BinOp::LogicalOr,l,r) => self.translate_conditional_disjunct(l,r,true_lab,false_lab),		    _ => {
		self.translate_conditional_other(expr,true_lab,false_lab)
	    }
	}
    }

    /// Translate a logical conjunction as a conditional. Since
    /// such connectives require short circuiting, these must be
    /// implementing using branches.
    fn translate_conditional_conjunct(&mut self, lhs: &Term, rhs: &Term, true_lab: Option<usize>, false_lab: Option<usize>) -> Result {
	match (true_lab,false_lab) {
	    (Some(_),None) => {
		// Harder case
		let lab = self.bytecode.fresh_label();
		self.translate_conditional(lhs, None, Some(lab))?;
		self.translate_conditional(rhs, true_lab, None)?;
		self.bytecode.push(Instruction::JUMPDEST(lab));
	    }
	    (None,Some(_)) => {
		// Easy case
		self.translate_conditional(lhs, None, false_lab)?;
		self.translate_conditional(rhs, true_lab, false_lab)?;
	    }
	    (_,_) => unreachable!()
	}
        // Done
        Ok(())
    }

    /// Translate a logical disjunction as a conditional. Since
    /// such connectives require short circuiting, these must be
    /// implementing using branches.
    fn translate_conditional_disjunct(&mut self, lhs: &Term, rhs: &Term, true_lab: Option<usize>, false_lab: Option<usize>) -> Result {
	match (true_lab,false_lab) {
	    (None,Some(_)) => {
		// Harder case
		let lab = self.bytecode.fresh_label();
		self.translate_conditional(lhs, Some(lab), None)?;
		self.translate_conditional(rhs, None, false_lab)?;
		self.bytecode.push(Instruction::JUMPDEST(lab));
	    }
	    (Some(_),None) => {
		// Easy case
		self.translate_conditional(lhs, true_lab, None)?;
		self.translate_conditional(rhs, true_lab, false_lab)?;
	    }
	    (_,_) => unreachable!()
	}
        // Done
        Ok(())
    }

    /// Translate a conditional expression which cannot be translated
    /// by exploiting branches.  In such case, we have to generate the
    /// boolean value and dispatch based on that.
    fn translate_conditional_other(&mut self, expr: &Term, true_lab: Option<usize>, false_lab: Option<usize>) -> Result {
        // Translate conditional expression
        self.translate(expr)?;
        //
        match (true_lab,false_lab) {
            (Some(lab),None) => {
                self.bytecode.push(Instruction::PUSHL(lab));
                self.bytecode.push(Instruction::JUMPI);
            }
            (None,Some(lab)) => {
                self.bytecode.push(Instruction::ISZERO);
                self.bytecode.push(Instruction::PUSHL(lab));
                self.bytecode.push(Instruction::JUMPI);
            }
            (_,_) => {
                unreachable!("")
            }
        }
        //
        Ok(())
    }

    // ============================================================================
    // Binary Expressions
    // ============================================================================

    /// Translate a binary operation.  Observe that logical operations
    /// exhibit _short-circuit behaviour_.
    fn translate_binary(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        match bop {
            BinOp::LogicalAnd | BinOp::LogicalOr => {
                self.translate_logical_connective(bop,lhs,rhs)
            }
            _ => {
                self.translate_binary_arithmetic(bop,lhs,rhs)
            }
        }
    }

    /// Translate one of the logical connectives (e.g. `&&` or `||`).
    /// These are more challenging than standard binary operators because
    /// they exhibit _short circuiting behaviour_.
    fn translate_logical_connective(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        self.translate(lhs)?;
        self.bytecode.push(Instruction::DUP(1));
        if bop == BinOp::LogicalAnd {
            self.bytecode.push(Instruction::ISZERO);
        }
        // Allocate fresh label
        let lab = self.bytecode.fresh_label();
        self.bytecode.push(Instruction::PUSHL(lab));
        self.bytecode.push(Instruction::JUMPI);
        self.bytecode.push(Instruction::POP);
        self.translate(rhs)?;
        self.bytecode.push(Instruction::JUMPDEST(lab));
        // Done
        Ok(())
    }

    /// Translate a binary arithmetic operation or comparison.  This is
    /// pretty straightforward, as we just load items on the stack and
    /// perform the op.  Observe that the right-hand side is loaded onto
    /// the stack first.
    fn translate_binary_arithmetic(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        self.translate(rhs)?;
        self.translate(lhs)?;
        //
        match bop {
            // standard
            BinOp::Add => self.bytecode.push(Instruction::ADD),
            BinOp::Subtract => self.bytecode.push(Instruction::SUB),
            BinOp::Divide => self.bytecode.push(Instruction::DIV),
            BinOp::Multiply => self.bytecode.push(Instruction::MUL),
            BinOp::Remainder => self.bytecode.push(Instruction::MOD),
            BinOp::Equals => self.bytecode.push(Instruction::EQ),
            BinOp::LessThan => self.bytecode.push(Instruction::LT),
            BinOp::GreaterThan => self.bytecode.push(Instruction::GT),
            // non-standard
            BinOp::NotEquals => {
                self.bytecode.push(Instruction::EQ);
                self.bytecode.push(Instruction::ISZERO);
            }
            BinOp::LessThanOrEquals => {
                self.bytecode.push(Instruction::GT);
                self.bytecode.push(Instruction::ISZERO);
            }
            BinOp::GreaterThanOrEquals => {
                self.bytecode.push(Instruction::LT);
                self.bytecode.push(Instruction::ISZERO);
            }
            _ => {
                unreachable!();
            }
        }
        //
        Ok(())
    }

    // ============================================================================
    // Array Access Expressions
    // ============================================================================

    /// Translate an array access of the form `src[index]`.  The actual
    /// form of the translation depends on whether its a direct access
    /// (e.g. to storage or memory), or indirect (e.g. via a pointer to
    /// memory).
    fn translate_array_access(&mut self, src: &Term, index: &Term) -> Result {
        match src {
            Term::MemoryAccess(r) => {
                self.translate_memory_access(*r,index)
            }
            _ => {
                Err(CompilerError::InvalidMemoryAccess)
            }
        }
    }

    fn translate_memory_access(&mut self, region: Region, index: &Term) -> Result {
        // Translate index expression
        self.translate(index)?;
        // Dispatch based on region
        match region {
            Region::Memory => {
                self.bytecode.push(Instruction::MLOAD);
            }
            Region::Storage => {
                self.bytecode.push(Instruction::SLOAD);
            }
            Region::CallData => {
                self.bytecode.push(Instruction::CALLDATALOAD);
            }
        }
        //
        Ok(())
    }

    // ============================================================================
    // Values
    // ============================================================================

    fn translate_literal(&mut self, digits: &[u8], radix: u32) -> Result {
        let val = from_be_digits(digits,radix);
        self.bytecode.push(make_push(val)?);
        Ok(())
    }
}

/// Construct a push instruction from a value.
fn make_push(val: u128) -> std::result::Result<Instruction,CompilerError> {
    let bytes = to_be_bytes(val);
    // Sanity check size of literal
    if bytes.len() > 32 {
        // Too big!!
        Err(CompilerError::LiteralOverflow)
    } else {
        Ok(Instruction::PUSH(bytes))
    }
}
