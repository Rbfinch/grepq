use serde::Serialize;
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

#[derive(Serialize)]
struct TetraFrequency {
    tetra: String,
    percentage: f32,
}

/// Function: tetranucleotide_frequencies
/// Calculates relative frequencies of tetranucleotides in a DNA sequence.
/// Returns a tuple with a JSON string of sorted tetranucleotide frequencies and the total count of unique tetranucleotides.
/// Only considers windows containing unambiguous bases (A, C, T, G).
pub fn tetranucleotide_frequencies(sequence: &[u8], limit: Option<usize>) -> (String, usize) {
    let mut tetra_counts: HashMap<String, usize> = HashMap::new();

    // If the sequence is too short, no tetranucleotides can be formed.
    if sequence.len() < 4 {
        return ("[]".to_string(), 0);
    }

    // Slide a window of size 4 across the sequence.
    for window in sequence.windows(4) {
        // Verify that all bases in this window are unambiguous.
        let is_unambiguous = window
            .iter()
            .all(|&base| matches!(base, b'A' | b'C' | b'T' | b'G'));

        if is_unambiguous {
            // Convert the window to a string if possible and count its occurrence.
            if let Ok(tetra) = std::str::from_utf8(window) {
                *tetra_counts.entry(tetra.to_string()).or_insert(0) += 1;
            }
        }
    }

    // Get the number of unique tetranucleotides.
    let unique_count = tetra_counts.len();

    // Return empty result if no valid tetranucleotides were found.
    if unique_count == 0 {
        return ("[]".to_string(), 0);
    }

    // Calculate the total count of all tetranucleotides for frequency calculation.
    let total_count: f32 = tetra_counts.values().sum::<usize>() as f32;

    // Convert counts to a vector of TetraFrequency structs with percentages.
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

    // Sort tetranucleotides by descending percentage.
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
