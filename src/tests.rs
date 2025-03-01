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
        // Regular sequences
        assert_eq!(quality::gc_content(b"GCGC"), 100.0);
        assert_eq!(quality::gc_content(b"ATAT"), 0.0);
        assert_eq!(quality::gc_content(b"ATGC"), 50.0);
        assert_eq!(quality::gc_content(b""), 0.0);
        assert_eq!(quality::gc_content(b"GGCC"), 100.0);

        // Sequences with ambiguous bases
        assert_eq!(quality::gc_content(b"GCNNGC"), 100.0); // Only GC bases counted
        assert_eq!(quality::gc_content(b"GCNNNN"), 100.0); // Only GC bases counted
        assert_eq!(quality::gc_content(b"ATNNNN"), 0.0); // Only AT bases counted
        assert_eq!(quality::gc_content(b"NNNNN"), 0.0); // No valid bases
        assert_eq!(quality::gc_content(b"GCRYSW"), 50.0); // Only unambiguous G,C counted
    }

    #[test]
    fn test_tetranucleotide_frequencies() {
        // Regular sequence
        let sequence = b"ATCGATCGATCG";
        let (frequencies, unique_count) = quality::tetranucleotide_frequencies(sequence);
        let result: HashMap<String, f64> = serde_json::from_str(&frequencies).unwrap();

        assert!(result.contains_key("ATCG"));
        assert!(result.contains_key("TCGA"));
        assert!(result.contains_key("CGAT"));
        assert!(result.contains_key("GATC"));
        assert_eq!(unique_count, 4);

        // Check if frequencies sum to approximately 100.0
        let sum: f64 = result.values().sum();
        assert!((sum - 100.0).abs() < 1e-10);

        // Check if values are properly rounded to 5 significant digits
        for value in result.values() {
            let rounded_check = (value * 100000.0).round() / 100000.0;
            assert_eq!(value, &rounded_check);
        }

        // Sequence with ambiguous bases
        let ambiguous = b"ATCGNATCGATCG";
        let (freq_amb, _count_amb) = quality::tetranucleotide_frequencies(ambiguous);
        let result_amb: HashMap<String, f64> = serde_json::from_str(&freq_amb).unwrap();

        // Should skip the tetranucleotide containing 'N'
        assert!(!result_amb.contains_key("ATCG"));
        assert!(result_amb.contains_key("GATC"));
        assert!(result_amb.len() < 5); // Should have fewer tetranucleotides due to N

        // Sequence with all ambiguous bases
        let all_ambiguous = b"NNNNNNNN";
        let (freq_all_amb, count_all_amb) = quality::tetranucleotide_frequencies(all_ambiguous);
        assert_eq!(freq_all_amb, "{}");
        assert_eq!(count_all_amb, 0);

        // Check if frequencies sum to approximately 1.0
        let sum: f64 = result.values().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }
}
