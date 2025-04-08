use std::str::FromStr;

/// The input alphabet
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Alphabet {
    /// Thymine
    T,
    /// Cytosine
    C,
    /// Adenine
    A,
    /// Guanine
    G,
}

impl FromStr for Alphabet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(());
        }
        match s {
            "T" | "t" => Ok(Alphabet::T),
            "C" | "c" => Ok(Alphabet::C),
            "A" | "a" => Ok(Alphabet::A),
            "G" | "g" => Ok(Alphabet::G),
            _ => Err(()),
        }
    }
}

impl Alphabet {
    /// Build an input vector from a string of characters.
    pub fn from(s: &str) -> Result<Vec<Self>, ()> {
        let mut result = Vec::new();
        for c in s.chars() {
            if c == '_' {
                // ignore
                continue;
            }
            result.push(c.to_string().parse()?);
        }
        Ok(result)
    }
}
