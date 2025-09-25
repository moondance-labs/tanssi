import { monitorBlockProduction } from "./zombie.ts";

export function addApiMonitoringToContext(context: any) {
    const originalFn = context.polkadotJs.bind(context);

    context.polkadotJs = (...args: any[]) => {
        const api = originalFn(...args);

        monitorBlockProduction([api]);

        return api;
    };
}
