// Module: quality
// Functions: convert_and_print

// The function convert_and_print takes a string as input and calculates the average, sum, and count of the ASCII values of the characters in the string. The function iterates over each character in the input string, calculates the ASCII value of the character, adjusts the value by subtracting 33, and adds the adjusted value to the sum. The function also keeps track of the count of characters processed. After processing all characters, the function calculates the average by dividing the sum by the count and prints the average, sum, and count.

// The function assumes that the fastq record is using Phred 33 encoding for the quality scores (hence subtracting 33 from the ASCII values).

// TODO: Reimplement using the byte slices from record.qual(), comparing to minimum_quality defined in initialise.rs (will need to include minimumQuality in the regex.json file)

pub fn convert_and_print(input: &str) {
    let mut sum: i32 = 0;
    let mut count = 0;
    for ch in input.chars() {
        let numeric_value = ch as u8;
        let adjusted_value = numeric_value - 33;
        sum += adjusted_value as i32;
        count += 1;
    }
    if count > 0 {
        let average = sum as f32 / count as f32;
        println!("Average: {}, Sum: {}, Count: {}", average, sum, count);
    }
}

// Quality string to be used for testing
pub const QUALITY_STRING: &str = r#"F#FFFFFFFFFF:FFFFFFF,FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF:FFFF:FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF:FFF,FFFFFFFFFFFFF,:FFF:FFFFFFFFFFFFFF:,FF,F:FFFFFFFFFFFFF::FF,FFFFFFFF:FFFFFFFFFFF:FFFFFFFFFFFFFFFFFF:FF,FF:FFFF,FFFFF,:FFF:FF,:FF:FFFFFF:FFFFFFFFFFF::FFFFFFFFFFF:FFFFFFF,FFFFF:F"#;

// Above string correctly prints as ... Average: 35.01, Sum: 10503, Count: 300

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_and_print() {
        let input = "ABC";
        convert_and_print(input);
        // Expected output:
        // 32
        // 33
        // 34
        // Average: 33.0
    }
}
