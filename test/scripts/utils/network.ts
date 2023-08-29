import { Options } from "yargs";
import { ApiPromise, WsProvider } from "@polkadot/api";

export type NetworkOptions = {
    url: Options & { type: "string" };
    network: Options & { type: "string" };
    finalized: Options & { type: "boolean" };
};

export type Argv = {
    url?: string;
    network?: string;
    finalized?: boolean;
};

export type TANSSI_NETWORK_NAME = "stagenet" | "alphanet" | "tanssi";
export type POLKADOT_NETWORK_NAME = "kusama" | "polkadot";
export type NETWORK_NAME = TANSSI_NETWORK_NAME | POLKADOT_NETWORK_NAME;

export const NETWORK_WS_URLS: { [name in NETWORK_NAME]: string } = {
    // TODO: set public endpoints when they exist
    stagenet: "",
    alphanet: "",
    tanssi: "",
    kusama: "wss://kusama-rpc.polkadot.io",
    polkadot: "wss://rpc.polkadot.io",
};

export const NETWORK_NAMES = Object.keys(NETWORK_WS_URLS) as NETWORK_NAME[];

export const NETWORK_YARGS_OPTIONS: NetworkOptions = {
    url: {
        type: "string",
        description: "Websocket url",
        conflicts: ["network"],
        string: true,
    },
    network: {
        type: "string",
        choices: NETWORK_NAMES,
        description: "Known network",
        string: true,
    },
    finalized: {
        type: "boolean",
        default: false,
        description: "listen to finalized only",
    },
};

export const getApiFor = async (argv: Argv) => {
    const wsProvider = getWsProviderFor(argv);
    return await ApiPromise.create({
        noInitWarn: true,
        provider: wsProvider,
    });
};

export function isKnownNetwork(name: string): name is NETWORK_NAME {
    return NETWORK_NAMES.includes(name as NETWORK_NAME);
}

export const getWsProviderForNetwork = (name: NETWORK_NAME) => {
    return new WsProvider(NETWORK_WS_URLS[name]);
};

// Supports providing an URL or a known network
export const getWsProviderFor = (argv: Argv) => {
    if (isKnownNetwork(argv.network)) {
        return getWsProviderForNetwork(argv.network);
    }
    return new WsProvider(argv.url);
};
