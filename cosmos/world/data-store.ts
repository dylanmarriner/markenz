
/**
 * World Data Store
 */

export class WorldDataStore {
  private data: Map<string, any> = new Map();

  set(key: string, value: any): void {
    this.data.set(key, value);
  }

  get(key: string): any {
    return this.data.get(key);
  }

  delete(key: string): boolean {
    return this.data.delete(key);
  }
}

export function createDataStore(): WorldDataStore {
  return new WorldDataStore();
}
