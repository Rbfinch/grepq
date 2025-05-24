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

use crate::kmer_utils::{get_canonical_kmer, KmerSize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Function: average_quality
// Calculates the average quality score for a sequence using the specified quality encoding.
pub fn average_quality(quality: &[u8], quality_encoding: &str) -> f32 {
    // Determine the offset based on the quality encoding.
    // Phred+33: ASCII offset 33, Phred+64: ASCII offset 64.
    let offset = match quality_encoding {
        "Phred+33" => 33,
        "Phred+64" => 64,
        _ => 33, // Default to Phred+33 if unknown encoding
    };

    // Initialize sum and count.
    let mut sum = 0;
    let count = quality.len() as i32;

    // Iterate over quality bytes, subtracting the offset to get the true quality score.
    for &q in quality {
        sum += q as i32 - offset;
    }

    // Calculate the average quality score. Return 0.0 if no bases are present.
    if count > 0 {
        sum as f32 / count as f32
    } else {
        0.0
    }
}

/// Function: gc_content
/// Calculates the GC content percentage of a DNA sequence.
/// Only counts unambiguous bases (A, C, T, G); ambiguous bases are ignored.
#[inline(always)]
pub fn gc_content(sequence: &[u8]) -> f32 {
    if sequence.is_empty() {
        return 0.0;
    }

    // Initialize counters for GC bases and total valid bases.
    let mut gc_count = 0;
    let mut valid_base_count = 0;

    // Loop over each base in the sequence.
    for &base in sequence {
        match base {
            b'G' | b'C' => {
                gc_count += 1;
                valid_base_count += 1;
            }
            b'A' | b'T' => {
                valid_base_count += 1;
            }
            _ => {} // Skip ambiguous or non-standard bases.
        }
    }

    // Avoid division by zero; return 0.0 if no valid bases found.
    if valid_base_count == 0 {
        0.0
    } else {
        // Calculate the percentage of GC bases among valid bases.
        (gc_count as f32 / valid_base_count as f32) * 100.0
    }
}

// Function: round_to_4_sig_figs
// Rounds a floating-point value to 4 significant figures.
fn round_to_4_sig_figs(value: f32) -> f32 {
    if value == 0.0 {
        return 0.0;
    }
    // Calculate scaling factor based on the logarithm of the absolute value.
    let scale = 10.0_f32.powf(4.0 - value.abs().log10().floor());
    (value * scale).round() / scale
}

#[derive(Serialize, Deserialize)]
struct KmerFrequency {
    kmer: String,
    percentage: f32,
}

/// Function: kmer_frequencies
/// Generic function for calculating relative frequencies of k-mers in a DNA sequence.
/// Returns a tuple with a JSON string of sorted k-mer frequencies and the total count of unique k-mers.
/// Only considers windows containing unambiguous bases (A, C, T, G).
pub fn kmer_frequencies(sequence: &[u8], k: usize, limit: Option<usize>) -> (String, usize) {
    let mut kmer_counts: HashMap<String, usize> = HashMap::new();

    // If the sequence is too short, no k-mers can be formed.
    if sequence.len() < k {
        return ("[]".to_string(), 0);
    }

    // Slide a window of size k across the sequence.
    for window in sequence.windows(k) {
        // Verify that all bases in this window are unambiguous.
        let is_unambiguous = window
            .iter()
            .all(|&base| matches!(base, b'A' | b'C' | b'T' | b'G'));

        if is_unambiguous {
            // Convert the window to a string if possible and count its occurrence.
            if let Ok(kmer) = std::str::from_utf8(window) {
                *kmer_counts.entry(kmer.to_string()).or_insert(0) += 1;
            }
        }
    }

    // Get the number of unique k-mers.
    let unique_count = kmer_counts.len();

    // Return empty result if no valid k-mers were found.
    if unique_count == 0 {
        return ("[]".to_string(), 0);
    }

    // Calculate the total count of all k-mers for frequency calculation.
    let total_count: f32 = kmer_counts.values().sum::<usize>() as f32;

    // Convert counts to a vector of KmerFrequency structs with percentages.
    let mut frequencies: Vec<KmerFrequency> = kmer_counts
        .into_iter()
        .map(|(kmer, count)| {
            let percentage = (count as f32 / total_count) * 100.0;
            KmerFrequency {
                kmer,
                percentage: round_to_4_sig_figs(percentage),
            }
        })
        .collect();

    // Sort k-mers by descending percentage.
    frequencies.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());

    // If a limit is provided, truncate the frequency list.
    if let Some(limit) = limit {
        frequencies.truncate(limit);
    }

    // Convert the frequency list to a JSON string.
    (
        serde_json::to_string(&frequencies).unwrap_or_else(|_| "[]".to_string()),
        unique_count,
    )
}

#[derive(Serialize, Deserialize)]
struct CanonicalKmerFrequency {
    kmer: String,
    percentage: f32,
}

/// Function: canonical_kmer_frequencies
/// Generic function for calculating relative frequencies of canonical k-mers in a DNA sequence.
/// A canonical k-mer is the lexicographically smaller of a k-mer and its reverse complement.
/// Returns a tuple with a JSON string of sorted canonical k-mer frequencies and the total count of unique canonical k-mers.
/// Only considers windows containing unambiguous bases (A, C, T, G).
pub fn canonical_kmer_frequencies(
    sequence: &[u8],
    kmer_size: KmerSize,
    limit: Option<usize>,
) -> (String, usize) {
    let k = kmer_size as usize;
    let mut ckmer_counts: HashMap<String, usize> = HashMap::new();

    // If the sequence is too short, no k-mers can be formed.
    if sequence.len() < k {
        return ("[]".to_string(), 0);
    }

    // Slide a window of size k across the sequence.
    for window in sequence.windows(k) {
        // Verify that all bases in this window are unambiguous.
        let is_unambiguous = window
            .iter()
            .all(|&base| matches!(base, b'A' | b'C' | b'T' | b'G'));

        if is_unambiguous {
            // Convert the window to a string if possible and count its occurrence.
            if let Ok(kmer) = std::str::from_utf8(window) {
                // Use the lookup table to get the canonical k-mer
                let canonical = get_canonical_kmer(kmer, kmer_size);
                *ckmer_counts.entry(canonical).or_insert(0) += 1;
            }
        }
    }

    // Get the number of unique canonical k-mers.
    let unique_count = ckmer_counts.len();

    // Return empty result if no valid k-mers were found.
    if unique_count == 0 {
        return ("[]".to_string(), 0);
    }

    // Calculate the total count of all k-mers for frequency calculation.
    let total_count: f32 = ckmer_counts.values().sum::<usize>() as f32;

    // Convert counts to a vector of CanonicalKmerFrequency structs with percentages.
    let mut frequencies: Vec<CanonicalKmerFrequency> = ckmer_counts
        .into_iter()
        .map(|(kmer, count)| {
            let percentage = (count as f32 / total_count) * 100.0;
            CanonicalKmerFrequency {
                kmer,
                percentage: round_to_4_sig_figs(percentage),
            }
        })
        .collect();

    // Sort canonical k-mers by descending percentage.
    frequencies.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());

    // If a limit is provided, truncate the frequency list.
    if let Some(limit) = limit {
        frequencies.truncate(limit);
    }

    // Convert the frequency list to a JSON string.
    (
        serde_json::to_string(&frequencies).unwrap_or_else(|_| "[]".to_string()),
        unique_count,
    )
}

// Legacy functions for backwards compatibility
#[derive(Serialize, Deserialize)]
struct TetraFrequency {
    tetra: String,
    percentage: f32,
}

/// Function: tetranucleotide_frequencies
/// Calculates relative frequencies of tetranucleotides in a DNA sequence.
/// Returns a tuple with a JSON string of sorted tetranucleotide frequencies and the total count of unique tetranucleotides.
/// Only considers windows containing unambiguous bases (A, C, T, G).
pub fn tetranucleotide_frequencies(sequence: &[u8], limit: Option<usize>) -> (String, usize) {
    let (json, count) = kmer_frequencies(sequence, 4, limit);

    // Map the generic kmer_frequencies results to tetranucleotide-specific format
    if count == 0 {
        return (json, count);
    }

    // Parse the JSON string back to the generic format
    let generic_freqs: Vec<KmerFrequency> = serde_json::from_str(&json).unwrap_or_default();

    // Convert to tetranucleotide-specific format
    let tetra_freqs: Vec<TetraFrequency> = generic_freqs
        .into_iter()
        .map(|freq| TetraFrequency {
            tetra: freq.kmer,
            percentage: freq.percentage,
        })
        .collect();

    (
        serde_json::to_string(&tetra_freqs).unwrap_or_else(|_| "[]".to_string()),
        count,
    )
}

#[derive(Serialize, Deserialize)]
struct CTetraFrequency {
    tetra: String,
    percentage: f32,
}

/// Function: canonical_tetranucleotide_frequencies
/// Calculates relative frequencies of canonical tetranucleotides in a DNA sequence.
/// A canonical tetranucleotide is the lexicographically smaller of a tetranucleotide and its reverse complement.
/// Returns a tuple with a JSON string of sorted canonical tetranucleotide frequencies and the total count of unique canonical tetranucleotides.
/// Only considers windows containing unambiguous bases (A, C, T, G).
pub fn canonical_tetranucleotide_frequencies(
    sequence: &[u8],
    limit: Option<usize>,
) -> (String, usize) {
    let (json, count) = canonical_kmer_frequencies(sequence, KmerSize::Tetra, limit);

    // Map the generic canonical_kmer_frequencies results to tetranucleotide-specific format
    if count == 0 {
        return (json, count);
    }

    // Parse the JSON string back to the generic format
    let generic_freqs: Vec<CanonicalKmerFrequency> =
        serde_json::from_str(&json).unwrap_or_default();

    // Convert to tetranucleotide-specific format
    let ctetra_freqs: Vec<CTetraFrequency> = generic_freqs
        .into_iter()
        .map(|freq| CTetraFrequency {
            tetra: freq.kmer,
            percentage: freq.percentage,
        })
        .collect();

    (
        serde_json::to_string(&ctetra_freqs).unwrap_or_else(|_| "[]".to_string()),
        count,
    )
}
