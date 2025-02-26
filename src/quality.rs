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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_content() {
        assert_eq!(gc_content(b"GCGC"), 100.0);
        assert_eq!(gc_content(b"ATAT"), 0.0);
        assert_eq!(gc_content(b"ATGC"), 50.0);
        assert_eq!(gc_content(b""), 0.0);
        assert_eq!(gc_content(b"GGCC"), 100.0);
    }
}
