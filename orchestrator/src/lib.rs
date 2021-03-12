#![no_std]

elrond_wasm::imports!();

use elrond_wasm::HexCallDataSerializer;

#[elrond_wasm_derive::contract(OrchestratorImpl)]
pub trait Orchestrator {
    #[init]
    fn init(& self, #[var_args] admins: VarArgs<Address>) {
        self.set_admins(& admins.into_vec());
    }

    // Endpoint only available to admins
    #[endpoint]
    fn add_address(& self, _address: Address) -> SCResult<()> {
        Ok(())
    }

    // Endpoint only available to admins
    #[endpoint]
    fn archive_address(& self, _address: Address) -> SCResult<()> {
        Ok(())
    }

    // Endpoint only available to admins
    #[endpoint]
    fn remove_address(& self, _address: Address) -> SCResult<()> {
        Ok(())
    }

    #[endpoint]
    fn execute_call(& self,
        function: BoxedBytes,
        #[var_args] arguments: VarArgs<BoxedBytes>
    ) -> SCResult<()>  {
        // TODO: Implement a choice algorithm.
        let address: Address = self.get_contract(0);
        // TODO: Use latest version, this implementation is no longer valid in elrond-wasm master version.
        let mut call_data = HexCallDataSerializer::new(function.as_slice());
        for arg in arguments.into_vec() {
            call_data.push_argument_bytes(arg.as_slice());
        }
        Ok(self.async_call(& address, & BigUint::zero(), call_data.as_slice()))
    }

    #[storage_set("admins")]
    fn set_admins(& self, admins: & Vec<Address>);

    #[storage_get("admins")]
    fn get_admins(& self) -> Vec<Address>;

    #[storage_set("contract")]
    fn set_contract(& self, contract_id: u8, contract: Address);

    #[storage_get("contract")]
    fn get_contract(& self, contract_id: u8) -> Address;

    #[storage_is_empty("contract")]
    fn is_contract_empty(& self, contract_id: u8) -> bool;
}
