use parity_codec_derive::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

// Copied types from https://github.com/Joystream/substrate-forum-module/releases/tag/v1.0.0
// This is the version of the forum used in the joystream Acropolis testnet release
// It would have made sense to add the cargo pacakge as a depedency, however
// there is build issue with multiple versions of schnorkel pacakge (which was fixed in a later release)
// see: https://github.com/paritytech/substrate/pull/3270

/// Convenient composite time stamp
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct BlockchainTimestamp<BlockNumber, Moment> {
    block: BlockNumber,
    time: Moment,
}

/// Represents a moderation outcome applied to a post or a thread.
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct ModerationAction<BlockNumber, Moment, AccountId> {
    /// When action occured.
    moderated_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Account forum sudo which acted.
    moderator_id: AccountId,

    /// Moderation rationale
    rationale: Vec<u8>,
}

/// Represents a revision of the text of a Post
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct PostTextChange<BlockNumber, Moment> {
    /// When this expiration occured
    expired_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Text that expired
    text: Vec<u8>,
}

/// Represents a post identifier
pub type PostId = u64;

/// Represents a thread post
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Post<BlockNumber, Moment, AccountId> {
    /// Post identifier
    id: PostId,

    /// Id of thread to which this post corresponds.
    thread_id: ThreadId,

    /// The post number of this post in its thread, i.e. total number of posts added (including this)
    /// to a thread when it was added.
    /// Is needed to give light clients assurance about getting all posts in a given range,
    // `created_at` is not sufficient.
    /// Starts at 1 for first post in thread.
    nr_in_thread: u32,

    /// Current text of post
    current_text: Vec<u8>,

    /// Possible moderation of this post
    moderation: Option<ModerationAction<BlockNumber, Moment, AccountId>>,

    /// Edits of post ordered chronologically by edit time.
    text_change_history: Vec<PostTextChange<BlockNumber, Moment>>,

    /// When post was submitted.
    created_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Author of post.
    author_id: AccountId,
}

/// Represents a thread identifier
pub type ThreadId = u64;

/// Represents a thread
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Thread<BlockNumber, Moment, AccountId> {
    /// Thread identifier
    id: ThreadId,

    /// Title
    title: Vec<u8>,

    /// Category in which this thread lives
    category_id: CategoryId,

    /// The thread number of this thread in its category, i.e. total number of thread added (including this)
    /// to a category when it was added.
    /// Is needed to give light clients assurance about getting all threads in a given range,
    /// `created_at` is not sufficient.
    /// Starts at 1 for first thread in category.
    nr_in_category: u32,

    /// Possible moderation of this thread
    moderation: Option<ModerationAction<BlockNumber, Moment, AccountId>>,

    /// Number of unmoderated and moderated posts in this thread.
    /// The sum of these two only increases, and former is incremented
    /// for each new post added to this thread. A new post is added
    /// with a `nr_in_thread` equal to this sum
    ///
    /// When there is a moderation
    /// of a post, the variables are incremented and decremented, respectively.
    ///
    /// These values are vital for light clients, in order to validate that they are
    /// not being censored from posts in a thread.
    num_unmoderated_posts: u32,
    num_moderated_posts: u32,

    /// When thread was established.
    created_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Author of post.
    author_id: AccountId,
}

/// Represents a category identifier
pub type CategoryId = u64;

/// Represents
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct ChildPositionInParentCategory {
    /// Id of parent category
    parent_id: CategoryId,

    /// Nr of the child in the parent
    /// Starts at 1
    child_nr_in_parent_category: u32,
}

/// Represents a category
#[derive(Serialize, Deserialize, Debug, Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct Category<BlockNumber, Moment, AccountId> {
    /// Category identifier
    id: CategoryId,

    /// Title
    title: Vec<u8>,

    /// Description
    description: Vec<u8>,

    /// When category was established.
    created_at: BlockchainTimestamp<BlockNumber, Moment>,

    /// Whether category is deleted.
    deleted: bool,

    /// Whether category is archived.
    archived: bool,

    /// Number of subcategories (deleted, archived or neither),
    /// unmoderated threads and moderated threads, _directly_ in this category.
    ///
    /// As noted, the first is unaffected by any change in state of direct subcategory.
    ///
    /// The sum of the latter two only increases, and former is incremented
    /// for each new thread added to this category. A new thread is added
    /// with a `nr_in_category` equal to this sum.
    ///
    /// When there is a moderation
    /// of a thread, the variables are incremented and decremented, respectively.
    ///
    /// These values are vital for light clients, in order to validate that they are
    /// not being censored from subcategories or threads in a category.
    num_direct_subcategories: u32,
    num_direct_unmoderated_threads: u32,
    num_direct_moderated_threads: u32,

    /// Position as child in parent, if present, otherwise this category is a root category
    position_in_parent_category: Option<ChildPositionInParentCategory>,

    /// Account of the moderator which created category.
    moderator_id: AccountId,
}
