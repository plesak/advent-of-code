#!/bin/bash

# credits to pplx.ai for most of this setup script

# Validate input
if [ $# -ne 1 ] || ! [[ $1 =~ ^[0-9]+$ ]] || [ $1 -lt 2 ] || [ $1 -gt 25 ]; then
    echo "Usage: $0 <number between 2 and 25>"
    exit 1
fi

X=$1
PREV=$((X-1))

# Copy template file
cp src/template.rs "src/day$X.rs"

# Download new input file
aoc download -I -i inputs/day"${X}".txt --year 2024 -d "${X}" || true;

# Copy test files
cp "tests/day${PREV}_part1.txt" "tests/day${X}_part1.txt"
cp "tests/day${PREV}_part2.txt" "tests/day${X}_part2.txt"


# Update lib.rs
echo -e "\npub mod day$X;" >> src/lib.rs

# Update main.rs
sed -i "/mod day$PREV;/a mod day$X;" src/main.rs
sed -i "/\"day${PREV}_part2\" => day${PREV}::part2(&input).to_string(),/a \ \ \ \ \ \ \ \ \ \ \ \ \"day${X}_part1\" => day${X}::part1(\&input).to_string(),\n            \"day${X}_part2\" => day${X}::part2(\&input).to_string()," src/main.rs

# Update benches/aoc_bench.rs
sed -i "/fn bench_day${PREV}_part2/a\\
fn bench_day${X}_part1(c: \&mut Criterion) {\\
    let input = include_str!(\"../inputs/day${X}.txt\");\\
    c.bench_function(\"day${X}_part1\", |b| b.iter(|| advent_of_code::day${X}::part1(input)));\\
}\\
\\
fn bench_day${X}_part2(c: \&mut Criterion) {\\
    let input = include_str!(\"../inputs/day${X}.txt\");\\
    c.bench_function(\"day${X}_part2\", |b| b.iter(|| advent_of_code::day${X}::part2(input)));\\
}" benches/aoc_bench.rs

sed -i "/bench_day${PREV}_part2,/a \ \ \ \ bench_day${X}_part1,\n    bench_day${X}_part2," benches/aoc_bench.rs

echo "Day $X setup complete!"
