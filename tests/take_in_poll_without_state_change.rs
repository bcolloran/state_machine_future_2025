//! Test that if a state transition function takes the `RentToOwn` without
//! changing states, then we return `Ok(NotReady)` for perpetuity.

#[macro_use]
extern crate state_machine_future;

mod util;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Poll};

#[derive(StateMachineFuture)]
pub enum Machine {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Ready))]
    Start,

    #[state_machine_future(ready, error)]
    Ready(usize),
}

impl PollMachine for Machine {
    fn poll_start<'a>(
        start: &'a mut RentToOwn<'a, Start>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterStart, usize>> {
        // Take the state.
        let _ = start.take();

        // But don't transition to a new state.
        Poll::Pending
    }
}

#[test]
fn taken_without_state_transition_is_never_ready() {
    let mut machine = Machine::start();

    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
}
