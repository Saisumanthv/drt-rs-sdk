use dharitri_sc_scenario::{
    dharitri_sc::{tuple_util::NestedTupleFlatten, types::RHListExec},
    scenario::tx_to_step::StepWrapper,
    scenario_model::TxResponse,
};
use dharitri_sdk::gateway::GatewayAsyncService;

use super::InteractorEnvExec;

pub struct InteractorExecStep<'w, GatewayProxy, Step, RH>
where
    GatewayProxy: GatewayAsyncService,
    RH: RHListExec<TxResponse, InteractorEnvExec<'w, GatewayProxy>>,
    RH::ListReturns: NestedTupleFlatten,
{
    pub(crate) step_wrapper: StepWrapper<InteractorEnvExec<'w, GatewayProxy>, Step, RH>,
}
