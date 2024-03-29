use elrond_wasm::{BoxedBytes, Vec};

derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PollInfo {
    pub question: BoxedBytes,
    pub choices: BoxedBytes,
    pub votes_distribution: Vec<u32>,
    pub deadline: u64,
    pub opt_vote_limit: Option<u32>
}
