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

#[derive(Clone, Debug, Default)]
pub struct Contract {
    verification_data: HashMap<u64, (Signature, Vec<u8>)>,
    accounts: HashMap<ActorId, Account>,
    farms: HashMap<ActorId, Farm>,
    arena: HashMap<ActorId, Option<ActorId>>,
    leader_board: HashMap<ActorId, u128>,
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
    pub fn mint(&mut self, id: u64, private_key: &[u8]) -> Result<Event, Error> {
        let secret_key = SecretKey::from_slice(private_key).map_err(|_| Error::IllegalKey)?;

        match self.verification_data.get(&id) {
            Some((signature, data)) => secret_key
                .public_key()
                .verify(data, &signature)
                .map_err(|_| Error::IllegalKey)?,
            None => return Err(Error::IllegalKey),
        };

        Ok(Event::Minted)
    }

    pub fn create(&mut self, signature: &[u8], signed_data: Vec<u8>) -> Result<Event, Error> {
        gstd::debug!("create() {:?}, {:?}", signature, signed_data);
        let signature = Signature::from_slice(signature).map_err(|_| Error::IllegalKey)?;
        let id = self.verification_data.len() as u64;
        self.verification_data.insert(id, (signature, signed_data));

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

    pub fn order(&mut self) {}

    pub fn accept_order(&mut self) {}

    pub fn make_challenge(&mut self, enemy: &Option<ActorId>) -> Result<Event, Error> {
        match enemy {
            Some(enemy) => self.arena.insert(msg::source(), Some(enemy.clone())),
            None => self.arena.insert(msg::source(), None),
        };
        Ok(Event::ReadyToChallenge)
    }

    pub fn leader_board(&self) {}
}
