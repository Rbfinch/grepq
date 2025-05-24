// MIT License

// Copyright (c) 2024 - present Nicholas D. Crosbie

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
        assert_eq!(initialise::convert_iupac_to_regex("N"), "[ACGT]"); // 'N' to [ACGT]
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
        assert_eq!(quality::gc_content(b"ATAT"), 0.0); // No GC bases.
        assert_eq!(quality::gc_content(b"ATGC"), 50.0); // Equal mix of GC and AT.
        assert_eq!(quality::gc_content(b""), 0.0); // Empty sequence returns 0.
        assert_eq!(quality::gc_content(b"GGCC"), 100.0); // All bases are G or C.

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
        let sum: f32 = result
            .iter()
            .map(|v| v["percentage"].as_f64().unwrap() as f32)
            .sum();
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
        temp_file
            .as_file()
            .write_all(json_content.as_bytes())
            .unwrap();
        let result = initialise::parse_patterns_file(temp_file.path().to_str().unwrap());
        assert!(
            result.is_err(),
            "Expected error due to invalid variant DNA sequence."
        );
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
        use std::fs;
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create a temp file with a .json extension
        let temp_file = NamedTempFile::new().unwrap();
        temp_file
            .as_file()
            .write_all(json_content.as_bytes())
            .unwrap();
        let json_path = temp_file.path().with_extension("json");
        fs::copy(temp_file.path(), &json_path).unwrap();

        let result = initialise::parse_patterns_file(json_path.to_str().unwrap());
        assert!(
            result.is_ok(),
            "Expected valid JSON pattern parsing to succeed."
        );

        if let Ok((regex_set, header, min_len, min_qual, quality_enc, regex_names, variants)) =
            result
        {
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

    // #[test]
    // fn test_tetranucleotide_frequencies_variable_kmer_size() {
    //     // Test: Verify that k-mer frequencies work with different sizes.
    //     let sequence = b"ATCGATCGATCG";

    //     // Test with trinucleotides (k=3)
    //     let (frequencies_3, unique_count_3) =
    //         quality::tetranucleotide_frequencies(sequence, Some(3));
    //     let result_3: Vec<Value> = serde_json::from_str(&frequencies_3).unwrap();
    //     assert_eq!(unique_count_3, 4);
    //     assert_eq!(result_3.len(), 4);

    //     // Test with pentanucleotides (k=5)
    //     let (frequencies_5, unique_count_5) =
    //         quality::tetranucleotide_frequencies(sequence, Some(5));
    //     let result_5: Vec<Value> = serde_json::from_str(&frequencies_5).unwrap();
    //     assert_eq!(unique_count_5, 4);
    //     assert_eq!(result_5.len(), 4);

    //     // Test with default value (should use k=4)
    //     let (frequencies_default, unique_count_default) =
    //         quality::tetranucleotide_frequencies(sequence, None);
    //     let result_default: Vec<Value> = serde_json::from_str(&frequencies_default).unwrap();
    //     assert_eq!(unique_count_default, 4);
    //     assert_eq!(result_default.len(), 4);
    // }

    #[test]
    fn test_tetranucleotide_frequencies_kmer_sorting() {
        // Test: Ensure k-mers are sorted by decreasing frequency
        let sequence = b"AAAAAAAAAATTTTGGGCCC";
        let (frequencies, _) = quality::tetranucleotide_frequencies(sequence, Some(4));
        let result: Vec<Value> = serde_json::from_str(&frequencies).unwrap();

        // Check that results are sorted by decreasing percentage
        let mut last_percentage = 100.0;
        for entry in result {
            let current_percentage = entry["percentage"].as_f64().unwrap();
            assert!(
                current_percentage <= last_percentage,
                "Frequencies not sorted properly"
            );
            last_percentage = current_percentage;
        }
    }

    #[test]
    fn test_additional_quality_encodings() {
        // Test Solexa quality encoding
        assert_eq!(quality::average_quality(b"IIIII", "Solexa"), 40.0);

        // Test Illumina 1.3+ encoding
        assert_eq!(quality::average_quality(b"IIIII", "Illumina 1.3+"), 40.0);

        // Test Illumina 1.5+ encoding
        assert_eq!(quality::average_quality(b"IIIII", "Illumina 1.5+"), 40.0);

        // Test with mixed quality scores
        assert_eq!(
            quality::average_quality(b"I@BCD", "Phred+33"),
            (40.0 + 31.0 + 33.0 + 34.0 + 35.0) / 5.0
        );

        // Test with very low quality scores
        assert_eq!(quality::average_quality(b"!!!!", "Phred+33"), 0.0);

        // Test with very high quality scores
        assert_eq!(quality::average_quality(b"~~~~", "Phred+33"), 93.0);
    }

    #[test]
    fn test_gc_content_with_long_sequences() {
        // Test with a longer sequence to ensure performance is reasonable
        let long_sequence = b"GCGCGCGCGCGCGCGCGCGCATATATATATATATAT";
        assert_eq!(quality::gc_content(long_sequence), 55.555557);

        // Test with a sequence containing a mix of all bases
        let mixed_sequence = b"ACGTACGTACGTACGTACGTACGT";
        assert_eq!(quality::gc_content(mixed_sequence), 50.0);
    }

    #[test]
    fn test_canonical_tetranucleotide_frequencies() {
        // Test: Verify that canonical tetranucleotide frequencies are computed correctly
        let sequence = b"ATCGATCGATCG";
        let (frequencies, unique_count) =
            quality::canonical_tetranucleotide_frequencies(sequence, Some(4));
        let result: Vec<Value> = serde_json::from_str(&frequencies).unwrap();

        // Check that we have fewer or equal unique tetranucleotides compared to regular method
        // due to merging of reverse complements
        let (_, reg_unique_count) = quality::tetranucleotide_frequencies(sequence, Some(4));
        assert!(
            unique_count <= reg_unique_count,
            "Canonical count should be <= regular count"
        );

        // The JSON result must have entries
        assert!(!result.is_empty());

        // Verify that the summed percentages are approximately 100%
        let sum: f32 = result
            .iter()
            .map(|v| v["percentage"].as_f64().unwrap() as f32)
            .sum();
        assert!((sum - 100.0).abs() < 1e-3);
    }

    #[test]
    fn test_canonical_tetranucleotide_with_palindromic_sequence() {
        // Test with a palindromic sequence
        let palindrome = b"ACGTACGT"; // Contains ACGT which is its own reverse complement
        let (frequencies, _) = quality::canonical_tetranucleotide_frequencies(palindrome, None);
        let result: Vec<Value> = serde_json::from_str(&frequencies).unwrap();

        // Check that palindromic k-mers are properly counted
        // Find ACGT in the results
        let acgt_entry = result
            .iter()
            .find(|&v| v["tetra"].as_str().unwrap() == "ACGT");
        assert!(
            acgt_entry.is_some(),
            "Palindromic k-mer ACGT should be present"
        );
    }

    #[test]
    fn test_empty_canonical_tetranucleotide_frequencies() {
        // Test: For sequences too short to form a tetranucleotide, expect no frequencies
        let (json, count) = quality::canonical_tetranucleotide_frequencies(b"AAA", Some(4));
        assert_eq!(json, "[]");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_canonical_tetranucleotide_with_ambiguous_bases() {
        // Test: Verify that tetranucleotides with ambiguous bases ('N') are skipped
        let ambiguous = b"ATCGNATCGATCG";
        let (freq_amb, _) = quality::canonical_tetranucleotide_frequencies(ambiguous, Some(4));
        let result_amb: Vec<Value> = serde_json::from_str(&freq_amb).unwrap();

        // There should be results, but fewer windows than the sequence length - k + 1
        // due to skipping windows with N
        assert!(!result_amb.is_empty());
        assert!(result_amb.len() < ambiguous.len() - 4 + 1);

        // A fully ambiguous sequence should yield an empty frequency list
        let all_ambiguous = b"NNNNNNNN";
        let (freq_all_amb, count_all_amb) =
            quality::canonical_tetranucleotide_frequencies(all_ambiguous, Some(4));
        assert_eq!(freq_all_amb, "[]");
        assert_eq!(count_all_amb, 0);
    }

    #[test]
    fn test_canonical_tetranucleotide_frequencies_sorting() {
        // Test: Ensure canonical k-mers are sorted by decreasing frequency
        let sequence = b"AAAAAAAAAATTTTGGGCCC";
        let (frequencies, _) = quality::canonical_tetranucleotide_frequencies(sequence, Some(4));
        let result: Vec<Value> = serde_json::from_str(&frequencies).unwrap();

        // Check that results are sorted by decreasing percentage
        let mut last_percentage = 100.0;
        for entry in result {
            let current_percentage = entry["percentage"].as_f64().unwrap();
            assert!(
                current_percentage <= last_percentage,
                "Frequencies not sorted properly"
            );
            last_percentage = current_percentage;
        }
    }

    #[test]
    fn test_canonical_tetranucleotide_frequencies_limit() {
        // Test: Verify that the limit parameter works correctly
        let sequence = b"ACGTACGTACGTACGTTTTTTTCCCCCGGGGGAAAAA";

        // Test with a reasonable limit
        let (frequencies_limited, _) =
            quality::canonical_tetranucleotide_frequencies(sequence, Some(3));
        let result_limited: Vec<Value> = serde_json::from_str(&frequencies_limited).unwrap();
        assert_eq!(result_limited.len(), 3, "Should limit to exactly 3 results");

        // Test with no limit
        let (frequencies_unlimited, _) =
            quality::canonical_tetranucleotide_frequencies(sequence, None);
        let result_unlimited: Vec<Value> = serde_json::from_str(&frequencies_unlimited).unwrap();
        assert!(
            result_unlimited.len() > 3,
            "Should return more than 3 results when unlimited"
        );
    }

    #[test]
    fn test_compare_regular_and_canonical_frequencies() {
        // Test: Compare regular and canonical tetranucleotide frequencies
        let sequence = b"ACGTACGTACGTACGTTGCATGCA";

        // Get both types of frequencies
        let (reg_freq, reg_count) = quality::tetranucleotide_frequencies(sequence, None);
        let (can_freq, can_count) = quality::canonical_tetranucleotide_frequencies(sequence, None);

        let reg_result: Vec<Value> = serde_json::from_str(&reg_freq).unwrap();
        let can_result: Vec<Value> = serde_json::from_str(&can_freq).unwrap();

        // Canonical count should be less than or equal to regular count
        assert!(
            can_count <= reg_count,
            "Canonical count should be <= regular count"
        );

        // The canonical result should have fewer or the same number of entries
        assert!(
            can_result.len() <= reg_result.len(),
            "Canonical result should have fewer or equal entries"
        );
    }
}
