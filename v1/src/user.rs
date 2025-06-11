use crate::abi::{self};
use crate::pb::user::v1::{ProfileUpdated, ProfileUpdatedEvents};
use abi::updraft::events::ProfileUpdated as ProfileUpdated_abi;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;


#[substreams::handlers::map]
fn map_user_updated(block: Block) -> Result<ProfileUpdatedEvents, substreams::errors::Error> {
    let mut events = vec![];

    for log in block.logs() {
        if let Some(event) = ProfileUpdated_abi::match_and_decode(log.log) {
            events.push(ProfileUpdated {
                owner: Hex::encode(event.owner),
                data: event.data,
            });
        }
    }

    Ok(ProfileUpdatedEvents { events })
}