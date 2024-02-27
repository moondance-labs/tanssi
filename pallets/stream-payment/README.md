# Stream payment pallet

A pallet to create payment streams, where users can setup recurrent payment at some rate per unit of
time. The pallet aims to be configurable and usage agnostic:

- Runtime configures which assets are supported by providing an `AssetId` type and a type
  implementing the `Assets` trait which only requires function needed by the pallet (increase
  deposit when creating or refilling a stream, decrease deposit when closing a stream, and
  transferring a deposit when the stream payment is performed). Both types allows to easily add new
  supported assets in the future while being retro-compatible. The pallet make few assumptions about
  how the funds are deposited (thanks to the custom trait), which should allow to easily support
  assets from various pallets/sources.
- Runtime configure which unit of time is supported to express the rate of payment. Units of time
  should be monotonically increasing. Users can then choose which unit of time they want to use.

The pallet provides the following calls:
- `open_stream(target, time_unit, asset_id, rate, initial_deposit)`: The origin creates a stream
  towards a target (payee), with given time unit, asset and rate. A deposit is made, which is able
  to pay for `initial_deposit / rate`. Streams are indexed using a `StreamId` which is returned with
  an event.
- `perform_payment(stream_id)`: can be called by anyone to update a stream, performing the payment
  for the elapsed time since the last update. All other calls implicitly call `perform_payment`,
  such that at any point in time you're guaranteed you'll be able to redeem the payment for the
  elapsed time; which allow to call it only when the funds are needed without fear of non-payment.
- `close_stream(stream_id)`: only callable by the source or target of the stream. It pays for the
  elapsed time then refund the remaining deposit to the source.
- `immediately_change_deposit(stream_id, asset_id, change)`: Change the deposit in the stream. It
  first perform a payment before applying the change, which means a source will not retro-actively
  pay for a drained stream. A target that provides services in exchange for payment should suspend
  the service as soon as updating the stream would make it drain, and should resume services once
  the stream is refilled. The call takes an asset id which must match the config asset id, which
  prevents unwanted amounts when a change request that changes the asset is accepted.
- `request_change(stream_id, kind, new_config, deposit_change)`: Allows to request changing the
  config of the stream. `kind` states if the change is a mere suggestion or is mandatory, in which
  case there is a provided deadline at which point payments will no longer occur. Requests that
  don't change the time unit or asset id and change the rate at a disadvantage for the caller is
  applied immediately. An existing request can be overritten by both parties if it was a suggestion,
  while only by the previous requester if it was mandatory. A nonce is increased to prevent to
  prevent one to frontrunner the acceptation of a request with another request. The target of the
  stream cannot provide a deposit change, while the source can. It is however mandatory to provide
  change with absolute value when changing asset.
- `accept_requested_change(stream_id, request_nonce, deposit_change)`: Accept the change for this
  stream id and request nonce. If one want to refuse a change they can either leave it as is (which
  will do nothing if the request is a suggestion, or stop payment when reaching the deadline if
  mandatory) or close the stream with `close_stream`. The target of the stream cannot provide a
  deposit change, while the source can. It is however mandatory to provide change with absolute
  value when changing asset.
- `cancel_change_request(stream_id)`: Cancel a change request, only callable by the requester of a
  previous request.

For UIs the pallet provides the following storages:
- `Streams: StreamId => Stream`: stream data indexed by stream id.
- `LookupStreamsWithSource: AccountId => StreamId => ()`: allows to list allow the streams with a
  given source by iterating over all storage keys with the key prefix corresponding to the account.
- `LookupStreamsWithTarget: AccountId => StreamId => ()`: same but for the target. Those last 2
  storages are solely for UIs to list incoming and outgoing streams. Key prefix is used to reduce
  the POV cost that would require a single Vec of StreamId.