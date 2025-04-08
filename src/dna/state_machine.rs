use crate::dna::codon::{to_amino_acid, PartialCodon};
use crate::dna::data::alphabet::Alphabet;
use crate::dna::data::alphabet::Alphabet::*;
use crate::dna::data::amino_acid::AminoAcid::{FM, M};
use crate::dna::protein::Protein;
use crate::dna::statistics::Statistics;

/// The state machine
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StateMachine {
    initialised: bool,
    partial_codon: PartialCodon,
    pub protein: Protein,
}

impl StateMachine {
    /// Takes and potentially modifies the state machine as it is run on the inputs. Returns
    ///  statistics about the information found in the inputs.
    pub fn run<'a>(mut self, inputs: impl Iterator<Item = &'a Alphabet>) -> Statistics {
        for &input in inputs {
            self.transition(input);
        }

        Statistics::from(self)
    }

    /// Takes and potentially modifies the state machine as it transitions based on the input.
    fn transition(&mut self, input: Alphabet) {
        if self.initialised {
            match self.partial_codon {
                PartialCodon::StageZero =>
                    self.partial_codon = PartialCodon::StageOne(input),
                PartialCodon::StageOne(f) =>
                    self.partial_codon = PartialCodon::StageTwo(f, input),
                PartialCodon::StageTwo(f, s) => {
                    match to_amino_acid((f, s, input)) {
                        Some(amino_acid) => self.protein.push(amino_acid),
                        None => self.initialised = false,
                    }
                    self.partial_codon = PartialCodon::StageZero
                }
            }
        } else {
            match self.partial_codon {
                PartialCodon::StageZero => {
                    match input {
                        T | C | G => self.partial_codon = PartialCodon::StageOne(input),
                        _ => {}
                    }
                }
                PartialCodon::StageOne(f) => {
                    self.partial_codon = if input == T {
                        PartialCodon::StageTwo(f, T)
                    } else {
                        PartialCodon::StageZero
                    }
                }
                PartialCodon::StageTwo(f, _) => {
                    if input == G {
                        self.initialised = true;
                        match f {
                            A | G => self.protein.push(M),
                            T => self.protein.push(FM),
                            _ => { panic!("invalid state") }
                        }
                    }
                    self.partial_codon = PartialCodon::StageZero
                }
            }
        }
    }
}
