use crate::dna::data::amino_acid::AminoAcid;
use crate::dna::protein::Protein;
use crate::dna::state_machine::StateMachine;

/// The statistics found in the input with the state machine
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Statistics {
    proteins: Vec<Protein>,
    remainder: Vec<AminoAcid>,
}

impl Statistics {
    /// Extract the statistics from the state machine
    pub fn from(state_machine: StateMachine) -> Self {
        Self { 
            proteins: state_machine.proteins
                .iter().take(state_machine.count)
                .cloned().collect(),
            remainder: state_machine.remainder
        }
    }

    /// Returns the number of a particular codon read from the input
    pub fn amino_acid_count(&self, amino_acid: AminoAcid) -> usize {
        self.proteins.iter().flatten().filter(|a| **a == amino_acid).count()
            + self.remainder.iter().filter(|a| **a == amino_acid).count()
    }

    /// Returns the number of proteins read from the input
    pub fn protein_count(&self) -> usize {
        self.proteins.len()
    }

    /// Returns a vector of the found proteins
    pub fn proteins(&self) -> Vec<Protein> {
        self.proteins.clone()
    }
}
