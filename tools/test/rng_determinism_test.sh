#!/bin/bash

# TEST-RNG-001: Verify RNG bit-identity across runs
# Exit 0 if pass, 1 if fail

set -e

SEED=1337
RUNS=2
OUTPUT_DIR="/tmp/rng_determinism_test"

echo "Running RNG determinism test with seed $SEED..."

# Clean up previous runs
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Run engine multiple times and collect RNG values
for i in $(seq 1 $RUNS); do
    echo "Run $i/$RUNS..."
    
    # Run engine and capture RNG audit log
    cargo run --bin engine --release -- \
        --genesis-seed $SEED \
        --tick-rate-ms 50 \
        --max-ticks 100 \
        --rng-audit-output "$OUTPUT_DIR/run_$i_rng_audit.log" \
        > "$OUTPUT_DIR/run_$i_output.log" 2>&1
    
    if [ $? -ne 0 ]; then
        echo "ERROR: Engine run $i failed"
        cat "$OUTPUT_DIR/run_$i_output.log"
        exit 1
    fi
done

# Compare RNG outputs
echo "Comparing RNG outputs across runs..."

if [ ! -f "$OUTPUT_DIR/run_1_rng_audit.log" ] || [ ! -f "$OUTPUT_DIR/run_2_rng_audit.log" ]; then
    echo "ERROR: RNG audit logs not found"
    exit 1
fi

# Sort logs for consistent comparison
sort "$OUTPUT_DIR/run_1_rng_audit.log" > "$OUTPUT_DIR/run_1_sorted.log"
sort "$OUTPUT_DIR/run_2_rng_audit.log" > "$OUTPUT_DIR/run_2_sorted.log"

# Compare the sorted logs
if diff -q "$OUTPUT_DIR/run_1_sorted.log" "$OUTPUT_DIR/run_2_sorted.log" > /dev/null; then
    echo "✅ PASS: RNG outputs are bit-identical across runs"
    
    # Generate summary
    TOTAL_DRAWS=$(wc -l < "$OUTPUT_DIR/run_1_sorted.log")
    echo "Total RNG draws verified: $TOTAL_DRAWS"
    
    # Show sample of RNG values
    echo "Sample RNG values:"
    head -10 "$OUTPUT_DIR/run_1_sorted.log"
    
    exit 0
else
    echo "❌ FAIL: RNG outputs differ between runs"
    echo "Differences:"
    diff "$OUTPUT_DIR/run_1_sorted.log" "$OUTPUT_DIR/run_2_sorted.log" | head -20
    exit 1
fi
