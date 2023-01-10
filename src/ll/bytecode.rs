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
use crate::il::{Term};
use crate::il::{Compiler,CompilerError};
use crate::ll::instruction;
use crate::ll::{Instruction,Offset};

// ============================================================================
// Bytecode Programs
// ============================================================================

/// Represents a sequence of zero or more bytecodes which can be
/// turned, for example, into a hex string.  Likewise, they can be
/// decompiled or further optimised.
pub struct Bytecode {
    /// The underlying bytecode sequence.
    bytecodes: Vec<Instruction>,
    /// Counts the number of labels
    labels: usize
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode{bytecodes:Vec::new(), labels:0}
    }

    pub fn push(&mut self, insn: Instruction) {
        self.bytecodes.push(insn);
    }

    /// Get access to the raw sequence of instructions.
    pub fn instructions(&self) -> &[Instruction] {
	&self.bytecodes
    }

    /// Return the number of labels in the instruction sequence thus
    /// far.
    pub fn fresh_label(&mut self) -> usize {
        let lab = self.labels;
        self.labels = self.labels + 1;
        lab
    }

    /// Translate this sequence of bytecode instructions into a
    /// sequence of raw bytes.  This can still fail in a number of
    /// ways.  For example, the target for a `PUSHL` does not match
    /// any known `JUMPEST` label; Or, the stack size is exceeded,
    /// etc.
    pub fn to_bytes(&self) -> Result<Vec<u8>,instruction::Error> {
        let offsets = self.determine_offsets();
        let mut bytes = Vec::new();
        //
        for b in &self.bytecodes {
            // Encode instruction
            b.encode(&offsets,&mut bytes)?;
        }
        // Done
        Ok(bytes)
    }

    /// Determine the offsets of all labels within the instruction
    /// sequence.  This is non-trivial because labels which are
    /// further away affect the overall size of the bytecode sequence
    /// (hence, a label can affect the offset of itself or other
    /// labels).
    fn determine_offsets(&self) -> Vec<Offset> {
        // Construct initial set of empty offsets
        let mut offsets = vec![Offset(0); self.labels];
        // Iterate to a fixpoint.
        while self.update_offsets(&mut offsets) {
            // Keep going until no more changes!
        }
        //
        offsets
    }

    /// Update the offset information, noting whether or not anything
    /// actually changed.  The key is that as we recalculate offsets
    /// we may find the width has changed.  If this happens, we have
    /// to recalculate all offsets again assuming the larger width(s).
    fn update_offsets(&self, offsets: &mut [Offset]) -> bool {
        let mut changed = false;
        let mut offset = 0u16;
        // Calculate label offsets
        for b in &self.bytecodes {
            match b {
                Instruction::JUMPDEST(lab) => {
                    // Extract old offset
                    let oldoff = offsets[*lab];
                    // Construct new offset
                    let newoff = Offset(offset);
                    // Check!
                    if oldoff != newoff {
                        // Offset has changed, but the key thing is
                        // whether or not its _width_ has changed.
                        changed |= oldoff.width() != newoff.width();
                        // Update new offset.
                        offsets[*lab] = newoff;
                    }
                }
                Instruction::PUSH(bs) => offset = offset + (bs.len() as u16),
                Instruction::PUSHL(lab) => {
                    // This time calculate a more accurate figure.
                    offset = offset + offsets[*lab].width()
                }
                _ => {}
            }
            offset = offset + 1;
        }
        //
        changed
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn try_from(terms: &[Term]) -> Result<Bytecode,CompilerError> {
    let mut bytecode = Bytecode::new();
    let mut compiler = Compiler::new(&mut bytecode);
    // Translate statements one-by-one
    for t in terms {
        compiler.translate(t)?;
    }
    // Done
    Ok(bytecode)
}

// ============================================================================
// Trait implementstions
// ============================================================================

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl TryFrom<&[Term]> for Bytecode {
    type Error = CompilerError;

    fn try_from(terms: &[Term]) -> Result<Bytecode,Self::Error> {
        try_from(terms)
    }
}

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl<const N: usize> TryFrom<&[Term;N]> for Bytecode {
    type Error = crate::il::CompilerError;

    fn try_from(terms: &[Term;N]) -> Result<Bytecode,Self::Error> {
        try_from(terms)
    }
}

/// Try and translate a bytecode sequence into byte sequence.  This
/// can fail for a number of reasons (e.g. dangling branches, stack
/// depth exceeded, etc).
impl TryInto<Vec<u8>> for Bytecode {
    type Error = instruction::Error;

    fn try_into(self) -> Result<Vec<u8>,Self::Error> {
        self.to_bytes()
    }
}
