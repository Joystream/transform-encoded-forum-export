mod forum_storage_types;
use forum_storage_types::*;

use parity_codec::Decode as Decoder;
// use parity_codec_derive::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

// use std::fs;
use std::io::{Write}; //, Read};

use primitives::{traits::Verify, AnySignature};

// Relevant Acropolis testnet runtime configuration
/// An index to a block.
pub type BlockNumber = u64; // !!! not u32 as in Rome substrate v2.0 !!!
/// Moment type
pub type Moment = u64;
/// Alias to pubkey that identifies an account on the chain.
pub type AccountId = <AccountSignature as Verify>::Signer;
/// The type used by accounts to prove their ID.
pub type AccountSignature = AnySignature;

// AccountId - 32 bytes public key
// We could can use this type instead of adding dependency on substrate sr-primitives pacakge
// However it is not serialized in the same way!
// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Serialize, Deserialize, Default)]
// pub struct AccountId(pub [u8; 32]);

fn main() {
    // let data = fs::read_to_string("forum_data_encoded.json").expect("failed reading file");
    // let mut data = Vec::new();
    // std::io::stdin().read_to_end(&mut data).unwrap();

    let encoded: EncodedForumData =
        serde_json::from_reader(std::io::stdin()).expect("failed parsing json");

    let posts = encoded
        .posts
        .into_iter()
        .map(|post| (post.0, decode_post(post.1).unwrap()))
        .collect();

    let categories = encoded
        .categories
        .into_iter()
        .map(|category| (category.0, decode_category(category.1).unwrap()))
        .collect();

    let threads = encoded
        .threads
        .into_iter()
        .map(|thread| (thread.0, decode_thread(thread.1).unwrap()))
        .collect();

    let serialized = SerializedForumData {
        categories,
        posts,
        threads,
    };

    let output = serde_json::to_string(&serialized).expect("failed to stringify");

    let mut handle = std::io::stdout();
    handle
        .write_all(output.as_bytes())
        .expect("failed to write to stdout");
}

#[derive(Serialize)]
struct SerializedForumData {
    categories: Vec<(CategoryId, Category<BlockNumber, Moment, AccountId>)>,
    posts: Vec<(PostId, Post<BlockNumber, Moment, AccountId>)>,
    threads: Vec<(ThreadId, Thread<BlockNumber, Moment, AccountId>)>,
}

#[derive(Deserialize)]
struct EncodedForumData {
    /// hex encoded categories
    categories: Vec<(CategoryId, String)>,
    /// hex encoded posts
    posts: Vec<(PostId, String)>,
    /// hex encoded threads
    threads: Vec<(ThreadId, String)>,
}

fn decode_post(encoded: String) -> Option<Post<BlockNumber, Moment, AccountId>> {
    // hex string must not include '0x' prefix!
    let encoded = hex::decode(encoded.as_bytes()).expect("failed to parse hex string");
    Decoder::decode(&mut encoded.as_slice())
}

fn decode_category(encoded: String) -> Option<Category<BlockNumber, Moment, AccountId>> {
    // hex string must not include '0x' prefix!
    let encoded = hex::decode(encoded.as_bytes()).expect("failed to parse hex string");
    Decoder::decode(&mut encoded.as_slice())
}

fn decode_thread(encoded: String) -> Option<Thread<BlockNumber, Moment, AccountId>> {
    // hex string must not include '0x' prefix!
    let encoded = hex::decode(encoded.as_bytes()).expect("failed to parse hex string");
    Decoder::decode(&mut encoded.as_slice())
}
