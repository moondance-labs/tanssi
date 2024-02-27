import { bnToU8a, stringToU8a } from "@polkadot/util";
import { blake2AsU8a } from "@polkadot/util-crypto";

// Tank account is blake2(b"modlpy/serpayment" + parahain ID)
export function paraIdTank(paraId: any): any {
    const seedBytes = stringToU8a("modlpy/serpayment");
    const paraIdBytes = bnToU8a(paraId, { bitLength: 32 });
    const combinedBytes = new Uint8Array(seedBytes.length + paraIdBytes.length);
    combinedBytes.set(seedBytes);
    combinedBytes.set(paraIdBytes, seedBytes.length);
    const para_tank = blake2AsU8a(combinedBytes, 256);
    return para_tank;
}
