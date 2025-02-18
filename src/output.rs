use seq_io::fastq::{Record, RefRecord};
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
