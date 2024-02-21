import { u16 } from "@polkadot/types";
import { DevModeContext } from "@moonwall/cli";
import { KeyringPair } from "@polkadot/keyring/types";
import type { AccountId20 } from "@polkadot/types/interfaces/runtime";

export const DUMMY_REVERT_BYTECODE = "0x60006000fd";
export const RELAY_SOURCE_LOCATION = { Xcm: { parents: 1, interior: "Here" } };
export const RELAY_SOURCE_LOCATION2 = { Xcm: { parents: 2, interior: "Here" } };
export const RELAY_V3_SOURCE_LOCATION = { V3: { parents: 1, interior: "Here" } } as any;
export const PARA_1000_SOURCE_LOCATION = {
    Xcm: { parents: 1, interior: { X1: { Parachain: 1000 } } },
};
export const PARA_2000_SOURCE_LOCATION = {
    Xcm: { parents: 1, interior: { X1: { Parachain: 2000 } } },
};
export const PARA_1001_SOURCE_LOCATION = {
    Xcm: { parents: 1, interior: { X1: { Parachain: 1001 } } },
};

export interface AssetMetadata {
    name: string;
    symbol: string;
    decimals: bigint;
    isFrozen: boolean;
}

export const relayAssetMetadata: AssetMetadata = {
    name: "DOT",
    symbol: "DOT",
    decimals: 12n,
    isFrozen: false,
};

export async function mockAssetCreation(
    context: DevModeContext,
    sudoAccount: KeyringPair,
    assetId: u16,
    admin: string | AccountId20,
    location: any,
    metadata: AssetMetadata,
    is_sufficient: boolean
) {
    const api = context.polkadotJs();
    // Register the asset
    await context.createBlock(
        api.tx.sudo
            .sudo(
                api.tx.utility.batch([
                    api.tx.foreignAssetsCreator.createForeignAsset(location, assetId, admin, is_sufficient, 1),
                    api.tx.assetRate.create(
                        assetId,
                        // this defines how much the asset costs with respect to the
                        // new asset
                        // in this case, asset*2=native
                        // that means that we will charge 0.5 of the native balance
                        2000000000000000000n
                    ),
                    api.tx.foreignAssets.forceSetMetadata(
                        assetId,
                        metadata.name,
                        metadata.symbol,
                        metadata.decimals,
                        metadata.isFrozen
                    ),
                ])
            )
            .signAsync(sudoAccount),
        { allowFailures: false }
    );

    const evmCodeAssetKey = api.query.evm.accountCodes.key(
        "0xfFfFFFffFffFFFFffFFfFfffFfFFFFFfffFF" + assetId.toHex().slice(2)
    );

    await context.createBlock(
        api.tx.sudo
            .sudo(
                api.tx.system.setStorage([
                    [
                        evmCodeAssetKey,
                        `0x${((DUMMY_REVERT_BYTECODE.length - 2) * 2)
                            .toString(16)
                            .padStart(2)}${DUMMY_REVERT_BYTECODE.slice(2)}`,
                    ],
                ])
            )
            .signAsync(sudoAccount),
        { allowFailures: false }
    );
    return;
}
