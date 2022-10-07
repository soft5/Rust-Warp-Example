use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

// Can be used to make identity_id also.
pub fn alphanumeric_key(len: usize) -> String {
    let alphanumeric_vec = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<Vec<u8>>();
    String::from_utf8(alphanumeric_vec).unwrap()
}
