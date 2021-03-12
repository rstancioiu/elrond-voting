use elrond_wasm_debug::*;
use orchestrator::*;

fn main() {
	let contract = OrchestratorImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(& contract));
}
