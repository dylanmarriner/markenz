#!/usr/bin/env bash

##############################################################################
# COMPREHENSIVE LOCAL GUARD SCRIPT
#
# Purpose: Entry point for local dev verification before commit
#
# Authority: docs/governance/RUST_REALITY_LOCK.md
#
# This script runs all validation checks locally:
# 1. reality_lock.sh - Pattern-based guard script
# 2. cargo fmt --check - Code formatting
# 3. cargo clippy -- -D warnings - Lint enforcement
# 4. cargo test --all - Unit and integration tests
#
# If any check fails, build is rejected.
#
##############################################################################

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

echo "═══════════════════════════════════════════════════════════════"
echo "RUNNING COMPREHENSIVE GUARD CHECKS"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Step 1: Reality Lock Script
echo "[1/4] Running reality_lock.sh..."
if ./tools/ci/reality_lock.sh; then
    echo "✅ Reality Lock check passed"
else
    echo "❌ Reality Lock check failed"
    exit 1
fi

echo ""

# Step 2: Code Formatting
echo "[2/4] Checking code format with cargo fmt..."
if cargo fmt -- --check >/dev/null 2>&1; then
    echo "✅ Code formatting check passed"
else
    echo "⚠️  Code formatting issues found"
    echo "   Run 'cargo fmt' to fix"
    exit 1
fi

echo ""

# Step 3: Clippy Lints
echo "[3/4] Running cargo clippy with strict warnings..."
if cargo clippy --all-targets --all-features -- -D warnings >/dev/null 2>&1; then
    echo "✅ Clippy check passed"
else
    echo "❌ Clippy check failed (compilation errors or lint violations)"
    echo "   Run 'cargo clippy --all-targets --all-features -- -D warnings' for details"
    exit 1
fi

echo ""

# Step 4: Tests
echo "[4/4] Running cargo test..."
if cargo test --all >/dev/null 2>&1; then
    echo "✅ All tests passed"
else
    echo "❌ Tests failed"
    echo "   Run 'cargo test --all' for details"
    exit 1
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "✅ ALL GUARD CHECKS PASSED"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Ready to commit. Good luck!"
echo ""
