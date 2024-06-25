#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use pallet_assets::Instance1;

use frame_support::{
	traits::{Currency, Incrementable, ReservableCurrency},
	PalletId,
};

use frame_support::sp_runtime::traits::{
	AccountIdConversion, CheckedAdd, CheckedDiv, CheckedMul, StaticLookup,
};

use enumflags2::BitFlags;

use pallet_nfts::{
	CollectionConfig, CollectionSetting, CollectionSettings, ItemConfig, ItemSettings, MintSettings,
};

use frame_system::RawOrigin;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

type BalanceOf<T> = <T as pallet_assets::Config<pallet_assets::Instance1>>::Balance;

type BalanceOf1<T> = <T as pallet_nft_fractionalization::Config>::AssetBalance;

type BalanceOf2<T> = <<T as pallet_nfts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[cfg(feature = "runtime-benchmarks")]
	pub struct NftHelper;

	#[cfg(feature = "runtime-benchmarks")]
	pub trait BenchmarkHelper<CollectionId, ItemId, AssetId, T> {
		fn to_collection(i: u32) -> CollectionId;
		fn to_nft(i: u32) -> ItemId;
		fn to_asset(i: u32) -> AssetId;
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl<CollectionId: From<u32>, ItemId: From<u32>, T: Config>
		BenchmarkHelper<CollectionId, ItemId, AssetId<T>, T> for NftHelper
	{
		fn to_collection(i: u32) -> CollectionId {
			i.into()
		}
		fn to_nft(i: u32) -> ItemId {
			i.into()
		}
		fn to_asset(i: u32) -> AssetId<T> {
			i.into()
		}
	}

	/// Infos regarding a listed nft of a real estate object on the marketplace.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct NftDetails<T: Config> {
		pub spv_created: bool,
		pub asset_id: u32,
		pub region: u32,
		pub location: LocationId<T>,
	}

	/// Infos regarding the listing of a real estate object.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct NftListingDetails<Balance, ItemId, CollectionId, T: Config> {
		pub real_estate_developer: AccountIdOf<T>,
		pub token_price: Balance,
		pub collected_funds: Balance,
		pub asset_id: u32,
		pub item_id: ItemId,
		pub collection_id: CollectionId,
		pub token_amount: u32,
	}

	/// Infos regarding the listing of a token.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct TokenListingDetails<Balance, ItemId, CollectionId, T: Config> {
		pub seller: AccountIdOf<T>,
		pub token_price: Balance,
		pub asset_id: u32,
		pub item_id: ItemId,
		pub collection_id: CollectionId,
		pub amount: u32,
	}

	/// Infos regarding the asset id.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct AssetDetails<ItemId, CollectionId, T: Config> {
		pub collection_id: CollectionId,
		pub item_id: ItemId,
		pub region: u32,
		pub location: LocationId<T>,
	}

	/// Infos regarding an offer.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct OfferDetails<Balance, T: Config> {
		pub buyer: AccountIdOf<T>,
		pub token_price: Balance,
		pub amount: u32,
	}

	/// Offer enum.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub enum Offer {
		Accept,
		Reject,
	}

	/// AccountId storage.
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct PalletIdStorage<T: Config> {
		pallet_id: AccountIdOf<T>,
	}

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_nfts::Config
		+ pallet_xcavate_whitelist::Config
 		+ pallet_assets::Config<Instance1>
		+ pallet_nft_fractionalization::Config 
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type representing the weight of this pallet.
		type WeightInfo: WeightInfo;

		/// The currency type.
		type Currency: Currency<AccountIdOf<Self>> + ReservableCurrency<AccountIdOf<Self>>;

		/// The marketplace's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		#[cfg(feature = "runtime-benchmarks")]
		type Helper: crate::BenchmarkHelper<
			<Self as pallet::Config>::CollectionId,
			<Self as pallet::Config>::ItemId,
			<Self as pallet_assets::Config<Instance1>>::AssetId,
			Self,
		>;

		/// The maximum amount of token of a nft.
		#[pallet::constant]
		type MaxNftToken: Get<u32>;

		/// Origin who can unlock new locations.
		type LocationOrigin: EnsureOrigin<Self::RuntimeOrigin>;

 		/// Collection id type from pallet nfts.
		type CollectionId: IsType<<Self as pallet_nfts::Config>::CollectionId>
			+ Parameter
			+ From<u32>
			+ Default
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;

		/// Item id type from pallet nfts.
		type ItemId: IsType<<Self as pallet_nfts::Config>::ItemId>
			+ Parameter
			+ From<u32>
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;
 
		/// Collection id type from pallet nft fractionalization.
 		type FractionalizeCollectionId: IsType<<Self as pallet_nft_fractionalization::Config>::NftCollectionId>
			+ Parameter
			+ From<CollectionId<Self>>
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;

		/// Item id type from pallet nft fractionalization.
		type FractionalizeItemId: IsType<<Self as pallet_nft_fractionalization::Config>::NftId>
			+ Parameter
			+ From<u32>
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;

		/// Asset id type from pallet nft fractionalization.
		type AssetId: IsType<<Self as pallet_nft_fractionalization::Config>::AssetId>
			+ Parameter
			+ From<u32>
			+ Ord
			+ Copy; 

		/// Asset id type from pallet assets.
		type AssetId2: IsType<<Self as pallet_assets::Config<Instance1>>::AssetId>
			+ Parameter
			+ From<u32>
			+ Ord
			+ Copy;

		/// The Trasury's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type TreasuryId: Get<PalletId>;

		/// The CommunityProjects's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type CommunityProjectsId: Get<PalletId>;

		/// The maximum length of data stored in for post codes.
		#[pallet::constant]
		type PostcodeLimit: Get<u32>;
	}

	pub type AssetId<T> = <T as Config>::AssetId;
	pub type AssetId2<T> = <T as Config>::AssetId2;
 	pub type CollectionId<T> = <T as Config>::CollectionId;
	pub type ItemId<T> = <T as Config>::ItemId; 
 	pub type FractionalizeCollectionId<T> = <T as Config>::FractionalizeCollectionId;
	pub type FractionalizeItemId<T> = <T as Config>::FractionalizeItemId; 
	pub type RegionId = u32;
	pub type LocationId<T> = BoundedVec<u8, <T as Config>::PostcodeLimit>;

 	pub(super) type NftListingDetailsType<T> = NftListingDetails<
		BalanceOf<T>,
		<T as pallet::Config>::ItemId,
		<T as pallet::Config>::CollectionId,
		T,
	>;

	pub(super) type ListingDetailsType<T> = TokenListingDetails<
		BalanceOf<T>,
		<T as pallet::Config>::ItemId,
		<T as pallet::Config>::CollectionId,
		T,
	>;

	/// Id for the next nft in a collection.
	#[pallet::storage]
	#[pallet::getter(fn next_nft_id)]
	pub(super) type NextNftId<T: Config> =
		StorageMap<_, Blake2_128Concat, <T as pallet::Config>::CollectionId, u32, ValueQuery>;

	/// Id of the possible next asset that would be used for
	/// Nft fractionalization.
	#[pallet::storage]
	#[pallet::getter(fn next_asset_id)]
	pub(super) type NextAssetId<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Id of the next region.
	#[pallet::storage]
	#[pallet::getter(fn next_region_id)]
	pub(super) type NextRegionId<T: Config> = StorageValue<_, RegionId, ValueQuery>;

	/// Id for the next offer for a listing.
	#[pallet::storage]
	#[pallet::getter(fn next_offer_id)]
	pub(super) type NextOfferId<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, u32, ValueQuery>;

	/// True if a location is registered.
	#[pallet::storage]
	#[pallet::getter(fn location_registration)]
	pub(super) type LocationRegistration<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		RegionId,
		Blake2_128Concat,
		LocationId<T>, 
		bool,
		ValueQuery,
	>;

	/// The Id for the next token listing.
	#[pallet::storage]
	#[pallet::getter(fn next_listing_id)]
	pub(super) type NextListingId<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Mapping of a collection id to the region.
	#[pallet::storage]
	#[pallet::getter(fn region_collections)]
	pub(super) type RegionCollections<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RegionId,
		<T as pallet::Config>::CollectionId,
		OptionQuery,
	>;

	/// Mapping from the Nft to the Nft details.
	#[pallet::storage]
	#[pallet::getter(fn registered_nft_details)]
	pub(super) type RegisteredNftDetails<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		<T as pallet::Config>::CollectionId,
		Blake2_128Concat,
		<T as pallet::Config>::ItemId,
		NftDetails<T>,
		OptionQuery,
	>;

	/// Mapping from the nft to the ongoing nft listing details.
	#[pallet::storage]
	#[pallet::getter(fn ongoing_object_listing)]
	pub(super) type OngoingObjectListing<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, NftListingDetailsType<T>, OptionQuery>;

	/// Mapping of the nft to the amount of listed token.
	#[pallet::storage]
	#[pallet::getter(fn listed_token)]
	pub(super) type ListedToken<T: Config> = StorageMap<_, Blake2_128Concat, u32, u32, OptionQuery>;

	/// Mapping of the listing to the buyer of the sold token.
	#[pallet::storage]
	#[pallet::getter(fn token_buyer)]
	pub(super) type TokenBuyer<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		BoundedVec<AccountIdOf<T>, T::MaxNftToken>,
		ValueQuery,
	>;

	/// Double mapping of the account id of the token owner
	/// and the listing to the amount of token.
	#[pallet::storage]
	#[pallet::getter(fn token_owner)]
	pub(super) type TokenOwner<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		u32,
		u32,
		ValueQuery,
	>;

	/// Mapping of the listing id to the listing details of a token listing.
	#[pallet::storage]
	#[pallet::getter(fn token_listings)]
	pub(super) type TokenListings<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, ListingDetailsType<T>, OptionQuery>;

	/// Mapping of the assetid to the vector of token holder.
	#[pallet::storage]
	#[pallet::getter(fn property_owner)]
	pub(super) type PropertyOwner<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		BoundedVec<AccountIdOf<T>, T::MaxNftToken>,
		ValueQuery,
	>;

	/// Mapping of assetid and accountid to the amount of token an account is holding of the asset.
	#[pallet::storage]
	#[pallet::getter(fn property_owner_token)]
	pub(super) type PropertyOwnerToken<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32,
		Blake2_128Concat,
		AccountIdOf<T>,
		u32,
		ValueQuery,
	>;

	/// Mapping of the assetid to the collectionid and nftid.
	#[pallet::storage]
	#[pallet::getter(fn asset_id_details)]
	pub(super) type AssetIdDetails<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		AssetDetails<<T as pallet::Config>::ItemId, <T as pallet::Config>::CollectionId, T>,
		OptionQuery,
	>;

	/// Mapping from listing to offer details.
	#[pallet::storage]
	#[pallet::getter(fn ongoing_offers)]
	pub(super) type OngoingOffer<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32,
		Blake2_128Concat,
		u32,
		OfferDetails<BalanceOf<T>, T>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new object has been listed on the marketplace.
 		ObjectListed {
			collection_index: <T as pallet::Config>::CollectionId,
			item_index: <T as pallet::Config>::ItemId,
			price: BalanceOf<T>,
			seller: AccountIdOf<T>,
		}, 
		/// A token has been bought.
		TokenBought {
			asset_id: u32,
			buyer: AccountIdOf<T>,
			price: BalanceOf<T>,
		},
 		/// Token from listed object have been bought.
		TokenBoughtObject {
			asset_id: u32,
			buyer: AccountIdOf<T>,
			amount: u32,
			price: BalanceOf<T>,
		},
		/// Token have been listed.
		TokenListed {
			asset_id: u32,
			price: BalanceOf<T>,
			seller: AccountIdOf<T>,
		},
		/// The price of the token listing has been updated.
		ListingUpdated { listing_index: u32, new_price: BalanceOf<T> },
		/// The nft has been delisted.
		ListingDelisted { listing_index: u32 },
		/// The price of the listed object has been updated.
		ObjectUpdated { listing_index: u32, new_price: BalanceOf<T> },
		/// New region has been created.
		RegionCreated { region_id: u32, collection_id: CollectionId<T> },
		/// New location has been created.
		LocationCreated { region_id: u32, location_id: LocationId<T> }, 
		/// A new offer has been made.
		OfferCreated { listing_id: u32, price: BalanceOf<T> },
		/// An offer has been cancelled.
		OfferCancelled { listing_id: u32, offer_id: u32},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// This index is not taken.
		InvalidIndex,
		/// The buyer doesn't have enough funds.
		NotEnoughFunds,
		/// Not enough token available to buy.
		NotEnoughTokenAvailable,
		/// Error by converting a type.
		ConversionError,
		/// Error by dividing a number.
		DivisionError,
		/// Error by multiplying a number.
		MultiplyError,
		/// No sufficient permission.
		NoPermission,
		/// The SPV has already been created.
		SpvAlreadyCreated,
		/// User did not pass the kyc.
		UserNotWhitelisted,
		ArithmeticUnderflow,
		ArithmeticOverflow,
		/// The token is not for sale.
		TokenNotForSale,
		/// The nft has not been registered on the marketplace.
		NftNotFound,
		/// There are already too many token buyer.
		TooManyTokenBuyer,
		/// This Region is not known.
		RegionUnknown,
		/// The location is already registered.
		LocationRegistered,
		/// The location is not registered.
		LocationUnknown,
		/// The object can not be divided in so many token.
		TooManyToken,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a new region for the marketplace.
		/// This function calls the nfts-pallet to create a new collection.
		///
		/// The origin must be the sudo.
		///
		/// Emits `RegionCreated` event when succesfful.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::create_new_region())]
		pub fn create_new_region(origin: OriginFor<T>) -> DispatchResult {
			T::LocationOrigin::ensure_origin(origin)?;
 			if pallet_nfts::NextCollectionId::<T>::get().is_none() {
				pallet_nfts::NextCollectionId::<T>::set(
					<T as pallet_nfts::Config>::CollectionId::initial_value(),
				);
			};
 			let collection_id = pallet_nfts::NextCollectionId::<T>::get().unwrap();
			let next_collection_id = collection_id.increment();
			pallet_nfts::NextCollectionId::<T>::set(next_collection_id); 
			let collection_id: CollectionId<T> = collection_id.into(); 
			let pallet_id: AccountIdOf<T> =
				<T as pallet::Config>::PalletId::get().into_account_truncating();
			pallet_nfts::Pallet::<T>::do_create_collection(
				collection_id.into(),
				pallet_id.clone(),
				pallet_id.clone(),
				Self::default_collection_config(),
				T::CollectionDeposit::get(),
				pallet_nfts::Event::Created {
					creator: pallet_id.clone(),
					owner: pallet_id,
					collection: collection_id.into(),
				},
			)?; 
 			let mut region_id = Self::next_region_id();
			RegionCollections::<T>::insert(region_id, collection_id);
			region_id = region_id.checked_add(1).ok_or(Error::<T>::ArithmeticOverflow)?;
			NextRegionId::<T>::put(region_id);
			Self::deposit_event(Event::<T>::RegionCreated { 
				region_id, 
				collection_id,
			}); 
			Ok(())
		}

 		/// Creates a new location for a region.
		///
		/// The origin must be the sudo.
		///
		/// Parameters:
		/// - `region`: The region where the new location should be created.
		/// - `location`: The postcode of the new location.
		///
		/// Emits `LocationCreated` event when succesfful.
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::create_new_location())]
		pub fn create_new_location(origin: OriginFor<T>, region: RegionId, location: LocationId<T>) -> DispatchResult {
			T::LocationOrigin::ensure_origin(origin)?;
			ensure!(Self::region_collections(region).is_some(), Error::<T>::RegionUnknown);
			ensure!(!Self::location_registration(region, location.clone()), Error::<T>::LocationRegistered);
			LocationRegistration::<T>::insert(region, location.clone(), true);
			Self::deposit_event(Event::<T>::LocationCreated { 
				region_id: region, 
				location_id: location,
			});
			Ok(())
		}

		/// List a real estate object. A new nft gets minted.
		/// This function calls the nfts-pallet to mint a new nft and sets the Metadata.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `region`: The region where the object is located.
		/// - `location`: The location where the object is located.
		/// - `token_price`: The price of a single token.
		/// - `token_amount`: The amount of tokens for a object.
		/// - `data`: The Metadata of the nft.
		///
		/// Emits `ObjectListed` event when succesfful
		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::list_object())]
		pub fn list_object(
			origin: OriginFor<T>,
			region: RegionId,
			location: LocationId<T>,
			token_price: BalanceOf<T>,
			token_amount: u32,
			data: BoundedVec<u8, <T as pallet_nfts::Config>::StringLimit>,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			ensure!(token_amount <= T::MaxNftToken::get(), Error::<T>::TooManyToken);
			let collection_id: CollectionId<T> =
				Self::region_collections(region).ok_or(Error::<T>::RegionUnknown)?;
			ensure!(Self::location_registration(region, location.clone()), Error::<T>::LocationUnknown);
			let mut next_item_id = Self::next_nft_id(collection_id);
			let mut asset_number: u32 = Self::next_asset_id();
			let mut asset_id: AssetId2<T> = asset_number.into();
			while pallet_assets::Pallet::<T, Instance1>::maybe_total_supply(asset_id.into())
				.is_some()
			{
				asset_number = asset_number.checked_add(1).ok_or(Error::<T>::ArithmeticOverflow)?;
				asset_id = asset_number.into();
			}
			let asset_id: AssetId<T> = asset_number.into();
			let item_id: ItemId<T> = next_item_id.into();
			let mut listing_id = Self::next_listing_id();
			let nft = NftListingDetails {
				real_estate_developer: signer.clone(),
				token_price,
				collected_funds: Default::default(),
				asset_id: asset_number,
				item_id,
				collection_id,
				token_amount,
			};
			pallet_nfts::Pallet::<T>::do_mint(
				collection_id.into(),
				item_id.into(),
				Some(Self::account_id()),
				Self::account_id(),
				Self::default_item_config(),
				|_, _| Ok(()),
			)?;
			let pallet_origin: OriginFor<T> = RawOrigin::Signed(Self::account_id()).into();
			pallet_nfts::Pallet::<T>::set_metadata(
				pallet_origin.clone(),
				collection_id.into(),
				item_id.into(),
				data.clone(),
			)?;
			let registered_nft_details =
				NftDetails { spv_created: Default::default(), asset_id: asset_number, region, location: location.clone() };
			RegisteredNftDetails::<T>::insert(collection_id, item_id, registered_nft_details);
			OngoingObjectListing::<T>::insert(listing_id, nft.clone());
			ListedToken::<T>::insert(listing_id, token_amount);

			let user_lookup = <T::Lookup as StaticLookup>::unlookup(Self::account_id());
			let nft_balance: BalanceOf1<T> = token_amount.into();
			let fractionalize_collection_id: FractionalizeCollectionId<T> =
				collection_id.try_into().map_err(|_| Error::<T>::ConversionError)?;
			let fractionalize_item_id: FractionalizeItemId<T> = next_item_id.into();
 			pallet_nft_fractionalization::Pallet::<T>::fractionalize(
				pallet_origin.clone(),
				fractionalize_collection_id.into(),
				fractionalize_item_id.into(),
				asset_id.into(),
				user_lookup,
				nft_balance,
			)?; 
			let asset_details = AssetDetails {
				collection_id,
				item_id,
				region,
				location,
			};
			AssetIdDetails::<T>::insert(asset_number, asset_details);
			next_item_id = next_item_id.checked_add(1).ok_or(Error::<T>::ArithmeticOverflow)?;
			asset_number = asset_number.checked_add(1).ok_or(Error::<T>::ArithmeticOverflow)?;
			NextNftId::<T>::insert(collection_id, next_item_id);
			NextAssetId::<T>::put(asset_number);
			listing_id = listing_id.checked_add(1).ok_or(Error::<T>::ArithmeticOverflow)?;
			NextListingId::<T>::put(listing_id);

			Self::deposit_event(Event::<T>::ObjectListed {
				collection_index: collection_id,
				item_index: item_id,
				price: token_price,
				seller: signer,
			});
			Ok(())
		}

		/// Buy listed token from the marketplace.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the investor wants to buy token from.
		/// - `amount`: The amount of token that the investor wants to buy.
		///
		/// Emits `TokenBoughtObject` event when succesfful.
		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::buy_token())]
		pub fn buy_token(origin: OriginFor<T>, listing_id: u32, amount: u32) -> DispatchResult {
			let origin = ensure_signed(origin.clone())?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let mut listed_token =
				ListedToken::<T>::take(listing_id).ok_or(Error::<T>::TokenNotForSale)?;
			ensure!(listed_token >= amount, Error::<T>::NotEnoughTokenAvailable);
			let mut nft_details =
				Self::ongoing_object_listing(listing_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(
				!Self::registered_nft_details(nft_details.collection_id, nft_details.item_id)
					.ok_or(Error::<T>::InvalidIndex)?
					.spv_created,
				Error::<T>::SpvAlreadyCreated
			);

			let transfer_price = nft_details
				.token_price
				.checked_mul(&Self::u64_to_balance_option(amount as u64)?)
				.ok_or(Error::<T>::MultiplyError)?;
			Self::transfer_funds(origin.clone(), Self::account_id(), transfer_price)?;
			listed_token =
				listed_token.checked_sub(amount).ok_or(Error::<T>::ArithmeticUnderflow)?;
			if !Self::token_buyer(listing_id).contains(&origin) {
				TokenBuyer::<T>::try_mutate(listing_id, |keys| {
					keys.try_push(origin.clone()).map_err(|_| Error::<T>::TooManyTokenBuyer)?;
					Ok::<(), DispatchError>(())
				})?;
			}
			let mut token_of_owner = TokenOwner::<T>::take(origin.clone(), listing_id);
			token_of_owner =
				token_of_owner.checked_add(amount).ok_or(Error::<T>::ArithmeticOverflow)?;
			nft_details.collected_funds = nft_details
				.collected_funds
				.checked_add(&transfer_price)
				.ok_or(Error::<T>::ArithmeticOverflow)?;
			OngoingObjectListing::<T>::insert(listing_id, nft_details.clone());
			TokenOwner::<T>::insert(origin.clone(), listing_id, token_of_owner);
			if listed_token == 0 {
				Self::distribute_nfts(listing_id)?;
			} else {
				ListedToken::<T>::insert(listing_id, listed_token);
			}
			Self::deposit_event(Event::<T>::TokenBoughtObject {
				asset_id: nft_details.asset_id,
				buyer: origin.clone(),
				amount,
				price: transfer_price,
			});
			Ok(())
		}

		/// Relist token on the marketplace.
		/// The nft must be registered on the marketplace.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `region`: The region where the object is located.
		/// - `item_id`: The item id of the nft.
		/// - `token_price`: The price of a single token.
		/// - `amount`: The amount of token of the real estate object that should be listed.
		///
		/// Emits `TokenListed` event when succesfful
		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::relist_token())]
		pub fn relist_token(
			origin: OriginFor<T>,
			region: RegionId,
			item_id: <T as pallet::Config>::ItemId,
			token_price: BalanceOf<T>,
			amount: u32,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;

			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let collection_id: CollectionId<T> =
				Self::region_collections(region).ok_or(Error::<T>::RegionUnknown)?;

			let nft_details = Self::registered_nft_details(collection_id, item_id)
				.ok_or(Error::<T>::NftNotFound)?;
			ensure!(Self::location_registration(region, nft_details.location), Error::<T>::LocationUnknown);
			let pallet_lookup = <T::Lookup as StaticLookup>::unlookup(Self::account_id());
			let asset_id: AssetId2<T> = nft_details.asset_id.into();
			let token_amount = amount.into();
			pallet_assets::Pallet::<T, Instance1>::transfer(
				origin,
				asset_id.into().into(),
				pallet_lookup,
				token_amount,
			)
			.map_err(|_| Error::<T>::NotEnoughFunds)?;
			let mut listing_id = Self::next_listing_id();
			let token_listing = TokenListingDetails {
				seller: signer.clone(),
				token_price,
				asset_id: nft_details.asset_id,
				item_id,
				collection_id,
				amount,
			};
			TokenListings::<T>::insert(listing_id, token_listing);
			listing_id = listing_id.checked_add(1).ok_or(Error::<T>::ArithmeticOverflow)?;
			NextListingId::<T>::put(listing_id);

			Self::deposit_event(Event::<T>::TokenListed {
				asset_id: nft_details.asset_id,
				price: token_price,
				seller: signer,
			});
			Ok(())
		}

		/// Buy token from the marketplace.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the investor wants to buy from.
		/// - `amount`: The amount of token the investor wants to buy.
		///
		/// Emits `TokenBought` event when succesfful.
		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::buy_relisted_token())]
		pub fn buy_relisted_token(origin: OriginFor<T>, listing_id: u32, amount: u32) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(origin.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let listing_details =
				TokenListings::<T>::take(listing_id).ok_or(Error::<T>::TokenNotForSale)?;
			ensure!(listing_details.amount >= amount, Error::<T>::NotEnoughTokenAvailable);
			let price = listing_details
				.token_price
				.checked_mul(&Self::u64_to_balance_option(amount.into())?)
				.ok_or(Error::<T>::MultiplyError)?;
			Self::buying_token_process(listing_id, origin.clone(), origin, listing_details, price, amount)?;
			Ok(())
		}

		/// Created an offer for a token listing.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the investor wants to buy from.
		/// - `offer_price`: The offer price for token that are offered.
		/// - `amount`: The amount of token that the investor wants to buy.
		///
		/// Emits `OfferCreated` event when succesfful.
		#[pallet::call_index(6)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::make_offer())]
		pub fn make_offer(
			origin: OriginFor<T>,
			listing_id: u32,
			offer_price: BalanceOf<T>,
			amount: u32,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let listing_details = Self::token_listings(listing_id).ok_or(Error::<T>::TokenNotForSale)?;
			ensure!(listing_details.amount >= amount, Error::<T>::NotEnoughTokenAvailable);
			let price = offer_price
				.checked_mul(&Self::u64_to_balance_option(amount.into())?)
				.ok_or(Error::<T>::MultiplyError)?;
			Self::transfer_funds(signer.clone(), Self::account_id(), price)?;
			let offer_details = OfferDetails {
				buyer: signer,
				token_price: offer_price,
				amount,
			};
			let mut offer_id = Self::next_offer_id(listing_id);
			OngoingOffer::<T>::insert(listing_id, offer_id, offer_details);
			offer_id = offer_id.checked_add(1).ok_or(Error::<T>::ArithmeticOverflow)?;
			NextOfferId::<T>::insert(listing_id, offer_id);
			Self::deposit_event(Event::<T>::OfferCreated { 
				listing_id,
				price: offer_price, 
			});
			Ok(())
		}

		/// Lets the investor handle an offer.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the investor wants to buy from.
		/// - `offer_id`: The offer that the seller wants to cancel.
		/// - `offer`: Enum for offer which is either Accept or Reject.
		#[pallet::call_index(7)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::handle_offer())]
		pub fn handle_offer(
			origin: OriginFor<T>,
			listing_id: u32,
			offer_id: u32,
			offer: Offer,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let listing_details = Self::token_listings(listing_id).ok_or(Error::<T>::TokenNotForSale)?;
			ensure!(listing_details.seller == signer, Error::<T>::NoPermission);
			let offer_details = OngoingOffer::<T>::take(listing_id, offer_id).ok_or(Error::<T>::InvalidIndex)?;
			let price = offer_details
				.token_price
 				.checked_mul(&Self::u64_to_balance_option(offer_details.amount.into())?)
				.ok_or(Error::<T>::MultiplyError)?;
			if offer == Offer::Accept {
				Self::buying_token_process(listing_id, Self::account_id(), offer_details.buyer, listing_details, price, offer_details.amount)?;
			} else {
				Self::transfer_funds(Self::account_id(), offer_details.buyer, price)?;
			}
			Ok(())
		}

		/// Lets the investor cancel an offer.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the investor wants to buy from.
		/// - `offer_id`: The offer that the seller wants to cancel.
		///
		/// Emits `OfferCancelled` event when succesfful.
		#[pallet::call_index(8)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_offer())]
		pub fn cancel_offer(
			origin: OriginFor<T>,
			listing_id: u32,
			offer_id: u32,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			let offer_details = OngoingOffer::<T>::take(listing_id, offer_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(offer_details.buyer == signer, Error::<T>::NoPermission);
			let price = offer_details
				.token_price
				.checked_mul(&Self::u64_to_balance_option(offer_details.amount.into())?)
				.ok_or(Error::<T>::MultiplyError)?;
		Self::transfer_funds(Self::account_id(), offer_details.buyer, price)?;
			Self::deposit_event(Event::<T>::OfferCancelled { listing_id, offer_id});
			Ok(())
		}

		/// Upgrade the price from a listing.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the seller wants to update.
		/// - `new_price`: The new price of the nft.
		///
		/// Emits `ListingUpdated` event when succesfful.
		#[pallet::call_index(9)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::upgrade_listing())]
		pub fn upgrade_listing(
			origin: OriginFor<T>,
			listing_id: u32,
			new_price: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let mut listing_details =
				Self::token_listings(listing_id).ok_or(Error::<T>::TokenNotForSale)?;
			ensure!(listing_details.seller == signer, Error::<T>::NoPermission);
			listing_details.token_price = new_price;
			TokenListings::<T>::insert(listing_id, listing_details);
			Self::deposit_event(Event::<T>::ListingUpdated {
				listing_index: listing_id,
				new_price,
			});
			Ok(())
		}

		/// Upgrade the price from a listed object.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the seller wants to update.
		/// - `new_price`: The new price of the object.
		///
		/// Emits `ObjectUpdated` event when succesfful.
		#[pallet::call_index(10)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::upgrade_object())]
		pub fn upgrade_object(
			origin: OriginFor<T>,
			listing_id: u32,
			new_price: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let mut nft_details =
				Self::ongoing_object_listing(listing_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(
				!Self::registered_nft_details(nft_details.collection_id, nft_details.item_id)
					.ok_or(Error::<T>::InvalidIndex)?
					.spv_created,
				Error::<T>::SpvAlreadyCreated
			);
			ensure!(ListedToken::<T>::contains_key(listing_id), Error::<T>::TokenNotForSale);
			nft_details.token_price = new_price;
			OngoingObjectListing::<T>::insert(listing_id, nft_details.clone());
			Self::deposit_event(Event::<T>::ObjectUpdated {
				listing_index: listing_id,
				new_price,
			});
			Ok(())
		}

		/// Delist the choosen listing from the marketplace.
		/// Works only for relisted token.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// Parameters:
		/// - `listing_id`: The listing that the seller wants to delist.
		///
		/// Emits `ListingDelisted` event when succesfful.
		#[pallet::call_index(11)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::delist_token())]
		pub fn delist_token(origin: OriginFor<T>, listing_id: u32) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(
				pallet_xcavate_whitelist::Pallet::<T>::whitelisted_accounts(signer.clone()),
				Error::<T>::UserNotWhitelisted
			);
			let listing_details =
				TokenListings::<T>::take(listing_id).ok_or(Error::<T>::TokenNotForSale)?;
			ensure!(listing_details.seller == signer, Error::<T>::NoPermission);
			let user_lookup = <T::Lookup as StaticLookup>::unlookup(signer.clone());
			let asset_id: AssetId2<T> = listing_details.asset_id.into();
			let token_amount = listing_details.amount.into();
			let pallet_origin: OriginFor<T> = RawOrigin::Signed(Self::account_id()).into();
			pallet_assets::Pallet::<T, Instance1>::transfer(
				pallet_origin,
				asset_id.into().into(),
				user_lookup,
				token_amount,
			)
			.map_err(|_| Error::<T>::NotEnoughFunds)?;
			Self::deposit_event(Event::<T>::ListingDelisted { listing_index: listing_id });
			Ok(())
		} 
	}

	impl<T: Config> Pallet<T> {
		
		/// Get the account id of the pallet
 		pub fn account_id() -> AccountIdOf<T> {
			<T as pallet::Config>::PalletId::get().into_account_truncating()
		}

		/// Get the account id of the treasury pallet
		pub fn treasury_account_id() -> AccountIdOf<T> {
			T::TreasuryId::get().into_account_truncating()
		}

		/// Get the account id of the community pallet
		pub fn community_account_id() -> AccountIdOf<T> {
			T::CommunityProjectsId::get().into_account_truncating()
		}

		/// Sends the token to the new owners and the funds to the real estate developer once all 100 token
		/// of a collection are sold.
		fn distribute_nfts(listing_id: u32) -> DispatchResult {
			let list = <TokenBuyer<T>>::take(listing_id);

			let nft_details =
				OngoingObjectListing::<T>::take(listing_id).ok_or(Error::<T>::InvalidIndex)?;
			let price = nft_details.collected_funds;
			Self::calculate_fees(price, Self::account_id(), nft_details.real_estate_developer)?;
			let origin: OriginFor<T> = RawOrigin::Signed(Self::account_id()).into();
			let asset_id: AssetId2<T> = nft_details.asset_id.into();
			for owner in list {
				let user_lookup = <T::Lookup as StaticLookup>::unlookup(owner.clone());
				let token: u64 = TokenOwner::<T>::take(owner.clone(), listing_id) as u64;
				let token_amount = token.try_into().map_err(|_| Error::<T>::ConversionError)?;
				pallet_assets::Pallet::<T, Instance1>::transfer(
					origin.clone(),
					asset_id.into().into(),
					user_lookup,
					token_amount,
				)
				.map_err(|_| Error::<T>::NotEnoughFunds)?;
				PropertyOwner::<T>::try_mutate(
					nft_details.asset_id,
					|keys| {
						keys.try_push(owner.clone()).map_err(|_| Error::<T>::TooManyTokenBuyer)?;
						Ok::<(), DispatchError>(())
					},
				)?;
				PropertyOwnerToken::<T>::insert(nft_details.asset_id, owner, token as u32)
			}
			let mut registered_nft_details =
				Self::registered_nft_details(nft_details.collection_id, nft_details.item_id)
					.ok_or(Error::<T>::InvalidIndex)?;
			registered_nft_details.spv_created = true;
			RegisteredNftDetails::<T>::insert(
				nft_details.collection_id,
				nft_details.item_id,
				registered_nft_details,
			);
			Ok(())
		}

		fn buying_token_process(listing_id: u32, transfer_from: AccountIdOf<T>, account: AccountIdOf<T>, mut listing_details: ListingDetailsType<T>, price: BalanceOf<T>, amount: u32) -> DispatchResult {
			
			Self::calculate_fees(price, transfer_from.clone(), listing_details.seller.clone())?;
			let user_lookup = <T::Lookup as StaticLookup>::unlookup(account.clone());
			let asset_id: AssetId2<T> = listing_details.asset_id.into();
			let token_amount = amount.into();
			let pallet_origin: OriginFor<T> = RawOrigin::Signed(Self::account_id()).into();
			pallet_assets::Pallet::<T, Instance1>::transfer(
				pallet_origin,
				asset_id.into().into(),
				user_lookup,
				token_amount,
			)
			.map_err(|_| Error::<T>::NotEnoughFunds)?;
			let mut old_token_owner_amount = PropertyOwnerToken::<T>::take(
				listing_details.asset_id,
				listing_details.seller.clone(),
			);
			old_token_owner_amount = old_token_owner_amount
				.checked_sub(amount)
				.ok_or(Error::<T>::ArithmeticUnderflow)?;
			if old_token_owner_amount == 0 {
				let mut owner_list = PropertyOwner::<T>::take(
					listing_details.asset_id,
				);
				let index =
					owner_list.iter().position(|x| *x == listing_details.seller.clone()).ok_or(Error::<T>::InvalidIndex)?;
				owner_list.remove(index);
				PropertyOwner::<T>::insert(
					listing_details.asset_id,
					owner_list,
				);
			} else {
				PropertyOwnerToken::<T>::insert(
					listing_details.asset_id,
					listing_details.seller.clone(),
					old_token_owner_amount,
				);
			}
			if Self::property_owner(listing_details.asset_id).contains(&account) {
				let mut buyer_token_amount = PropertyOwnerToken::<T>::take(listing_details.asset_id, account.clone());
				buyer_token_amount = buyer_token_amount
					.checked_add(amount)
					.ok_or(Error::<T>::ArithmeticOverflow)?;
				PropertyOwnerToken::<T>::insert(
					listing_details.asset_id,
					account.clone(),
					buyer_token_amount,
				);				
			} else {
				PropertyOwner::<T>::try_mutate(
					listing_details.asset_id,
					|keys| {
						keys.try_push(account.clone()).map_err(|_| Error::<T>::TooManyTokenBuyer)?;
						Ok::<(), DispatchError>(())
					},
				)?;
				PropertyOwnerToken::<T>::insert(
					listing_details.asset_id,
					account.clone(),
					amount,
				);
			}
			listing_details.amount = listing_details.amount.checked_sub(amount).ok_or(Error::<T>::ArithmeticUnderflow)?;
			if listing_details.amount > 0 {
				TokenListings::<T>::insert(listing_id, listing_details.clone());
				let _ = OngoingOffer::<T>::clear_prefix(listing_id, 2000, None);
			}
			Self::deposit_event(Event::<T>::TokenBought {
				asset_id: listing_details.asset_id,
				buyer: account.clone(),
				price: listing_details.token_price,
			});
			Ok(())
		}

		fn calculate_fees(
			price: BalanceOf<T>,
			sender: AccountIdOf<T>,
			receiver: AccountIdOf<T>,
		) -> DispatchResult {
			let fees = price
				.checked_div(&Self::u64_to_balance_option(100)?)
				.ok_or(Error::<T>::DivisionError)?;
			let treasury_id = Self::treasury_account_id();
			let treasury_fees = fees
				.checked_mul(&Self::u64_to_balance_option(90)?)
				.ok_or(Error::<T>::MultiplyError)?
				.checked_div(&Self::u64_to_balance_option(100)?)
				.ok_or(Error::<T>::DivisionError)?;
			let community_projects_id = Self::community_account_id();
			let community_fees = fees
				.checked_div(&Self::u64_to_balance_option(10)?)
				.ok_or(Error::<T>::DivisionError)?;
			let seller_part = price
				.checked_mul(&Self::u64_to_balance_option(99)?)
				.ok_or(Error::<T>::MultiplyError)?
				.checked_div(&Self::u64_to_balance_option(100)?)
				.ok_or(Error::<T>::DivisionError)?;
			Self::transfer_funds(sender.clone(), treasury_id, treasury_fees)?;
			Self::transfer_funds(sender.clone(), community_projects_id, community_fees)?;
			Self::transfer_funds(sender, receiver, seller_part)?;
			Ok(())
		}

		/// Set the default collection configuration for creating a collection.
		fn default_collection_config() -> CollectionConfig<
			BalanceOf2<T>,
			BlockNumberFor<T>,
			<T as pallet_nfts::Config>::CollectionId,
		> {
			Self::collection_config_from_disabled_settings(
				CollectionSetting::DepositRequired.into(),
			)
		}

		fn collection_config_from_disabled_settings(
			settings: BitFlags<CollectionSetting>,
		) -> CollectionConfig<
			BalanceOf2<T>,
			BlockNumberFor<T>,
			<T as pallet_nfts::Config>::CollectionId,
		> {
			CollectionConfig {
				settings: CollectionSettings::from_disabled(settings),
				max_supply: None,
				mint_settings: MintSettings::default(),
			}
		}

		/// Set the default item configuration for minting a nft.
		fn default_item_config() -> ItemConfig {
			ItemConfig { settings: ItemSettings::all_enabled() }
		}

		/// Converts a u64 to a balance.
		pub fn u64_to_balance_option(input: u64) -> Result<BalanceOf<T>, Error<T>> {
			input.try_into().map_err(|_| Error::<T>::ConversionError)
		}

		fn transfer_funds(
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let u32_amunt = TryInto::<u32>::try_into(amount).map_err(|_| Error::<T>::ConversionError)?;
			let origin: OriginFor<T> = RawOrigin::Signed(from).into();
			let account_lookup = <T::Lookup as StaticLookup>::unlookup(to);
			let asset_id: AssetId2<T> = 1.into();
			let token_amount = u32_amunt.into();
			Ok(pallet_assets::Pallet::<T, Instance1>::transfer(
				origin,
				asset_id.into().into(),
				account_lookup,
				token_amount,
			)
			.map_err(|_| Error::<T>::NotEnoughFunds)?)
		} 
		
	}
}
