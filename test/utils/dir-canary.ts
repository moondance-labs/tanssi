import path from "node:path";
import { randomUUID } from "node:crypto";
import { promises as fs } from "node:fs";

/**
 * dir-canary.ts — tiny test helper to detect directory recreation.
 *
 * Use case:
 *   In some tests we check that directories are not deleted by doing directory.exists().
 *   That works in most cases but it fails on the edge case of the directory being
 *   deleted and then created again.
 *
 * Approach:
 *   We write a one-off ".test-canary" file with a random UUID inside the target directory.
 *   If the directory truly persists, the same file with the same contents will still be
 *   there later. If the directory was removed/recreated (or cleaned), the canary will be
 *   missing or have different contents.
 */

export type CanarySnapshot = { file: string; value: string };

/**
 * Create the canary for this directory; throw if it already exists.
 */
export async function snapshotCanary(dir: string): Promise<CanarySnapshot> {
    const file = path.join(dir, ".test-canary");
    const value = randomUUID();
    try {
        await fs.writeFile(file, value, { flag: "wx", encoding: "utf8" }); // fail if exists
        return { file, value };
    } catch (e) {
        const err = e as NodeJS.ErrnoException;
        if (err.code === "EEXIST") throw new Error(`Canary already exists: ${file}`);
        if (err.code === "ENOENT") throw new Error(`Directory not found for canary: ${dir}`);
        throw err;
    }
}

/** Throws if the canary value changed (dir likely deleted & recreated). */
export async function verifyCanary(s: CanarySnapshot): Promise<void> {
    try {
        const now = await fs.readFile(s.file, "utf8");
        if (now !== s.value) {
            throw new Error(`Canary changed: ${s.file} — directory was likely removed and recreated`);
        }
    } catch (e: any) {
        if (e.code === "ENOENT") {
            // Canary (or its dir) is gone ⇒ dir was likely deleted/recreated
            throw new Error(`Canary missing: ${s.file} — directory was likely removed and recreated`);
        }
        throw e; // surface unexpected I/O errors
    }
}
