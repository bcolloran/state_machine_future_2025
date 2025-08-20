// FIXME: This compile-time test currently fails; starting a machine in a state with context needs further updates for futures-lite.
/*
//! Test that we can access context type.

#[macro_use]
extern crate state_machine_future;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context as TaskContext, Poll};

pub struct Ctx {}

#[derive(StateMachineFuture)]
#[state_machine_future(context = "Ctx")]
pub enum WithContext {
    #[state_machine_future(start, transitions(Ready))]
    Start,

    #[state_machine_future(ready)]
    Ready(()),

    #[state_machine_future(error)]
    Error(()),
}

impl PollWithContext for WithContext {
    fn poll_start<'s, 'c>(
        _: &'s mut RentToOwn<'s, Start>,
        _: &mut TaskContext<'_>,
        _: &'c mut RentToOwn<'c, Ctx>,
    ) -> Poll<Result<AfterStart, ()>> {
        unimplemented!()
    }
}

#[test]
fn given_sm_with_no_start_args_only_takes_context() {
    let context = Ctx {};

    let _ = WithContext::start_in(Ready(()), context);
}
*/
