/**
 * ROLE: EXECUTABLE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * CONNECTED VIA: MCP
 * EXECUTED VIA: windsurf
 * USED BY: web
 * PURPOSE: WebGPU voxel renderer
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

import { useEffect, useRef, useState } from 'react';

interface ChunkData {
  x: number;
  y: number;
  z: number;
  data: ArrayBuffer;
}

declare global {
  interface Navigator {
    gpu: any | undefined;
  }
}

interface GPUCanvasContext {
  configure(config: any): void;
  getCurrentTexture(): any;
  createView(): any;
}

export const Renderer: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [status, setStatus] = useState<string>('Initializing WebGPU...');
  const [chunks, setChunks] = useState<Map<string, ChunkData>>(new Map());

  useEffect(() => {
    const initWebGPU = async () => {
      if (!navigator.gpu) {
        setStatus('WebGPU not supported');
        return;
      }

      const canvas = canvasRef.current;
      if (!canvas) return;

      const adapter = await navigator.gpu.requestAdapter();
      if (!adapter) {
        setStatus('No WebGPU adapter found');
        return;
      }

      const device = await adapter.requestDevice();
      const context = canvas.getContext('webgpu') as GPUCanvasContext | null;
      
      if (!context) {
        setStatus('Failed to get WebGPU context');
        return;
      }

      const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
      
      context.configure({
        device,
        format: presentationFormat,
        alphaMode: 'premultiplied',
      });

      setStatus('WebGPU initialized');
      
      // Connect to chunk WebSocket
      const ws = new WebSocket('ws://localhost:3000/ws/chunks');
      
      ws.onopen = () => {
        setStatus('Connected to server');
        // Request initial chunk
        ws.send(JSON.stringify({ x: 0, y: 0, z: 0 }));
      };
      
      ws.onmessage = (event) => {
        if (event.data instanceof Blob) {
          event.data.arrayBuffer().then((buffer) => {
            // Parse binary chunk data
            const view = new DataView(buffer);
            const x = view.getInt32(0, true); // little endian
            const y = view.getInt32(4, true);
            const z = view.getInt32(8, true);
            const data = buffer.slice(12); // Skip header
            
            const chunkKey = `${x},${y},${z}`;
            setChunks(prev => new Map(prev).set(chunkKey, { x, y, z, data }));
            setStatus(`Loaded chunk ${chunkKey}`);
          });
        }
      };
      
      ws.onerror = () => {
        setStatus('WebSocket error');
      };
      
      ws.onclose = () => {
        setStatus('WebSocket closed');
      };

      // Simple render loop
      const render = () => {
        if (!context || !device) return;
        
        const commandEncoder = device.createCommandEncoder();
        const textureView = context.getCurrentTexture().createView();
        
        const renderPass = commandEncoder.beginRenderPass({
          colorAttachments: [
            {
              view: textureView,
              clearValue: { r: 0.1, g: 0.2, b: 0.3, a: 1.0 },
              loadOp: 'clear',
              storeOp: 'store',
            },
          ],
        });
        
        renderPass.end();
        device.queue.submit([commandEncoder.finish()]);
      };

      // Start render loop
      let animationId: number;
      const frame = () => {
        render();
        animationId = requestAnimationFrame(frame);
      };
      frame();

      return () => {
        cancelAnimationFrame(animationId);
        ws.close();
      };
    };

    initWebGPU();
  }, []);

  return (
    <div style={{ width: '100vw', height: '100vh', position: 'relative' }}>
      <canvas
        ref={canvasRef}
        width={800}
        height={600}
        style={{ width: '100%', height: '100%', display: 'block' }}
      />
      <div style={{
        position: 'absolute',
        top: 10,
        left: 10,
        background: 'rgba(0, 0, 0, 0.7)',
        color: 'white',
        padding: '10px',
        borderRadius: '5px',
        fontFamily: 'monospace'
      }}>
        <div>Status: {status}</div>
        <div>Chunks loaded: {chunks.size}</div>
        {Array.from(chunks.entries()).map(([key, chunk]) => (
          <div key={key}>Chunk {key}: {chunk.data.byteLength} bytes</div>
        ))}
      </div>
    </div>
  );
};

