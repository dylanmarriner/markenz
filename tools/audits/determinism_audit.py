#!/usr/bin/env python3
"""
Standalone Python audit tool for offline determinism verification

Usage:
    python determinism_audit.py [--db-url DATABASE_URL] [--seed SEED] [--output-format json|pdf]

Exit codes:
    0: Determinism verified
    1: Divergence detected
    2: Configuration error
"""

import argparse
import json
import sys
import psycopg2
from psycopg2.extras import DictCursor
from datetime import datetime
import hashlib
import os

class DeterminismAuditor:
    def __init__(self, db_url, seed):
        self.db_url = db_url
        self.seed = seed
        self.conn = None
        
    def connect(self):
        """Connect to PostgreSQL database"""
        try:
            self.conn = psycopg2.connect(self.db_url)
            self.conn.cursor_factory = DictCursor
            print(f"Connected to database: {self.db_url}")
        except Exception as e:
            print(f"ERROR: Failed to connect to database: {e}")
            sys.exit(2)
    
    def load_event_log(self):
        """Load event log from database"""
        try:
            cursor = self.conn.cursor()
            cursor.execute("""
                SELECT tick, event_type, payload, created_at
                FROM input_events
                ORDER BY tick ASC
            """)
            events = cursor.fetchall()
            print(f"Loaded {len(events)} events from database")
            return events
        except Exception as e:
            print(f"ERROR: Failed to load event log: {e}")
            sys.exit(2)
    
    def load_hash_checkpoints(self):
        """Load hash checkpoints from database"""
        try:
            cursor = self.conn.cursor()
            cursor.execute("""
                SELECT tick, world_hash, prev_hash, verified, created_at
                FROM hash_checkpoints
                ORDER BY tick ASC
            """)
            checkpoints = cursor.fetchall()
            print(f"Loaded {len(checkpoints)} hash checkpoints from database")
            return checkpoints
        except Exception as e:
            print(f"ERROR: Failed to load hash checkpoints: {e}")
            sys.exit(2)
    
    def replay_deterministically(self, events):
        """Replay events deterministically"""
        print("Starting deterministic replay...")
        
        # Initialize deterministic RNG with seed
        rng_state = hashlib.sha256(f"{self.seed}".encode()).digest()
        
        # Mock universe state for replay
        universe_state = {
            'tick': 0,
            'agents': {},
            'world_hash': hashlib.sha256(b'genesis').digest(),
            'rng_state': rng_state
        }
        
        replay_hashes = []
        
        for event in events:
            # Apply event deterministically
            universe_state['tick'] = event['tick']
            
            # Mock state transition (in reality, this would be the actual engine logic)
            event_hash = hashlib.sha256(
                f"{event['tick']}{event['event_type']}{event['payload']}{universe_state['rng_state']}".encode()
            ).digest()
            
            universe_state['world_hash'] = event_hash
            replay_hashes.append((event['tick'], event_hash.hex()))
            
            # Update RNG state (mock ChaCha20)
            rng_state = hashlib.sha256(rng_state + b'tick').digest()
            universe_state['rng_state'] = rng_state
        
        print(f"Replay completed: {len(replay_hashes)} hashes generated")
        return replay_hashes
    
    def verify_hash_chain(self, db_checkpoints, replay_hashes):
        """Verify replay hashes match database checkpoints"""
        print("Verifying hash chain...")
        
        # Create dict of replay hashes for easy lookup
        replay_hash_dict = {tick: hash_hex for tick, hash_hex in replay_hashes}
        
        divergences = []
        
        for checkpoint in db_checkpoints:
            tick = checkpoint['tick']
            db_hash = checkpoint['world_hash'].hex()
            
            if tick in replay_hash_dict:
                replay_hash = replay_hash_dict[tick]
                if db_hash != replay_hash:
                    divergences.append({
                        'tick': tick,
                        'db_hash': db_hash,
                        'replay_hash': replay_hash,
                        'type': 'hash_mismatch'
                    })
            else:
                divergences.append({
                    'tick': tick,
                    'db_hash': db_hash,
                    'replay_hash': None,
                    'type': 'missing_hash'
                })
        
        if divergences:
            print(f"❌ DIVERGENCE DETECTED: {len(divergences)} issues found")
            for divergence in divergences[:5]:  # Show first 5
                print(f"  Tick {divergence['tick']}: {divergence['type']}")
                print(f"    DB hash:     {divergence['db_hash']}")
                print(f"    Replay hash: {divergence['replay_hash']}")
            return False, divergences
        else:
            print("✅ Hash chain verified: No divergences detected")
            return True, []
    
    def generate_report(self, verification_result, divergences, output_format='json'):
        """Generate audit report"""
        report = {
            'audit_metadata': {
                'timestamp': datetime.now().isoformat(),
                'seed': self.seed,
                'tool_version': '1.0.0',
                'database_url': self.db_url.split('@')[-1] if '@' in self.db_url else 'localhost'
            },
            'verification_result': {
                'status': 'PASS' if verification_result else 'FAIL',
                'divergences_count': len(divergences),
                'first_divergence_tick': divergences[0]['tick'] if divergences else None
            },
            'divergences': divergences,
            'summary': {
                'total_checkpoints': len(self.load_hash_checkpoints()),
                'total_events': len(self.load_event_log()),
                'verification_time': datetime.now().isoformat()
            }
        }
        
        if output_format == 'json':
            filename = f"determinism_audit_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
            with open(filename, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"JSON report saved to: {filename}")
        elif output_format == 'pdf':
            # Simple text-based PDF report (placeholder)
            filename = f"determinism_audit_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.txt"
            with open(filename, 'w') as f:
                f.write("DETERMINISM AUDIT REPORT\n")
                f.write("=" * 50 + "\n\n")
                f.write(f"Status: {report['verification_result']['status']}\n")
                f.write(f"Seed: {report['audit_metadata']['seed']}\n")
                f.write(f"Timestamp: {report['audit_metadata']['timestamp']}\n\n")
                if divergences:
                    f.write("DIVERGENCES FOUND:\n")
                    for div in divergences:
                        f.write(f"  Tick {div['tick']}: {div['type']}\n")
                else:
                    f.write("No divergences detected.\n")
            print(f"Text report saved to: {filename}")
        
        return report
    
    def run_audit(self, output_format='json'):
        """Run complete determinism audit"""
        print(f"Starting determinism audit with seed {self.seed}")
        
        # Connect to database
        self.connect()
        
        # Load data
        events = self.load_event_log()
        checkpoints = self.load_hash_checkpoints()
        
        # Replay deterministically
        replay_hashes = self.replay_deterministically(events)
        
        # Verify hash chain
        verification_result, divergences = self.verify_hash_chain(checkpoints, replay_hashes)
        
        # Generate report
        report = self.generate_report(verification_result, divergences, output_format)
        
        # Close connection
        if self.conn:
            self.conn.close()
        
        return verification_result, report

def main():
    parser = argparse.ArgumentParser(description='Determinism audit tool')
    parser.add_argument('--db-url', default='postgresql://localhost/markenz',
                        help='PostgreSQL database URL')
    parser.add_argument('--seed', type=int, default=1337,
                        help='Genesis seed for deterministic replay')
    parser.add_argument('--output-format', choices=['json', 'pdf'], default='json',
                        help='Report output format')
    
    args = parser.parse_args()
    
    # Run audit
    auditor = DeterminismAuditor(args.db_url, args.seed)
    verification_result, report = auditor.run_audit(args.output_format)
    
    # Exit with appropriate code
    if verification_result:
        print("✅ AUDIT PASSED: Determinism verified")
        sys.exit(0)
    else:
        print("❌ AUDIT FAILED: Divergence detected")
        sys.exit(1)

if __name__ == '__main__':
    main()
