//! Test that the generated code has the right transition types.

#[macro_use]
extern crate state_machine_future;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Poll};

pub struct MyReady;
pub struct MyError;

#[derive(StateMachineFuture)]
pub enum Fsm {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Middle, End))]
    Begin,

    #[state_machine_future(transitions(End))]
    Middle(()),

    #[state_machine_future(ready)]
    #[state_machine_future(error)]
    End(()),
}

pub fn check_begin_transitions(x: AfterBegin) {
    match x {
        AfterBegin::Middle(Middle(())) | AfterBegin::End(End(())) => unimplemented!(),
    }
}

pub fn check_middle_transitions(x: AfterMiddle) {
    match x {
        AfterMiddle::End(End(())) => unimplemented!(),
    }
}

impl PollFsm for Fsm {
    fn poll_begin<'a>(
        _: &'a mut RentToOwn<'a, Begin>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterBegin, ()>> {
        unimplemented!()
    }

    fn poll_middle<'a>(
        _: &'a mut RentToOwn<'a, Middle>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterMiddle, ()>> {
        unimplemented!()
    }
}
