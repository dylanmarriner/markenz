import React, { useState, useEffect } from 'react';
import { CheckpointHandler, HashCheckpoint } from '../server/hashing/checkpoint_handler';

interface DeterminismStatus {
  hash_chain_valid: boolean;
  current_tick: number;
  current_world_hash: string;
  latest_checkpoint: HashCheckpoint | null;
  snapshot_intervals: number[];
  rng_audit_samples: Array<{
    tick: number;
    subsystem: string;
    stream_id: number;
    callsite: string;
    counter: number;
    value: number;
  }>;
}

export default function DeterminismStatusPage() {
  const [status, setStatus] = useState<DeterminismStatus>({
    hash_chain_valid: false,
    current_tick: 0,
    current_world_hash: '',
    latest_checkpoint: null,
    snapshot_intervals: [],
    rng_audit_samples: [],
  });
  const [loading, setLoading] = useState(true);
  const [selectedSubsystem, setSelectedSubsystem] = useState<string>('all');

  useEffect(() => {
    const fetchData = async () => {
      try {
        // Mock WebSocket connection for live data
        const ws = new WebSocket('ws://localhost:8080/ws');
        
        ws.onmessage = (event) => {
          const data = JSON.parse(event.data);
          if (data.type === 'hash_checkpoint') {
            setStatus(prev => ({
              ...prev,
              current_tick: data.tick,
              current_world_hash: data.world_hash,
              hash_chain_valid: data.verified,
            }));
          }
        };

        // Fetch initial data
        const response = await fetch('/api/determinism-status');
        const data = await response.json();
        setStatus(data);

        // Fetch RNG audit log samples
        const auditResponse = await fetch('/api/rng-audit-log?limit=50');
        const auditData = await auditResponse.json();
        setStatus(prev => ({ ...prev, rng_audit_samples: auditData }));

      } catch (error) {
        console.error('Failed to fetch determinism status:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  const handleReplayTest = async () => {
    try {
      setLoading(true);
      const response = await fetch('/api/replay-test', { method: 'POST' });
      const result = await response.json();
      
      if (result.success) {
        alert('Replay test passed! Determinism verified.');
      } else {
        alert(`Replay test failed: ${result.error}`);
      }
    } catch (error) {
      console.error('Replay test failed:', error);
      alert('Replay test failed. See console for details.');
    } finally {
      setLoading(false);
    }
  };

  const filteredAuditSamples = selectedSubsystem === 'all' 
    ? status.rng_audit_samples 
    : status.rng_audit_samples.filter(sample => sample.subsystem === selectedSubsystem);

  if (loading) {
    return <div className="p-4">Loading determinism status...</div>;
  }

  return (
    <div className="p-6 max-w-6xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Determinism Status</h1>
      
      {/* Hash Chain Status */}
      <div className="bg-white rounded-lg shadow-md p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4">Hash Chain Status</h2>
        <div className="flex items-center space-x-4">
          <div className={`w-4 h-4 rounded-full ${status.hash_chain_valid ? 'bg-green-500' : 'bg-red-500'}`}></div>
          <span className="text-lg">
            {status.hash_chain_valid ? '✅ Valid' : '❌ Broken'}
          </span>
        </div>
        
        <div className="mt-4 grid grid-cols-2 gap-4">
          <div>
            <span className="font-medium">Current Tick:</span> {status.current_tick}
          </div>
          <div>
            <span className="font-medium">World Hash:</span> 
            <span className="font-mono text-sm ml-2">{status.current_world_hash}</span>
          </div>
        </div>
      </div>

      {/* Snapshot Timeline */}
      <div className="bg-white rounded-lg shadow-md p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4">Snapshot Timeline</h2>
        <div className="space-y-2">
          {status.snapshot_intervals.map((tick, index) => (
            <div key={index} className="flex items-center space-x-2">
              <div className="w-2 h-2 bg-blue-500 rounded-full"></div>
              <span>Snapshot at tick {tick}</span>
            </div>
          ))}
        </div>
      </div>

      {/* RNG Audit Log Viewer */}
      <div className="bg-white rounded-lg shadow-md p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4">RNG Audit Log</h2>
        
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2">Filter by Subsystem:</label>
          <select 
            value={selectedSubsystem} 
            onChange={(e) => setSelectedSubsystem(e.target.value)}
            className="border rounded px-3 py-2"
          >
            <option value="all">All Subsystems</option>
            <option value="Physics">Physics</option>
            <option value="Environment">Environment</option>
            <option value="Biology">Biology</option>
            <option value="Cognition">Cognition</option>
            <option value="Genetics">Genetics</option>
            <option value="Governance">Governance</option>
          </select>
        </div>

        <div className="overflow-x-auto">
          <table className="min-w-full border-collapse">
            <thead>
              <tr className="bg-gray-100">
                <th className="border p-2 text-left">Tick</th>
                <th className="border p-2 text-left">Subsystem</th>
                <th className="border p-2 text-left">Stream ID</th>
                <th className="border p-2 text-left">Callsite</th>
                <th className="border p-2 text-left">Counter</th>
                <th className="border p-2 text-left">Value</th>
              </tr>
            </thead>
            <tbody>
              {filteredAuditSamples.slice(0, 20).map((sample, index) => (
                <tr key={index} className="hover:bg-gray-50">
                  <td className="border p-2">{sample.tick}</td>
                  <td className="border p-2">{sample.subsystem}</td>
                  <td className="border p-2">{sample.stream_id}</td>
                  <td className="border p-2 font-mono text-xs">{sample.callsite}</td>
                  <td className="border p-2">{sample.counter}</td>
                  <td className="border p-2 font-mono">{sample.value}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Replay Test Button */}
      <div className="bg-white rounded-lg shadow-md p-6">
        <h2 className="text-xl font-semibold mb-4">Determinism Verification</h2>
        <button
          onClick={handleReplayTest}
          disabled={loading}
          className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 disabled:bg-gray-400"
        >
          {loading ? 'Running...' : 'Run Replay Test'}
        </button>
        <p className="text-sm text-gray-600 mt-2">
          Runs a single-run determinism verification by replaying the current state.
        </p>
      </div>
    </div>
  );
}
