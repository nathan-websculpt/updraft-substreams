use crate::abi::{self};
use crate::pb::idea::v1::{IdeaCreated, IdeaCreatedEvents};
use abi::updraft::events::IdeaCreated as IdeaCreated_abi;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;


#[substreams::handlers::map]
fn map_idea_created(block: Block) -> Result<IdeaCreatedEvents, substreams::errors::Error> {
    let mut events = vec![];

    for log in block.logs() {
        if let Some(event) = IdeaCreated_abi::match_and_decode(log.log) {
            events.push(IdeaCreated {
                idea: Hex::encode(event.idea),
                creator: Hex::encode(event.creator),
                contributor_fee: event.contributor_fee.to_string(),
                contribution: event.contribution.to_string(),
                data: event.data,
            });
        }
    }

    Ok(IdeaCreatedEvents { events })
}