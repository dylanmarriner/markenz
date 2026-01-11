/**
 * ROLE: EXECUTABLE
 * REGISTERED IN: MARKENZ_M1_FOUNDATION
 * CONNECTED VIA: MCP
 * EXECUTED VIA: windsurf
 * USED BY: web
 * PURPOSE: Vite build configuration
 * FAILURE MODES: PANIC_ON_INVALID_STATE
 *
 * Authority: antigravity
 */

import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    host: true
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom']
        }
      }
    }
  },
  define: {
    'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV || 'development')
  }
})

