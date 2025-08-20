// FIXME: This test fails to compile; taking context when transitioning to error needs updated trait handling with futures-lite.
/*
//! Test that we can take context when transitioning to error.

#[macro_use]
extern crate state_machine_future;

mod util;

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
        context: &'c mut RentToOwn<'c, Ctx>,
    ) -> Poll<Result<AfterStart, ()>> {
        context.take();
        Poll::Ready(Err(()))
    }
}

#[test]
fn given_sm_with_context_can_take_context_on_error() {
    let context = Ctx {};

    let mut machine = WithContext::start(context);

    assert_eq!(util::poll(&mut machine), Poll::Ready(Err(())));
}
*/
