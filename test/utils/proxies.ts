// total =  T::ProxyDepositBase::get() + T::ProxyDepositFactor::get() * num_proxies.into()
import { deposit } from "./account.ts";

const proxyDepositBase = deposit(1, 8);
const proxyDepositFactor = deposit(0, 33);
export const totalForProxies = (n: number): bigint => {
    if (n === 0) {
        return 0n;
    }

    return proxyDepositBase + proxyDepositFactor * BigInt(n);
};
