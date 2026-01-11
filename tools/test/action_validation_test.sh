#!/bin/bash

# Action Validation Determinism Test
# Exit 0 if pass, 1 if fail

echo "Running action validation determinism test..."

cd "$(dirname "$0")/../.."

# Test 1: Create scenario and validate actions
echo "Creating initial scenario and validating actions..."
cargo run --bin test_action_validation -- --scenario basic > /tmp/initial_validation.txt 2>&1
if [ $? -ne 0 ]; then
    echo "FAIL: Initial action validation failed"
    exit 1
fi

# Test 2: Replay scenario with identical inputs
echo "Replaying scenario with identical inputs..."
cargo run --bin test_action_validation -- --scenario basic --replay > /tmp/replayed_validation.txt 2>&1
if [ $? -ne 0 ]; then
    echo "FAIL: Replayed action validation failed"
    exit 1
fi

# Test 3: Compare validation results
echo "Comparing validation results..."
diff /tmp/initial_validation.txt /tmp/replayed_validation.txt > /tmp/validation_diff.txt 2>&1
if [ $? -eq 0 ]; then
    echo "PASS: Action validation is deterministic"
    exit 0
else
    echo "FAIL: Action validation is non-deterministic"
    echo "Differences found:"
    cat /tmp/validation_diff.txt
    exit 1
fi
