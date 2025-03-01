use std::collections::HashMap;

// Calculate average quality of a sequence; supports Phred+33 and
// Phred+64 encodings.

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
#[inline(always)]
pub fn gc_content(sequence: &[u8]) -> f32 {
    if sequence.is_empty() {
        return 0.0;
    }

    let gc_count = sequence
        .iter()
        .filter(|&&base| base == b'G' || base == b'C')
        .count();

    (gc_count as f32 / sequence.len() as f32) * 100.0
}

/// Calculate relative frequencies of tetranucleotides in a DNA sequence
/// Returns a JSON string containing tetranucleotide counts and their relative frequencies
pub fn tetranucleotide_frequencies(sequence: &[u8]) -> String {
    let mut tetra_counts: HashMap<String, usize> = HashMap::new();

    // Need at least 4 nucleotides to form a tetranucleotide
    if sequence.len() < 4 {
        return "{}".to_string();
    }

    // Count tetranucleotides using a sliding window
    for window in sequence.windows(4) {
        if let Ok(tetra) = std::str::from_utf8(window) {
            *tetra_counts.entry(tetra.to_string()).or_insert(0) += 1;
        }
    }

    // Calculate total count for relative frequency calculation
    let total_count: f64 = tetra_counts.values().sum::<usize>() as f64;

    // Create a map with frequencies
    let frequencies: HashMap<String, f64> = tetra_counts
        .into_iter()
        .map(|(tetra, count)| (tetra, count as f64 / total_count))
        .collect();

    // Convert to JSON string
    serde_json::to_string(&frequencies).unwrap_or_else(|_| "{}".to_string())
}
