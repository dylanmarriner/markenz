import React, { useState, useEffect } from 'react';

interface RngAuditEntry {
  tick: number;
  subsystem: string;
  stream_id: number;
  callsite: string;
  counter: number;
  value: number;
}

interface RngAuditViewerProps {
  auditLog: RngAuditEntry[];
}

export default function RngAuditViewer({ auditLog }: RngAuditViewerProps) {
  const [filteredLog, setFilteredLog] = useState<RngAuditEntry[]>(auditLog);
  const [selectedSubsystem, setSelectedSubsystem] = useState<string>('all');
  const [selectedTick, setSelectedTick] = useState<string>('');

  useEffect(() => {
    let filtered = auditLog;

    if (selectedSubsystem !== 'all') {
      filtered = filtered.filter(entry => entry.subsystem === selectedSubsystem);
    }

    if (selectedTick) {
      const tick = parseInt(selectedTick);
      if (!isNaN(tick)) {
        filtered = filtered.filter(entry => entry.tick === tick);
      }
    }

    setFilteredLog(filtered);
  }, [auditLog, selectedSubsystem, selectedTick]);

  const subsystems = Array.from(new Set(auditLog.map(entry => entry.subsystem))).sort();
  const ticks = Array.from(new Set(auditLog.map(entry => entry.tick))).sort();

  const getDrawCounts = () => {
    const counts: Record<string, Record<number, number>> = {};
    
    filteredLog.forEach(entry => {
      if (!counts[entry.subsystem]) {
        counts[entry.subsystem] = {};
      }
      counts[entry.subsystem][entry.tick] = (counts[entry.subsystem][entry.tick] || 0) + 1;
    });

    return counts;
  };

  const drawCounts = getDrawCounts();

  return (
    <div className="rng-audit-viewer">
      <h3 className="text-lg font-semibold mb-4">RNG Audit Log Viewer</h3>
      
      {/* Filters */}
      <div className="flex space-x-4 mb-4">
        <div>
          <label className="block text-sm font-medium mb-1">Subsystem:</label>
          <select 
            value={selectedSubsystem} 
            onChange={(e) => setSelectedSubsystem(e.target.value)}
            className="border rounded px-3 py-1"
          >
            <option value="all">All Subsystems</option>
            {subsystems.map(subsystem => (
              <option key={subsystem} value={subsystem}>{subsystem}</option>
            ))}
          </select>
        </div>
        
        <div>
          <label className="block text-sm font-medium mb-1">Tick:</label>
          <select 
            value={selectedTick} 
            onChange={(e) => setSelectedTick(e.target.value)}
            className="border rounded px-3 py-1"
          >
            <option value="">All Ticks</option>
            {ticks.map(tick => (
              <option key={tick} value={tick.toString}>Tick {tick}</option>
            ))}
          </select>
        </div>
      </div>

      {/* Draw Counts Summary */}
      <div className="mb-4">
        <h4 className="font-medium mb-2">Draw Counts per Subsystem/Tick:</h4>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-2">
          {Object.entries(drawCounts).map(([subsystem, tickCounts]) => (
            <div key={subsystem} className="border rounded p-2">
              <div className="font-medium text-sm">{subsystem}</div>
              {Object.entries(tickCounts).map(([tick, count]) => (
                <div key={tick} className="text-xs text-gray-600">
                  Tick {tick}: {count} draws
                </div>
              ))}
            </div>
          ))}
        </div>
      </div>

      {/* Audit Log Table */}
      <div className="overflow-x-auto">
        <table className="min-w-full border-collapse text-sm">
          <thead>
            <tr className="bg-gray-100">
              <th className="border p-2 text-left">Tick</th>
              <th className="border p-2 text-left">Subsystem</th>
              <th className="border p-2 text-left">Stream</th>
              <th className="border p-2 text-left">Callsite</th>
              <th className="border p-2 text-left">Counter</th>
              <th className="border p-2 text-left">Value</th>
            </tr>
          </thead>
          <tbody>
            {filteredLog.slice(0, 100).map((entry, index) => (
              <tr key={index} className="hover:bg-gray-50">
                <td className="border p-2">{entry.tick}</td>
                <td className="border p-2">
                  <span className={`px-2 py-1 rounded text-xs ${
                    entry.subsystem === 'Physics' ? 'bg-blue-100 text-blue-800' :
                    entry.subsystem === 'Environment' ? 'bg-green-100 text-green-800' :
                    entry.subsystem === 'Biology' ? 'bg-yellow-100 text-yellow-800' :
                    entry.subsystem === 'Cognition' ? 'bg-purple-100 text-purple-800' :
                    entry.subsystem === 'Genetics' ? 'bg-red-100 text-red-800' :
                    'bg-gray-100 text-gray-800'
                  }`}>
                    {entry.subsystem}
                  </span>
                </td>
                <td className="border p-2">{entry.stream_id}</td>
                <td className="border p-2 font-mono text-xs">{entry.callsite}</td>
                <td className="border p-2">{entry.counter}</td>
                <td className="border p-2 font-mono">{entry.value}</td>
              </tr>
            ))}
          </tbody>
        </table>
        
        {filteredLog.length > 100 && (
          <div className="text-sm text-gray-600 mt-2">
            Showing first 100 of {filteredLog.length} entries
          </div>
        )}
      </div>
    </div>
  );
}
