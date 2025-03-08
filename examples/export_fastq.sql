-- export_fastq.sql
-- Nick Crosbie, 2025

-- This script exports the FASTQ data from the database to a file called output.fastq
-- run the following to execute this script:
--    sqlite3 your_database.db < export_fastq.sql

-- Create a view that concatenates the FASTQ components into one record
CREATE VIEW fastq_export AS
    SELECT '@' || header || char(10) ||
           sequence  || char(10) ||
           '+'       || char(10) ||
           quality   AS fastq_record
    FROM fastq_data;
    
-- Commands to export the FASTQ records to a file
.mode list
.output output.fastq
SELECT fastq_record FROM fastq_export;
.output stdout