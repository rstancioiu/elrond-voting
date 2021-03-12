extern crate orchestrator;
use orchestrator::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/orchestrator.wasm",
		Box::new(|context| Box::new(OrchestratorgImpl::new(context))),
	);
	contract_map
}

#[test]
fn orchestrator_init() {
	parse_execute_mandos("mandos/orchestratorg-init.scen.json", &contract_map());
}
