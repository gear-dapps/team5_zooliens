use app_io::*;
use ed25519_compact::{SecretKey, Signature};
use gmeta::Metadata;
use gstd::{
    errors::{ContractError, Result as GstdResult},
    msg,
    prelude::*,
    ActorId, MessageId,
};
use hashbrown::HashMap;

type EncodedNft = Vec<u8>;

#[derive(Clone, Debug, Default)]
pub struct Contract {
    verification_data: HashMap<MonsterId, (Signature, EncodedNft)>,
    accounts: HashMap<ActorId, Monster>,
    farms: HashMap<ActorId, Farm>,
    arena: HashMap<ActorId, Option<ActorId>>,
    leader_board: HashMap<ActorId, u128>,
    monster_registry: HashMap<MonsterId, Monster>,
    monster_owner: HashMap<MonsterId, Option<ActorId>>,
    // market: HashMap<ActorId, Order>, // nft-marketplace
    order_registry: HashMap<OrderId, Order>,
    // monster, buyer, order
    // a monster can receive orders created by multiple buyers
    order_by_monster: HashMap<MonsterId, HashMap<ActorId, OrderId>>,
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
        Action::Mint(Mint { id, private_key }) => contract.mint(id, &private_key),
        Action::ProfileInfo(actor_id) => contract.profile_info(&actor_id),
        Action::ToChallenge(enemy) => contract.make_challenge(&enemy),

        Action::Claim(actor) => contract.claim(&actor),
        Action::Order(item) => todo!(),
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
    // let query = msg::load().expect("Failed to load or decode `StateQuery` from `meta_state()`");
    // let state = common_state();

    // util::to_leak_ptr(reply.encode())
    &[0, 0]
}

#[no_mangle]
extern "C" fn state() {}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
}

impl Contract {
    pub fn mint(&mut self, id: MonsterId, private_key: &[u8]) -> Result<Event, Error> {
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
                // self.accounts.insert(player_id, v)
                self.monster_registry.insert(id, monster);
                // Ok(Event::Minted(nft_contract_id))
                self.transfer_monster_to(id, &player_id);
                Ok(Event::Minted(id))
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
        let id: MonsterId = self.verification_data.len() as u64 as MonsterId;
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

    pub fn claim(&mut self, actor: &ActorId) -> Result<Event, Error> {
        Ok(Event::Claimed(999))
    }

    pub fn order(&mut self, id: MonsterId, price: u128) {
        let player_id = msg::source();
        let order = Order {
            monster_id: id,
            price: price,
            buyer: player_id,
        };
        // register order
        let order_id = self.order_registry.len() as u32;
        self.order_registry.insert(order_id, order);

        // TODO: insert order into order_by_monster

        // self.order_by_monster.insert(...);
    }

    pub fn accept_order(&mut self, id: OrderId) {
        let order = self.order_registry.get(&id);
        let order = order.cloned().unwrap();

        let current_owner = self.monster_owner.get(&order.monster_id);
        let current_owner = current_owner.unwrap().unwrap();

        // ensure player is current owner
        let player_id = msg::source();
        if player_id != current_owner {
            panic!("only current owner can accept offer")
        }

        // transfer ownership of monster
        self.transfer_monster_to(order.monster_id, &order.buyer);

        // TODO: transfer balance from buyer to seller

        // TODO: remove order from order_registry and order_by_owner

        // TODO: emit event
    }

    fn transfer_monster_to(&mut self, monster: MonsterId, to: &ActorId) -> () {
        self.monster_owner.insert(monster, Some(*to));
    }

    pub fn make_challenge(&mut self, enemy: &Option<ActorId>) -> Result<Event, Error> {
        match enemy {
            Some(enemy) => self.arena.insert(msg::source(), Some(enemy.clone())),
            None => self.arena.insert(msg::source(), None),
        };
        Ok(Event::ReadyToChallenge)
    }

    pub fn leader_board(&self) {}
}
