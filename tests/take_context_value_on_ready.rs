// FIXME: Fails to compile after upgrade; taking context by value when transitioning to ready needs further porting work.
/*
//! Test that we can take a value from context when transitioning to ready.

#[macro_use]
extern crate state_machine_future;

mod util;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context as TaskContext, Poll};

pub struct Ctx {
    value: String,
}

#[derive(StateMachineFuture)]
#[state_machine_future(context = "Ctx")]
pub enum WithContext {
    #[state_machine_future(start, transitions(Ready))]
    Start,

    #[state_machine_future(ready)]
    Ready(String),

    #[state_machine_future(error)]
    Error(()),
}

impl PollWithContext for WithContext {
    fn poll_start<'s, 'c>(
        _: &'s mut RentToOwn<'s, Start>,
        _: &mut TaskContext<'_>,
        context: &'c mut RentToOwn<'c, Ctx>,
    ) -> Poll<Result<AfterStart, ()>> {
        let context = context.take();
        let value = context.value;
        transition!(Ready(value))
    }
}

#[test]
fn given_sm_with_context_can_take_context_value_on_ready() {
    let context = Ctx {
        value: String::from("foo"),
    };

    let mut machine = WithContext::start(context);

    assert_eq!(util::poll(&mut machine), Poll::Ready(Ok(String::from("foo"))));
}
*/
