//! Test that the generated code is somewhat robust in the face of states with
//! names of types its using.

#[macro_use]
extern crate state_machine_future;

#[allow(dead_code)]
fn futures() {}
#[allow(dead_code)]
fn state_machine_future() {}

#[allow(unused_macros)]
macro_rules! futures {
    () => {};
}
#[allow(unused_macros)]
macro_rules! state_machine_future {
    () => {};
}

#[derive(StateMachineFuture)]
pub enum Fsm {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Future))]
    AsyncState,

    #[state_machine_future(transitions(Poll))]
    Future,

    #[state_machine_future(transitions(RentToOwn))]
    Poll,

    #[state_machine_future(transitions(StateMachineFuture))]
    RentToOwn,

    #[state_machine_future(ready)]
    #[state_machine_future(error)]
    StateMachineFuture(()),
}

impl PollFsm for Fsm {
    fn poll_async_state<'a>(
        _: &'a mut state_machine_future::RentToOwn<'a, AsyncState>,
        _: &mut state_machine_future::export::Context<'_>,
    ) -> state_machine_future::export::Poll<core::result::Result<AfterAsyncState, ()>> {
        unimplemented!()
    }

    fn poll_future<'a>(
        _: &'a mut state_machine_future::RentToOwn<'a, Future>,
        _: &mut state_machine_future::export::Context<'_>,
    ) -> state_machine_future::export::Poll<core::result::Result<AfterFuture, ()>> {
        unimplemented!()
    }

    fn poll_poll<'a>(
        _: &'a mut state_machine_future::RentToOwn<'a, Poll>,
        _: &mut state_machine_future::export::Context<'_>,
    ) -> state_machine_future::export::Poll<core::result::Result<AfterPoll, ()>> {
        unimplemented!()
    }

    fn poll_rent_to_own<'a>(
        _: &'a mut state_machine_future::RentToOwn<'a, RentToOwn>,
        _: &mut state_machine_future::export::Context<'_>,
    ) -> state_machine_future::export::Poll<core::result::Result<AfterRentToOwn, ()>> {
        unimplemented!()
    }
}
