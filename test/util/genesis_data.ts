/// Utilities to convert from ChainSpec to ContainerChainGenesisData and back

import { ApiPromise } from "@polkadot/api";
import { hexToString, stringToHex } from "@polkadot/util";

export function chainSpecToContainerChainGenesisData(paraApi: ApiPromise, chainSpec: any): any {
    const storage = chainSpecStorageToOnChainStorage(chainSpec.genesis);
    const extensions = "0x";
    const properties = chainSpecPropertiesToOnChainProperties(chainSpec.properties);
    const g = paraApi.createType("TpContainerChainGenesisDataContainerChainGenesisData", {
        storage: storage,
        name: stringToHex(chainSpec.name),
        id: stringToHex(chainSpec.id),
        forkId: chainSpec.forkId ? stringToHex(chainSpec.forkId) : null,
        extensions: extensions,
        properties: properties,
    });
    return g;
}

export function containerChainGenesisDataToChainSpec(
    containerChainGenesisData: any,
    para_id: any,
    chainType: any,
    relay_chain: any
): any {
    const g = {
        name: hexToString(containerChainGenesisData.name.toHex()),
        id: hexToString(containerChainGenesisData.id.toHex()),
        forkId: containerChainGenesisData.forkId.isSome ? hexToString(containerChainGenesisData.forkId.toHex()) : null,
        chainType: chainType,
        bootNodes: [],
        telemetryEndpoints: null,
        protocolId: `container-chain-${para_id}`,
        properties: onChainPropertiesToChainSpecProperties(containerChainGenesisData.properties),
        relay_chain: relay_chain,
        para_id: para_id,
        codeSubstitutes: {},
        genesis: onChainStorageToChainSpecStorage(containerChainGenesisData.storage),
    };
    return g;
}

export function chainSpecStorageToOnChainStorage(genesis: any): any {
    return Object.entries(genesis.raw.top).map((keyValue) => {
        const [key, value] = keyValue;

        return {
            key: key,
            value: value,
        };
    });
}

export function onChainStorageToChainSpecStorage(storage: any): any {
    const top = {};

    storage.forEach((x) => {
        top[x.key.toHex()] = x.value.toHex();
    });

    const s = {
        raw: {
            top: top,
            childrenDefault: {},
        },
    };
    return s;
}

export function chainSpecPropertiesToOnChainProperties(properties: any): any {
    return {
        tokenMetadata: {
            tokenSymbol: stringToHex(properties.tokenSymbol),
            ss58Format: properties.ss58Format,
            tokenDecimals: properties.tokenDecimals,
        },
        isEthereum: properties.isEthereum || false,
    };
}

export function onChainPropertiesToChainSpecProperties(properties: any): any {
    return {
        tokenSymbol: hexToString(properties.tokenMetadata.tokenSymbol.toHex()),
        ss58Format: properties.tokenMetadata.ss58Format.toNumber(),
        tokenDecimals: properties.tokenMetadata.tokenDecimals.toNumber(),
        isEthereum: properties.isEthereum == true ? true : false,
    };
}
