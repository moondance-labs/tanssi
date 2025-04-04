import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

export function getKeyringNimbusIdHex(name: string) {
    const keyring = new Keyring({ type: "sr25519" });
    const key = keyring.addFromUri(`//${name}`, { name: `${name} default` });
    return u8aToHex(key.publicKey);
}

// Create a map of collator key "5C5p..." to collator name "Collator1000-01".
export function createCollatorKeyToNameMap(paraApi: ApiPromise, collatorNames: string[]): Record<string, string> {
    const collatorName: Record<string, string> = {};

    for (const name of collatorNames) {
        const hexAddress = getKeyringNimbusIdHex(name);
        const k = paraApi.createType("AccountId", hexAddress);
        collatorName[k] = name;
    }

    return collatorName;
}
