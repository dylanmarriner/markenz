#!/bin/bash

# Terrain Generation Determinism Test
# Exit 0 if pass, 1 if fail

echo "Running terrain generation determinism test..."

cd "$(dirname "$0")/../.."

# Test 1: Generate chunks (0,0)–(10,10) with seed S
echo "Generating initial chunks..."
cargo run --bin test_terrain_determinism -- --seed 1337 --chunks 10 10 > /tmp/initial_chunks.txt 2>&1
if [ $? -ne 0 ]; then
    echo "FAIL: Initial chunk generation failed"
    exit 1
fi

# Test 2: Generate same chunks with seed S again
echo "Generating chunks again with same seed..."
cargo run --bin test_terrain_determinism -- --seed 1337 --chunks 10 10 > /tmp/repeated_chunks.txt 2>&1
if [ $? -ne 0 ]; then
    echo "FAIL: Repeated chunk generation failed"
    exit 1
fi

# Test 3: Compare results
echo "Comparing chunk generation results..."
diff /tmp/initial_chunks.txt /tmp/repeated_chunks.txt > /tmp/diff_output.txt 2>&1
if [ $? -eq 0 ]; then
    echo "PASS: Terrain generation is deterministic"
    exit 0
else
    echo "FAIL: Terrain generation is non-deterministic"
    echo "Differences found:"
    cat /tmp/diff_output.txt
    exit 1
fi
