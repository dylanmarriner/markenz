#!/bin/bash

# Verify RNG audit log is deterministic
# Exit 0 if pass, 1 if fail

set -e

SEED=1337
TICKS=100
OUTPUT_DIR="/tmp/rng_audit_replay_test"

echo "Running RNG audit log determinism test..."

# Clean up previous runs
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Run engine twice with same seed
for run in 1 2; do
    echo "Run $run/2..."
    
    cargo run --bin engine --release -- \
        --genesis-seed $SEED \
        --tick-rate-ms 50 \
        --max-ticks $TICKS \
        --rng-audit-output "$OUTPUT_DIR/run_${run}_rng_audit.log" \
        > "$OUTPUT_DIR/run_${run}_output.log" 2>&1
    
    if [ $? -ne 0 ]; then
        echo "ERROR: Engine run $run failed"
        cat "$OUTPUT_DIR/run_${run}_output.log"
        exit 1
    fi
done

# Compare audit logs
echo "Comparing RNG audit logs..."

if [ ! -f "$OUTPUT_DIR/run_1_rng_audit.log" ] || [ ! -f "$OUTPUT_DIR/run_2_rng_audit.log" ]; then
    echo "ERROR: RNG audit logs not found"
    exit 1
fi

# Sort logs for consistent comparison (they should already be ordered by tick)
sort "$OUTPUT_DIR/run_1_rng_audit.log" > "$OUTPUT_DIR/run_1_sorted.log"
sort "$OUTPUT_DIR/run_2_rng_audit.log" > "$OUTPUT_DIR/run_2_sorted.log"

# Compare the sorted logs
if diff -q "$OUTPUT_DIR/run_1_sorted.log" "$OUTPUT_DIR/run_2_sorted.log" > /dev/null; then
    echo "✅ PASS: RNG audit logs are identical across runs"
    
    # Generate summary
    TOTAL_ENTRIES=$(wc -l < "$OUTPUT_DIR/run_1_sorted.log")
    echo "Total audit entries verified: $TOTAL_ENTRIES"
    
    # Show subsystem breakdown
    echo "RNG draws by subsystem:"
    awk '{print $3}' "$OUTPUT_DIR/run_1_sorted.log" | sort | uniq -c | sort -nr
    
    # Show sample entries
    echo "Sample audit entries:"
    head -5 "$OUTPUT_DIR/run_1_sorted.log"
    
    exit 0
else
    echo "❌ FAIL: RNG audit logs differ between runs"
    echo "Differences:"
    diff "$OUTPUT_DIR/run_1_sorted.log" "$OUTPUT_DIR/run_2_sorted.log" | head -20
    exit 1
fi
