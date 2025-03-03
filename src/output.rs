use rusqlite::{Connection, Result as SqlResult};
use seq_io::fastq::{Record, RefRecord};
use std::fs::read_to_string;
use std::io::Write;

// Write record with ID
#[inline(always)]
pub fn write_record_with_id<W: Write>(
    writer: &mut W,
    record: &RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
) {
    head_buffer.clear();
    seq_buffer.clear();
    head_buffer.extend_from_slice(record.head());
    seq_buffer.extend_from_slice(record.seq());
    writer.write_all(b"@").unwrap();
    writer.write_all(head_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(seq_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
}

// Write full record
#[inline(always)]
pub fn write_full_record<W: Write>(
    writer: &mut W,
    record: &RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
    qual_buffer: &mut Vec<u8>,
) {
    head_buffer.clear();
    seq_buffer.clear();
    qual_buffer.clear();
    head_buffer.extend_from_slice(record.head());
    seq_buffer.extend_from_slice(record.seq());
    qual_buffer.extend_from_slice(record.qual());
    writer.write_all(b"@").unwrap();
    writer.write_all(head_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(seq_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(b"+").unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(qual_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
}

// Write record in FASTA format
#[inline(always)]
pub fn write_record_with_fasta<W: Write>(
    writer: &mut W,
    record: &RefRecord,
    head_buffer: &mut Vec<u8>,
    seq_buffer: &mut Vec<u8>,
) {
    head_buffer.clear();
    seq_buffer.clear();
    head_buffer.extend_from_slice(record.head());
    seq_buffer.extend_from_slice(record.seq());
    writer.write_all(b">").unwrap();
    writer.write_all(head_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
    writer.write_all(seq_buffer).unwrap();
    writer.write_all(b"\n").unwrap();
}

pub fn create_sqlite_db() -> SqlResult<Connection> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let db_name = format!("fastq_{}.db", timestamp);
    let conn = Connection::open(&db_name)?;

    // Create fastq_data table with conditional average_quality field
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

    // Create query table with queried_file column
    conn.execute(
        "CREATE TABLE query (
            query TEXT,
            queried_file TEXT
        )",
        [],
    )?;

    Ok(conn)
}

pub fn create_sqlite_db_with_quality() -> SqlResult<Connection> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let db_name = format!("fastq_{}.db", timestamp);
    let conn = Connection::open(&db_name)?;

    // Create fastq_data table with average_quality and variants fields
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

    // Create regex table with queried_file column
    conn.execute(
        "CREATE TABLE query (
            query TEXT,
            queried_file TEXT
        )",
        [],
    )?;

    Ok(conn)
}

pub fn write_regex_to_db(
    conn: &Connection,
    patterns_file: &str,
    queried_file: &str,
) -> SqlResult<()> {
    let file_content = read_to_string(patterns_file)
        .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

    if patterns_file.ends_with(".json") {
        // For JSON files, write the entire content as a single row
        conn.execute(
            "INSERT INTO query (query, queried_file) VALUES (?1, ?2)",
            [&file_content, queried_file],
        )?;
    } else {
        // For txt files, write one regex per row, but queried_file only in first row
        let mut first = true;
        for line in file_content.lines() {
            if !line.trim().is_empty() {
                let file_param = if first {
                    first = false;
                    queried_file
                } else {
                    "" // Empty string for all subsequent rows
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
