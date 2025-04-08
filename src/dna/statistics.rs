use crate::dna::data::amino_acid::AminoAcid;
use crate::dna::protein::Protein;
use crate::dna::state_machine::StateMachine;

/// The statistics found in the input with the state machine
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Statistics {
    protein: Protein,
}

impl Statistics {
    /// Extract the statistics from the state machine
    pub fn from(state_machine: StateMachine) -> Self {
        todo!()
    }

    /// Returns the number of a particular codon read from the input
    pub fn amino_acid_count(&self, amino_acid: AminoAcid) -> usize {
        todo!()
    }

    /// Returns the number of codons read from the input
    pub fn protein_count(&self) -> usize {
        todo!()
    }

    /// Returns a vector of the found proteins
    pub fn proteins(&self) -> Vec<Protein> {
        todo!()
    }
}
