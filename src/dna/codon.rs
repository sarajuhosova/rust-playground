use crate::dna::codon::PartialCodon::StageZero;
use crate::dna::data::alphabet::Alphabet;
use crate::dna::data::amino_acid::AminoAcid;
use crate::dna::data::amino_acid::AminoAcid::*;
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

type Codon = (Alphabet, Alphabet, Alphabet);

pub fn to_amino_acid(codon: Codon) -> Option<AminoAcid> {
    match codon {
        (Alphabet::T, Alphabet::T, Alphabet::T | Alphabet::C) => Some(F),
        (Alphabet::T, Alphabet::T, Alphabet::A | Alphabet::G) => Some(L),
        (Alphabet::T, Alphabet::C, _)                         => Some(S),
        (Alphabet::T, Alphabet::A, Alphabet::T | Alphabet::C) => Some(Y),
        (Alphabet::T, Alphabet::A, Alphabet::A | Alphabet::G) => None,
        (Alphabet::T, Alphabet::G, Alphabet::T | Alphabet::C) => Some(C),
        (Alphabet::T, Alphabet::G, Alphabet::A)               => None,
        (Alphabet::T, Alphabet::G, Alphabet::G)               => Some(W),
        (Alphabet::C, Alphabet::T, _)                         => Some(L),
        (Alphabet::C, Alphabet::C, _)                         => Some(P),
        (Alphabet::C, Alphabet::A, Alphabet::T | Alphabet::C) => Some(H),
        (Alphabet::C, Alphabet::A, Alphabet::A | Alphabet::G) => Some(Q),
        (Alphabet::C, Alphabet::G, _)                         => Some(R),
        (Alphabet::A, Alphabet::T, Alphabet::G)               => Some(M),
        (Alphabet::A, Alphabet::T, _)                         => Some(I),
        (Alphabet::A, Alphabet::C, _)                         => Some(T),
        (Alphabet::A, Alphabet::A, Alphabet::T | Alphabet::C) => Some(N),
        (Alphabet::A, Alphabet::A, Alphabet::A | Alphabet::G) => Some(K),
        (Alphabet::A, Alphabet::G, Alphabet::T | Alphabet::C) => Some(S),
        (Alphabet::A, Alphabet::G, Alphabet::A | Alphabet::G) => Some(R),
        (Alphabet::G, Alphabet::T, _)                         => Some(V),
        (Alphabet::G, Alphabet::C, _)                         => Some(A),
        (Alphabet::G, Alphabet::A, Alphabet::T | Alphabet::C) => Some(D),
        (Alphabet::G, Alphabet::A, Alphabet::A | Alphabet::G) => Some(E),
        (Alphabet::G, Alphabet::G, _)                         => Some(G),
    }
}
