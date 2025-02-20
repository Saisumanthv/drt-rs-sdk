// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct PriceAggregatorProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for PriceAggregatorProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = PriceAggregatorProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        PriceAggregatorProxyMethods { wrapped_tx: tx }
    }
}

pub struct PriceAggregatorProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> PriceAggregatorProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<usize>,
        Arg4: ProxyArg<usize>,
        Arg5: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        staking_token: Arg0,
        staking_amount: Arg1,
        slash_amount: Arg2,
        slash_quorum: Arg3,
        submission_count: Arg4,
        oracles: Arg5,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&staking_token)
            .argument(&staking_amount)
            .argument(&slash_amount)
            .argument(&slash_quorum)
            .argument(&submission_count)
            .argument(&oracles)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PriceAggregatorProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn change_amounts<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        staking_amount: Arg0,
        slash_amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeAmounts")
            .argument(&staking_amount)
            .argument(&slash_amount)
            .original_result()
    }

    pub fn add_oracles<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        oracles: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addOracles")
            .argument(&oracles)
            .original_result()
    }

    /// Also receives submission count, 
    /// so the owner does not have to update it manually with setSubmissionCount before this call 
    pub fn remove_oracles<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        submission_count: Arg0,
        oracles: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeOracles")
            .argument(&submission_count)
            .argument(&oracles)
            .original_result()
    }

    pub fn submit<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<u8>,
    >(
        self,
        from: Arg0,
        to: Arg1,
        submission_timestamp: Arg2,
        price: Arg3,
        decimals: Arg4,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("submit")
            .argument(&from)
            .argument(&to)
            .argument(&submission_timestamp)
            .argument(&price)
            .argument(&decimals)
            .original_result()
    }

    pub fn submit_batch<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, MultiValue5<ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, u64, BigUint<Env::Api>, u8>>>,
    >(
        self,
        submissions: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("submitBatch")
            .argument(&submissions)
            .original_result()
    }

    pub fn latest_round_data(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, PriceFeed<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("latestRoundData")
            .original_result()
    }

    pub fn latest_price_feed<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        from: Arg0,
        to: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue6<u32, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, u64, BigUint<Env::Api>, u8>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("latestPriceFeed")
            .argument(&from)
            .argument(&to)
            .original_result()
    }

    pub fn latest_price_feed_optional<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        from: Arg0,
        to: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue6<u32, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, u64, BigUint<Env::Api>, u8>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("latestPriceFeedOptional")
            .argument(&from)
            .argument(&to)
            .original_result()
    }

    pub fn set_submission_count<
        Arg0: ProxyArg<usize>,
    >(
        self,
        submission_count: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setSubmissionCount")
            .argument(&submission_count)
            .original_result()
    }

    pub fn get_oracles(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getOracles")
            .original_result()
    }

    pub fn set_pair_decimals<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<u8>,
    >(
        self,
        from: Arg0,
        to: Arg1,
        decimals: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setPairDecimals")
            .argument(&from)
            .argument(&to)
            .argument(&decimals)
            .original_result()
    }

    pub fn get_pair_decimals<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        from: Arg0,
        to: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u8> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPairDecimals")
            .argument(&from)
            .argument(&to)
            .original_result()
    }

    pub fn submission_count(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("submission_count")
            .original_result()
    }

    pub fn pause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pause")
            .original_result()
    }

    pub fn unpause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unpause")
            .original_result()
    }

    pub fn paused_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isPaused")
            .original_result()
    }

    pub fn stake(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .original_result()
    }

    pub fn unstake<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        unstake_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unstake")
            .argument(&unstake_amount)
            .original_result()
    }

    pub fn vote_slash_member<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        member_to_slash: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("voteSlashMember")
            .argument(&member_to_slash)
            .original_result()
    }

    pub fn cancel_vote_slash_member<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        member_to_slash: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("cancelVoteSlashMember")
            .argument(&member_to_slash)
            .original_result()
    }

    pub fn slash_member<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        member_to_slash: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("slashMember")
            .argument(&member_to_slash)
            .original_result()
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct PriceFeed<Api>
where
    Api: ManagedTypeApi,
{
    pub round_id: u32,
    pub from: ManagedBuffer<Api>,
    pub to: ManagedBuffer<Api>,
    pub timestamp: u64,
    pub price: BigUint<Api>,
    pub decimals: u8,
}

#[type_abi]
#[derive(TopEncode)]
pub struct NewRoundEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub price: BigUint<Api>,
    pub timestamp: u64,
    pub decimals: u8,
    pub block: u64,
    pub epoch: u64,
}
