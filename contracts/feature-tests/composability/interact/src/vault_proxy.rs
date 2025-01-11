// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct VaultProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for VaultProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = VaultProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        VaultProxyMethods { wrapped_tx: tx }
    }
}

pub struct VaultProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> VaultProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        opt_arg_to_echo: Arg0,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, OptionalValue<ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&opt_arg_to_echo)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> VaultProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn echo_arguments<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        args: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_arguments")
            .argument(&args)
            .original_result()
    }

    pub fn echo_arguments_without_storage<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        args: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_arguments_without_storage")
            .argument(&args)
            .original_result()
    }

    pub fn echo_caller(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_caller")
            .original_result()
    }

    pub fn accept_funds(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("accept_funds")
            .original_result()
    }

    pub fn accept_funds_echo_payment(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, MultiValueEncoded<Env::Api, MoaOrDcdtTokenPaymentMultiValue<Env::Api>>> {
        self.wrapped_tx
            .raw_call("accept_funds_echo_payment")
            .original_result()
    }

    pub fn accept_funds_single_dcdt_transfer(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("accept_funds_single_dcdt_transfer")
            .original_result()
    }

    pub fn reject_funds(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("reject_funds")
            .original_result()
    }

    pub fn retrieve_funds_with_transfer_exec<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        token: Arg0,
        amount: Arg1,
        opt_receive_func: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("retrieve_funds_with_transfer_exec")
            .argument(&token)
            .argument(&amount)
            .argument(&opt_receive_func)
            .original_result()
    }

    pub fn retrieve_funds<
        Arg0: ProxyArg<MoaOrDcdtTokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token: Arg0,
        nonce: Arg1,
        amount: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("retrieve_funds")
            .argument(&token)
            .argument(&nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn retrieve_funds_moa_or_single_dcdt(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("retrieve_funds_moa_or_single_dcdt")
            .original_result()
    }

    pub fn retrieve_funds_multi_dcdt(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("retrieve_funds_multi_dcdt")
            .original_result()
    }

    pub fn retrieve_multi_funds_async<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        token_payments: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("retrieve_multi_funds_async")
            .argument(&token_payments)
            .original_result()
    }

    pub fn burn_and_create_retrieve_async(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("burn_and_create_retrieve_async")
            .original_result()
    }

    pub fn explicit_panic(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("explicit_panic")
            .original_result()
    }

    pub fn get_owner_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("get_owner_address")
            .original_result()
    }

    /// We already leave a trace of the calls using the event logs; 
    /// this additional counter has the role of showing that storage also gets saved correctly. 
    pub fn call_counts<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        endpoint: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("call_counts")
            .argument(&endpoint)
            .original_result()
    }

    pub fn num_called_retrieve_funds_promises(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("num_called_retrieve_funds_promises")
            .original_result()
    }

    pub fn num_async_calls_sent_from_child(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("num_async_calls_sent_from_child")
            .original_result()
    }
}
