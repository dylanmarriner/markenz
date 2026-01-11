#!/bin/bash

# TEST-SNAPSHOT-EQ-001: Full equivalence proof
# Exit 0 if pass, 1 if fail

set -e

SEED=1337
TICKS=1000
SNAPSHOT_TICKS=(250 500 750)
OUTPUT_DIR="/tmp/replay_equivalence_detailed_test"

echo "Running detailed replay equivalence test..."

# Clean up previous runs
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Full run: 0 → 1000 ticks
echo "Running full engine from 0 to $TICKS ticks..."
cargo run --bin engine --release -- \
    --genesis-seed $SEED \
    --tick-rate-ms 10 \
    --max-ticks $TICKS \
    --hash-output "$OUTPUT_DIR/full_run_hashes.txt" \
    > "$OUTPUT_DIR/full_run_output.log" 2>&1

if [ $? -ne 0 ]; then
    echo "ERROR: Full engine run failed"
    cat "$OUTPUT_DIR/full_run_output.log"
    exit 1
fi

# Snapshot runs: snapshot at specified ticks → continue to 1000
for snapshot_tick in "${SNAPSHOT_TICKS[@]}"; do
    echo "Running snapshot test from tick $snapshot_tick to $TICKS..."
    
    cargo run --bin engine --release -- \
        --genesis-seed $SEED \
        --load-snapshot "$OUTPUT_DIR/snapshot_$(printf "%010d" $snapshot_tick).bin" \
        --tick-rate-ms 10 \
        --max-ticks $((TICKS - snapshot_tick)) \
        --hash-output "$OUTPUT_DIR/snapshot_${snapshot_tick}_hashes.txt" \
        > "$OUTPUT_DIR/snapshot_${snapshot_tick}_output.log" 2>&1
    
    if [ $? -ne 0 ]; then
        echo "ERROR: Snapshot run from tick $snapshot_tick failed"
        cat "$OUTPUT_DIR/snapshot_${snapshot_tick}_output.log"
        exit 1
    fi
    
    # Compare hashes for ticks $snapshot_tick+1 to 1000
    echo "Comparing hashes for ticks $((snapshot_tick + 1)) to $TICKS..."
    
    # Extract relevant hashes from full run
    sed -n "$((snapshot_tick + 2)),$((TICKS + 1))p" "$OUTPUT_DIR/full_run_hashes.txt" > "$OUTPUT_DIR/full_run_hashes_${snapshot_tick}_end.txt"
    
    # Compare with snapshot run
    if diff -q "$OUTPUT_DIR/full_run_hashes_${snapshot_tick}_end.txt" "$OUTPUT_DIR/snapshot_${snapshot_tick}_hashes.txt" > /dev/null; then
        echo "✅ PASS: Snapshot at tick $snapshot_tick produces identical hashes"
    else
        echo "❌ FAIL: Snapshot at tick $snapshot_tick produces different hashes"
        echo "Differences:"
        diff "$OUTPUT_DIR/full_run_hashes_${snapshot_tick}_end.txt" "$OUTPUT_DIR/snapshot_${snapshot_tick}_hashes.txt" | head -10
        exit 1
    fi
done

# Generate comprehensive report
echo "Generating comprehensive report..."

cat > "$OUTPUT_DIR/report.md" << EOF
# Replay Equivalence Detailed Test Report

## Test Configuration
- **Seed**: $SEED
- **Full run ticks**: 0 → $TICKS
- **Snapshot points**: ${SNAPSHOT_TICKS[*]}
- **Test date**: $(date)

## Results Summary

### Full Run
- **Status**: ✅ PASS
- **Total hashes**: $(wc -l < "$OUTPUT_DIR/full_run_hashes.txt")
- **First hash**: $(head -1 "$OUTPUT_DIR/full_run_hashes.txt")
- **Last hash**: $(tail -1 "$OUTPUT_DIR/full_run_hashes.txt")

### Snapshot Runs
EOF

for snapshot_tick in "${SNAPSHOT_TICKS[@]}"; do
    MATCHING_HASHES=$(wc -l < "$OUTPUT_DIR/snapshot_${snapshot_tick}_hashes.txt")
    echo "- **Snapshot at tick $snapshot_tick**: ✅ PASS ($MATCHING_HASHES matching hashes)" >> "$OUTPUT_DIR/report.md"
done

cat >> "$OUTPUT_DIR/report.md" << EOF

## Detailed Verification

All snapshot replay runs produce identical hash sequences to the full run for their respective tick ranges. This proves:

1. **Snapshot format consistency**: Snapshots capture complete state
2. **Deterministic replay**: Loading from any tick produces identical forward evolution
3. **No state loss**: No information is lost during snapshot serialization/deserialization
4. **RNG continuity**: Random number generator state is properly preserved

## Hash Verification

EOF

# Add hash verification details
for snapshot_tick in "${SNAPSHOT_TICKS[@]}"; do
    echo "### Snapshot at tick $snapshot_tick" >> "$OUTPUT_DIR/report.md"
    echo "- Ticks verified: $((snapshot_tick + 1)) to $TICKS" >> "$OUTPUT_DIR/report.md"
    echo "- Matching hashes: $(wc -l < "$OUTPUT_DIR/snapshot_${snapshot_tick}_hashes.txt")" >> "$OUTPUT_DIR/report.md"
    echo "" >> "$OUTPUT_DIR/report.md"
done

echo "## Conclusion" >> "$OUTPUT_DIR/report.md"
echo "✅ **TEST PASSED**: All snapshot replay tests demonstrate perfect equivalence with full runs." >> "$OUTPUT_DIR/report.md"

echo "Report saved to: $OUTPUT_DIR/report.md"
echo "✅ PASS: All snapshot equivalence tests passed"

exit 0
