// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/calls";

import type { ApiTypes, AugmentedCall, DecoratedCallBase } from "@polkadot/api-base/types";
import type { Bytes, Null, Option, Result, U64, Vec, u128, u32 } from "@polkadot/types-codec";
import type { AnyNumber, IMethod, ITuple } from "@polkadot/types-codec/types";
import type {
    BabeEquivocationProof,
    BabeGenesisConfiguration,
    Epoch,
    OpaqueKeyOwnershipProof,
} from "@polkadot/types/interfaces/babe";
import type { BeefyAuthoritySet, BeefyNextAuthoritySet } from "@polkadot/types/interfaces/beefy";
import type { CheckInherentsResult, InherentData } from "@polkadot/types/interfaces/blockbuilder";
import type { BlockHash } from "@polkadot/types/interfaces/chain";
import type { AuthorityId } from "@polkadot/types/interfaces/consensus";
import type { CallDryRunEffects, XcmDryRunApiError, XcmDryRunEffects } from "@polkadot/types/interfaces/dryRunApi";
import type { Extrinsic } from "@polkadot/types/interfaces/extrinsics";
import type { GenesisBuildErr } from "@polkadot/types/interfaces/genesisBuilder";
import type { AuthorityList, GrandpaEquivocationProof, SetId } from "@polkadot/types/interfaces/grandpa";
import type { OpaqueMetadata } from "@polkadot/types/interfaces/metadata";
import type { MmrBatchProof, MmrEncodableOpaqueLeaf, MmrError } from "@polkadot/types/interfaces/mmr";
import type { FeeDetails, RuntimeDispatchInfo } from "@polkadot/types/interfaces/payment";
import type {
    AccountId,
    Balance,
    Block,
    BlockNumber,
    Call,
    ExtrinsicInclusionMode,
    Hash,
    Header,
    Index,
    KeyTypeId,
    OriginCaller,
    RuntimeCall,
    Slot,
    Weight,
    WeightV2,
} from "@polkadot/types/interfaces/runtime";
import type { RuntimeVersion } from "@polkadot/types/interfaces/state";
import type { ApplyExtrinsicResult } from "@polkadot/types/interfaces/system";
import type { TransactionSource, TransactionValidity } from "@polkadot/types/interfaces/txqueue";
import type { VersionedMultiLocation, VersionedXcm } from "@polkadot/types/interfaces/xcm";
import type { XcmPaymentApiError } from "@polkadot/types/interfaces/xcmPaymentApi";
import type { Error } from "@polkadot/types/interfaces/xcmRuntimeApi";
import type { XcmVersionedAssetId, XcmVersionedLocation, XcmVersionedXcm } from "@polkadot/types/lookup";
import type { IExtrinsic, Observable } from "@polkadot/types/types";

export type __AugmentedCall<ApiType extends ApiTypes> = AugmentedCall<ApiType>;
export type __DecoratedCallBase<ApiType extends ApiTypes> = DecoratedCallBase<ApiType>;

declare module "@polkadot/api-base/types/calls" {
    interface AugmentedCalls<ApiType extends ApiTypes> {
        /** 0xbc9d89904f5b923f/1 */
        accountNonceApi: {
            /**
             * The API to query account nonce (aka transaction index)
             **/
            accountNonce: AugmentedCall<ApiType, (accountId: AccountId | string | Uint8Array) => Observable<Index>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x687ad44ad37f03c2/1 */
        authorityDiscoveryApi: {
            /**
             * Retrieve authority identifiers of the current and next authority set.
             **/
            authorities: AugmentedCall<ApiType, () => Observable<Vec<AuthorityId>>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xcbca25e39f142387/2 */
        babeApi: {
            /**
             * Return the genesis configuration for BABE. The configuration is only read on genesis.
             **/
            configuration: AugmentedCall<ApiType, () => Observable<BabeGenesisConfiguration>>;
            /**
             * Returns information regarding the current epoch.
             **/
            currentEpoch: AugmentedCall<ApiType, () => Observable<Epoch>>;
            /**
             * Returns the slot that started the current epoch.
             **/
            currentEpochStart: AugmentedCall<ApiType, () => Observable<Slot>>;
            /**
             * Generates a proof of key ownership for the given authority in the current epoch.
             **/
            generateKeyOwnershipProof: AugmentedCall<
                ApiType,
                (
                    slot: Slot | AnyNumber | Uint8Array,
                    authorityId: AuthorityId | string | Uint8Array
                ) => Observable<Option<OpaqueKeyOwnershipProof>>
            >;
            /**
             * Returns information regarding the next epoch (which was already previously announced).
             **/
            nextEpoch: AugmentedCall<ApiType, () => Observable<Epoch>>;
            /**
             * Submits an unsigned extrinsic to report an equivocation.
             **/
            submitReportEquivocationUnsignedExtrinsic: AugmentedCall<
                ApiType,
                (
                    equivocationProof:
                        | BabeEquivocationProof
                        | { offender?: any; slotNumber?: any; firstHeader?: any; secondHeader?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof: OpaqueKeyOwnershipProof | string | Uint8Array
                ) => Observable<Option<Null>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x2a5e924655399e60/1 */
        beefyMmrApi: {
            /**
             * Return the currently active BEEFY authority set proof.
             **/
            authoritySetProof: AugmentedCall<ApiType, () => Observable<BeefyAuthoritySet>>;
            /**
             * Return the next/queued BEEFY authority set proof.
             **/
            nextAuthoritySetProof: AugmentedCall<ApiType, () => Observable<BeefyNextAuthoritySet>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x40fe3ad401f8959a/6 */
        blockBuilder: {
            /**
             * Apply the given extrinsic.
             **/
            applyExtrinsic: AugmentedCall<
                ApiType,
                (extrinsic: Extrinsic | IExtrinsic | string | Uint8Array) => Observable<ApplyExtrinsicResult>
            >;
            /**
             * Check that the inherents are valid.
             **/
            checkInherents: AugmentedCall<
                ApiType,
                (
                    block: Block | { header?: any; extrinsics?: any } | string | Uint8Array,
                    data: InherentData | { data?: any } | string | Uint8Array
                ) => Observable<CheckInherentsResult>
            >;
            /**
             * Finish the current block.
             **/
            finalizeBlock: AugmentedCall<ApiType, () => Observable<Header>>;
            /**
             * Generate inherent extrinsics.
             **/
            inherentExtrinsics: AugmentedCall<
                ApiType,
                (inherent: InherentData | { data?: any } | string | Uint8Array) => Observable<Vec<Extrinsic>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xdf6acb689907609b/5 */
        core: {
            /**
             * Execute the given block.
             **/
            executeBlock: AugmentedCall<
                ApiType,
                (block: Block | { header?: any; extrinsics?: any } | string | Uint8Array) => Observable<Null>
            >;
            /**
             * Initialize a block with the given header.
             **/
            initializeBlock: AugmentedCall<
                ApiType,
                (
                    header:
                        | Header
                        | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any }
                        | string
                        | Uint8Array
                ) => Observable<ExtrinsicInclusionMode>
            >;
            /**
             * Returns the version of the runtime.
             **/
            version: AugmentedCall<ApiType, () => Observable<RuntimeVersion>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x91b1c8b16328eb92/2 */
        dryRunApi: {
            /**
             * Dry run call
             **/
            dryRunCall: AugmentedCall<
                ApiType,
                (
                    origin: OriginCaller | { System: any } | string | Uint8Array,
                    call: RuntimeCall | IMethod | string | Uint8Array,
                    resultXcmsVersion: u32 | AnyNumber | Uint8Array
                ) => Observable<Result<CallDryRunEffects, XcmDryRunApiError>>
            >;
            /**
             * Dry run XCM program
             **/
            dryRunXcm: AugmentedCall<
                ApiType,
                (
                    originLocation:
                        | VersionedMultiLocation
                        | { V0: any }
                        | { V1: any }
                        | { V2: any }
                        | { V3: any }
                        | { V4: any }
                        | { v5: any }
                        | string
                        | Uint8Array,
                    xcm:
                        | VersionedXcm
                        | { V0: any }
                        | { V1: any }
                        | { V2: any }
                        | { V3: any }
                        | { V4: any }
                        | { V5: any }
                        | string
                        | Uint8Array
                ) => Observable<Result<XcmDryRunEffects, XcmDryRunApiError>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xfbc577b9d747efd6/1 */
        genesisBuilder: {
            /**
             * Build `RuntimeGenesisConfig` from a JSON blob not using any defaults and store it in the storage.
             **/
            buildConfig: AugmentedCall<
                ApiType,
                (json: Bytes | string | Uint8Array) => Observable<Result<ITuple<[]>, GenesisBuildErr>>
            >;
            /**
             * Creates the default `RuntimeGenesisConfig` and returns it as a JSON blob.
             **/
            createDefaultConfig: AugmentedCall<ApiType, () => Observable<Bytes>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xed99c5acb25eedf5/3 */
        grandpaApi: {
            /**
             * Get current GRANDPA authority set id.
             **/
            currentSetId: AugmentedCall<ApiType, () => Observable<SetId>>;
            /**
             * Generates a proof of key ownership for the given authority in the given set.
             **/
            generateKeyOwnershipProof: AugmentedCall<
                ApiType,
                (
                    setId: SetId | AnyNumber | Uint8Array,
                    authorityId: AuthorityId | string | Uint8Array
                ) => Observable<Option<OpaqueKeyOwnershipProof>>
            >;
            /**
             * Get the current GRANDPA authorities and weights. This should not change except for when changes are scheduled and the corresponding delay has passed.
             **/
            grandpaAuthorities: AugmentedCall<ApiType, () => Observable<AuthorityList>>;
            /**
             * Submits an unsigned extrinsic to report an equivocation.
             **/
            submitReportEquivocationUnsignedExtrinsic: AugmentedCall<
                ApiType,
                (
                    equivocationProof:
                        | GrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof: OpaqueKeyOwnershipProof | string | Uint8Array
                ) => Observable<Option<Null>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x9ffb505aa738d69c/1 */
        locationToAccountApi: {
            /**
             * Converts `Location` to `AccountId`
             **/
            convertLocation: AugmentedCall<
                ApiType,
                (
                    location: XcmVersionedLocation | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => Observable<Result<AccountId, Error>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x37e397fc7c91f5e4/2 */
        metadata: {
            /**
             * Returns the metadata of a runtime
             **/
            metadata: AugmentedCall<ApiType, () => Observable<OpaqueMetadata>>;
            /**
             * Returns the metadata at a given version.
             **/
            metadataAtVersion: AugmentedCall<
                ApiType,
                (version: u32 | AnyNumber | Uint8Array) => Observable<Option<OpaqueMetadata>>
            >;
            /**
             * Returns the supported metadata versions.
             **/
            metadataVersions: AugmentedCall<ApiType, () => Observable<Vec<u32>>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x91d5df18b0d2cf58/2 */
        mmrApi: {
            /**
             * Generate MMR proof for the given block numbers.
             **/
            generateProof: AugmentedCall<
                ApiType,
                (
                    blockNumbers: Vec<BlockNumber> | (BlockNumber | AnyNumber | Uint8Array)[],
                    bestKnownBlockNumber: Option<BlockNumber> | null | Uint8Array | BlockNumber | AnyNumber
                ) => Observable<Result<ITuple<[Vec<MmrEncodableOpaqueLeaf>, MmrBatchProof]>, MmrError>>
            >;
            /**
             * Return the number of MMR blocks in the chain.
             **/
            mmrLeafCount: AugmentedCall<ApiType, () => Observable<Result<U64, MmrError>>>;
            /**
             * Return the on-chain MMR root hash.
             **/
            mmrRoot: AugmentedCall<ApiType, () => Observable<Result<Hash, MmrError>>>;
            /**
             * Verify MMR proof against on-chain MMR.
             **/
            verifyProof: AugmentedCall<
                ApiType,
                (
                    leaves: Vec<MmrEncodableOpaqueLeaf> | (MmrEncodableOpaqueLeaf | string | Uint8Array)[],
                    proof: MmrBatchProof | { leafIndices?: any; leafCount?: any; items?: any } | string | Uint8Array
                ) => Observable<Result<ITuple<[]>, MmrError>>
            >;
            /**
             * Verify MMR proof against given root hash.
             **/
            verifyProofStateless: AugmentedCall<
                ApiType,
                (
                    root: Hash | string | Uint8Array,
                    leaves: Vec<MmrEncodableOpaqueLeaf> | (MmrEncodableOpaqueLeaf | string | Uint8Array)[],
                    proof: MmrBatchProof | { leafIndices?: any; leafCount?: any; items?: any } | string | Uint8Array
                ) => Observable<Result<ITuple<[]>, MmrError>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xf78b278be53f454c/2 */
        offchainWorkerApi: {
            /**
             * Starts the off-chain task for given block header.
             **/
            offchainWorker: AugmentedCall<
                ApiType,
                (
                    header:
                        | Header
                        | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any }
                        | string
                        | Uint8Array
                ) => Observable<Null>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xab3c0572291feb8b/1 */
        sessionKeys: {
            /**
             * Decode the given public session keys.
             **/
            decodeSessionKeys: AugmentedCall<
                ApiType,
                (encoded: Bytes | string | Uint8Array) => Observable<Option<Vec<ITuple<[Bytes, KeyTypeId]>>>>
            >;
            /**
             * Generate a set of session keys with optionally using the given seed.
             **/
            generateSessionKeys: AugmentedCall<
                ApiType,
                (seed: Option<Bytes> | null | Uint8Array | Bytes | string) => Observable<Bytes>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xd2bc9897eed08f15/3 */
        taggedTransactionQueue: {
            /**
             * Validate the transaction.
             **/
            validateTransaction: AugmentedCall<
                ApiType,
                (
                    source: TransactionSource | "InBlock" | "Local" | "External" | number | Uint8Array,
                    tx: Extrinsic | IExtrinsic | string | Uint8Array,
                    blockHash: BlockHash | string | Uint8Array
                ) => Observable<TransactionValidity>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x37c8bb1350a9a2a8/4 */
        transactionPaymentApi: {
            /**
             * The transaction fee details
             **/
            queryFeeDetails: AugmentedCall<
                ApiType,
                (
                    uxt: Extrinsic | IExtrinsic | string | Uint8Array,
                    len: u32 | AnyNumber | Uint8Array
                ) => Observable<FeeDetails>
            >;
            /**
             * The transaction info
             **/
            queryInfo: AugmentedCall<
                ApiType,
                (
                    uxt: Extrinsic | IExtrinsic | string | Uint8Array,
                    len: u32 | AnyNumber | Uint8Array
                ) => Observable<RuntimeDispatchInfo>
            >;
            /**
             * Query the output of the current LengthToFee given some input
             **/
            queryLengthToFee: AugmentedCall<ApiType, (length: u32 | AnyNumber | Uint8Array) => Observable<Balance>>;
            /**
             * Query the output of the current WeightToFee given some input
             **/
            queryWeightToFee: AugmentedCall<
                ApiType,
                (weight: Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => Observable<Balance>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xf3ff14d5ab527059/3 */
        transactionPaymentCallApi: {
            /**
             * The call fee details
             **/
            queryCallFeeDetails: AugmentedCall<
                ApiType,
                (
                    call: Call | IMethod | string | Uint8Array,
                    len: u32 | AnyNumber | Uint8Array
                ) => Observable<FeeDetails>
            >;
            /**
             * The call info
             **/
            queryCallInfo: AugmentedCall<
                ApiType,
                (
                    call: Call | IMethod | string | Uint8Array,
                    len: u32 | AnyNumber | Uint8Array
                ) => Observable<RuntimeDispatchInfo>
            >;
            /**
             * Query the output of the current LengthToFee given some input
             **/
            queryLengthToFee: AugmentedCall<ApiType, (length: u32 | AnyNumber | Uint8Array) => Observable<Balance>>;
            /**
             * Query the output of the current WeightToFee given some input
             **/
            queryWeightToFee: AugmentedCall<
                ApiType,
                (weight: Weight | { refTime?: any; proofSize?: any } | string | Uint8Array) => Observable<Balance>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x6ff52ee858e6c5bd/1 */
        xcmPaymentApi: {
            /**
             * The API to query acceptable payment assets
             **/
            queryAcceptablePaymentAssets: AugmentedCall<
                ApiType,
                (
                    version: u32 | AnyNumber | Uint8Array
                ) => Observable<Result<Vec<XcmVersionedAssetId>, XcmPaymentApiError>>
            >;
            /**
             *
             **/
            queryWeightToAssetFee: AugmentedCall<
                ApiType,
                (
                    weight: WeightV2 | { refTime?: any; proofSize?: any } | string | Uint8Array,
                    asset: XcmVersionedAssetId | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => Observable<Result<u128, XcmPaymentApiError>>
            >;
            /**
             *
             **/
            queryXcmWeight: AugmentedCall<
                ApiType,
                (
                    message: XcmVersionedXcm | { V3: any } | { V4: any } | { V5: any } | string | Uint8Array
                ) => Observable<Result<WeightV2, XcmPaymentApiError>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
    } // AugmentedCalls
} // declare module
