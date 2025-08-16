export class IndexedDBStore {
  private db: IDBDatabase;
  private store_name: string;
  private key_prefix: string;

  private constructor(db: IDBDatabase, store_name: string, key_prefix: string) {
    this.db = db;
    this.store_name = store_name;
    this.key_prefix = key_prefix;
  }

  public static open(
    store_name: string,
    key_prefix: string
  ): Promise<IndexedDBStore> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open("wie");

      request.onsuccess = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        resolve(new IndexedDBStore(db, store_name, key_prefix));
      };

      request.onerror = (event) => {
        reject((event.target as IDBOpenDBRequest).error);
      };

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        db.createObjectStore(store_name);
      };
    });
  }

  public get_record_ids(): Promise<Int32Array> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(this.store_name, "readonly");
      const store = transaction.objectStore(this.store_name);
      const request = store.getAllKeys();

      request.onsuccess = (event) => {
        const result = (event.target as IDBRequest).result as Array<string>;
        const ids = new Int32Array(
          result.map((x) => x.replace(this.key_prefix, "")).map(Number)
        );
        resolve(ids);
      };

      request.onerror = (event) => {
        reject((event.target as IDBRequest).error);
      };
    });
  }

  public set(id: number, data: Uint8Array): Promise<void> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(this.store_name, "readwrite");
      const store = transaction.objectStore(this.store_name);
      const request = store.put(data, `${this.key_prefix}${id}`);

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = (event) => {
        reject((event.target as IDBRequest).error);
      };
    });
  }

  public get(id: number): Promise<Uint8Array | null> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(this.store_name, "readonly");
      const store = transaction.objectStore(this.store_name);
      const request = store.get(`${this.key_prefix}${id}`);

      request.onsuccess = (event) => {
        resolve((event.target as IDBRequest).result as Uint8Array | null);
      };

      request.onerror = (event) => {
        reject((event.target as IDBRequest).error);
      };
    });
  }

  public delete(id: number): Promise<void> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(this.store_name, "readwrite");
      const store = transaction.objectStore(this.store_name);
      const request = store.delete(`${this.key_prefix}${id}`);

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = (event) => {
        reject((event.target as IDBRequest).error);
      };
    });
  }
}
