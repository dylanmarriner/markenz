#!/usr/bin/env python3
"""
Phase 0 Determinism Guard Tool

Enforces deterministic programming constraints for Markenz Phase 0.
Scans source code for prohibited patterns and APIs that would break determinism.

This is a critical enforcement tool for Phase 0 compliance.
Any violations found must block CI/CD pipelines.
"""

import argparse
import json
import re
import sys
from pathlib import Path
from typing import List, Dict, Any, Set, Tuple

class DeterminismGuard:
    def __init__(self, scan_path: str):
        self.scan_path = Path(scan_path)
        self.violations = []
        self.prohibited_patterns = {
            # Nondeterministic APIs (forbidden in authority code)
            'Math.random': r'Math\.random\(',
            'Date.now': r'Date\.now\(',
            'SystemTime': r'SystemTime|UNIX_EPOCH',
            'random': r'\.random\(\)',  # Generic random calls
            
            # Agent identity conditionals (forbidden structural patterns)
            'agent_id_conditional': r'if\s*.*agent_id\s*==\s*["\'][^"\']*["\']',
            'agent_switch': r'switch\s*\([^)]*agent_id[^)]*\)',
            
            # Per-agent feature flags (forbidden)
            'agent_feature_flag': r'cfg\s*\(\s*feature\s*=\s*["\'][^"\']*agent["\']',
            
            # Placeholder implementations (forbidden by No-Mock law)
            'todo': r'TODO|FIXME|stub|mock|fake|placeholder',
            
            # Wall clock usage in authority path
            'wall_clock': r'wall.*clock|system.*time|gettimeofday|clock\(\)',
        }
        
        # Files that should never contain prohibited patterns
        self.authority_files = [
            'apps/engine/src/',
            'crates/world/src/',
            'crates/physics/src/',
            'crates/biology/src/',
            'crates/genetics/src/',
            'crates/cognition/src/',
            'crates/persistence/src/',
            'crates/events/src/',
        ]
        
        # Patterns that are allowed in specific contexts
        self.allowed_contexts = {
            'Math.random': [
                'tools/audits/',  # Audit tools may use random for testing
                'tests/',         # Test files may use random for test data
            ],
            'Date.now': [
                'tools/audits/',  # Audit tools may timestamp reports
                'tests/',         # Test files may use current time
            ],
        }

    def scan_file(self, file_path: Path) -> List[Dict[str, Any]]:
        """Scan a single file for prohibited patterns"""
        violations = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                relative_path = str(file_path.relative_to(self.scan_path))
                
                # Check if this is an authority file
                is_authority_file = any(
                    relative_path.startswith(auth_path) 
                    for auth_path in self.authority_files
                )
                
                # Scan for each prohibited pattern
                for pattern_name, pattern in self.prohibited_patterns.items():
                    matches = re.finditer(pattern, content, re.MULTILINE | re.IGNORECASE)
                    
                    for match in matches:
                        line_num = content[:match.start()].count('\n') + 1
                        line_content = content.split('\n')[line_num - 1].strip()
                        
                        # Check if this pattern is allowed in this context
                        if self.is_pattern_allowed(pattern_name, relative_path):
                            continue
                        
                        violation = {
                            'file': relative_path,
                            'line': line_num,
                            'pattern': pattern_name,
                            'match': match.group(),
                            'line_content': line_content.strip(),
                            'severity': 'CRITICAL' if is_authority_file else 'HIGH',
                            'reason': self.get_violation_reason(pattern_name)
                        }
                        violations.append(violation)
                        
        except Exception as e:
            violations.append({
                'file': str(file_path.relative_to(self.scan_path)),
                'error': f'Failed to scan file: {e}',
                'severity': 'ERROR'
            })
        
        return violations

    def is_pattern_allowed(self, pattern_name: str, file_path: str) -> bool:
        """Check if a prohibited pattern is allowed in this context"""
        if pattern_name in self.allowed_contexts:
            for allowed_path in self.allowed_contexts[pattern_name]:
                if file_path.startswith(allowed_path):
                    return True
        return False

    def get_violation_reason(self, pattern_name: str) -> str:
        """Get human-readable reason for violation"""
        reasons = {
            'Math.random': 'Nondeterministic random number generation breaks replay consistency',
            'Date.now': 'Wall clock usage breaks deterministic time evolution',
            'SystemTime': 'System time usage breaks deterministic time evolution',
            'random': 'Generic random function usage breaks determinism',
            'agent_id_conditional': 'Agent-ID conditionals violate structural parity requirements',
            'agent_switch': 'Agent-based switch statements violate structural parity',
            'agent_feature_flag': 'Per-agent feature flags violate parity requirements',
            'todo': 'Placeholder implementations violate No-Mock/No-Stub law',
            'wall_clock': 'Wall clock usage in authority path breaks determinism',
        }
        return reasons.get(pattern_name, f'Unknown violation: {pattern_name}')

    def scan_all_files(self) -> None:
        """Scan all relevant files for prohibited patterns"""
        print(f"🔍 Scanning {self.scan_path} for determinism violations...")
        
        total_violations = []
        
        # Scan only authority files for strict compliance
        for auth_path in self.authority_files:
            full_path = self.scan_path / auth_path
            if full_path.exists():
                for file_path in full_path.rglob('*.rs'):
                    violations = self.scan_file(file_path)
                    total_violations.extend(violations)
        
        self.violations = total_violations
        
        # Report results
        self.print_results()

    def print_results(self) -> None:
        """Print scan results with appropriate exit codes"""
        print()
        
        if not self.violations:
            print("✅ PASS: No determinism violations found")
            print("   All authority files comply with Phase 0 constraints")
            return
        
        print(f"❌ FAIL: Found {len(self.violations)} determinism violations")
        print()
        
        # Group violations by severity
        critical_violations = [v for v in self.violations if v['severity'] == 'CRITICAL']
        high_violations = [v for v in self.violations if v['severity'] == 'HIGH']
        
        # Print critical violations first
        if critical_violations:
            print("🚨 CRITICAL VIOLATIONS (Authority Files):")
            for violation in critical_violations:
                print(f"   {violation['file']}:{violation['line']} - {violation['reason']}")
                print(f"      Pattern: {violation['pattern']}")
                print(f"      Code: {violation['line_content']}")
                print()
        
        # Print high severity violations
        if high_violations:
            print("⚠️  HIGH SEVERITY VIOLATIONS:")
            for violation in high_violations:
                print(f"   {violation['file']}:{violation['line']} - {violation['reason']}")
                print(f"      Pattern: {violation['pattern']}")
                print(f"      Code: {violation['line_content']}")
                print()
        
        # Print errors
        error_violations = [v for v in self.violations if v['severity'] == 'ERROR']
        if error_violations:
            print("💥 SCAN ERRORS:")
            for violation in error_violations:
                print(f"   {violation['file']} - {violation['error']}")
                print()

    def generate_report(self) -> Dict[str, Any]:
        """Generate comprehensive violation report"""
        return {
            'scan_metadata': {
                'scan_path': str(self.scan_path),
                'total_violations': len(self.violations),
                'critical_violations': len([v for v in self.violations if v['severity'] == 'CRITICAL']),
                'high_violations': len([v for v in self.violations if v['severity'] == 'HIGH']),
            },
            'violations': self.violations,
            'phase_0_compliance': {
                'determinism_enforced': len(self.violations) == 0,
                'no_nondeterministic_apis': len([v for v in self.violations if 'random' in v['pattern'] or 'time' in v['pattern']]) == 0,
                'structural_parity': len([v for v in self.violations if 'agent' in v['pattern']]) == 0,
                'no_placeholders': len([v for v in self.violations if v['pattern'] == 'todo']) == 0,
            }
        }

    def save_report(self, output_file: str) -> None:
        """Save violation report to file"""
        report = self.generate_report()
        
        try:
            with open(output_file, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"📄 Report saved to {output_file}")
        except Exception as e:
            print(f"❌ Failed to save report: {e}")

def main():
    parser = argparse.ArgumentParser(description='Phase 0 Determinism Guard Tool')
    parser.add_argument('scan_path', help='Path to scan for determinism violations')
    parser.add_argument('--output', type=str, help='Output report file (optional)')
    parser.add_argument('--json', action='store_true', help='Output results in JSON format')
    
    args = parser.parse_args()
    
    # Create guard instance
    guard = DeterminismGuard(args.scan_path)
    
    # Run scan
    guard.scan_all_files()
    
    # Save report if requested
    if args.output:
        guard.save_report(args.output)
    
    # Output JSON if requested
    if args.json:
        report = guard.generate_report()
        print(json.dumps(report, indent=2))
    
    # Exit with appropriate code
    sys.exit(0 if not guard.violations else 1)

if __name__ == '__main__':
    main()
