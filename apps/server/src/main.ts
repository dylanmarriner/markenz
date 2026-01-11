import express from 'express';
import { WebSocketServer, WebSocket } from 'ws';
import { createServer } from 'http';
import dotenv from 'dotenv';
import { Database } from './database';

// Load environment variables
dotenv.config();

interface HealthStatus {
    status: 'healthy' | 'unhealthy';
    database: 'connected' | 'disconnected';
    timestamp: string;
}

class MarkenzServer {
    private app: express.Application;
    private server: any;
    private wss: WebSocketServer;
    private db: Database;
    private port: number;
    private wsPort: number;

    constructor() {
        this.app = express();
        this.server = createServer(this.app);
        this.port = parseInt(process.env['PORT'] || '3000', 10);
        this.wsPort = parseInt(process.env['WS_PORT'] || '3001', 10);
        
        // Initialize database connection
        const databaseUrl = process.env['DATABASE_URL'];
        if (!databaseUrl) {
            throw new Error('DATABASE_URL environment variable is required');
        }
        
        this.db = new Database(databaseUrl);
        this.wss = new WebSocketServer({ port: this.wsPort });
        
        this.setupMiddleware();
        this.setupRoutes();
        this.setupWebSocket();
    }

    private setupMiddleware(): void {
        this.app.use(express.json());
        this.app.use((req, res, next) => {
            res.header('Access-Control-Allow-Origin', '*');
            res.header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
            res.header('Access-Control-Allow-Headers', 'Origin, X-Requested-With, Content-Type, Accept, Authorization');
            if (req.method === 'OPTIONS') {
                res.sendStatus(200);
            } else {
                next();
            }
        });
    }

    private setupRoutes(): void {
        // Health check endpoint
        this.app.get('/health', (_req, res) => {
            const status: HealthStatus = {
                status: 'healthy',
                database: 'connected',
                timestamp: new Date().toISOString()
            };
            res.json(status);
        });

        // Input event submission
        this.app.post('/api/events', async (req, res) => {
            try {
                const { tick, source_id, payload_json } = req.body;
                
                // Validate required fields
                if (typeof tick !== 'number' || typeof source_id !== 'number' || !payload_json) {
                    res.status(400).json({ error: 'Missing required fields: tick, source_id, payload_json' });
                    return;
                }

                // Compute hash chain
                const prevHash = await this.getPreviousHash();
                const hash = Database.computeHash({ tick, source_id, payload_json, prev_hash: prevHash });
                
                const inputEvent = {
                    tick: BigInt(tick),
                    source_id: BigInt(source_id),
                    payload_json,
                    hash,
                    prev_hash: prevHash
                };

                await this.db.appendInputEvent(inputEvent);
                
                res.status(201).json({ 
                    success: true, 
                    hash: hash.toString('hex'),
                    tick 
                });
            } catch (error) {
                console.error('Error processing input event:', error);
                res.status(500).json({ error: 'Internal server error' });
            }
        });

        // Get events for a tick
        this.app.get('/api/events/:tick', async (req, res) => {
            try {
                const tick = BigInt(req.params.tick);
                const events = await this.db.getInputEventsForTick(tick);
                res.json(events);
            } catch (error) {
                console.error('Error fetching events:', error);
                res.status(500).json({ error: 'Internal server error' });
            }
        });

        // Get latest snapshot
        this.app.get('/api/snapshots/latest', async (_req, res) => {
            try {
                const snapshot = await this.db.getLatestSnapshot();
                if (!snapshot) {
                    res.status(404).json({ error: 'No snapshots found' });
                    return;
                }
                res.json(snapshot);
            } catch (error) {
                console.error('Error fetching snapshot:', error);
                res.status(500).json({ error: 'Internal server error' });
            }
        });

        // Verify hash chain
        this.app.get('/api/verify/:fromTick/:toTick', async (req, res) => {
            try {
                const fromTick = BigInt(req.params.fromTick);
                const toTick = BigInt(req.params.toTick);
                const isValid = await this.db.verifyHashChain(fromTick, toTick);
                res.json({ valid: isValid });
            } catch (error) {
                console.error('Error verifying hash chain:', error);
                res.status(500).json({ error: 'Internal server error' });
            }
        });
    }

    private setupWebSocket(): void {
        this.wss.on('connection', (ws: WebSocket) => {
            console.log('WebSocket client connected');
            
            ws.on('message', async (message: Buffer) => {
                try {
                    const data = JSON.parse(message.toString());
                    console.log('Received WebSocket message:', data);
                    
                    // Handle different message types
                    switch (data.type) {
                        case 'subscribe_events':
                            // Subscribe to observation events
                            ws.send(JSON.stringify({ type: 'subscribed', topic: 'events' }));
                            break;
                        case 'subscribe_hashes':
                            // Subscribe to hash checkpoints
                            ws.send(JSON.stringify({ type: 'subscribed', topic: 'hashes' }));
                            break;
                        default:
                            ws.send(JSON.stringify({ error: 'Unknown message type' }));
                    }
                } catch (error) {
                    console.error('Error handling WebSocket message:', error);
                    ws.send(JSON.stringify({ error: 'Invalid message format' }));
                }
            });

            ws.on('close', () => {
                console.log('WebSocket client disconnected');
            });

            ws.on('error', (error) => {
                console.error('WebSocket error:', error);
            });
        });

        console.log(`WebSocket server listening on port ${this.wsPort}`);
    }

    private async getPreviousHash(): Promise<Buffer | null> {
        try {
            const latestSnapshot = await this.db.getLatestSnapshot();
            return latestSnapshot ? latestSnapshot.input_hash : null;
        } catch (error) {
            console.error('Error getting previous hash:', error);
            return null;
        }
    }

    async start(): Promise<void> {
        try {
            // Initialize database connection and verify schema
            await this.db.initialize();
            console.log('Database initialized successfully');

            // Start HTTP server
            this.server.listen(this.port, () => {
                console.log(`Markenz Server listening on port ${this.port}`);
                console.log(`Health check available at http://localhost:${this.port}/health`);
            });
        } catch (error) {
            console.error('Failed to start server:', error);
            process.exit(1);
        }
    }

    async stop(): Promise<void> {
        console.log('Shutting down server...');
        
        // Close WebSocket server
        this.wss.close();
        
        // Close HTTP server
        this.server.close();
        
        // Close database connection
        await this.db.close();
        
        console.log('Server shutdown complete');
    }
}

// Handle graceful shutdown
process.on('SIGINT', async () => {
    console.log('Received SIGINT, shutting down gracefully...');
    process.exit(0);
});

process.on('SIGTERM', async () => {
    console.log('Received SIGTERM, shutting down gracefully...');
    process.exit(0);
});

// Start the server
const server = new MarkenzServer();
server.start().catch((error) => {
    console.error('Failed to start Markenz Server:', error);
    process.exit(1);
});
