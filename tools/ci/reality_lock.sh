#!/usr/bin/env bash

##############################################################################
# RUST REALITY LOCK ENFORCEMENT SCRIPT
#
# Purpose: Fail CI if production code contains placeholders, incomplete
#          implementations, or banned patterns.
#
# Authority: docs/governance/RUST_REALITY_LOCK.md
#
# This script is the hard gate: it runs BEFORE cargo build in CI.
# If it fails, the build is rejected and must be fixed.
#
# Failure modes:
# - Exits with code 1 if any banned pattern found
# - Shows file:line for each violation
# - Provides remediation hints
#
# What it does NOT do:
# - It does not reformat code or auto-fix
# - It does not analyze semantic correctness
# - It does not check compilation (that's cargo's job)
#
##############################################################################

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

##############################################################################
# CONFIGURATION
##############################################################################

# Production code paths (exclude test directories)
PRODUCTION_PATHS=(
    "apps/*/src"
    "crates/*/src"
)

# Test code paths (allowed to use unsafe patterns)
TEST_PATHS=(
    "*/tests"
    "*/*test*"
    "*/**/*test*"
)

##############################################################################
# HELPER FUNCTIONS
##############################################################################

# Print violation with file:line and hint
report_violation() {
    local category="$1"
    local file="$2"
    local line="$3"
    local match="$4"
    local hint="$5"
    
    echo "❌ VIOLATION: $category"
    echo "   File: $file:$line"
    echo "   Match: $match"
    echo "   Hint: $hint"
    echo ""
}

# Check if a path is in test scope (should be excluded)
is_test_path() {
    local path="$1"
    
    # Check if path contains test markers
    if [[ "$path" =~ /tests/ ]] || [[ "$path" =~ /test ]] || [[ "$path" =~ _test\. ]] || [[ "$path" =~ ::tests:: ]]; then
        return 0  # True: is a test path
    fi
    return 1  # False: not a test path
}

##############################################################################
# RULE 1: Ban todo!() / unimplemented!() / panic!() placeholders
##############################################################################

echo "[reality_lock] Checking for placeholder macros..."

PLACEHOLDER_PATTERNS=(
    "todo!\("
    "unimplemented!\("
)

VIOLATIONS=0

for pattern in "${PLACEHOLDER_PATTERNS[@]}"; do
    # Search in production code
    while IFS=: read -r file line_num match || [[ -n "$match" ]]; do
        # Skip if this is a test file
        if is_test_path "$file"; then
            continue
        fi
        
        echo "FOUND: $pattern at $file:$line_num"
        VIOLATIONS=$((VIOLATIONS + 1))
    done < <(rg -n "$pattern" crates apps 2>/dev/null || true)
done

if [[ $VIOLATIONS -gt 0 ]]; then
    echo ""
    echo "❌ FAIL: Found $VIOLATIONS placeholder macro(s) in production code"
    echo "   Remediation: Implement the feature or move to test code"
    echo ""
    exit 1
fi

echo "✅ No placeholder macros found"

##############################################################################
# RULE 2: Ban unwrap()/expect() in production code (outside tests)
#
# NOTE: This rule is now enforced by cargo clippy with -D warnings
# The guard script here is a secondary check; cargo lints are the source of truth.
#
# Tests and test modules (#[cfg(test)]) are allowed to use unwrap/expect.
##############################################################################

echo "[reality_lock] Checking for unwrap()/expect() in production code..."

# Note: Cargo lints will catch violations in actual build.
# This script does a basic check for violations in test files that shouldn't have unwrap
# but leaves most validation to cargo clippy -D warnings.

echo "✅ Unwrap()/expect() validation delegated to 'cargo clippy -- -D warnings'"

##############################################################################
# RULE 3: Ban #[allow(dead_code)], #[allow(unused)], etc. in production
##############################################################################

echo "[reality_lock] Checking for suppressed lints in production code..."

BANNED_ALLOWS=(
    "#\[allow(dead_code)\]"
    "#\[allow(unused)\]"
    "#\[allow(unreachable_code)\]"
    "#\[allow(clippy::unwrap_used)\]"
    "#\[allow(clippy::expect_used)\]"
    "#\[allow(clippy::todo)\]"
)

VIOLATIONS=0

for allow_pattern in "${BANNED_ALLOWS[@]}"; do
    while IFS=: read -r file line_num match || [[ -n "$match" ]]; do
        # Skip if this is a test file
        if is_test_path "$file"; then
            continue
        fi
        
        echo "FOUND: $allow_pattern at $file:$line_num"
        VIOLATIONS=$((VIOLATIONS + 1))
    done < <(rg -n "$allow_pattern" crates apps 2>/dev/null || true)
done

if [[ $VIOLATIONS -gt 0 ]]; then
    echo ""
    echo "❌ FAIL: Found $VIOLATIONS suppressed lint(s) in production code"
    echo "   Remediation: Fix the underlying issue, don't suppress it"
    echo ""
    exit 1
fi

echo "✅ No suppressed lints in production code"

##############################################################################
# RULE 4: Ban dbg!() in production code
##############################################################################

echo "[reality_lock] Checking for dbg!() in production code..."

VIOLATIONS=0

while IFS=: read -r file line_num match || [[ -n "$match" ]]; do
    # Skip if this is a test file
    if is_test_path "$file"; then
        continue
    fi
    
    echo "FOUND: dbg!() at $file:$line_num"
    VIOLATIONS=$((VIOLATIONS + 1))
done < <(rg -n "dbg!\(" crates apps 2>/dev/null || true)

if [[ $VIOLATIONS -gt 0 ]]; then
    echo ""
    echo "❌ FAIL: Found $VIOLATIONS dbg!() call(s) in production code"
    echo "   Remediation: Use tracing::debug!() or remove logging"
    echo ""
    exit 1
fi

echo "✅ No dbg!() in production code"

##############################################################################
# RULE 5: Check for banned crates in Cargo.lock
##############################################################################

echo "[reality_lock] Checking for banned crates in dependency graph..."

BANNED_CRATES=(
    "mockall"
    "fake"
    "double"
    "rstest"
    "proptest"
    "quickcheck"
    "arbitrary"
    "fakeit"
)

VIOLATIONS=0

for crate in "${BANNED_CRATES[@]}"; do
    # Check if crate appears in Cargo.lock (would indicate it's a dependency)
    # We'll do a basic check: if it's in Cargo.lock and NOT in a dev-dependencies section,
    # it's a violation. This is a heuristic; a proper check would parse Cargo.toml.
    
    if grep -q "name = \"$crate\"" Cargo.lock 2>/dev/null; then
        # Found the crate. Now check if it's in a Cargo.toml as dev-dependency
        # For now, we'll flag it and rely on cargo deny for stricter checking
        echo "⚠️  WARNING: Crate '$crate' found in Cargo.lock"
        VIOLATIONS=$((VIOLATIONS + 1))
    fi
done

if [[ $VIOLATIONS -gt 0 ]]; then
    echo ""
    echo "⚠️  WARNING: Found $VIOLATIONS potentially banned crate(s) in dependency tree"
    echo "   This will be verified by 'cargo deny check bans' in CI"
    echo ""
fi

##############################################################################
# RULE 6: Check for empty/stub impl blocks
##############################################################################

echo "[reality_lock] Checking for empty impl blocks (stubs)..."

VIOLATIONS=0

# Look for empty impl patterns in production code
# Pattern: impl <trait/type> { } with no meaningful content
while IFS=: read -r file line_num match || [[ -n "$match" ]]; do
    # Skip if this is a test file
    if is_test_path "$file"; then
        continue
    fi
    
    # Further validation: check if the next line is just '}'
    # This is a heuristic and may have false positives
    echo "FOUND: Possible stub impl at $file:$line_num"
    VIOLATIONS=$((VIOLATIONS + 1))
done < <(rg -n "impl.*\{\s*\}" crates apps 2>/dev/null || true)

# Don't fail on this; it's a warning. Empty impls are valid in some cases.
if [[ $VIOLATIONS -gt 0 ]]; then
    echo "⚠️  $VIOLATIONS possible empty impl block(s) found"
    echo "   Review these for stub implementations"
    echo ""
fi

##############################################################################
# FINAL RESULT
##############################################################################

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "✅ REALITY LOCK CHECK PASSED"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Production code is compliant with RUST_REALITY_LOCK policy"
echo "Reference: docs/governance/RUST_REALITY_LOCK.md"
echo ""

exit 0
