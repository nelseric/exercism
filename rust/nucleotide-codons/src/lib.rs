
#[derive(Debug)]
pub struct CodonInfo(Vec<(&'static str, &'static str)>);

impl CodonInfo {
  pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
    for coding in &self.0 {
      if coding.0 == codon {
        return Ok(coding.1)
      }
    }
    return Err("No Match")
  }
}

pub fn parse(codon_names: Vec<(&'static str, &'static str)>) -> CodonInfo {
  let mut codon_abbrev = vec![
    ("TGY", "cysteine"),
    ("GTN", "valine"),
    ("ATH", "isoleucine"),
    ("CGN", "arginine"),
    ("MGR", "arginine")];

  // codon_abbrev.push_all(&codon_names);
  for item in codon_names {
    codon_abbrev.push(item);
  }

  CodonInfo(codon_abbrev)
}

