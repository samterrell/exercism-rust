#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(position) = dna.find(|c| !"GCTA".contains(c)) {
            Err(position)
        } else {
            Ok(Self(String::from(dna.to_uppercase())))
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!(),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(position) = rna.find(|c| !"CGAU".contains(c)) {
            Err(position)
        } else {
            Ok(Self(String::from(rna.to_uppercase())))
        }
    }
}
