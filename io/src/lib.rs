#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = State;
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct State {
    pub monsters: Vec<(ActorId, Monster)>,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Action {
    Mint(Mint),
    ProfileInfo(ActorId),
    Create(Create),

    Claim(ActorId),

    Order(Item),
    AcceptOrder,

    ToChallenge(Option<ActorId>),
    LeaveChallenge(Account),
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct Mint {
    pub nft_contract_id: ActorId,
    pub private_key: Vec<u8>,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct Create {
    pub signature: Vec<u8>,
    pub signed_data: Vec<u8>,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Event {
    ProfileInfo(Monster),
    Claimed(u128),
    ReadyToChallenge,
    ChallengeResult,
    Created(ActorId),
    Minted(ActorId),
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct Item {
    pub price: u128,
    // pub nft: Nft,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct Characteristics {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub stamina: u32,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct Farm {
    pub increase_value: u128,
    pub previous_claim_date: u128,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct Account {
    pub id: ActorId,
    pub name: String,
    pub monsters: Vec<Monster>,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct Monster {
    pub nft_contract_id: ActorId,
    pub inventory: Vec<Item>,
    pub name: String,
    pub characteristics: Characteristics,
    pub energy: u32,
    pub level: u32,
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Error {
    NotFound(ActorId),
    WrongId,
    IllegalKey,
}
