extern crate community_voting;
use community_voting::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/community-voting.wasm",
		Box::new(|context| Box::new(CommunityVotingImpl::new(context))),
	);
	contract_map
}

#[test]
fn community_voting_init() {
	parse_execute_mandos("mandos/community-voting-init.scen.json", &contract_map());
}
