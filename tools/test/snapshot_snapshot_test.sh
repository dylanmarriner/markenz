#!/bin/bash

# TEST-SNAPSHOT-001: Verify snapshot format consistency
# Exit 0 if pass, 1 if fail

set -e

SEED=1337
OUTPUT_DIR="/tmp/snapshot_snapshot_test"

echo "Running snapshot consistency test..."

# Clean up previous runs
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Run engine 100 ticks and save snapshot at tick 50
echo "Running full engine for 100 ticks..."
cargo run --bin engine --release -- \
    --genesis-seed $SEED \
    --tick-rate-ms 50 \
    --max-ticks 100 \
    --snapshot-interval 50 \
    --snapshot-dir "$OUTPUT_DIR" \
    --hash-output "$OUTPUT_DIR/full_run_hashes.txt" \
    > "$OUTPUT_DIR/full_run_output.log" 2>&1

if [ $? -ne 0 ]; then
    echo "ERROR: Full engine run failed"
    cat "$OUTPUT_DIR/full_run_output.log"
    exit 1
fi

# Load snapshot at tick 50 and continue 50 more ticks
echo "Loading snapshot at tick 50 and continuing..."
cargo run --bin engine --release -- \
    --genesis-seed $SEED \
    --load-snapshot "$OUTPUT_DIR/snapshot_0000000050.bin" \
    --tick-rate-ms 50 \
    --max-ticks 50 \
    --hash-output "$OUTPUT_DIR/snapshot_run_hashes.txt" \
    > "$OUTPUT_DIR/snapshot_run_output.log" 2>&1

if [ $? -ne 0 ]; then
    echo "ERROR: Snapshot run failed"
    cat "$OUTPUT_DIR/snapshot_run_output.log"
    exit 1
fi

# Compare hashes for ticks 50-100
echo "Comparing hashes for ticks 50-100..."

# Extract hashes for ticks 50-100 from full run
sed -n '51,100p' "$OUTPUT_DIR/full_run_hashes.txt" > "$OUTPUT_DIR/full_run_hashes_50_100.txt"

# Compare with snapshot run (which should contain ticks 50-100)
if diff -q "$OUTPUT_DIR/full_run_hashes_50_100.txt" "$OUTPUT_DIR/snapshot_run_hashes.txt" > /dev/null; then
    echo "✅ PASS: Snapshot replay produces identical hashes"
    
    # Generate summary
    MATCHING_HASHES=$(wc -l < "$OUTPUT_DIR/full_run_hashes_50_100.txt")
    echo "Matching hashes verified: $MATCHING_HASHES"
    
    exit 0
else
    echo "❌ FAIL: Snapshot replay produces different hashes"
    echo "Differences:"
    diff "$OUTPUT_DIR/full_run_hashes_50_100.txt" "$OUTPUT_DIR/snapshot_run_hashes.txt" | head -20
    exit 1
fi
