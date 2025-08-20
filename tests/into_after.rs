//! Test that the `AfterBlah` types implement `From<Successor>` for all
//! `Successor` typestates that come after the `Blah` typestate.

#[macro_use]
extern crate state_machine_future;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Poll};

#[derive(StateMachineFuture)]
pub enum Machine {
    /// Choose which next state to go into depending on what start value is
    /// given.
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Ready, TransitionMacro))]
    Start,

    #[state_machine_future(transitions(Ready))]
    TransitionMacro,

    #[state_machine_future(ready)]
    Ready(usize),

    #[state_machine_future(error)]
    Error(usize),
}

impl PollMachine for Machine {
    fn poll_start<'a>(
        _: &'a mut RentToOwn<'a, Start>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterStart, usize>> {
        Poll::Ready(Ok(Ready(1).into()))
    }

    fn poll_transition_macro<'a>(
        _: &'a mut RentToOwn<'a, TransitionMacro>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterTransitionMacro, usize>> {
        transition!(Ready(2))
    }
}
