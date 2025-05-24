use std::collections::HashMap;

// Include the generated canonical k-mers tables
include!(concat!(env!("OUT_DIR"), "/canonical_kmers_4.rs"));
include!(concat!(env!("OUT_DIR"), "/canonical_kmers_5.rs"));
include!(concat!(env!("OUT_DIR"), "/canonical_kmers_6.rs"));
include!(concat!(env!("OUT_DIR"), "/canonical_kmers_7.rs"));

#[derive(Clone, Copy)]
pub enum KmerSize {
    Tetra = 4,
    Penta = 5,
    Hexa = 6,
    Hepta = 7,
}

pub fn get_canonical_kmer(kmer: &str, size: KmerSize) -> String {
    match size {
        KmerSize::Tetra => CANONICAL_K_MERS_4
            .get(kmer)
            .map(|&s| s.to_string())
            .unwrap_or(kmer.to_string()),
        KmerSize::Penta => CANONICAL_K_MERS_5
            .get(kmer)
            .map(|&s| s.to_string())
            .unwrap_or(kmer.to_string()),
        KmerSize::Hexa => CANONICAL_K_MERS_6
            .get(kmer)
            .map(|&s| s.to_string())
            .unwrap_or(kmer.to_string()),
        KmerSize::Hepta => CANONICAL_K_MERS_7
            .get(kmer)
            .map(|&s| s.to_string())
            .unwrap_or(kmer.to_string()),
    }
}

pub fn count_kmers(sequence: &str, k: usize) -> HashMap<String, usize> {
    let mut kmer_counts = HashMap::new();

    if sequence.len() < k {
        return kmer_counts;
    }

    for i in 0..=sequence.len() - k {
        let kmer = &sequence[i..i + k];
        // Skip kmers with non-ACGT characters
        if kmer
            .chars()
            .all(|c| matches!(c, 'A' | 'C' | 'G' | 'T' | 'a' | 'c' | 'g' | 't'))
        {
            *kmer_counts.entry(kmer.to_uppercase()).or_insert(0) += 1;
        }
    }

    kmer_counts
}

pub fn count_canonical_kmers(sequence: &str, size: KmerSize) -> HashMap<String, usize> {
    let k = size as usize;
    let mut kmer_counts = HashMap::new();

    if sequence.len() < k {
        return kmer_counts;
    }

    for i in 0..=sequence.len() - k {
        let kmer = &sequence[i..i + k].to_uppercase();
        // Skip kmers with non-ACGT characters
        if kmer.chars().all(|c| matches!(c, 'A' | 'C' | 'G' | 'T')) {
            let canonical = get_canonical_kmer(kmer, size);
            *kmer_counts.entry(canonical).or_insert(0) += 1;
        }
    }

    kmer_counts
}
