// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! Crate containing various traits used by moondance crates allowing to connect pallet
//! with each other or with mocks.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use {
    core::marker::PhantomData,
    frame_support::{
        traits::{BatchesFootprints, EnqueueMessage, QueueFootprint, QueueFootprintQuery},
        BoundedSlice,
    },
    pallet_message_queue::OnQueueChanged,
    sp_core::MaxEncodedLen,
};

/// Means of converting an origin to a different one before calling an EnqueueMessage impl
/// Useful for certain cases like pallet parachains inclusion
/// This pallet assumes single-aggregate origin which is not the case in our runtime
pub struct MessageQueueWrapper<Origin, NewOrigin, InnerMessageQueue>(
    PhantomData<(Origin, NewOrigin, InnerMessageQueue)>,
);

impl<Origin, NewOrigin, InnerMessageQueue> EnqueueMessage<Origin>
    for MessageQueueWrapper<Origin, NewOrigin, InnerMessageQueue>
where
    Origin: Clone + MaxEncodedLen,
    NewOrigin: From<Origin> + Clone + MaxEncodedLen,
    InnerMessageQueue: EnqueueMessage<NewOrigin>,
{
    type MaxMessageLen = <InnerMessageQueue as EnqueueMessage<NewOrigin>>::MaxMessageLen;

    fn enqueue_message(message: BoundedSlice<u8, Self::MaxMessageLen>, origin: Origin) {
        let new_origin: NewOrigin = origin.into();
        InnerMessageQueue::enqueue_message(message, new_origin)
    }

    fn enqueue_messages<'a>(
        messages: impl Iterator<Item = BoundedSlice<'a, u8, Self::MaxMessageLen>>,
        origin: Origin,
    ) {
        let new_origin: NewOrigin = origin.into();
        InnerMessageQueue::enqueue_messages(messages, new_origin)
    }

    fn sweep_queue(origin: Origin) {
        let new_origin: NewOrigin = origin.into();
        InnerMessageQueue::sweep_queue(new_origin)
    }
}

/// Means of converting an origin to a different one before calling a QueueFootprintQuery impl
/// Useful for certain cases like pallet parachains inclusion
/// This pallet assumes single-aggregate origin which is not the case in our runtime
impl<Origin, NewOrigin, InnerMessageQueue> QueueFootprintQuery<Origin>
    for MessageQueueWrapper<Origin, NewOrigin, InnerMessageQueue>
where
    Origin: Clone + MaxEncodedLen,
    NewOrigin: From<Origin> + Clone + MaxEncodedLen,
    InnerMessageQueue: QueueFootprintQuery<NewOrigin>,
{
    type MaxMessageLen = <InnerMessageQueue as QueueFootprintQuery<NewOrigin>>::MaxMessageLen;
    fn get_batches_footprints<'a>(
        origin: Origin,
        msgs: impl Iterator<Item = BoundedSlice<'a, u8, Self::MaxMessageLen>>,
        total_pages_limit: u32,
    ) -> BatchesFootprints {
        let new_origin: NewOrigin = origin.into();
        InnerMessageQueue::get_batches_footprints(new_origin, msgs, total_pages_limit)
    }

    fn footprint(origin: Origin) -> QueueFootprint {
        let new_origin: NewOrigin = origin.into();
        InnerMessageQueue::footprint(new_origin)
    }
}

/// Means of converting an origin to a different one before calling an OnQueueChanged impl
/// Useful for certain cases like pallet parachains inclusion
/// This pallet assumes single-aggregate origin which is not the case in our runtime
pub struct OnQueueChangedWrapper<Origin, NewOrigin, InnerOnQueueChanged>(
    PhantomData<(Origin, NewOrigin, InnerOnQueueChanged)>,
);

// This one we need to not do anything in case we
impl<Origin, NewOrigin, InnerOnQueueChanged> OnQueueChanged<Origin>
    for OnQueueChangedWrapper<Origin, NewOrigin, InnerOnQueueChanged>
where
    Origin: Clone + MaxEncodedLen + TryInto<NewOrigin>,
    NewOrigin: Clone + MaxEncodedLen,
    InnerOnQueueChanged: OnQueueChanged<NewOrigin>,
{
    fn on_queue_changed(origin: Origin, fp: QueueFootprint) {
        // Do not do anyhting if the result is not OK
        let new_origin = origin.try_into();
        match new_origin {
            Ok(new_origin) => InnerOnQueueChanged::on_queue_changed(new_origin, fp),
            _ => {}
        }
    }
}
