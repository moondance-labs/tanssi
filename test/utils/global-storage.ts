/*
 *   This module is responsible to cache the data between smoke tests executions.
 *   The data is serialized and stored in a file in the tmp directory.
 *   The serialization lib supports data structures like Set/Map/Date/BigInt
 *   By default the key ttl is 15 minutes, but it can be overridden.
 */

import fs from "node:fs";
import serializeJavascript from "serialize-javascript";

export type StorageEntryType = { value: unknown; createdAt: number; ttlMs: number };
const STORAGE_PATH = "tmp/test_runtime_global_storage_v1.json";

// Before get(), always call has(), it will invalidate the cache by ttl
export const globalStorageGet = <T>(key: string): T => {
    const globalData = loadAndDeserialize();

    console.debug("Getting the key", key);

    const value = globalData.get(key);
    if (value === undefined) {
        throw new Error(`Key "${key}" not found in global storage`);
    }

    return value.value as T;
};

export const globalStorageSet = (key: string, value: unknown, ttlMs = 15 * 60 * 1000) => {
    console.debug("Setting the key/value:", key);

    const globalData = loadAndDeserialize();

    globalData.set(key, { value, createdAt: Date.now(), ttlMs });

    serializeAndPersist(globalData);
};

export const globalStorageHas = (key: string) => {
    const globalData = loadAndDeserialize();
    console.debug("Checking global storage for key:", key);

    const record = globalData.get(key);
    if (!record) {
        console.debug("No data for storage key:", key);

        return false;
    }

    if (Date.now() - record.createdAt > record.ttlMs) {
        globalData.delete(key);
        console.debug("The value is outdated for storage key:", key);

        return false;
    }

    console.debug("Data exists for key:", key);

    return true;
};

function serializeAndPersist(data: Map<string, StorageEntryType>): void {
    const serialized = serializeJavascript(data);
    fs.writeFileSync(STORAGE_PATH, serialized, "utf-8");
}

function loadAndDeserialize(): Map<string, StorageEntryType> {
    if (!fs.existsSync(STORAGE_PATH)) {
        return new Map();
    }
    const serialized = fs.readFileSync(STORAGE_PATH, "utf-8");

    // Since we serialize the data, we are safe
    // https://www.npmjs.com/package/serialize-javascript#deserializing
    // biome-ignore lint/security/noGlobalEval: this usage is safe in our context
    return eval(`(${serialized})`);
}
