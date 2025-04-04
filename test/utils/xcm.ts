import { type DevModeContext, customDevRpcRequest, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { XcmpMessageFormat } from "@polkadot/types/interfaces";
import type {
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
    XcmV3JunctionNetworkId,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";
import { type BN, hexToU8a, stringToU8a, u8aToHex } from "@polkadot/util";
import { xxhashAsU8a } from "@polkadot/util-crypto";

// Creates and returns the tx that overrides the paraHRMP existence
// This needs to be inserted at every block in which you are willing to test
// state changes
// The reason is that set_validation_data inherent overrides it
export function mockHrmpChannelExistanceTx(
    context: DevModeContext,
    para: number,
    maxCapacity: number,
    maxTotalSize: number,
    maxMessageSize: number
) {
    // This constructs the relevant state to be inserted
    const relevantMessageState = {
        dmqMqcHead: "0x0000000000000000000000000000000000000000000000000000000000000000",
        relayDispatchQueueSize: [0, 0],
        egressChannels: [
            [
                para,
                {
                    maxCapacity,
                    maxTotalSize,
                    maxMessageSize,
                    msgCount: 0,
                    totalSize: 0,
                    mqcHead: null,
                },
            ],
        ],
        ingressChannels: [
            [
                para,
                {
                    maxCapacity,
                    maxTotalSize,
                    maxMessageSize,
                    msgCount: 0,
                    totalSize: 0,
                    mqcHead: null,
                },
            ],
        ],
    };

    const stateToInsert: CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot = context
        .polkadotJs()
        .createType(
            "CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot",
            relevantMessageState
        ) as any;

    // Get keys to modify state
    const module = xxhashAsU8a(new TextEncoder().encode("ParachainSystem"), 128);
    const account_key = xxhashAsU8a(new TextEncoder().encode("RelevantMessagingState"), 128);

    const overallKey = new Uint8Array([...module, ...account_key]);

    return context.polkadotJs().tx.system.setStorage([[u8aToHex(overallKey), u8aToHex(stateToInsert.toU8a())]]);
}

export function descendSiblingOriginFromAddress20(
    context: DevModeContext,
    address: `0x${string}` = "0x0101010101010101010101010101010101010101",
    paraId = 1
) {
    const toHash = new Uint8Array([
        ...new TextEncoder().encode("SiblingChain"),
        ...context.polkadotJs().createType("Compact<u32>", paraId).toU8a(),
        ...context
            .polkadotJs()
            .createType("Compact<u32>", "AccountKey20".length + 20)
            .toU8a(),
        ...new TextEncoder().encode("AccountKey20"),
        ...context.polkadotJs().createType("AccountId", address).toU8a(),
    ]);

    return {
        originAddress: address,
        descendOriginAddress: u8aToHex(context.polkadotJs().registry.hash(toHash).slice(0, 20)),
    };
}

export function descendSiblingOriginFromAddress32(
    context: DevModeContext,
    address: `0x${string}` = "0x0101010101010101010101010101010101010101010101010101010101010101",
    paraId = 1
) {
    const toHash = new Uint8Array([
        ...new TextEncoder().encode("SiblingChain"),
        ...context.polkadotJs().createType("Compact<u32>", paraId).toU8a(),
        ...context
            .polkadotJs()
            .createType("Compact<u32>", "AccountId32".length + 32)
            .toU8a(),
        ...new TextEncoder().encode("AccountId32"),
        ...context.polkadotJs().createType("AccountId32", address).toU8a(),
    ]);

    return {
        originAddress: address,
        descendOriginAddress: u8aToHex(context.polkadotJs().registry.hash(toHash).slice(0, 32)),
    };
}

export function descendParentOriginFromAddress32(
    context: DevModeContext,
    address: `0x${string}` = "0x0101010101010101010101010101010101010101010101010101010101010101"
) {
    const toHash = new Uint8Array([
        ...new TextEncoder().encode("ParentChain"),
        ...context
            .polkadotJs()
            .createType("Compact<u32>", "AccountId32".length + 32)
            .toU8a(),
        ...new TextEncoder().encode("AccountId32"),
        ...context.polkadotJs().createType("AccountId32", address).toU8a(),
    ]);

    return {
        originAddress: address,
        descendOriginAddress: u8aToHex(context.polkadotJs().registry.hash(toHash).slice(0, 32)),
    };
}

export function descendParentOriginForAddress20(
    context: DevModeContext,
    address: `0x${string}` = "0x0101010101010101010101010101010101010101010101010101010101010101"
) {
    const toHash = new Uint8Array([
        ...new TextEncoder().encode("ParentChain"),
        ...context
            .polkadotJs()
            .createType("Compact<u32>", "AccountId32".length + 32)
            .toU8a(),
        ...new TextEncoder().encode("AccountId32"),
        ...context.polkadotJs().createType("AccountId32", address).toU8a(),
    ]);

    return {
        originAddress: address,
        descendOriginAddress: u8aToHex(context.polkadotJs().registry.hash(toHash).slice(0, 20)),
    };
}

export function sovereignAccountOfSiblingForAddress32(context: DevModeContext, paraId: number): string {
    return u8aToHex(
        new Uint8Array([
            ...new TextEncoder().encode("sibl"),
            ...context.polkadotJs().createType("u32", paraId).toU8a(),
            ...new Uint8Array(24),
        ])
    );
}

export function sovereignAccountOfSiblingForAddress20(context: DevModeContext, paraId: number): string {
    return u8aToHex(
        new Uint8Array([
            ...new TextEncoder().encode("sibl"),
            ...context.polkadotJs().createType("u32", paraId).toU8a(),
            ...new Uint8Array(12),
        ])
    );
}

export interface RawXcmMessage {
    type: string;
    payload: any;
    format?: string;
}

export function buildXcmpMessage(context: DevModeContext, message: RawXcmMessage): number[] {
    const format = message.format != null ? message.format : "ConcatenatedVersionedXcm";
    const xcmpFormat: XcmpMessageFormat = context.polkadotJs().createType("XcmpMessageFormat", format) as any;
    const receivedMessage: XcmVersionedXcm = context.polkadotJs().createType(message.type, message.payload) as any;

    return [...xcmpFormat.toU8a(), ...receivedMessage.toU8a()];
}

export function buildDmpMessage(context: DevModeContext, message: RawXcmMessage): number[] {
    const receivedMessage: XcmVersionedXcm = context.polkadotJs().createType("XcmVersionedXcm", message.payload) as any;

    return [...receivedMessage.toU8a()];
}

export function buildUmpMessage(context: DevModeContext, message: RawXcmMessage): number[] {
    const receivedMessage: XcmVersionedXcm = context.polkadotJs().createType("XcmVersionedXcm", message.payload) as any;

    return [...receivedMessage.toU8a()];
}

export async function injectHrmpMessage(context: DevModeContext, paraId: number, message?: RawXcmMessage) {
    const totalMessage = message != null ? buildXcmpMessage(context, message) : [];
    // Send RPC call to inject XCM message
    await customDevRpcRequest("xcm_injectHrmpMessage", [paraId, totalMessage]);
}

export async function injectDmpMessage(context: DevModeContext, message?: RawXcmMessage) {
    const totalMessage = message != null ? buildDmpMessage(context, message) : [];
    // Send RPC call to inject XCM message
    await customDevRpcRequest("xcm_injectDownwardMessage", [totalMessage]);
}

export async function injectUmpMessage(context: DevModeContext, message?: RawXcmMessage) {
    const totalMessage = message != null ? buildUmpMessage(context, message) : [];
    // Send RPC call to inject XCM message
    await customDevRpcRequest("xcm_injectUpwardMessage", [totalMessage]);
}

// Weight a particular message using the xcm utils precompile
export async function weightMessage(context: DevModeContext, message: XcmVersionedXcm) {
    return (await context.readPrecompile?.({
        precompileName: "XcmUtils",
        functionName: "weightMessage",
        args: [message.toHex()],
    })) as bigint;
}

export async function injectHrmpMessageAndSeal(context: DevModeContext, paraId: number, message?: RawXcmMessage) {
    await injectHrmpMessage(context, paraId, message);
    // Create a block in which the XCM will be executed
    await context.createBlock();
}

export async function injectDmpMessageAndSeal(context: DevModeContext, message?: RawXcmMessage) {
    await injectDmpMessage(context, message);
    // Create a block in which the XCM will be executed
    await context.createBlock();
}

export async function injectUmpMessageAndSeal(context: DevModeContext, message?: RawXcmMessage) {
    await injectUmpMessage(context, message);
    // Create a block in which the XCM will be executed
    await context.createBlock();
}

interface Junction {
    Parachain?: number;
    AccountId32?: {
        network: "Any" | XcmV3JunctionNetworkId["type"];
        id: Uint8Array | string;
    };
    AccountIndex64?: {
        network: "Any" | XcmV3JunctionNetworkId["type"];
        index: number;
    };
    AccountKey20?: {
        network: "Any" | XcmV3JunctionNetworkId["type"];
        key: Uint8Array | string;
    };
    PalletInstance?: number;
    GeneralIndex?: bigint;
    GeneralKey?: { length: number; data: Uint8Array };
    OnlyChild?: null;
    Plurality?: { id: any; part: any };
    GlobalConsensus?: "Any" | XcmV3JunctionNetworkId["type"];
}

interface Junctions {
    Here?: null;
    X1?: Junction;
    X2?: [Junction, Junction];
    X3?: [Junction, Junction, Junction];
    X4?: [Junction, Junction, Junction, Junction];
    X5?: [Junction, Junction, Junction, Junction, Junction];
    X6?: [Junction, Junction, Junction, Junction, Junction, Junction];
    X7?: [Junction, Junction, Junction, Junction, Junction, Junction, Junction];
    X8?: [Junction, Junction, Junction, Junction, Junction, Junction, Junction, Junction];
}

export interface MultiLocation {
    parents: number;
    interior: Junctions;
}

export interface XcmFragmentConfig {
    assets: {
        multilocation: MultiLocation;
        fungible: bigint;
    }[];
    weight_limit?: BN;
    descend_origin?: string;
    beneficiary?: string;
}

export class XcmFragment {
    config: XcmFragmentConfig;
    instructions: any[];

    constructor(config: XcmFragmentConfig) {
        this.config = config;
        this.instructions = [];
    }

    // Add a `ReserveAssetDeposited` instruction
    reserve_asset_deposited(): this {
        this.instructions.push({
            ReserveAssetDeposited: this.config.assets.map(({ multilocation, fungible }) => {
                return {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: { Fungible: fungible },
                };
            }, this),
        });
        return this;
    }

    // Add a `ReceiveTeleportedAsset` instruction
    teleported_assets_received(): this {
        this.instructions.push({
            ReceiveTeleportedAsset: this.config.assets.map(({ multilocation, fungible }) => {
                return {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: { Fungible: fungible },
                };
            }, this),
        });
        return this;
    }

    // Add a `WithdrawAsset` instruction
    withdraw_asset(): this {
        this.instructions.push({
            WithdrawAsset: this.config.assets.map(({ multilocation, fungible }) => {
                return {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: { Fungible: fungible },
                };
            }, this),
        });
        return this;
    }

    // Add one or more `BuyExecution` instruction
    // if weight_limit is not set in config, then we put unlimited
    buy_execution(fee_index = 0, repeat = 1n): this {
        const weightLimit =
            this.config.weight_limit != null ? { Limited: this.config.weight_limit } : { Unlimited: null };
        for (let i = 0; i < repeat; i++) {
            this.instructions.push({
                BuyExecution: {
                    fees: {
                        id: {
                            Concrete: this.config.assets[fee_index].multilocation,
                        },
                        fun: { Fungible: this.config.assets[fee_index].fungible },
                    },
                    weightLimit: weightLimit,
                },
            });
        }
        return this;
    }

    // Add one or more `BuyExecution` instruction
    // if weight_limit is not set in config, then we put unlimited
    refund_surplus(repeat = 1n): this {
        for (let i = 0; i < repeat; i++) {
            this.instructions.push({
                RefundSurplus: null,
            });
        }
        return this;
    }

    // Add a `ClaimAsset` instruction
    claim_asset(index = 0): this {
        this.instructions.push({
            ClaimAsset: {
                assets: [
                    {
                        id: {
                            Concrete: this.config.assets[index].multilocation,
                        },
                        fun: { Fungible: this.config.assets[index].fungible },
                    },
                ],
                // Ticket seems to indicate the version of the assets
                ticket: {
                    parents: 0,
                    interior: { X1: { GeneralIndex: 3 } },
                },
            },
        });
        return this;
    }

    // Add a `ClearOrigin` instruction
    clear_origin(repeat = 1n): this {
        for (let i = 0; i < repeat; i++) {
            this.instructions.push({ ClearOrigin: null as any });
        }
        return this;
    }

    // Add a `DescendOrigin` instruction
    descend_origin(): this {
        if (this.config.descend_origin != null) {
            if (hexToU8a(this.config.descend_origin).length === 32) {
                this.instructions.push({
                    DescendOrigin: {
                        X1: {
                            AccountId32: {
                                network: "Any",
                                id: this.config.descend_origin,
                            },
                        },
                    },
                });
            } else {
                this.instructions.push({
                    DescendOrigin: {
                        X1: {
                            AccountKey20: {
                                network: "Any",
                                key: this.config.descend_origin,
                            },
                        },
                    },
                });
            }
        } else {
            console.warn("!Building a DescendOrigin instruction without a configured descend_origin");
        }
        return this;
    }

    // Add a `DepositAsset` instruction
    deposit_asset(max_assets = 1n, network: "Any" | XcmV3JunctionNetworkId["type"] = "Any"): this {
        if (this.config.beneficiary === null) {
            console.warn("!Building a DepositAsset instruction without a configured beneficiary");
        } else {
            if (hexToU8a(this.config.beneficiary).length === 20) {
                this.instructions.push({
                    DepositAsset: {
                        assets: { Wild: "All" },
                        maxAssets: max_assets,
                        beneficiary: {
                            parents: 0,
                            interior: {
                                X1: { AccountKey20: { network, key: this.config.beneficiary } },
                            },
                        },
                    },
                });
            } else {
                this.instructions.push({
                    DepositAsset: {
                        assets: { Wild: "All" },
                        maxAssets: max_assets,
                        beneficiary: {
                            parents: 0,
                            interior: {
                                X1: { AccountId32: { network, id: this.config.beneficiary } },
                            },
                        },
                    },
                });
            }
        }
        return this;
    }

    // Add a `DepositAsset` instruction for xcm v3
    deposit_asset_v3(max_assets = 1n, network: XcmV3JunctionNetworkId["type"] | null = null): this {
        if (this.config.beneficiary === null) {
            console.warn("!Building a DepositAsset instruction without a configured beneficiary");
        } else {
            if (hexToU8a(this.config.beneficiary).length === 20) {
                this.instructions.push({
                    DepositAsset: {
                        assets: { Wild: { AllCounted: max_assets } },
                        beneficiary: {
                            parents: 0,
                            interior: {
                                X1: { AccountKey20: { network, key: this.config.beneficiary } },
                            },
                        },
                    },
                });
            } else {
                this.instructions.push({
                    DepositAsset: {
                        assets: { Wild: { AllCounted: max_assets } },
                        beneficiary: {
                            parents: 0,
                            interior: {
                                X1: { AccountId32: { network, id: this.config.beneficiary } },
                            },
                        },
                    },
                });
            }
        }
        return this;
    }

    // Add a `SetErrorHandler` instruction, appending all the nested instructions
    set_error_handler_with(callbacks: (() => any)[]): this {
        const error_instructions: any[] = [];
        for (const cb of callbacks) {
            cb.call(this);
            // As each method in the class pushes to the instruction stack, we pop
            error_instructions.push(this.instructions.pop());
        }
        this.instructions.push({
            SetErrorHandler: error_instructions,
        });
        return this;
    }

    // Add a `SetAppendix` instruction, appending all the nested instructions
    set_appendix_with(callbacks: (() => any)[]): this {
        const appendix_instructions: any[] = [];
        for (const cb of callbacks) {
            cb.call(this);
            // As each method in the class pushes to the instruction stack, we pop
            appendix_instructions.push(this.instructions.pop());
        }
        this.instructions.push({
            SetAppendix: appendix_instructions,
        });
        return this;
    }

    // Add a `Trap` instruction
    trap(): this {
        this.instructions.push({
            Trap: 0,
        });
        return this;
    }

    // Utility function to support functional style method call chaining bound to `this` context
    with(callback): this {
        return callback.call(this);
    }

    // Pushes the given instruction
    push_any(instruction: any): this {
        this.instructions.push(instruction);
        return this;
    }

    // Returns a V2 fragment payload
    as_v2(): any {
        return {
            V2: this.instructions,
        };
    }

    /// XCM V3 calls
    as_v3(): any {
        return {
            V3: replaceNetworkAny(this.instructions),
        };
    }

    // Add a `BurnAsset` instruction
    burn_asset(amount = 0n): this {
        this.instructions.push({
            BurnAsset: this.config.assets.map(({ multilocation, fungible }) => {
                return {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: { Fungible: amount === 0n ? fungible : amount },
                };
            }, this),
        });
        return this;
    }

    // Add a `ReportHolding` instruction
    report_holding(
        destination: MultiLocation = {
            parents: 1,
            interior: { X1: { Parachain: 1000 } },
        },
        query_id: number = Math.floor(Math.random() * 1000),
        max_weight: { refTime: bigint; proofSize: bigint } = {
            refTime: 1_000_000_000n,
            proofSize: 1_000_000_000n,
        }
    ): this {
        this.instructions.push({
            ReportHolding: {
                response_info: {
                    destination,
                    query_id,
                    max_weight,
                },
                assets: { Wild: "All" },
            },
        });
        return this;
    }

    // Add a `ExpectAsset` instruction
    expect_asset(): this {
        this.instructions.push({
            ExpectAsset: this.config.assets.map(({ multilocation, fungible }) => {
                return {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: { Fungible: fungible },
                };
            }, this),
        });
        return this;
    }

    // Add a `ExpectOrigin` instruction
    expect_origin(
        multilocation: MultiLocation = {
            parents: 1,
            interior: { X1: { Parachain: 1000 } },
        }
    ): this {
        this.instructions.push({
            ExpectOrigin: multilocation,
        });
        return this;
    }

    // Add a `ExpectError` instruction
    expect_error(index = 0, error = "Unimplemented"): this {
        this.instructions.push({
            ExpectError: [index, error],
        });
        return this;
    }

    // Add a `ExpectTransactStatus` instruction
    expect_transact_status(status = "Success"): this {
        this.instructions.push({
            ExpectTransactStatus: status,
        });
        return this;
    }

    // Add a `QueryPallet` instruction
    query_pallet(
        destination: MultiLocation = {
            parents: 1,
            interior: { X1: { Parachain: 1000 } },
        },
        query_id: number = Math.floor(Math.random() * 1000),
        module_name = "pallet_balances",
        max_weight: { refTime: bigint; proofSize: bigint } = {
            refTime: 1_000_000_000n,
            proofSize: 1_000_000_000n,
        }
    ): this {
        this.instructions.push({
            QueryPallet: {
                module_name: stringToU8a(module_name),
                response_info: {
                    destination,
                    query_id,
                    max_weight,
                },
            },
        });
        return this;
    }

    // Add a `ExpectPallet` instruction
    expect_pallet(
        index = 0,
        name = "Balances",
        module_name = "pallet_balances",
        crate_major = 4,
        min_crate_minor = 0
    ): this {
        this.instructions.push({
            ExpectPallet: {
                index,
                name: stringToU8a(name),
                module_name: stringToU8a(module_name),
                crate_major,
                min_crate_minor,
            },
        });
        return this;
    }

    // Add a `ReportTransactStatus` instruction
    report_transact_status(
        destination: MultiLocation = {
            parents: 1,
            interior: { X1: { Parachain: 1000 } },
        },
        query_id: number = Math.floor(Math.random() * 1000),
        max_weight: { refTime: bigint; proofSize: bigint } = {
            refTime: 1_000_000_000n,
            proofSize: 1_000_000_000n,
        }
    ): this {
        this.instructions.push({
            ReportTransactStatus: {
                destination,
                query_id,
                max_weight,
            },
        });
        return this;
    }

    // Add a `ClearTransactStatus` instruction
    clear_transact_status(): this {
        this.instructions.push({
            ClearTransactStatus: null as any,
        });
        return this;
    }

    // Add a `UniversalOrigin` instruction
    universal_origin(junction: Junction): this {
        this.instructions.push({
            UniversalOrigin: junction,
        });
        return this;
    }

    // Add a `ExportMessage` instruction
    export_message(
        xcm_hex = "",
        network: "Any" | XcmV3JunctionNetworkId["type"] = "Ethereum",
        destination: Junctions = { X1: { Parachain: 1000 } }
    ): this {
        const callVec = stringToU8a(xcm_hex);
        const xcm = Array.from(callVec);
        this.instructions.push({
            ExportMessage: {
                network,
                destination,
                xcm,
            },
        });
        return this;
    }

    // Add a `LockAsset` instruction
    lock_asset(
        multilocation: MultiLocation = this.config.assets[0].multilocation,
        fungible: bigint = this.config.assets[0].fungible,
        unlocker: MultiLocation = this.config.assets[0].multilocation
    ): this {
        this.instructions.push({
            LockAsset: {
                asset: {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: {
                        Fungible: fungible,
                    },
                },
                unlocker,
            },
        });
        return this;
    }

    // Add a `UnlockAsset` instruction
    unlock_asset(
        multilocation: MultiLocation = this.config.assets[0].multilocation,
        fungible: bigint = this.config.assets[0].fungible,
        target: MultiLocation = this.config.assets[0].multilocation
    ): this {
        this.instructions.push({
            UnlockAsset: {
                asset: {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: {
                        Fungible: fungible,
                    },
                },
                target,
            },
        });
        return this;
    }

    // Add a `NoteUnlockable` instruction
    note_unlockable(
        multilocation: MultiLocation = this.config.assets[0].multilocation,
        fungible: bigint = this.config.assets[0].fungible,
        owner: MultiLocation = this.config.assets[0].multilocation
    ): this {
        this.instructions.push({
            NoteUnlockable: {
                asset: {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: {
                        Fungible: fungible,
                    },
                },
                owner,
            },
        });
        return this;
    }

    // Add a `RequestUnlock` instruction
    request_unlock(
        multilocation: MultiLocation = this.config.assets[0].multilocation,
        fungible: bigint = this.config.assets[0].fungible,
        locker: MultiLocation = this.config.assets[0].multilocation
    ): this {
        this.instructions.push({
            RequestUnlock: {
                asset: {
                    id: {
                        Concrete: multilocation,
                    },
                    fun: {
                        Fungible: fungible,
                    },
                },
                locker,
            },
        });
        return this;
    }

    // Add a `SetFeesMode` instruction
    set_fees_mode(jit_withdraw = true): this {
        this.instructions.push({
            SetFeesMode: { jit_withdraw },
        });
        return this;
    }

    // Add a `SetTopic` instruction
    set_topic(topic = "0xk89103a9CF04c71Dbc94D0b566f7A2"): this {
        this.instructions.push({
            SetTopic: Array.from(stringToU8a(topic)),
        });
        return this;
    }

    // Add a `ClearTopic` instruction
    clear_topic(): this {
        this.instructions.push({
            ClearTopic: null as any,
        });
        return this;
    }

    // Add a `AliasOrigin` instruction
    alias_origin(
        destination: MultiLocation = {
            parents: 1,
            interior: { X1: { Parachain: 1000 } },
        }
    ): this {
        this.instructions.push({
            AliasOrigin: destination,
        });
        return this;
    }

    // Add a `UnpaidExecution` instruction
    unpaid_execution(
        destination: MultiLocation = {
            parents: 1,
            interior: { X1: { Parachain: 1000 } },
        }
    ): this {
        const weight_limit =
            this.config.weight_limit != null ? { Limited: this.config.weight_limit } : { Unlimited: null };
        this.instructions.push({
            UnpaidExecution: {
                weight_limit,
                check_origin: destination,
            },
        });
        return this;
    }

    // Overrides the weight limit of the first buyExeuction encountered
    // with the measured weight
    async override_weight(context: DevModeContext): Promise<this> {
        const message: XcmVersionedXcm = context.polkadotJs().createType("XcmVersionedXcm", this.as_v2()) as any;

        const instructions = message.asV2;
        for (let i = 0; i < instructions.length; i++) {
            if (instructions[i].isBuyExecution === true) {
                const newWeight = await weightMessage(context, message);
                this.instructions[i] = {
                    BuyExecution: {
                        fees: instructions[i].asBuyExecution.fees,
                        weightLimit: { Limited: newWeight },
                    },
                };
                break;
            }
        }
        return this;
    }
}

function replaceNetworkAny(obj: AnyObject | Array<AnyObject>): any {
    if (Array.isArray(obj)) {
        return obj.map((item) => replaceNetworkAny(item));
    }
    if (typeof obj === "object" && obj !== null) {
        const newObj: AnyObject = {};
        for (const key in obj) {
            if (key === "network" && obj[key] === "Any") {
                newObj[key] = null;
            } else {
                newObj[key] = replaceNetworkAny(obj[key]);
            }
        }
        return newObj;
    }
    return obj;
}

type AnyObject = {
    [key: string]: any;
};

export const extractPaidDeliveryFees = async (context: DevModeContext) => {
    const records = await context.polkadotJs().query.system.events();

    const filteredEvents = records
        .map(({ event }) => (context.polkadotJs().events.polkadotXcm.FeesPaid.is(event) ? event : undefined))
        .filter((event) => event);

    return filteredEvents[0]?.data[1][0].fun.asFungible.toBigInt();
};

export const extractPaidDeliveryFeesDancelight = async (context: DevModeContext) => {
    const records = await context.polkadotJs().query.system.events();

    const filteredEvents = records
        .map(({ event }) => (context.polkadotJs().events.xcmPallet.FeesPaid.is(event) ? event : undefined))
        .filter((event) => event);

    return filteredEvents[0]?.data[1][0].fun.asFungible.toBigInt();
};

export const getLastSentUmpMessageFee = async (context: DevModeContext, baseDelivery: bigint, txByteFee: bigint) => {
    const upwardMessages = await context.polkadotJs().query.parachainSystem.upwardMessages();
    expect(upwardMessages.length > 0, "There is no upward message").to.be.true;
    const sentXcm = upwardMessages[0];

    // We need to slice once to get to the actual message (version)
    const messageBytes = sentXcm.slice(1);

    const txPrice = baseDelivery + txByteFee * BigInt(messageBytes.length);
    const deliveryFeeFactor = await context.polkadotJs().query.parachainSystem.upwardDeliveryFeeFactor();
    const fee = (BigInt(deliveryFeeFactor.toString()) * txPrice) / BigInt(10 ** 18);
    return fee;
};

export const getLastSentDmpMessageFee = async (
    context: DevModeContext,
    baseDelivery: bigint,
    txByteFee: bigint,
    paraId: number
) => {
    const downwardMessages = await context.polkadotJs().query.dmp.downwardMessageQueues(paraId);
    expect(downwardMessages.length > 0, "There is no downward message").to.be.true;
    const sentXcm = downwardMessages[0].msg;

    // We need to slice once to get to the actual message (version)
    const messageBytes = sentXcm.slice(1);

    const txPrice = baseDelivery + txByteFee * BigInt(messageBytes.length);
    const deliveryFeeFactor = await context.polkadotJs().query.dmp.deliveryFeeFactor(paraId);
    const fee = (BigInt(deliveryFeeFactor.toString()) * txPrice) / BigInt(10 ** 18);
    return fee;
};

export const getLastSentHrmpMessageFee = async (
    context: DevModeContext,
    paraId: number,
    baseDelivery: bigint,
    txByteFee: bigint
) => {
    const sentXcm = await context.polkadotJs().query.xcmpQueue.outboundXcmpMessages(paraId, 0);
    expect(sentXcm.length > 0, `There is no hrmp message for para id ${paraId}`).to.be.true;
    // We need to slice 2 first bytes to get to the actual message (version plus HRMP)
    const messageBytes = sentXcm.slice(2);

    const txPrice = baseDelivery + txByteFee * BigInt(messageBytes.length);
    const deliveryFeeFactor = await context.polkadotJs().query.xcmpQueue.deliveryFeeFactor(paraId);
    const fee = (BigInt(deliveryFeeFactor.toString()) * txPrice) / BigInt(10 ** 18);
    return fee;
};

export const getParathreadRelayTankAddress = async (
    relayApi: ApiPromise,
    tanssiParaId: number,
    containerParaId: number
) => {
    const targetParaId = relayApi.createType("ParaId", containerParaId);
    const containerAddress = u8aToHex(
        new Uint8Array([...new TextEncoder().encode("para"), ...targetParaId.toU8a()])
    ).padEnd(66, "0");

    // We are going to generate the address from the relay runtime apis
    const address = await relayApi.call.locationToAccountApi.convertLocation({
        V3: {
            parents: 0,
            interior: {
                X2: [
                    {
                        Parachain: tanssiParaId,
                    },
                    {
                        AccountId32: {
                            network: null,
                            id: containerAddress,
                        },
                    },
                ],
            },
        },
    });
    return address.asOk.toHuman();
};
