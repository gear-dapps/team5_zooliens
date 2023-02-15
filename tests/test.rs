use app_io::{Action, Create, Error, Event, Mint};
use ed25519_compact::{KeyPair, Seed};
use gtest::{Log, Program, RunResult, System};
use nft_io::*;

use gear_lib::non_fungible_token::token::{TokenId, TokenMetadata};

const SEED: [u8; 32] = [5u8; 32];

const DATA: [u8; 32] = [9u8; 32];

const USERS: [u64; 6] = [10, 11, 12, 13, 14, 15];

const NFT_PATH: &str = "target/nft-0.2.5.opt.wasm";

#[test]
fn create() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let result = program.send_bytes(USERS[0], []);

    assert!(!result.main_failed());

    for (i, user) in USERS.iter().enumerate() {
        init_nft(&system, *user, format!("Zoomlien {}", *user));
        let key_pair = KeyPair::from_seed(Seed::new(SEED));
        let secret_key = key_pair.sk;

        let signature = secret_key.sign(&DATA, None);

        let payload = Create {
            signature: signature.to_vec(),
            signed_data: DATA.to_vec(),
        };

        let action = Action::Create(payload);

        let result = program.send(*user, action);

        assert!(result.contains(&Log::builder().payload(Event::Created(i as u64))));
    }

    // assert!(result.log().is_empty());
}

#[test]
fn mint() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let result = program.send_bytes(USERS[0], []);

    assert!(!result.main_failed());

    for (i, user) in USERS.iter().enumerate() {
        init_nft(&system, *user, format!("Zoomlien {}", *user));
        let key_pair = KeyPair::from_seed(Seed::new(SEED));
        let secret_key = key_pair.sk;

        let signature = secret_key.sign(&DATA, None);

        let payload = Create {
            signature: signature.to_vec(),
            signed_data: DATA.to_vec(),
        };

        let action = Action::Create(payload);

        let result = program.send(*user, action);

        assert!(result.contains(&Log::builder().payload(Event::Created(i as u64))));

        let result = program.send(
            USERS[1],
            Action::Mint(Mint {
                id: i as u64,
                private_key: secret_key.to_vec(),
            }),
        );

        assert!(result.contains(&Log::builder().payload(Event::Minted)));
    }
}

pub fn init_nft(sys: &System, owner: u64, name: String) {
    let nft_program = Program::from_file(sys, "./target/nft-0.2.5.opt.wasm");

    let res = nft_program.send(
        owner,
        nft_io::InitNFT {
            name,
            symbol: String::from("ZML"),
            base_uri: String::from(""), // link to image
            royalties: None,
        },
    );

    assert!(!res.main_failed());

    let res = nft_program.send(
        owner,
        nft_io::NFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "MyNFT".to_string(),
                description: "NFTForAuction".to_string(),
                media: "".to_string(),
                reference: "".to_string(),
            },
            transaction_id: 0u64,
        },
    );

    assert!(!res.main_failed());

    let res = nft_owner(&nft_program, owner, 0.into());
    let log = Log::builder().dest(owner).payload(nft_io::NFTEvent::Owner {
        owner: owner.into(),
        token_id: 0.into(),
    });
    assert!(res.contains(&log));

    let res = nft_program.send(
        owner,
        nft_io::NFTAction::Approve {
            to: 1.into(),
            token_id: 0.into(),
            transaction_id: 0u64,
        },
    );

    assert!(!res.main_failed());
}

#[allow(dead_code)]
pub fn nft_owner(nft_program: &Program, from: u64, token_id: TokenId) -> RunResult {
    nft_program.send(from, nft_io::NFTAction::Owner { token_id })
}

pub fn init(sys: &System) -> Program {
    USERS
        .iter()
        .for_each(|user| sys.mint_to(*user, 1_000_000_000));
    let owner_user = USERS[0];

    sys.init_logger();

    let auction_program = Program::current(sys);

    auction_program.send(owner_user, ());

    init_nft(sys, owner_user, "her".to_string());

    auction_program
}
