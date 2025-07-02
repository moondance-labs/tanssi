import type { ApiPromise } from "@polkadot/api";

export function calculateIdentityDeposit(api: ApiPromise, info: any): bigint {
    const identityInfo = api.registry.createType("IdentityInfo", info);

    const byteLength = identityInfo.toU8a().length;

    const basicDeposit = api.consts.identity.basicDeposit.toBigInt();
    const byteDeposit = api.consts.identity.byteDeposit.toBigInt();

    return basicDeposit + byteDeposit * BigInt(byteLength);
}

export function calculateSubIdentityDeposit(api: ApiPromise, amount: number): bigint {
    if (amount === 0) {
        return 0n;
    }

    return api.consts.identity.subAccountDeposit.toBigInt() * BigInt(amount);
}
