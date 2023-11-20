import { DevModeContext } from "@moonwall/cli";
import { u8aToHex } from "@polkadot/util";

export function descendOriginFromAddress20(
    context: DevModeContext,
    address: `0x${string}` = "0x0101010101010101010101010101010101010101",
    paraId: number = 1
) {
    const toHash = new Uint8Array([
        ...new TextEncoder().encode("SiblingChain"),
        ...context.polkadotJs().createType("Compact<u32>", paraId).toU8a(),
        ...context
            .polkadotJs()
            .createType("Compact<u32>", "AccountKey20".length + 20)
            .toU8a(),
        ...new TextEncoder().encode("AccountKey20"),
        ...context.polkadotJs().createType("AccountId", address).toU8a(),
    ]);

    return {
        originAddress: address,
        descendOriginAddress: u8aToHex(context.polkadotJs().registry.hash(toHash).slice(0, 20)),
    };
}
