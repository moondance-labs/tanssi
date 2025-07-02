import type { ApiPromise } from "@polkadot/api";

export function calculateIdentityDeposit(api: ApiPromise, info: any) {
    const identityInfo = api.registry.createType("IdentityInfo", info);

    const byteLength = identityInfo.toU8a().length;

    const basicDeposit = api.consts.identity.basicDeposit.toBigInt();
    const byteDeposit = api.consts.identity.byteDeposit.toBigInt();

    return basicDeposit + byteDeposit * BigInt(byteLength);
}
