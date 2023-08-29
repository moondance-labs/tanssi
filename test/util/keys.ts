import { Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

export function getKeyringNimbusIdHex(name: string) {
    const keyring = new Keyring({ type: "sr25519" });
    const key = keyring.addFromUri("//" + name, { name: name + " default" });
    return u8aToHex(key.publicKey);
}
