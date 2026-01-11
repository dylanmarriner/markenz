/**
 * ROLE: EXECUTABLE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * CONNECTED VIA: MCP
 * EXECUTED VIA: windsurf
 * USED BY: web
 * PURPOSE: Main React application component
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

import { Renderer } from './features/renderer';

function App() {
  return (
    <div className="App">
      <Renderer />
    </div>
  );
}

export default App;

