import "@tanssi/api-augment";
/// Utilities to convert from ChainSpec to ContainerChainGenesisData and back
import type { ApiPromise } from "@polkadot/api";
import { hexToString, stringToHex } from "@polkadot/util";
import type { DpContainerChainGenesisDataContainerChainGenesisData } from "@polkadot/types/lookup";

export function chainSpecToContainerChainGenesisData(paraApi: ApiPromise, chainSpec: any): any {
    const storage = chainSpecStorageToOnChainStorage(chainSpec.genesis);
    const extensions = "0x";
    const properties = chainSpecPropertiesToOnChainProperties(chainSpec.properties);
    const g = paraApi.createType("DpContainerChainGenesisDataContainerChainGenesisData", {
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

    for (const x of storage) {
        top[x.key.toHex()] = x.value.toHex();
    }

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
        isEthereum: properties.isEthereum === true,
    };
}

export function generateEmptyGenesisData(api: ApiPromise, isCode = false) {
    const storage = isCode
        ? [
              {
                  // ":code" key
                  key: "0x3a636f6465",
                  // code value (must be at least 9 bytes length)
                  value: "0x0102030405060708091011",
              },
          ]
        : [
              {
                  key: "0x636f6465",
                  value: "0x010203040506",
              },
          ];

    const genesisData = api.createType<DpContainerChainGenesisDataContainerChainGenesisData>(
        "DpContainerChainGenesisDataContainerChainGenesisData",
        {
            storage,
            name: "0x436f6e7461696e657220436861696e2032303030",
            id: "0x636f6e7461696e65722d636861696e2d32303030",
            forkId: null,
            extensions: "0x",
            properties: {
                tokenMetadata: {
                    tokenSymbol: "0x61626364",
                    ss58Format: 42,
                    tokenDecimals: 12,
                },
                isEthereum: false,
            },
        }
    );

    return genesisData;
}
