// total =  T::ProxyDepositBase::get() + T::ProxyDepositFactor::get() * num_proxies.into()
import type { ApiPromise } from "@polkadot/api";

export const totalForProxies = (api: ApiPromise, n: number): bigint => {
    if (n === 0) {
        return 0n;
    }

    const proxyDepositBase = api.consts.proxy.proxyDepositBase.toBigInt();
    const proxyDepositFactor = api.consts.proxy.proxyDepositFactor.toBigInt();

    return proxyDepositBase + proxyDepositFactor * BigInt(n);
};
