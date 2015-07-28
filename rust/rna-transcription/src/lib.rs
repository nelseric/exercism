use std::convert::AsRef;

#[derive(PartialEq, Eq, Debug)]
pub struct RibonucleicAcid {
  nucleotides: String
}

impl RibonucleicAcid {
  pub fn new(nucleotides: &str) -> RibonucleicAcid {
    RibonucleicAcid { nucleotides: nucleotides.to_string() }
  }
}

impl AsRef<str> for RibonucleicAcid{
  fn as_ref(&self) -> &str {
    self.nucleotides.as_ref()
  }
}


fn transcribe_dna_to_rna(c: char) -> char {
  match c {
    'C' => 'G',
    'G' => 'C',
    'A' => 'U',
    'T' => 'A',
     _  =>  c
  }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DeoxyribonucleicAcid {
    nucleotides: String
}

impl DeoxyribonucleicAcid {
  pub fn new(nucleotides: &str) -> DeoxyribonucleicAcid {
    DeoxyribonucleicAcid { nucleotides: nucleotides.to_string() }
  }

  pub fn to_rna(&self) -> RibonucleicAcid {
    let mut rna = String::with_capacity(self.nucleotides.len());

    for c in self.nucleotides.chars() {
      rna.push(transcribe_dna_to_rna(c));
    }

    RibonucleicAcid::new(&rna)
  }
}