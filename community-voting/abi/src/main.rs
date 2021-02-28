use elrond_wasm_debug::*;
use community_voting::*;

fn main() {
	let contract = CommunityVotingImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(& contract));
}
