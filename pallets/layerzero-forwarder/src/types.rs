use crate::Config;
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::{
    CloneNoBound, DebugNoBound, Deserialize, EqNoBound, PartialEqNoBound, Serialize,
};
use sp_runtime::BoundedVec;
use tp_bridge::layerzero_message::{LayerZeroAddress, LayerZeroEndpoint};
use tp_traits::__reexports::{Decode, DecodeWithMemTracking, Encode, RuntimeDebug, TypeInfo};

pub type ChainId = u32;
pub type PalletIndex = u32;
pub type CallIndex = u32;

#[derive(
    DebugNoBound,
    PartialEqNoBound,
    EqNoBound,
    Encode,
    Decode,
    CloneNoBound,
    TypeInfo,
    MaxEncodedLen,
    DecodeWithMemTracking,
)]
#[scale_info(skip_type_params(T))]
pub struct MessageForwardingConfig<T: Config> {
    /// List of whitelisted (LayerZeroEndpoint, LayerZeroAddress) tuples allowed to forward messages to this chain
    pub whitelisted_senders:
        BoundedVec<(LayerZeroEndpoint, LayerZeroAddress), T::MaxWhitelistedSenders>,
    /// Pallet index and call index to be used when forwarding messages to this chain
    pub notification_destination: (PalletIndex, CallIndex),
}
