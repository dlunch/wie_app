class IndexedDBStore {
  private db: IDBDatabase;
  private store_name: string;

  private constructor(db: IDBDatabase, store_name: string) {
    this.db = db;
    this.store_name = store_name;
  }

  public static open(
    db_name: string,
    store_name: string
  ): Promise<IndexedDBStore> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(db_name);

      request.onsuccess = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        resolve(new IndexedDBStore(db, store_name));
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

  public get_record_ids(): Promise<Uint8Array> {
    return new Promise((resolve, reject) => {
      const transaction = this.db.transaction(this.store_name, "readonly");
      const store = transaction.objectStore(this.store_name);
      const request = store.getAllKeys();

      request.onsuccess = (event) => {
        resolve((event.target as IDBRequest).result as Uint8Array);
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
      const request = store.put(data, id);

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
      const request = store.get(id);

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
      const request = store.delete(id);

      request.onsuccess = () => {
        resolve();
      };

      request.onerror = (event) => {
        reject((event.target as IDBRequest).error);
      };
    });
  }
}
