use app_io::{Action, Create, Error, Event, Mint};
use ed25519_compact::{KeyPair, Seed};
use gtest::{Log, Program, System};

const SEED: [u8; 32] = [5u8; 32];

const DATA: [u8; 32] = [9u8; 32];

const USER: [u64; 6] = [4, 5, 6, 7, 8, 9];

#[test]
fn create() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let mut result = program.send_bytes(2, []);

    assert!(!result.main_failed());

    let key_pair = KeyPair::from_seed(Seed::new(SEED));

    let public_key = key_pair.pk;
    let secret_key = key_pair.sk;

    let signature = secret_key.sign(&DATA, None);

    let payload = Create {
        signature: signature.to_vec(),
        signed_data: DATA.to_vec(),
    };

    let action = Action::Create(payload);

    let result = program.send(0, action);

    // assert!(result.contains(&Log::builder().payload( Event::Created(0))));

    assert!(result.contains(&Log::builder().payload(Event::Created(0))));
    // assert!(result.log().is_empty());
}

#[test]
fn mint() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let mut result = program.send_bytes(USER[0], []);

    assert!(!result.main_failed());

    let key_pair = KeyPair::from_seed(Seed::new(SEED));

    let public_key = key_pair.pk;
    let secret_key = key_pair.sk;

    let signature = secret_key.sign(&DATA, None);

    let payload = Create {
        signature: signature.to_vec(),
        signed_data: DATA.to_vec(),
    };

    let action = Action::Create(payload);

    let result = program.send(USER[1], action);
    assert!(result.contains(&Log::builder().payload(Event::Created(0))));

    let result = program.send(
        USER[1],
        Action::Mint(Mint {
            id: 0,
            private_key: secret_key.to_vec(),
        }),
    );

    assert!(result.contains(&Log::builder().payload(Event::Minted)));
}

pub fn init_nft(sys: &System, owner: u64) {
    let nft_program = Program::from_file(sys, "./target/nft-0.2.5.opt.wasm");

    let res = nft_program.send(
        owner,
        nft_io::InitNFT {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            base_uri: String::from(""),
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