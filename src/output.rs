use rusqlite::{Connection, Result as SqlResult};
use seq_io::fastq::{Record, RefRecord};
use std::fs::read_to_string;
use std::io::Write;

// Write record with ID
// Function: write_record_with_id
// Description: Writes only the FASTQ record's ID and sequence.
// Parameters:
// - writer: Output handler to write the formatted data.
// - record: Reference to the current FASTQ record.
// - head_buffer: Buffer used to temporarily hold the header.
// - seq_buffer: Buffer used to temporarily hold the sequence.
#[inline(always)]
pub fn write_record_with_id<W: Write>(
    writer: &mut W,
    record: &RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
) {
    head_buffer.clear(); // Ensure header buffer is empty.
    seq_buffer.clear();  // Ensure sequence buffer is empty.
    head_buffer.extend_from_slice(record.head()); // Cache header from record.
    seq_buffer.extend_from_slice(record.seq());    // Cache sequence from record.
    writer.write_all(b"@").unwrap();                // FASTQ header prefix.
    writer.write_all(head_buffer).unwrap();         // Write header.
    writer.write_all(b"\n").unwrap();               // Newline separator.
    writer.write_all(seq_buffer).unwrap();          // Write sequence.
    writer.write_all(b"\n").unwrap();               // Newline after sequence.
}

// Write full record
// Function: write_full_record
// Description: Writes the complete FASTQ record (header, sequence, and quality).
// Parameters:
// - writer: Output writer stream.
// - record: Reference to the current FASTQ record.
// - head_buffer: Buffer used to store the header temporarily.
// - seq_buffer: Buffer used to store the sequence temporarily.
// - qual_buffer: Buffer used to store the quality scores temporarily.
#[inline(always)]
pub fn write_full_record<W: Write>(
    writer: &mut W,
    record: &RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
    qual_buffer: &mut Vec<u8>,
) {
    head_buffer.clear(); // Clear header buffer.
    seq_buffer.clear();  // Clear sequence buffer.
    qual_buffer.clear(); // Clear quality buffer.
    head_buffer.extend_from_slice(record.head()); // Cache header.
    seq_buffer.extend_from_slice(record.seq());    // Cache sequence.
    qual_buffer.extend_from_slice(record.qual());    // Cache quality scores.
    writer.write_all(b"@").unwrap();    // Begin FASTQ record with '@'.
    writer.write_all(head_buffer).unwrap(); // Write header.
    writer.write_all(b"\n").unwrap();   // Newline.
    writer.write_all(seq_buffer).unwrap(); // Write sequence.
    writer.write_all(b"\n").unwrap();   // Newline.
    writer.write_all(b"+").unwrap();    // Separator line for quality.
    writer.write_all(b"\n").unwrap();   // Newline.
    writer.write_all(qual_buffer).unwrap(); // Write quality scores.
    writer.write_all(b"\n").unwrap();   // Newline.
}

// Write record in FASTA format
// Function: write_record_with_fasta
// Description: Converts a FASTQ record to FASTA format.
// Parameters:
// - writer: Output writer stream.
// - record: Reference to the current FASTQ record.
// - head_buffer: Buffer to store the header temporarily.
// - seq_buffer: Buffer to store the sequence temporarily.
#[inline(always)]
pub fn write_record_with_fasta<W: Write>(
    writer: &mut W,
    record: &RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
) {
    head_buffer.clear(); // Clear header buffer.
    seq_buffer.clear();  // Clear sequence buffer.
    head_buffer.extend_from_slice(record.head()); // Cache header.
    seq_buffer.extend_from_slice(record.seq());    // Cache sequence.
    writer.write_all(b">").unwrap();               // FASTA header prefix.
    writer.write_all(head_buffer).unwrap();          // Write header.
    writer.write_all(b"\n").unwrap();               // Newline.
    writer.write_all(seq_buffer).unwrap();           // Write sequence.
    writer.write_all(b"\n").unwrap();               // Newline.
}

// Function: create_sqlite_db
// Description: Creates a SQLite database file for storing FASTQ records without quality metrics.
// Returns: A rusqlite::Connection wrapped in a Result on success.
pub fn create_sqlite_db() -> SqlResult<Connection> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let db_name = format!("fastq_{}.db", timestamp);
    let conn = Connection::open(&db_name)?;

    // Create fastq_data table with essential record fields.
    conn.execute(
        "CREATE TABLE fastq_data (
            header TEXT,
            sequence TEXT,
            quality TEXT,
            length INTEGER,
            GC REAL,
            GC_int INTEGER,
            nTN INTEGER,
            TNF TEXT
        )",
        [],
    )?;

    // Create query table for storing the patterns and queried file information.
    conn.execute(
        "CREATE TABLE query (
            query TEXT,
            queried_file TEXT
        )",
        [],
    )?;

    Ok(conn)
}

// Function: create_sqlite_db_with_quality
// Description: Creates a SQLite database file for storing FASTQ records including quality metrics and variants.
// Returns: A rusqlite::Connection wrapped in a Result on success.
pub fn create_sqlite_db_with_quality() -> SqlResult<Connection> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let db_name = format!("fastq_{}.db", timestamp);
    let conn = Connection::open(&db_name)?;

    // Create fastq_data table with extra fields for quality and variant information.
    conn.execute(
        "CREATE TABLE fastq_data (
            header TEXT,
            sequence TEXT,
            quality TEXT,
            length INTEGER,
            GC REAL,
            GC_int INTEGER,
            nTN INTEGER,
            TNF TEXT,
            average_quality REAL,
            variants TEXT
        )",
        [],
    )?;

    // Create query table as above.
    conn.execute(
        "CREATE TABLE query (
            query TEXT,
            queried_file TEXT
        )",
        [],
    )?;

    Ok(conn)
}

// Function: write_regex_to_db
// Description: Inserts regex pattern data and query information into the SQLite database.
// Parameters:
// - conn: Reference to the open SQLite connection.
// - patterns_file: Path to the file containing regex patterns (JSON or text).
// - queried_file: The file that was queried using these patterns.
pub fn write_regex_to_db(
    conn: &Connection,
    patterns_file: &str,
    queried_file: &str,
) -> SqlResult<()> {
    // Read entire contents of the patterns file.
    let file_content = read_to_string(patterns_file)
        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

    if patterns_file.ends_with(".json") {
        // For JSON files, store the full content as a single entry.
        conn.execute(
            "INSERT INTO query (query, queried_file) VALUES (?1, ?2)",
            [&file_content, queried_file],
        )?;
    } else {
        // For text files, split each non-empty line into separate rows.
        // Only the first row gets the queried_file information.
        let mut first = true;
        for line in file_content.lines() {
            if !line.trim().is_empty() {
                let file_param = if first {
                    first = false;
                    queried_file
                } else {
                    "" // Subsequent lines: queried_file remains empty.
                };

                conn.execute(
                    "INSERT INTO query (query, queried_file) VALUES (?1, ?2)",
                    [line.trim(), file_param],
                )?;
            }
        }
    }
    Ok(())
}
