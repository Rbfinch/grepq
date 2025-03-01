#[cfg(test)]
mod test_module {
    use crate::initialise;
    use crate::quality;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_quality_scores() {
        assert_eq!(quality::average_quality(b"IIIII", "Phred+33"), 40.0);
        assert_eq!(quality::average_quality(b"hhhhh", "Phred+64"), 40.0);
        assert_eq!(quality::average_quality(b"", "Phred+33"), 0.0);
    }

    #[test]
    fn test_iupac_conversion() {
        assert_eq!(initialise::convert_iupac_to_regex("ACTG"), "ACTG");
        assert_eq!(initialise::convert_iupac_to_regex("N"), "[ACGT]");
        assert_eq!(initialise::convert_iupac_to_regex("Y"), "[CT]");
        assert_eq!(initialise::convert_iupac_to_regex("R"), "[AG]");
        assert_eq!(initialise::convert_iupac_to_regex("W"), "[AT]");
        assert_eq!(initialise::convert_iupac_to_regex("S"), "[CG]");
        assert_eq!(initialise::convert_iupac_to_regex("K"), "[GT]");
        assert_eq!(initialise::convert_iupac_to_regex("M"), "[AC]");
        assert_eq!(initialise::convert_iupac_to_regex("B"), "[CGT]");
        assert_eq!(initialise::convert_iupac_to_regex("D"), "[AGT]");
        assert_eq!(initialise::convert_iupac_to_regex("H"), "[ACT]");
        assert_eq!(initialise::convert_iupac_to_regex("V"), "[ACG]");
    }

    // #[test]
    // #[should_panic(expected = "Invalid IUPAC code: X")]
    // fn test_invalid_iupac() {
    //     initialise::convert_iupac_to_regex("X");
    // }

    #[test]
    fn test_pattern_parsing() {
        let temp_file = NamedTempFile::new().unwrap();
        let patterns = "[AG]AAT[AT]G[AG]CGGGG\n\
                       CCCCG[CT]C[AT]ATT[CT]\n\
                       GG[AG][ACGT]GGC[ACGT]GCAG\n\
                       CTGC[ACGT]GCC[ACTG][CT]CC\n\
                       G[CT][CT]G[CT]CGTCAGC\n\
                       GCTGACG[AG]C[AG][AG]C\n\
                       C[ACG]GC[ACGT]GC[CT]GCGG\n\
                       CCGC[AG]GC[AGCT]GC[CGT]G\n\
                       TAGA[AT]ACCC[ACGT][ACGT]G\n\
                       C[ACGT][ACGT]GGGT[AT]TCTA\n\
                       CGAGCGCAACCC\n\
                       GGGTTGCGCTCG\n\
                       AGG[CT]GGGGA[CT]GA\n\
                       TC[AG]TCCCC[AG]CCT\n\
                       [GC][CT]GGCG[ACGT]ACGGG\n\
                       CCCGT[ACGT]CGCC[AG][GC]\n\
                       GA[AG]GAACCTTAC\n\
                       GTAAGGTTC[CT]TC\n\
                       GTGGTTTAATTC\n\
                       GAATTAAACCAC\n\
                       G[CT]AC[AT]C[AT]CCGCC\n\
                       GGCGG[AT]G[AT]GT[AG]C\n\
                       GC[TG]ACACACG[CT]G\n\
                       C[AG]CGTGTGT[AC]GC\n\
                       G[AC]GGTGAAAT[TG]C\n\
                       G[AC]ATTTCACC[TG]C\n\
                       AT[CT][AC]TGGCTCAG\n\
                       CTGAGCCA[TG][AG]AT\n\
                       AGTC[AG]TAACAAG\n\
                       CTTGTTA[CT]GACT";
        temp_file.as_file().write_all(patterns.as_bytes()).unwrap();

        let result = initialise::parse_patterns_file(temp_file.path().to_str().unwrap())
            .expect("Failed to parse patterns file");
        assert_eq!(result.0.patterns().len(), 30);
    }

    #[test]
    fn test_gc_content() {
        assert_eq!(quality::gc_content(b"GCGC"), 100.0);
        assert_eq!(quality::gc_content(b"ATAT"), 0.0);
        assert_eq!(quality::gc_content(b"ATGC"), 50.0);
        assert_eq!(quality::gc_content(b""), 0.0);
        assert_eq!(quality::gc_content(b"GGCC"), 100.0);
    }

    #[test]
    fn test_tetranucleotide_frequencies() {
        let sequence = b"ATCGATCGATCG";
        let frequencies = quality::tetranucleotide_frequencies(sequence);
        let result: HashMap<String, f64> = serde_json::from_str(&frequencies).unwrap();

        // Check if we have the expected tetranucleotides
        assert!(result.contains_key("ATCG"));
        assert!(result.contains_key("TCGA"));
        assert!(result.contains_key("CGAT"));
        assert!(result.contains_key("GATC"));

        // Check if frequencies sum to approximately 1.0
        let sum: f64 = result.values().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }
}
