
/**
 * Socket.io wrapper module
 */

export class Server {
  on(event: string, callback: (...args: any[]) => void): void {}
  emit(event: string, ...args: any[]): void {}
}

export function createServer(options?: any): Server {
  return new Server();
}

export function io(srv: any, options?: any): Server {
  return new Server();
}
