export class IndexedDBFilesystem {
  private static readonly DB_NAME = "wie_filesystem";
  private static readonly STORE = "files";

  private db: IDBDatabase;

  private constructor(db: IDBDatabase) {
    this.db = db;
  }

  public static open(): Promise<IndexedDBFilesystem> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(IndexedDBFilesystem.DB_NAME, 1);

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains(IndexedDBFilesystem.STORE)) {
          db.createObjectStore(IndexedDBFilesystem.STORE);
        }
      };

      request.onsuccess = (event) => {
        resolve(new IndexedDBFilesystem((event.target as IDBOpenDBRequest).result));
      };

      request.onerror = (event) => {
        reject((event.target as IDBOpenDBRequest).error);
      };
    });
  }

  public exists(aid: string, path: string): Promise<boolean> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(IndexedDBFilesystem.STORE, "readonly");
      const store = transaction.objectStore(IndexedDBFilesystem.STORE);
      const request = store.getKey([aid, path]);

      request.onsuccess = () => {
        resolve(request.result !== undefined);
      };

      request.onerror = () => {
        reject(request.error);
      };
    });
  }

  public get(aid: string, path: string): Promise<Uint8Array | undefined> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(IndexedDBFilesystem.STORE, "readonly");
      const store = transaction.objectStore(IndexedDBFilesystem.STORE);
      const request = store.get([aid, path]);

      request.onsuccess = () => {
        resolve(request.result as Uint8Array | undefined);
      };

      request.onerror = () => {
        reject(request.error);
      };
    });
  }

  public write(aid: string, path: string, offset: number, data: Uint8Array): Promise<number> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(IndexedDBFilesystem.STORE, "readwrite");
      const store = transaction.objectStore(IndexedDBFilesystem.STORE);
      const get_request = store.get([aid, path]);

      get_request.onsuccess = () => {
        const existing = get_request.result as Uint8Array | undefined;
        const needed = offset + data.length;
        const next_length = Math.max(existing?.length ?? 0, needed);
        const next = new Uint8Array(next_length);
        if (existing) {
          next.set(existing);
        }
        next.set(data, offset);

        const put_request = store.put(next, [aid, path]);
        put_request.onerror = () => reject(put_request.error);
      };

      get_request.onerror = () => {
        reject(get_request.error);
      };

      transaction.oncomplete = () => {
        resolve(data.length);
      };
      transaction.onerror = () => {
        reject(transaction.error);
      };
    });
  }

  public truncate(aid: string, path: string, length: number): Promise<void> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(IndexedDBFilesystem.STORE, "readwrite");
      const store = transaction.objectStore(IndexedDBFilesystem.STORE);
      const get_request = store.get([aid, path]);

      get_request.onsuccess = () => {
        const existing = (get_request.result as Uint8Array | undefined) ?? new Uint8Array(0);
        const next = new Uint8Array(length);
        next.set(existing.subarray(0, Math.min(existing.length, length)));

        const put_request = store.put(next, [aid, path]);
        put_request.onerror = () => reject(put_request.error);
      };

      get_request.onerror = () => {
        reject(get_request.error);
      };

      transaction.oncomplete = () => {
        resolve();
      };
      transaction.onerror = () => {
        reject(transaction.error);
      };
    });
  }
}
