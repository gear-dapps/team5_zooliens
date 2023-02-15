use app_io::*;
use ed25519_compact::{SecretKey, Signature};
use gmeta::Metadata;
use gstd::{errors::Result as GstdResult, msg, prelude::*, util, ActorId, MessageId};
use hashbrown::HashMap;
use nft_io::NFTAction;
use primitive_types::U256;

type EncodedNft = Vec<u8>;

#[derive(Clone, Debug, Default)]
pub struct Contract {
    verification_data: HashMap<ActorId, (Signature, EncodedNft)>,
    accounts: HashMap<ActorId, Monster>,
    arena: HashMap<ActorId, Option<ActorId>>,
    // leader_board: HashMap<ActorId, u128>,
    // farms: HashMap<ActorId, Farm>,
    // market: HashMap<ActorId, Order>, // nft-marketplace
}

static mut STATE: Option<Contract> = None;

fn static_mut_state() -> &'static mut Contract {
    match unsafe { &mut STATE } {
        Some(state) => state,
        None => unreachable!("State can't be uninitialized"),
    }
}

#[no_mangle]
extern "C" fn init() {
    unsafe { STATE = Some(Default::default()) }
}

#[gstd::async_main]
async fn main() {
    let payload: Action = msg::load().unwrap();

    let contract = static_mut_state();
    let result = match payload {
        Action::Create(Create {
            signature,
            signed_data,
        }) => contract.create(&signature, signed_data),
        Action::Mint(Mint {
            nft_contract_id,
            private_key,
        }) => contract.mint(nft_contract_id, &private_key).await,
        Action::ProfileInfo(actor_id) => contract.profile_info(&actor_id),
        Action::ToChallenge(enemy) => contract.make_challenge(&enemy),

        Action::Claim(actor) => contract.claim(&actor),
        Action::Order(_item) => todo!(),
        Action::AcceptOrder => todo!(),
        Action::LeaveChallenge(_) => todo!(),
    };
    gstd::debug!("result {:?}", result);
    match result {
        Ok(event) => msg::reply(event, 0).expect("Can't send reply Event"),
        Err(error) => msg::reply(error, 0).expect("Can't send reply Error"),
    };
}

fn common_state() -> <ContractMetadata as Metadata>::State {
    State {}
}

#[no_mangle]
extern "C" fn meta_state() -> *const [i32; 2] {
    let state = common_state();

    util::to_leak_ptr(state.encode())
}

#[no_mangle]
extern "C" fn state() {
    reply(common_state()).expect(
        "Failed to encode or reply with `<AuctionMetadata as Metadata>::State` from `state()`",
    );
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    reply(metahash).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

impl Contract {
    pub async fn mint(&mut self, id: ActorId, private_key: &[u8]) -> Result<Event, Error> {
        let secret_key = SecretKey::from_slice(private_key).map_err(|_| Error::IllegalKey)?;

        match self.verification_data.get(&id) {
            Some((signature, data)) => {
                secret_key
                    .public_key()
                    .verify(data, &signature)
                    .map_err(|_| Error::IllegalKey)?;

                let player_id = msg::source();

                let nft_contract_id =
                    ActorId::from_slice(data).expect("Can't create NFT-contract id from slice");

                let monster = Monster {
                    nft_contract_id,
                    inventory: Default::default(),
                    name: Default::default(),
                    characteristics: Default::default(),
                    energy: 100,
                    level: 0,
                };

                msg::send_for_reply(
                    nft_contract_id.clone(),
                    NFTAction::Transfer {
                        to: player_id,
                        token_id: U256::from(0),
                        transaction_id: 0,
                    },
                    0,
                )
                .unwrap()
                .await
                .expect("Error in nft transfer");

                self.accounts.insert(player_id, monster);
                Ok(Event::Minted(nft_contract_id))
            }
            None => return Err(Error::IllegalKey),
        }
    }

    pub fn create(
        &mut self,
        signature: &[u8],
        signed_nft_program_id: Vec<u8>,
    ) -> Result<Event, Error> {
        gstd::debug!("create() {:?}, {:?}", signature, signed_nft_program_id);
        let signature = Signature::from_slice(signature).map_err(|_| Error::IllegalKey)?;
        // let id = self.verification_data.len() as u64;
        let id = ActorId::from_slice(&signed_nft_program_id).unwrap();
        self.verification_data
            .insert(id, (signature, signed_nft_program_id));

        Ok(Event::Created(id))
    }

    pub fn profile_info(&self, id: &ActorId) -> Result<Event, Error> {
        match self.accounts.get(id) {
            Some(profile) => Ok(Event::ProfileInfo(profile.clone())),
            None => Err(Error::NotFound(*id)),
        }
    }

    pub fn claim(&mut self, _actor: &ActorId) -> Result<Event, Error> {
        Ok(Event::Claimed(999))
    }

    pub fn make_challenge(&mut self, enemy: &Option<ActorId>) -> Result<Event, Error> {
        match enemy {
            Some(enemy) => self.arena.insert(msg::source(), Some(enemy.clone())),
            None => self.arena.insert(msg::source(), None),
        };
        Ok(Event::ReadyToChallenge)
    }
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}
