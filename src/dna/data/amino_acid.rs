//! This module defines the input alphabet of the assignment.
//! https://gitlab.ewi.tudelft.nl/reit/course-rust-basics/-/blob/main/assignment1/src/data.rs

/// Codons from the Standard DNA codon table
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AminoAcid {
    /// FormylMethionine
    FM,
    /// Alanine
    A,
    /// Cysteine
    C,
    /// Aspartic Acid
    D,
    /// Glumatic Acid
    E,
    /// Phenylalanine
    F,
    /// Glycine
    G,
    /// Histidine
    H,
    /// Isoleucine
    I,
    /// Lysine
    K,
    /// Leucine
    L,
    /// Methionine
    M,
    /// Asparagine
    N,
    /// Proline
    P,
    /// Glumatine
    Q,
    /// Arginine
    R,
    /// Serine
    S,
    /// Threonine
    T,
    /// Valine
    V,
    /// Tryptophan
    W,
    /// Tyrosine
    Y,
}

impl AminoAcid {

    pub fn is_initial(&self) -> bool {
        match self {
            AminoAcid::FM | AminoAcid::M => true,
            _ => false,
        }
    }

}
