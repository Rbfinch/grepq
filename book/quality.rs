use serde::Serialize;
use std::collections::HashMap;

pub fn average_quality(quality: &[u8], quality_encoding: &str) -> f32 {
    // Determine the offset based on the quality encoding
    let offset = match quality_encoding {
        "Phred+33" => 33,
        "Phred+64" => 64,
        _ => 33, // Default to Phred+33 if unknown encoding
    };

    // Initialize sum and count
    let mut sum = 0;
    let count = quality.len() as i32;

    // Calculate the sum of quality scores adjusted by the offset
    for &q in quality {
        sum += q as i32 - offset;
    }

    // Calculate and return the average quality score
    if count > 0 {
        sum as f32 / count as f32
    } else {
        0.0
    }
}

/// Calculate GC content percentage of a DNA sequence
/// Only considers unambiguous bases (ACTG) in both count and total length
#[inline(always)]
pub fn gc_content(sequence: &[u8]) -> f32 {
    if sequence.is_empty() {
        return 0.0;
    }

    let mut gc_count = 0;
    let mut valid_base_count = 0;

    for &base in sequence {
        match base {
            b'G' | b'C' => {
                gc_count += 1;
                valid_base_count += 1;
            }
            b'A' | b'T' => {
                valid_base_count += 1;
            }
            _ => {} // Skip ambiguous bases
        }
    }

    if valid_base_count == 0 {
        0.0
    } else {
        (gc_count as f32 / valid_base_count as f32) * 100.0
    }
}

fn round_to_4_sig_figs(value: f32) -> f32 {
    if value == 0.0 {
        return 0.0;
    }
    let scale = 10.0_f32.powf(4.0 - value.abs().log10().floor());
    (value * scale).round() / scale
}

#[derive(Serialize)]
struct TetraFrequency {
    tetra: String,
    percentage: f32,
}

/// Calculate relative frequencies of tetranucleotides in a DNA sequence
/// Returns a tuple containing:
/// - JSON string of tetranucleotide frequencies
/// - Count of unique tetranucleotides
///
/// Only considers unambiguous bases (ACTG)
///
pub fn tetranucleotide_frequencies(sequence: &[u8], limit: Option<usize>) -> (String, usize) {
    let mut tetra_counts: HashMap<String, usize> = HashMap::new();

    // Need at least 4 nucleotides to form a tetranucleotide
    if sequence.len() < 4 {
        return ("[]".to_string(), 0);
    }

    // Count tetranucleotides using a sliding window
    for window in sequence.windows(4) {
        // Check if window contains only unambiguous bases (ACTG)
        let is_unambiguous = window
            .iter()
            .all(|&base| matches!(base, b'A' | b'C' | b'T' | b'G'));

        if is_unambiguous {
            if let Ok(tetra) = std::str::from_utf8(window) {
                *tetra_counts.entry(tetra.to_string()).or_insert(0) += 1;
            }
        }
    }

    // Get number of unique tetranucleotides
    let unique_count = tetra_counts.len();

    // If no valid tetranucleotides found, return empty result
    if unique_count == 0 {
        return ("[]".to_string(), 0);
    }

    // Calculate total count for relative frequency calculation
    let total_count: f32 = tetra_counts.values().sum::<usize>() as f32;

    // Create a vector of TetraFrequency structs, sorted by frequency
    let mut frequencies: Vec<TetraFrequency> = tetra_counts
        .into_iter()
        .map(|(tetra, count)| {
            let percentage = (count as f32 / total_count) * 100.0;
            TetraFrequency {
                tetra,
                percentage: round_to_4_sig_figs(percentage),
            }
        })
        .collect();

    frequencies.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());

    // Apply the limit if provided
    if let Some(limit) = limit {
        frequencies.truncate(limit);
    }

    // Convert to JSON string
    (
        serde_json::to_string(&frequencies).unwrap_or_else(|_| "[]".to_string()),
        unique_count,
    )
}
