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

use {
    snowbridge_core::{AgentId, ChannelId, ParaId, TokenId},
    xcm::latest::prelude::*,
};
/// Helper trait to set up token and channel characteristics
pub trait TokenChannelSetterBenchmarkHelperTrait {
    /// Set up the token and location info
    fn set_up_token(_location: Location, _token_id: TokenId) {}
    /// Set up channel info
    fn set_up_channel(_channel_id: ChannelId, _para_id: ParaId, _agent_id: AgentId) {}
}
impl TokenChannelSetterBenchmarkHelperTrait for () {}
