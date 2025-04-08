use crate::dna::codon::PartialCodon::StageZero;
use crate::dna::data::alphabet::Alphabet;
use crate::dna::data::amino_acid::AminoAcid;
use std::option::Option::*;

/// Represents how far along a codon we've parsed. Can be in one of three states.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PartialCodon {
    StageZero,
    StageOne(Alphabet),
    StageTwo(Alphabet, Alphabet)
}

impl Default for PartialCodon {
    fn default() -> Self { StageZero }
}

pub type Codon = (Alphabet, Alphabet, Alphabet);

pub fn is_initial(codon: Codon) -> bool {
    use Alphabet::*;
    
    match codon {
        (A, T, G) => true,
        (_, _, _)                     => false
    }
}

pub fn as_initial(codon: Codon) -> Option<AminoAcid> {
    use Alphabet::*;
    
    match codon {
        (A | G, T, G) => Some(AminoAcid::M),
        (T, T, G)     => Some(AminoAcid::FM),
        (_, _, _)     => None
    }
}

pub fn to_amino_acid(codon: Codon) -> Option<AminoAcid> {
    use Alphabet::*;
    
    match codon {
        (T, T, T | C) => Some(AminoAcid::F),
        (T, T, A | G) => Some(AminoAcid::L),
        (T, C, _)     => Some(AminoAcid::S),
        (T, A, T | C) => Some(AminoAcid::Y),
        (T, A, A | G) => None,
        (T, G, T | C) => Some(AminoAcid::C),
        (T, G, A)     => None,
        (T, G, G)     => Some(AminoAcid::W),
        (C, T, _)     => Some(AminoAcid::L),
        (C, C, _)     => Some(AminoAcid::P),
        (C, A, T | C) => Some(AminoAcid::H),
        (C, A, A | G) => Some(AminoAcid::Q),
        (C, G, _)     => Some(AminoAcid::R),
        (A, T, G)     => Some(AminoAcid::M),
        (A, T, _)     => Some(AminoAcid::I),
        (A, C, _)     => Some(AminoAcid::T),
        (A, A, T | C) => Some(AminoAcid::N),
        (A, A, A | G) => Some(AminoAcid::K),
        (A, G, T | C) => Some(AminoAcid::S),
        (A, G, A | G) => Some(AminoAcid::R),
        (G, T, _)     => Some(AminoAcid::V),
        (G, C, _)     => Some(AminoAcid::A),
        (G, A, T | C) => Some(AminoAcid::D),
        (G, A, A | G) => Some(AminoAcid::E),
        (G, G, _)     => Some(AminoAcid::G),
    }
}
