use crate::abi::{self};
use crate::pb::idea::v1::{IdeaCreated, IdeaCreatedEvents};
use abi::updraft::events::IdeaCreated as IdeaCreated_abi;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto};

use crate::pb::contract::v1 as contract;
use crate::pb::contract::v1::Events;


use crate::pb::ideacontribution::v1::{IdeaContribution, IdeaContributionEvents};
use abi::idea::events::Contributed as IdeaContribution_abi;


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
                created_at_block: block.number,
            });
        }
    }

    Ok(IdeaCreatedEvents { events })
}

// for the emited Idea Addresses that happen on an Idea-Created Event
#[substreams::handlers::store]
fn store_idea_contracts_deployed(ideas: IdeaCreatedEvents, store: StoreSetProto<IdeaCreated>) {
    for idea in ideas.events {
        let idea_address = &idea.idea;
        store.set(idea.created_at_block, format!("idea:{idea_address}"), &idea);
    }
}

#[substreams::handlers::map]
fn map_idea_events(block: Block, ideas_store: StoreGetProto<IdeaCreated>) -> Result<IdeaContributionEvents, substreams::errors::Error> {
    let mut events: Vec<IdeaContribution> = vec![]; 

    for log in block.logs() {
        if let Some(event) = IdeaContribution_abi::match_and_decode(log.log) {
            events.push(IdeaContribution {
                addr: Hex::encode(event.addr),
                position_index: event.position_index.to_string(),
                amount: event.amount.to_string(),
                total_shares: event.total_shares.to_string(),
                total_tokens: event.total_tokens.to_string(),
            });
        }
    }

    Ok(IdeaContributionEvents { events })
}

// #[substreams::handlers::map]
// fn map_idea_events(block: Block, ideas_store: StoreGetProto<IdeaCreated>) -> Result<Events, substreams::errors::Error> {
//     let mut events = Events::default();

//     for trx in block.transactions() {
//         for (log, call_view) in trx.logs_with_calls() {
//             let idea_address = &Hex(&log.address).to_string();
//             if store.has(format!("idea:{idea_address}")) {
//                 // decode event from the child contract
//                 if log.matches_signature("Contributed(address indexed addr,uint256 positionIndex,uint256 amount,uint256 totalShares,uint256 totalTokens)") {
//                     let val = log.param(0).unwrap().to_u256();
//                     // handle event
//                 }
//             }
//         }
//     }

//     Ok(events)
// }

