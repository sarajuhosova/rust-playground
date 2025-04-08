use crate::dna::codon::{as_initial, to_amino_acid, Codon, PartialCodon};
use crate::dna::data::alphabet::Alphabet;
use crate::dna::data::amino_acid::AminoAcid;
use crate::dna::protein::Protein;
use crate::dna::statistics::Statistics;

/// The state machine
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StateMachine {
    initialised: bool,
    partial_codon: PartialCodon,
    pub proteins: Vec<Protein>,
    pub count: usize,
    pub remainder: Vec<AminoAcid>
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

    fn add_to_protein(&mut self, amino_acid: AminoAcid) {
        self.proteins[self.count].push(amino_acid);
    }

    fn add_codon(&mut self, codon: Codon) {
        if !self.initialised {
            match as_initial(codon) {
                Some(amino_acid) => {
                    self.initialised = true;
                    self.proteins.push(vec![amino_acid]);
                }
                None => {
                    match to_amino_acid(codon) {
                        Some(amino_acid) => { self.remainder.push(amino_acid) }
                        None => {}
                    }
                }
            }
        } else {
            match to_amino_acid(codon) {
                Some(amino_acid) => { self.proteins[self.count].push(amino_acid) }
                None => { 
                    self.initialised = false;
                    self.count += 1;
                }
            }
        }
    }

    /// Takes and potentially modifies the state machine as it transitions based on the input.
    fn transition(&mut self, input: Alphabet) {
        match self.partial_codon {
            PartialCodon::StageZero => {
                self.partial_codon = PartialCodon::StageOne(input)
            }
            PartialCodon::StageOne(f) => {
                self.partial_codon = PartialCodon::StageTwo(f, input)
            }
            PartialCodon::StageTwo(f, s) => {
                self.add_codon((f, s, input));
                self.partial_codon = PartialCodon::StageZero
            }
        }
    }
}
