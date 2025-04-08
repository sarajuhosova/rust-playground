use crate::dna::data::amino_acid::AminoAcid;
use crate::dna::protein::Protein;
use crate::dna::state_machine::StateMachine;

/// The statistics found in the input with the state machine
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Statistics {
    proteins: Vec<Protein>,
}

impl Statistics {
    /// Extract the statistics from the state machine
    pub fn from(state_machine: StateMachine) -> Self {
        let mut proteins = Vec::new();
        let mut latest = Protein::default();
        for i in state_machine.protein {
            if i.is_initial() {
                proteins.push(latest);
                latest = Protein::default();
            }
            latest.push(i);
        }
        proteins.push(latest);

        Self { proteins }
    }

    /// Returns the number of a particular codon read from the input
    pub fn amino_acid_count(&self, amino_acid: AminoAcid) -> usize {
        self.proteins.iter().flatten().filter(|a| **a == amino_acid).count()
    }

    /// Returns the number of codons read from the input
    pub fn protein_count(&self) -> usize {
        self.proteins.len()
    }

    /// Returns a vector of the found proteins
    pub fn proteins(&self) -> Vec<Protein> {
        self.proteins.clone()
    }
}
