#!/bin/bash

# Gathering, Mining, Crafting Determinism Test
# Exit 0 if pass, 1 if fail

echo "Running gathering/mining/crafting determinism test..."

cd "$(dirname "$0")/../.."

# Test 1: Test gather in grassland → produces berries
echo "Testing gathering in grassland..."
cargo run --bin test_mechanics -- --action gather --biome grassland > /tmp/gather_test.txt 2>&1
if [ $? -ne 0 ]; then
    echo "FAIL: Gathering test failed"
    exit 1
fi

# Test 2: Test mine in mountain → produces ore
echo "Testing mining in mountain..."
cargo run --bin test_mechanics -- --action mine --biome mountain > /tmp/mine_test.txt 2>&1
if [ $? -ne 0 ]; then
    echo "FAIL: Mining test failed"
    exit 1
fi

# Test 3: Test craft recipe → produces tool from resources
echo "Testing crafting recipe..."
cargo run --bin test_mechanics -- --action craft --recipe basic_tool > /tmp/craft_test.txt 2>&1
if [ $? -ne 0 ]; then
    echo "FAIL: Crafting test failed"
    exit 1
fi

# Test 4: Replay with same inputs
echo "Replaying mechanics tests with same inputs..."
cargo run --bin test_mechanics -- --action gather --biome grassland --replay > /tmp/gather_replay.txt 2>&1
cargo run --bin test_mechanics -- --action mine --biome mountain --replay > /tmp/mine_replay.txt 2>&1
cargo run --bin test_mechanics -- --action craft --recipe basic_tool --replay > /tmp/craft_replay.txt 2>&1

# Test 5: Compare outputs
echo "Comparing mechanics outputs..."
diff /tmp/gather_test.txt /tmp/gather_replay.txt > /tmp/gather_diff.txt 2>&1
diff /tmp/mine_test.txt /tmp/mine_replay.txt > /tmp/mine_diff.txt 2>&1
diff /tmp/craft_test.txt /tmp/craft_replay.txt > /tmp/craft_diff.txt 2>&1

if [ $? -eq 0 ] && [ -s /tmp/gather_diff.txt ] && [ -s /tmp/mine_diff.txt ] && [ -s /tmp/craft_diff.txt ]; then
    echo "PASS: Gathering, mining, and crafting are deterministic"
    exit 0
else
    echo "FAIL: Gathering, mining, or crafting is non-deterministic"
    echo "Gathering differences:"
    cat /tmp/gather_diff.txt
    echo "Mining differences:"
    cat /tmp/mine_diff.txt
    echo "Crafting differences:"
    cat /tmp/craft_diff.txt
    exit 1
fi
