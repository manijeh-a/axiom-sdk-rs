use crate::scaffold::AxiomCircuitScaffold;
use crate::subquery::caller::SubqueryCaller;
use crate::tests::utils::account_call;
use crate::tests::utils::MyCircuitInput;
use crate::tests::utils::MyCircuitVirtualInput;
use crate::types::AxiomCircuitParams;

use axiom_codec::HiLo;

use super::utils::header_call;
use crate::tests::shared_tests::{mock_test, single_instance_test};
use axiom_eth::{
    halo2_base::{
        gates::{circuit::BaseCircuitParams, RangeChip},
        AssignedValue,
    },
    halo2curves::bn256::Fr,
    rlc::circuit::builder::RlcCircuitBuilder,
};
use ethers::providers::{Http, JsonRpcClient};
use std::sync::{Arc, Mutex};
use test_case::test_case;

macro_rules! base_test_struct {
    ($struct_name:ident, $subquery_call:ident) => {
        #[derive(Debug, Clone, Default)]
        struct $struct_name;
        impl<P: JsonRpcClient> AxiomCircuitScaffold<P, Fr> for $struct_name {
            type CircuitInput = MyCircuitInput;
            type VirtualCircuitInput = MyCircuitVirtualInput<Fr>;

            fn virtual_assign_phase0(
                &self,
                builder: &mut RlcCircuitBuilder<Fr>,
                _range: &RangeChip<Fr>,
                subquery_caller: Arc<Mutex<SubqueryCaller<P, Fr>>>,
                _callback: &mut Vec<HiLo<AssignedValue<Fr>>>,
                _inputs: Self::VirtualCircuitInput,
            ) {
                $subquery_call(builder, subquery_caller);
            }
        }
    };
}

fn get_base_test_params() -> AxiomCircuitParams {
    let params = BaseCircuitParams {
        k: 12,
        num_advice_per_phase: vec![4],
        num_lookup_advice_per_phase: vec![1],
        num_fixed: 1,
        num_instance_columns: 1,
        lookup_bits: Some(11),
    };
    AxiomCircuitParams::Base(params)
}

base_test_struct!(AccountTest, account_call);
base_test_struct!(HeaderTest, header_call);

#[test_case(AccountTest)]
#[test_case(HeaderTest)]
pub fn mock<S: AxiomCircuitScaffold<Http, Fr>>(circuit: S) {
    let params = get_base_test_params();
    mock_test(params, circuit);
}

#[test_case(AccountTest)]
#[test_case(HeaderTest)]
pub fn single_instance<S: AxiomCircuitScaffold<Http, Fr>>(circuit: S) {
    let params = get_base_test_params();
    single_instance_test(params, circuit);
}