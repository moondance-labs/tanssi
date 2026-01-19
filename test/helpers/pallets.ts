// @ts-nocheck

import type { DevModeContext } from "@moonwall/cli";

/**
 * Get the pallet index for a given pallet name
 */
export const getPalletIndex = async (palletName: string, context: DevModeContext): Promise<number> => {
    const metadata = await context.polkadotJs().rpc.state.getMetadata();
    const pallets = metadata.asLatest.pallets;
    const pallet = pallets.find((p) => p.name.toString() === palletName);
    if (!pallet) {
        throw new Error(`Pallet ${palletName} not found`);
    }
    return pallet.index.toNumber();
};
