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
use std::fmt;
use crate::ll::{Instruction,Instruction::*};
use crate::analysis::AbstractValue;

// ============================================================================
// Disassembly
// ============================================================================

/// Identifies a sequential block of instructions within the original
/// bytecode sequence.  That is, a sequence does not contain a jump
/// destination (other than at the very start), and ends either with a
/// terminating instruction (e.g. `RETURN`, `REVERT`, etc) or an
/// unconditional branch (to another block).
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Block {
    /// Starting offset (in bytes) of this block.
    pub start: usize,
    /// End offset (in bytes) of this block.  That is the first byte
    /// which is not part of this block.
    pub end: usize
}

impl Block {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end);
        //
        Block{start,end}
    }

    /// Check whether this block encloses (i.e. includes) the given
    /// bytecode address.
    pub fn encloses(&self, pc: usize) -> bool {
        self.start <= pc && pc < self.end
    }
}

// ============================================================================
// Abstract State
// ============================================================================

/// An abstract state provides information about the possible states
/// of the EVM at a given point.
pub trait AbstractState : Clone {
    /// Determines whether a given block is considered reachable or
    /// not.
    fn is_reachable(&self) -> bool;
    /// Apply a given instruction to this state, yielding an updated
    /// state.
    fn transfer(self, insn: &Instruction) -> Self;
    /// Apply a given branch to this stage, yielding an updated state
    /// at the point of the branch.
    fn branch(&self, target: usize, insn: &Instruction) -> Self;
    /// Merge this state with another, whilst returning a flag
    /// indicating whether anything changed.
    fn merge(&mut self, other: Self) -> bool;
    /// Determine value on top of stack
    fn peek(&self, n: usize) -> AbstractValue;
    /// Identify bottom value
    fn bottom() -> Self;
    /// Identify origin value
    fn origin() -> Self;
}

impl AbstractState for () {
    /// Default implementation indicates everything is reachable.
    fn is_reachable(&self) -> bool { true }
    /// Default implementation does nothing
    fn transfer(self, _insn: &Instruction) -> Self { self.clone() }
    /// Default implementation does nothing
    fn branch(&self, _target: usize, _insn: &Instruction) -> Self { self.clone() }
    /// Default implementation does nothing
    fn merge(&mut self, _other: Self) -> bool { false }
    /// Does nothing
    fn peek(&self,_n: usize) -> AbstractValue { AbstractValue::Unknown }
    /// Identify bottom value
    fn bottom() -> Self { () }
    /// Identify origin value
    fn origin() -> Self { () }
}

// ============================================================================
// Disassembly
// ============================================================================

/// Identifies all contiguous code blocks within the bytecode program.
/// Here, a block is a sequence of bytecodes terminated by either
/// `STOP`, `REVERT`, `RETURN` or `JUMP`.  Observe that a `JUMPDEST`
/// can only appear as the first instruction of a block.  In fact,
/// every reachable block (except the root block) begins with a
/// `JUMPDEST`.
pub struct Disassembly<'a,T = ()> {
    /// The bytes we are disassembling.
    bytes: &'a [u8],
    /// The set of known blocks (in order).
    blocks: Vec<Block>,
    /// The (incoming) contexts for each block.
    contexts: Vec<T>
}

impl<'a,T> Disassembly<'a,T>
where T:AbstractState {
    pub fn new(bytes: &'a [u8]) -> Self {
        // Perform linear scan of blocks
        let blocks = Self::scan_blocks(bytes);
        // Construct default contexts
        let mut contexts = vec![T::bottom(); blocks.len()];
        // Update origin context
        contexts[0] = T::origin();
        // Done
        Disassembly{bytes, blocks, contexts}
    }

    /// Get the state at a given program location.
    pub fn get_state(&self, loc: usize) -> T {
        // Determine enclosing block
        let bid = self.get_enclosing_block_id(loc);
        let blk = &self.blocks[bid];
        let mut ctx = self.contexts[bid].clone();
        let mut pc = blk.start;
        // Reconstruct state
        while pc < loc {
            // Decode instruction at the current position
            let insn = Instruction::decode(pc,&self.bytes);
            // Apply the transfer function!
            ctx = ctx.transfer(&insn);
            // Next instruction
            pc = pc + insn.length(&[]);
        }
        // Done
        ctx
    }

    /// Get the enclosing block for a given bytecode location.
    pub fn get_enclosing_block(&self, pc: usize) -> &Block {
        for i in 0..self.blocks.len() {
            if self.blocks[i].encloses(pc) {
                return &self.blocks[i];
            }
        }
        panic!("invalid bytecode address");
    }

    /// Determine whether a given block is currently considered
    /// reachable or not.  Observe the root block (`id=0`) is _always_
    /// considered reachable.
    pub fn is_block_reachable(&self, id: usize) -> bool {
        id == 0 || self.contexts[id].is_reachable()
    }

    /// Read a slice of bytes from the bytecode program, padding with
    /// zeros as necessary.
    pub fn read_bytes(&self, start: usize, end: usize) -> Vec<u8> {
        let n = self.bytes.len();

        if start >= n {
            vec![0; end-start]
        } else if end > n {
            // Determine lower potion
            let mut slice = self.bytes[start..n].to_vec();
            // Probably a more idiomatic way to do this?
            for _i in end .. n { slice.push(0); }
            //
            slice
        } else {
            // Easy case
            self.bytes[start..end].to_vec()
        }
    }

    /// Refine this disassembly to something (ideally) more precise
    /// use a fixed point dataflow analysis.  This destroys the
    /// original disassembly.
    pub fn refine<S>(self) -> Disassembly<'a,S>
    where S:AbstractState+From<T> {
        let mut contexts = Vec::new();
        // Should be able to do this with a map?
        for ctx in self.contexts {
            contexts.push(S::from(ctx));
        }
        // Done
        Disassembly{bytes: self.bytes, blocks: self.blocks, contexts}
    }

    /// Flattern the disassembly into a sequence of instructions.
    pub fn to_vec(&self) -> Vec<Instruction> {
        let mut insns = Vec::new();
        // Iterate blocks in order
        for i in 0..self.blocks.len() {
            let blk = &self.blocks[i];
            let ctx = &self.contexts[i];
            // Check for reachability
            if i == 0 || ctx.is_reachable() {
                // Disassemble block
                self.disassemble_into(blk,&mut insns);
            } else {
                // Not reachable, so must be data.
                let data = self.read_bytes(blk.start,blk.end);
                //
                insns.push(DATA(data));
            }
        }
        //
        insns
    }


    // ================================================================
    // Helpers
    // ================================================================

    /// Disassemble a given block into a sequence of instructions.
    fn disassemble_into(&self, blk: &Block, insns: &mut Vec<Instruction>) {
        let mut pc = blk.start;
        // Parse the block
        while pc < blk.end {
            // Decode instruction at the current position
            let insn = Instruction::decode(pc,&self.bytes);
            // Increment PC for next instruction
            pc = pc + insn.length(&[]);
            //
            insns.push(insn);
        }
    }

    /// Perform a linear scan splitting out the blocks.  This is an
    /// over approximation of the truth, as some blocks may turn out
    /// to be unreachable (e.g. they are data).
    fn scan_blocks(bytes: &[u8]) -> Vec<Block> {
        let mut blocks = Vec::new();
        // Current position in bytecodes
        let mut pc = 0;
        // Identifies start of current block.
        let mut start = 0;
        // Parse the block
        while pc < bytes.len() {
            // Decode instruction at the current position
            let insn = Instruction::decode(pc,&bytes);
            // Increment PC for next instruction
            pc = pc + insn.length(&[]);
            // Check whether terminating instruction
            match insn {
                JUMPDEST(_) => {
                    // Determine whether start of this block, or next
                    // block.
                    if (pc - 1) != start {
                        // Start of next block
                        blocks.push(Block::new(start,pc-1));
                        start = pc - 1;
                    }
                }
                INVALID|JUMP|RETURN|REVERT|STOP => {
                    blocks.push(Block::new(start,pc));
                    start = pc;
                }
                _ => {}
            }
        }
        // Append last block (if necessary)
        if start != pc {
            blocks.push(Block::new(start,pc));
        }
        // Done
        blocks
    }


    /// Determine the enclosing block number for a given bytecode
    /// address.
    fn get_enclosing_block_id(&self, pc: usize) -> usize {
        for i in 0..self.blocks.len() {
            if self.blocks[i as usize].encloses(pc) {
                return i;
            }
        }
        panic!("invalid bytecode address");
    }
}

impl<'a,T> Disassembly<'a,T>
where T:AbstractState+fmt::Display {

    /// Apply flow analysis to refine the results of this disassembly.
    pub fn build(mut self) -> Self {
        let mut changed = true;
        //
        while changed {
            // Reset indicator
            changed = false;
            // Iterate blocks in order
            for i in 0..self.blocks.len() {
                // Sanity check whether block unreachable.
                if !self.is_block_reachable(i) { continue; }
                // Yes, is reachable so continue.
                let blk = &self.blocks[i];
                let mut ctx = self.contexts[i].clone();
                let mut pc = blk.start;
                // println!("BLOCK (start={}, end={}): {:?}", pc, blk.end, i);
                // println!("CONTEXT (pc={}): {}", pc, ctx);
                // Parse the block
                while pc < blk.end {
                    // Decode instruction at the current position
                    let insn = Instruction::decode(pc as usize,&self.bytes);
                    // Check whether a branch is possible
                    if insn.can_branch() && ctx.peek(0).is_known() {
                        // Determine branch target
                        let target : u16 = ctx.peek(0).unwrap().into();
                        // Determine branch context
                        let branch_ctx = ctx.branch(target as usize,&insn);
                        // Convert target into block ID.
                        let block_id = self.get_enclosing_block_id(target as usize);
                        // println!("Branch: target={} (block {})",target,block_id);
                        // println!("Before merge (pc={}): {}", pc, self.contexts[block_id]);
                        // Merge in updated state
                        changed |= self.contexts[block_id].merge(branch_ctx);
                        // println!("After merge (pc={}): {}", pc, self.contexts[block_id]);
                    }
                    // Apply the transfer function!
                    // print!("{:#08x}: {}",pc,ctx);
                    ctx = ctx.transfer(&insn);
                    // println!(" ==>\t{:?}\t==> {}",insn,ctx);
                    // Next instruction
                    pc = pc + insn.length(&[]);
                }
                // Merge state into following block.
                if (i+1) < self.blocks.len() {
                    changed |= self.contexts[i+1].merge(ctx);
                }
            }
        }
        self
    }
}

// ============================================================================
// Disassemble Trait
// ============================================================================

/// Provides a default disassembly pipeline for standard types
/// (e.g. string slices, byte slices, etc).
pub trait Disassemble {
    fn disassemble<'a>(&'a self) -> Disassembly<'a,()>;
}

impl<T:AsRef<[u8]>> Disassemble for T {
    fn disassemble<'a>(&'a self) -> Disassembly<'a,()> {
        Disassembly::new(self.as_ref())
    }
}
