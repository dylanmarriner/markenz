#!/bin/bash

# TEST-DET-001: Verify 3+ runs produce identical hash sequence
# Exit 0 if pass, 1 if fail

set -e

SEED=1337
RUNS=3
TICKS=1000
OUTPUT_DIR="/tmp/cross_run_hash_equality_test"

echo "Running cross-run hash equality test with seed $SEED for $TICKS ticks..."

# Clean up previous runs
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Run engine multiple times and collect hashes
for i in $(seq 1 $RUNS); do
    echo "Run $i/$RUNS..."
    
    cargo run --bin engine --release -- \
        --genesis-seed $SEED \
        --tick-rate-ms 10 \
        --max-ticks $TICKS \
        --hash-output "$OUTPUT_DIR/run_$i_hashes.txt" \
        > "$OUTPUT_DIR/run_$i_output.log" 2>&1
    
    if [ $? -ne 0 ]; then
        echo "ERROR: Engine run $i failed"
        cat "$OUTPUT_DIR/run_$i_output.log"
        exit 1
    fi
    
    # Verify we got the expected number of hashes
    HASH_COUNT=$(wc -l < "$OUTPUT_DIR/run_$i_hashes.txt")
    if [ "$HASH_COUNT" -ne $((TICKS + 1)) ]; then
        echo "ERROR: Expected $((TICKS + 1)) hashes, got $HASH_COUNT"
        exit 1
    fi
done

# Compare all runs
echo "Comparing hash sequences across all runs..."

# Compare run 1 with run 2
if ! diff -q "$OUTPUT_DIR/run_1_hashes.txt" "$OUTPUT_DIR/run_2_hashes.txt" > /dev/null; then
    echo "❌ FAIL: Hashes differ between run 1 and run 2"
    echo "First difference:"
    diff "$OUTPUT_DIR/run_1_hashes.txt" "$OUTPUT_DIR/run_2_hashes.txt" | head -5
    exit 1
fi

# Compare run 1 with run 3
if ! diff -q "$OUTPUT_DIR/run_1_hashes.txt" "$OUTPUT_DIR/run_3_hashes.txt" > /dev/null; then
    echo "❌ FAIL: Hashes differ between run 1 and run 3"
    echo "First difference:"
    diff "$OUTPUT_DIR/run_1_hashes.txt" "$OUTPUT_DIR/run_3_hashes.txt" | head -5
    exit 1
fi

# All runs match
echo "✅ PASS: All $RUNS runs produce identical hash sequences"

# Generate report
echo "Generating report..."
TOTAL_HASHES=$(wc -l < "$OUTPUT_DIR/run_1_hashes.txt")
echo "Total hashes verified: $TOTAL_HASHES"

# Compute checksum of combined hashes for verification
cat "$OUTPUT_DIR/run_1_hashes.txt" | sha256sum > "$OUTPUT_DIR/combined_hashes_checksum.txt"
COMBINED_CHECKSUM=$(cut -d' ' -f1 < "$OUTPUT_DIR/combined_hashes_checksum.txt")
echo "Combined hashes checksum: $COMBINED_CHECKSUM"

# Show sample hashes
echo "Sample hashes (first 5):"
head -5 "$OUTPUT_DIR/run_1_hashes.txt"

echo "Sample hashes (last 5):"
tail -5 "$OUTPUT_DIR/run_1_hashes.txt"

# Create detailed report
cat > "$OUTPUT_DIR/report.txt" << EOF
Cross-Run Hash Equality Test Report
===================================

Test Configuration:
- Seed: $SEED
- Runs: $RUNS
- Ticks per run: $TICKS
- Total hashes verified: $TOTAL_HASHES

Results:
✅ PASS: All runs produce identical hash sequences
✅ PASS: No divergence detected in any run
✅ PASS: Determinism verified across $RUNS independent runs

Checksum Verification:
- Combined hashes SHA256: $COMBINED_CHECKSUM

Test completed successfully at $(date)
EOF

echo "Report saved to: $OUTPUT_DIR/report.txt"
exit 0
