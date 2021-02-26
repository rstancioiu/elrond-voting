use elrond_wasm::{BoxedBytes, Vec};

derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PollInfo {
    pub question: BoxedBytes,
    pub answers: Vec<BoxedBytes>,
    pub vote_distribution: Vec<u32>,
    pub deadline: u64,
    pub vote_limit: u32
}
