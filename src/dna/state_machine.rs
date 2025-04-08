use crate::dna::data::alphabet::Alphabet;
use crate::dna::statistics::Statistics;

/// The state machine
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StateMachine {}

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
        todo!()
    }

    /// Takes a codon, and converts it into an amino acid. [`transition`](Self::transition)
    /// should at some point call this function once three letters have been accepted
    fn read_amino_acid(&mut self, codon: (Alphabet, Alphabet, Alphabet)) {
        todo!()
    }
}
