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

  public load_all(): Promise<Array<[string, string, Uint8Array]>> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(IndexedDBFilesystem.STORE, "readonly");
      const store = transaction.objectStore(IndexedDBFilesystem.STORE);
      const keys_request = store.getAllKeys();
      const values_request = store.getAll();

      transaction.oncomplete = () => {
        const keys = keys_request.result as Array<[string, string]>;
        const values = values_request.result as Uint8Array[];
        resolve(keys.map((key, i) => [key[0], key[1], values[i]]));
      };

      transaction.onerror = () => {
        reject(transaction.error);
      };
    });
  }

  public set(aid: string, path: string, data: Uint8Array): Promise<void> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(IndexedDBFilesystem.STORE, "readwrite");
      const store = transaction.objectStore(IndexedDBFilesystem.STORE);
      const request = store.put(data, [aid, path]);

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = (event) => {
        reject((event.target as IDBRequest).error);
      };
    });
  }

  public delete(aid: string, path: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(IndexedDBFilesystem.STORE, "readwrite");
      const store = transaction.objectStore(IndexedDBFilesystem.STORE);
      const request = store.delete([aid, path]);

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = (event) => {
        reject((event.target as IDBRequest).error);
      };
    });
  }
}
