use crate::abi::{self};
use crate::pb::solution::v1::{SolutionCreated, SolutionCreatedEvents};
use abi::updraft::events::SolutionCreated as SolutionCreated_abi;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;


#[substreams::handlers::map]
fn map_solution_created(block: Block) -> Result<SolutionCreatedEvents, substreams::errors::Error> {
    let mut events = vec![];

    for log in block.logs() {
        if let Some(event) = SolutionCreated_abi::match_and_decode(log.log) {
            events.push(SolutionCreated {
                solution: Hex::encode(event.solution),
                creator: Hex::encode(event.creator),
                idea: Hex::encode(event.idea),
                funding_token: Hex::encode(event.funding_token),             // IERC20
                stake: event.stake.to_string(),
                goal: event.goal.to_string(),
                deadline: event.deadline.to_string(),
                contributor_fee: event.contributor_fee.to_string(),
                data: event.data,
            });
        }
    }

    Ok(SolutionCreatedEvents { events })
}