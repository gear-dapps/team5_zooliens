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
