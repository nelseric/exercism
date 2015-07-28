class DNA(genome: String) {

    val nucleotides = Seq('A', 'T', 'C', 'G')

    if (!genome.matches("[GCTA]*"))
        throw new IllegalArgumentException("Invalid nucleotides")
    def nucleotideCounts() = {
        nucleotides.map {
            nt: Char => (nt, count(nt))
        }.toMap
    }

    def count(nucleotide: Char) = {
        if (!nucleotides.contains(nucleotide))
            throw new IllegalArgumentException("Invalid nucleotide")
        genome.count(_ == nucleotide)
    }
}