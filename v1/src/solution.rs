use crate::abi::{self};
use crate::pb::solution::v1::{SolutionCreated, SolutionCreatedEvents};
use abi::updraft::events::SolutionCreated as SolutionCreated_abi;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto};

use crate::pb::solutioncontribution::v1::{SolutionContribution, SolutionContributionEvents};
use abi::solution::events::Contributed as SolutionContribution_abi;


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
                created_at_block: block.number,
            });
        }
    }

    Ok(SolutionCreatedEvents { events })
}

// for the emited Solution Addresses that happen on an Solution-Created Event
#[substreams::handlers::store]
fn store_solution_contracts_deployed(solutions: SolutionCreatedEvents, store: StoreSetProto<SolutionCreated>) {
    for solution in solutions.events {
        let solution_address = &solution.solution;
        store.set(solution.created_at_block, format!("solution:{solution_address}"), &solution);
    }
}

#[substreams::handlers::map]
fn map_solution_events(block: Block, solutions_store: StoreGetProto<SolutionCreated>) -> Result<SolutionContributionEvents, substreams::errors::Error> {
    let mut events: Vec<SolutionContribution> = vec![]; 

    for log in block.logs() {
        if let Some(event) = SolutionContribution_abi::match_and_decode(log.log) {
            events.push(SolutionContribution {
                addr: Hex::encode(event.addr),
                position_index: event.position_index.to_string(),
                amount: event.amount.to_string(),
                total_shares: event.total_shares.to_string(),
                total_tokens: event.total_tokens.to_string(),
            });
        }
    }

    Ok(SolutionContributionEvents { events })
}