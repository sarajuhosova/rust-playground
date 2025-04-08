//! Assignment 1: Impossible States
//!
//! In this exercise you will design and build a little state machine program in Rust that takes in
//!  a stream of DNA data. Your task is to gather some statistics about this data. Use the standard
//!  [DNA Codon table](https://en.wikipedia.org/w/index.php?title=DNA_and_RNA_codon_tables&oldid=1247507045#Standard_DNA_codon_table)
//!  to translate the codons (triples) into their corresponding `data::AminoAcid`. You may assume
//!  that the input data encodes one or more proteins by starting with a start codon, and that the
//!  input data starts on the edge of a codon. However, there may be non-protein-encoding codons
//!  before, between and after the start-stop sequences. Note that codons that are in a start
//!  position encode both the start of a protein _and_ an amino acid, *and the amino acid may be
//!  different that the usual one it codes for*:
//!
//! The wikipedia article also says:
//!
//! > In the standard code, the sequence ATG—read as methionine—can serve as a start codon and,
//! > <...>, initiates translation. <...>, start codons in the standard code may also
//! > include GTG or TTG; these codons normally represent valine and leucine, respectively, but as
//! > start codons they are translated as methionine or formylmethionine.
//!
//! Which means that ATG, GTG and TTG are start codons. They initiate translation, which means that
//! before you see one of these you should discard codons. GTG and TTG are normally interpreted as
//! valine and leucine, but if they're used as a start codon they are methionine and
//! formylmethionine.
//!
//! Note that you are already given some datastructures in the [`data`] module: [`Alphabet`] and
//! [`AminoAcid`]
//!
//! Step 1: Populate fields for [`PartialCodon`] and [`StateMachine`]
//! Step 2: Build the conversion from letters to amino acids [`StateMachine::read_amino_acid`]
//! Step 3: Implement the transition function from one state and the input to the next state. Each
//!         transition accepts one letter and changes the state of the [`StateMachine`]
//! Step 4: In [`Statistics::from`] extract just the finished protein from the state machine.
//!         Now make sure the methods on [`Statistics`] return the right values
//! Step 5: Write some unit tests
//!
//! Use:
//! ```bash
//! cargo build
//! // and maybe
//! cargo test
//! ```
//!
//! To see if your code works!

mod data;
mod state_machine;
mod protein;
mod statistics;
mod codon;

use data::alphabet::Alphabet;
use data::amino_acid::AminoAcid;
use state_machine::StateMachine;

/// An in-file unit testing module
#[cfg(test)]
mod tests {
    use super::*;

    /// Sequence 1, contains 1 protein sequence, one codon before, one codon after.
    const SEQ_1: &str = "CTA_ATG_CCT_CGT_TGA_TTT";

    /// Sequence 2, contains 2 protein sequences and an incomplete one at the end, some codons in
    ///  between.
    const SEQ_2: &str = "CTA_ATG_CCT_CGT_TGA_TTT_TTG_CCT_CAT_TAG_ATG_CCT";

    /// Testing the count of a particular amino acid
    #[test]
    fn test_leucine_count() {
        let example_input: Vec<Alphabet> =
            Alphabet::from(SEQ_1).expect("Input string should only contain letters CTAG");

        let state_machine = StateMachine::default();

        let statistics = state_machine.run(example_input.iter());

        assert_eq!(statistics.amino_acid_count(AminoAcid::L), 1);

        let example_input: Vec<Alphabet> =
            Alphabet::from(SEQ_2).expect("Input string should only contain letters CTAG");

        let state_machine = StateMachine::default();

        let statistics = state_machine.run(example_input.iter());

        assert_eq!(statistics.amino_acid_count(AminoAcid::L), 1);
    }

    /// Testing the reading of proteins
    #[test]
    fn test_protein_read() {
        use AminoAcid::*;

        let example_input: Vec<Alphabet> =
            Alphabet::from(SEQ_1).expect("Input string should only contain letters CTAG");

        let state_machine = StateMachine::default();

        let statistics = state_machine.run(example_input.iter());

        assert_eq!(statistics.protein_count(), 1);
        assert_eq!(&statistics.proteins()[0], &[M, P, R]);

        let example_input: Vec<Alphabet> =
            Alphabet::from(SEQ_2).expect("Input string should only contain letters CTAG");

        let state_machine = StateMachine::default();

        let statistics = state_machine.run(example_input.iter());

        assert_eq!(statistics.protein_count(), 2);
        assert_eq!(&statistics.proteins()[0], &[M, P, R]);
        assert_eq!(&statistics.proteins()[1], &[FM, P, H]);
    }
}
