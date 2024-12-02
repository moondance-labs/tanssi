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

#[macro_export]
macro_rules! assert_expected_events {
	( $chain:ident, vec![$( $event_pat:pat => { $($attr:ident : $condition:expr, )* }, )*] ) => {
		let mut message: Vec<String> = Vec::new();
		let mut events = <$chain as xcm_emulator::Chain>::events();

		$(
			let mut event_received = false;
			let mut meet_conditions = true;
			let mut index_match = 0;
			let mut event_message: Vec<String> = Vec::new();

			for (index, event) in events.iter().enumerate() {
				// Variable to record current event's meet conditions
                #[allow(unused_mut)] // To suppress warning in case no conditions are declared
				let mut current_event_meet_conditions = true;
				match event {
					$event_pat => {
						event_received = true;

                        #[allow(unused_mut)] // To suppress warning in case no conditions are declared
						let mut conditions_message: Vec<String> = Vec::new();

						$(
							// We only want to record condition error messages in case it did not happened before
							// Only the first partial match is recorded
							if !$condition && event_message.is_empty() {
								conditions_message.push(
									format!(
										" - The attribute {:?} = {:?} did not met the condition {:?}\n",
										stringify!($attr),
										$attr,
										stringify!($condition)
									)
								);
							}
							current_event_meet_conditions &= $condition;
						)*

						// Set the variable to latest matched event's condition evaluation result
						meet_conditions = current_event_meet_conditions;

						// Set the index where we found a perfect match
						if event_received && meet_conditions {
							index_match = index;
							break;
						} else {
							event_message.extend(conditions_message);
						}
					},
					_ => {}
				}
			}

			if event_received && !meet_conditions  {
				message.push(
					format!(
						"\n\n{}::\x1b[31m{}\x1b[0m was received but some of its attributes did not meet the conditions:\n{}",
						stringify!($chain),
						stringify!($event_pat),
						event_message.concat()
					)
				);
			} else if !event_received {
				message.push(
					format!(
						"\n\n{}::\x1b[31m{}\x1b[0m was never received. All events:\n{:#?}",
						stringify!($chain),
						stringify!($event_pat),
						<$chain as xcm_emulator::Chain>::events(),
					)
				);
			} else {
				// If we find a perfect match we remove the event to avoid being potentially assessed multiple times
				events.remove(index_match);
			}
		)*

		if !message.is_empty() {
			// Log events as they will not be logged after the panic
			<$chain as xcm_emulator::Chain>::events().iter().for_each(|event| {
				log::debug!(target: concat!("events::", stringify!($chain)), "{:?}", event);
			});
			panic!("{}", message.concat())
		}
	}
}
