// This module contains unit tests for various components of the grepq tool,
// including quality scoring, IUPAC conversion, pattern parsing, GC content calculation,
// and tetranucleotide frequency computation.

#[cfg(test)]
mod test_module {
    // Import modules used in tests.
    use crate::initialise;
    use crate::quality;
    use serde_json::Value;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_quality_scores() {
        // Additional comment: Testing quality score calculations using two different encodings.
        // Test: Verify average quality calculations for different quality encodings.
        // Testing Phred+33 encoding: Each 'I' (ASCII 73) gives a score of 73 - 33 = 40.
        assert_eq!(quality::average_quality(b"IIIII", "Phred+33"), 40.0);
        // Additional comment: Verifying Phred+64 encoding.
        // Testing Phred+64 encoding: Each 'h' (ASCII 104) gives 104 - 64 = 40.
        assert_eq!(quality::average_quality(b"hhhhh", "Phred+64"), 40.0);
        // Additional comment: Edge case when quality string is empty.
        // Testing with an empty quality string should return 0.0.
        assert_eq!(quality::average_quality(b"", "Phred+33"), 0.0);
    }

    #[test]
    fn test_iupac_conversion() {
        // Additional comment: Testing correct conversion for both unambiguous and ambiguous nucleotide codes.
        // Test: Verify conversion of IUPAC nucleotide codes to regex strings.
        // Standard bases remain unchanged.
        assert_eq!(initialise::convert_iupac_to_regex("ACTG"), "ACTG");
        // Ambiguous codes are converted to their regex equivalents.
        assert_eq!(initialise::convert_iupac_to_regex("N"), "[ACGT]");  // 'N' to [ACGT]
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

    #[test]
    #[should_panic(expected = "Illegal character found in pattern")]
    fn test_illegal_iupac_conversion() {
        // Additional comment: This test confirms that an illegal character triggers a panic.
        // Test: Ensure that the conversion panics when an illegal character is encountered.
        initialise::convert_iupac_to_regex("AXTG");
    }

    #[test]
    fn test_pattern_parsing() {
        // Additional comment: Validates that a multi-line pattern file is correctly parsed.
        // Test: Validate parsing of a multi-line pattern file.
        // Create a temporary file and write several regex patterns.
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
        // Write the patterns into the temporary file.
        temp_file.as_file().write_all(patterns.as_bytes()).unwrap();
        // Parse the file and ensure that exactly 30 regex patterns are obtained.
        let result = initialise::parse_patterns_file(temp_file.path().to_str().unwrap())
            .expect("Failed to parse patterns file");
        assert_eq!(result.0.patterns().len(), 30);
    }

    #[test]
    fn test_gc_content() {
        // Additional comment: Testing GC content calculation with various types of sequences.
        // Test: Confirm GC content is correctly calculated.
        assert_eq!(quality::gc_content(b"GCGC"), 100.0); // All bases are G or C.
        assert_eq!(quality::gc_content(b"ATAT"), 0.0);   // No GC bases.
        assert_eq!(quality::gc_content(b"ATGC"), 50.0);   // Equal mix of GC and AT.
        assert_eq!(quality::gc_content(b""), 0.0);         // Empty sequence returns 0.
        assert_eq!(quality::gc_content(b"GGCC"), 100.0);    // All bases are G or C.

        // Test with ambiguous bases: only standard bases should be counted.
        assert_eq!(quality::gc_content(b"GCNGC"), 100.0);
        assert_eq!(quality::gc_content(b"GCNNNN"), 100.0);
        assert_eq!(quality::gc_content(b"ATNNNN"), 0.0);
        assert_eq!(quality::gc_content(b"NNNNN"), 0.0);
        assert_eq!(quality::gc_content(b"GCRYSW"), 100.0);
    }

    #[test]
    fn test_tetranucleotide_frequencies() {
        // Additional comment: Verifying tetranucleotide frequency computation and sorting by percentage.
        // Test: Verify that tetranucleotide frequencies are computed correctly.
        let sequence = b"ATCGATCGATCG";
        let (frequencies, unique_count) = quality::tetranucleotide_frequencies(sequence, Some(4));
        let result: Vec<Value> = serde_json::from_str(&frequencies).unwrap();

        // There should be exactly 4 unique tetranucleotides.
        assert_eq!(unique_count, 4);
        // The JSON result must have 4 entries.
        assert_eq!(result.len(), 4);

        // Verify that the summed percentages are approximately 100%.
        let sum: f32 = result.iter().map(|v| v["percentage"].as_f64().unwrap() as f32).sum();
        assert!((sum - 100.0).abs() < 1e-3);

        // Test: Verify that tetranucleotides with ambiguous bases ('N') are skipped.
        let ambiguous = b"ATCGNATCGATCG";
        let (freq_amb, _count_amb) = quality::tetranucleotide_frequencies(ambiguous, Some(4));
        let result_amb: Vec<Value> = serde_json::from_str(&freq_amb).unwrap();
        assert!(result_amb.len() < 5);

        // Test: A fully ambiguous sequence should yield an empty frequency list.
        let all_ambiguous = b"NNNNNNNN";
        let (freq_all_amb, count_all_amb) =
            quality::tetranucleotide_frequencies(all_ambiguous, Some(4));
        assert_eq!(freq_all_amb, "[]");
        assert_eq!(count_all_amb, 0);
    }

    #[test]
    fn test_empty_tetranucleotide_frequencies() {
        // Additional comment: Confirm that sequences too short to form any tetranucleotide yield empty results.
        // Test: For sequences too short to form a tetranucleotide, expect no frequencies.
        let (json, count) = quality::tetranucleotide_frequencies(b"AAA", Some(4));
        assert_eq!(json, "[]");
        assert_eq!(count, 0);
    }
    
    #[test]
    fn test_iupac_case_insensitivity_conversion() {
        // Additional comment: Ensures that lowercase inputs are correctly transformed to uppercase.
        // Test: Ensure that input in lowercase is converted to uppercase
        // and that ambiguous bases convert correctly.
        assert_eq!(initialise::convert_iupac_to_regex("actg"), "ACTG");
        assert_eq!(initialise::convert_iupac_to_regex("n"), "[ACGT]");
    }

    #[test]
    fn test_plain_text_pattern_parsing() {
        // Additional comment: Tests that a plain text pattern file with a ".txt" extension is parsed correctly.
        // Test: Ensure plain-text pattern file parsing works.
        // Create a temporary file with a ".txt" extension containing two patterns.
        use std::fs;
        let temp_file = NamedTempFile::new().unwrap();
        let patterns = "ACTG\nN";
        temp_file.as_file().write_all(patterns.as_bytes()).unwrap();
        // Copy to a ".txt" file since parse_patterns_file checks the file extension.
        let txt_path = temp_file.path().with_extension("txt");
        fs::copy(temp_file.path(), &txt_path).unwrap();
        
        let result = initialise::parse_patterns_file(txt_path.to_str().unwrap())
            .expect("Failed to parse plain text pattern file");
        // Expect exactly 2 regex patterns.
        assert_eq!(result.0.patterns().len(), 2);
        
        // Cleanup the temporary ".txt" file.
        let _ = fs::remove_file(&txt_path);
    }

    #[test]
    fn test_complex_iupac_conversion() {
        // Additional comment: Validates that a complex IUPAC pattern converts into the expected regex.
        // Test: Verify conversion of a complex IUPAC pattern.
        // Input: "NYRSWKMBDHV" should be converted to the corresponding regex.
        let pattern = "NYRSWKMBDHV";
        // Expected conversion breakdown:
        // N -> [ACGT]
        // Y -> [CT]
        // R -> [AG]
        // S -> [CG]
        // W -> [AT]
        // K -> [GT]
        // M -> [AC]
        // B -> [CGT]
        // D -> [AGT]
        // H -> [ACT]
        // V -> [ACG]
        let expected = "[ACGT][CT][AG][CG][AT][GT][AC][CGT][AGT][ACT][ACG]";
        assert_eq!(initialise::convert_iupac_to_regex(pattern), expected);
    }

    #[test]
    fn test_invalid_variant_json() {
        // Additional comment: This test ensures that parsing a JSON pattern file containing a variant with an invalid DNA sequence fails.
        let json_content = r#"
        {
            "regexSet": {
                "regexSetName": "InvalidVariantTest",
                "regex": [
                    {
                        "regexName": "TestRegex",
                        "regexString": "ACTG",
                        "variants": [
                            {"variantName": "InvalidVariant", "variantString": "AXTG"}
                        ]
                    }
                ]
            }
        }
        "#;
        use std::io::Write;
        use tempfile::NamedTempFile;
        let temp_file = NamedTempFile::new().unwrap();
        temp_file.as_file().write_all(json_content.as_bytes()).unwrap();
        let result = initialise::parse_patterns_file(temp_file.path().to_str().unwrap());
        assert!(result.is_err(), "Expected error due to invalid variant DNA sequence.");
    }

    #[test]
    fn test_valid_json_pattern_parsing() {
        // Additional comment: This test verifies that a minimal valid JSON pattern file is parsed successfully.
        let json_content = r#"
        {
            "regexSet": {
                "regexSetName": "ValidTest",
                "regex": [
                    {
                        "regexName": "TestRegex",
                        "regexString": "ACTG"
                    }
                ],
                "headerRegex": "^@",
                "minimumSequenceLength": 4,
                "minimumAverageQuality": 30,
                "qualityEncoding": "Phred+33"
            }
        }
        "#;
        use std::io::Write;
        use std::fs;
        use tempfile::NamedTempFile;
        
        // Create a temp file with a .json extension
        let temp_file = NamedTempFile::new().unwrap();
        temp_file.as_file().write_all(json_content.as_bytes()).unwrap();
        let json_path = temp_file.path().with_extension("json");
        fs::copy(temp_file.path(), &json_path).unwrap();
        
        let result = initialise::parse_patterns_file(json_path.to_str().unwrap());
        assert!(result.is_ok(), "Expected valid JSON pattern parsing to succeed.");
        
        if let Ok((regex_set, header, min_len, min_qual, quality_enc, regex_names, variants)) = result {
            // Verify one regex is parsed.
            assert_eq!(regex_set.patterns().len(), 1);
            // Verify headerRegex was parsed.
            assert!(header.is_some());
            // Verify minimum sequence length.
            assert_eq!(min_len, Some(4));
            // Verify minimum quality.
            assert_eq!(min_qual, Some(30.0));
            // Verify quality encoding.
            assert_eq!(quality_enc, Some("Phred+33".to_string()));
            // Verify regex names length.
            assert_eq!(regex_names.len(), 1);
            // Verify no variants (since none are provided).
            assert_eq!(variants.len(), 0);
        }
        
        // Clean up the temp file
        let _ = fs::remove_file(&json_path);
    }
}
