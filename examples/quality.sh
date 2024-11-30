#!/bin/bash

## This script is used to determine the quality score encoding of a fastq file. 
# It is from https://www.biostars.org/p/63225/

cat "$1" | \
head -n 40 | \
awk '{if(NR%4==0) printf("%s",$0);}' | \
od -A n -t u1 | \
awk '
BEGIN {
  min = 100;
  max = 0;
}

{
  for (i = 1; i <= NF; i++) {
    if ($i > max) max = $i;
    if ($i < min) min = $i;
  }
}

END {
  if (max <= 74 && min < 59) print "Phred+33";
  else if (max > 73 && min >= 64) print "Phred+64";
  else if (min >= 59 && min < 64 && max > 73) print "Solexa+64";
  else print "Unknown score encoding\!";
}
'