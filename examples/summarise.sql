-- summarise.sql
-- Nick Crosbie, 2025

-- Set output to CSV format
.mode csv
.headers on

-- Create a view to extract matches from the variants field
--    sqlite3 path/to/database.db < path/to/summarise.sql

CREATE VIEW IF NOT EXISTS match_summary AS
    SELECT json_extract(value, '$.match') AS match_string,
           COUNT(*) AS match_count
    FROM fastq_data,
         json_each(fastq_data.variants)
    GROUP BY match_string;

-- Select the total number of matches for each unique match string
WITH match_totals AS (
    SELECT match_string, SUM(match_count) AS total_matches
    FROM match_summary
    GROUP BY match_string
)
SELECT match_string, total_matches
FROM match_totals
UNION ALL
SELECT 'grand total', SUM(total_matches)
FROM match_totals;
