// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/consts";

import type { ApiTypes, AugmentedConst } from "@polkadot/api-base/types";
import type { u128, u16, u32, u64, u8 } from "@polkadot/types-codec";
import type { Codec } from "@polkadot/types-codec/types";
import type {
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
} from "@polkadot/types/lookup";

export type __AugmentedConst<ApiType extends ApiTypes> = AugmentedConst<ApiType>;

declare module "@polkadot/api-base/types/consts" {
    interface AugmentedConsts<ApiType extends ApiTypes> {
        balances: {
            /**
             * The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!
             *
             * If you _really_ need it to be zero, you can enable the feature `insecure_zero_ed` for this pallet. However, you
             * do so at your own risk: this will open up a major DoS vector. In case you have multiple sources of provider
             * references, you may also get unexpected behaviour if you set this to zero.
             *
             * Bottom line: Do yourself a favour and make it at least one!
             */
            existentialDeposit: u128 & AugmentedConst<ApiType>;
            /** The maximum number of individual freeze locks that can exist on an account at any time. */
            maxFreezes: u32 & AugmentedConst<ApiType>;
            /** The maximum number of holds that can exist on an account at any time. */
            maxHolds: u32 & AugmentedConst<ApiType>;
            /** The maximum number of locks that should exist on an account. Not strictly enforced, but used for weight estimation. */
            maxLocks: u32 & AugmentedConst<ApiType>;
            /** The maximum number of named reserves that can exist on an account. */
            maxReserves: u32 & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
        proxy: {
            /**
             * The base amount of currency needed to reserve for creating an announcement.
             *
             * This is held when a new storage item holding a `Balance` is created (typically 16 bytes).
             */
            announcementDepositBase: u128 & AugmentedConst<ApiType>;
            /**
             * The amount of currency needed per announcement made.
             *
             * This is held for adding an `AccountId`, `Hash` and `BlockNumber` (typically 68 bytes) into a pre-existing storage value.
             */
            announcementDepositFactor: u128 & AugmentedConst<ApiType>;
            /** The maximum amount of time-delayed announcements that are allowed to be pending. */
            maxPending: u32 & AugmentedConst<ApiType>;
            /** The maximum amount of proxies allowed for a single account. */
            maxProxies: u32 & AugmentedConst<ApiType>;
            /**
             * The base amount of currency needed to reserve for creating a proxy.
             *
             * This is held for an additional storage item whose value size is `sizeof(Balance)` bytes and whose key size is
             * `sizeof(AccountId)` bytes.
             */
            proxyDepositBase: u128 & AugmentedConst<ApiType>;
            /**
             * The amount of currency needed per proxy added.
             *
             * This is held for adding 32 bytes plus an instance of `ProxyType` more into a pre-existing storage value. Thus,
             * when configuring `ProxyDepositFactor` one should take into account `32 + proxy_type.encode().len()` bytes of data.
             */
            proxyDepositFactor: u128 & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
        registrar: {
            depositAmount: u128 & AugmentedConst<ApiType>;
            /** Max length of encoded genesis data */
            maxGenesisDataSize: u32 & AugmentedConst<ApiType>;
            /** Max length of para id list */
            maxLengthParaIds: u32 & AugmentedConst<ApiType>;
            sessionDelay: u32 & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
        system: {
            /** Maximum number of block number to block hash mappings to keep (oldest pruned first). */
            blockHashCount: u32 & AugmentedConst<ApiType>;
            /** The maximum length of a block (in bytes). */
            blockLength: FrameSystemLimitsBlockLength & AugmentedConst<ApiType>;
            /** Block & extrinsics weights: base values and limits. */
            blockWeights: FrameSystemLimitsBlockWeights & AugmentedConst<ApiType>;
            /** The weight of runtime database operations the runtime can invoke. */
            dbWeight: SpWeightsRuntimeDbWeight & AugmentedConst<ApiType>;
            /**
             * The designated SS58 prefix of this chain.
             *
             * This replaces the "ss58Format" property declared in the chain spec. Reason is that the runtime should know
             * about the prefix in order to make use of it as an identifier of the chain.
             */
            ss58Prefix: u16 & AugmentedConst<ApiType>;
            /** Get the chain's current version. */
            version: SpVersionRuntimeVersion & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
        timestamp: {
            /**
             * The minimum period between blocks.
             *
             * Be aware that this is different to the _expected_ period that the block production apparatus provides. Your
             * chosen consensus system will generally work with this to determine a sensible block time. For example, in the
             * Aura pallet it will be double this period on default settings.
             */
            minimumPeriod: u64 & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
        transactionPayment: {
            /**
             * A fee mulitplier for `Operational` extrinsics to compute "virtual tip" to boost their `priority`
             *
             * This value is multipled by the `final_fee` to obtain a "virtual tip" that is later added to a tip component in
             * regular `priority` calculations. It means that a `Normal` transaction can front-run a similarly-sized
             * `Operational` extrinsic (with no tip), by including a tip value greater than the virtual tip.
             *
             * ```rust,ignore
             * // For `Normal`
             * let priority = priority_calc(tip);
             *
             * // For `Operational`
             * let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
             * let priority = priority_calc(tip + virtual_tip);
             * ```
             *
             * Note that since we use `final_fee` the multiplier applies also to the regular `tip` sent with the transaction.
             * So, not only does the transaction get a priority bump based on the `inclusion_fee`, but we also amplify the
             * impact of tips applied to `Operational` transactions.
             */
            operationalFeeMultiplier: u8 & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
        txPause: {
            /**
             * Maximum length for pallet name and call name SCALE encoded string names.
             *
             * TOO LONG NAMES WILL BE TREATED AS PAUSED.
             */
            maxNameLen: u32 & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
        utility: {
            /** The limit on the number of batched calls. */
            batchedCallsLimit: u32 & AugmentedConst<ApiType>;
            /** Generic const */
            [key: string]: Codec;
        };
    } // AugmentedConsts
} // declare module
