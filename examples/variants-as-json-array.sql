-- variants-as-json-array.sql
-- Nick Crosbie, 2025

-- Outputs all variants data as a single JSON array
--    Usage: sqlite3 path/to/database.db < path/to/variants-as-json-array.sql

-- Set minimal output options for raw, unescaped output
.output variants.json
.echo off
.mode ascii
.headers off
.separator ""

-- Output the JSON array with proper wrapping brackets
SELECT '[' || group_concat(variants, ',') || ']'
FROM fastq_data
WHERE fastq_data.variants IS NOT NULL
  AND fastq_data.variants != '[]';
