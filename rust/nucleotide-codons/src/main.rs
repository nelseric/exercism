extern crate nucleotide_codons as codons;

fn main() {
  let d = codons::parse(make_pairs());
  println!("{:?}", d.name_for("TGT"));
}

fn make_pairs() -> Vec<(&'static str, &'static str)> {
    let grouped = vec![
        ("isoleucine", vec!["ATT", "ATC", "ATA"]),
        ("leucine", vec!["CTT", "CTC", "CTA", "CTG", "TTA", "TTG"]),
        ("valine", vec!["GTT", "GTC", "GTA", "GTG"]),
        ("phenylalanine", vec!["TTT", "TTC"]),
        ("methionine", vec!["ATG"]),
        ("cysteine", vec!["TGT", "TGC"]),
        ("alanine", vec!["GCT", "GCC", "GCA", "GCG"]),
        ("glycine", vec!["GGT", "GGC", "GGA", "GGG"]),
        ("proline", vec!["CCT", "CCC", "CCA", "CCG"]),
        ("threonine", vec!["ACT", "ACC", "ACA", "ACG"]),
        ("serine", vec!["TCT", "TCC", "TCA", "TCG", "AGT", "AGC"]),
        ("tyrosine", vec!["TAT", "TAC"]),
        ("tryptophan", vec!["TGG"]),
        ("glutamine", vec!["CAA", "CAG"]),
        ("asparagine", vec!["AAT", "AAC"]),
        ("histidine", vec!["CAT", "CAC"]),
        ("glutamic acid", vec!["GAA", "GAG"]),
        ("aspartic acid", vec!["GAT", "GAC"]),
        ("lysine", vec!["AAA", "AAG"]),
        ("arginine", vec!["CGT", "CGC", "CGA", "CGG", "AGA", "AGG"]),
        ("stop codon", vec!["TAA", "TAG", "TGA"])];
    let mut pairs = Vec::<(&'static str, &'static str)>::new();
    for (name, codons) in grouped.into_iter() {
        for codon in codons {
            pairs.push((codon, name));
        }
    };
    pairs.sort_by(|&(_, a), &(_, b)| a.cmp(b));
    return pairs
}