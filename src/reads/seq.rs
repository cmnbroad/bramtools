
// DNA base
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DNABase { A, C, G, T, N }

impl DNABase {
    pub fn from_char(c : char) -> Option<DNABase> {
        match c.to_ascii_uppercase() {
            'A' => Some(DNABase::A),
            'C' => Some(DNABase::C),
            'G' => Some(DNABase::G),
            'T' => Some(DNABase::T),
            'N' => Some(DNABase::N),
            _ => None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seq<T: Sized> {
    bases: Vec<T>,
    quals: Vec<u8>
}

impl<T> Seq<T> {
    pub fn get_bases(&self) -> &[T] {
        return &self.bases;
    }
}

pub type DNASeq = Seq<DNABase>;

impl DNASeq {
    pub fn new(bases_str: &str) -> DNASeq {
        //chars: Vec<DNABase> = Vec<DNABase>::with_capacity(len(basesString));
        let mut bases: Vec<DNABase> = vec![];
        for c in bases_str.chars() {
            bases.push(DNABase::from_char(c).unwrap());
        }
        return DNASeq { bases: bases, quals: vec![] };
    }

}

#[cfg(test)]
mod test {
    use crate::reads::seq::DNASeq;

    #[test]
    fn test_seq() {
        let _seq = DNASeq::new("ACGT");
        let _i = 37;
    }

    #[test]
    fn test_fasta() {
        let _seq = DNASeq::new("ACGT");
        let _i = 37;
    }

}
