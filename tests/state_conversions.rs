//! Test that the generated states can be converted to the state enum.

#[macro_use]
extern crate state_machine_future;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Poll};

#[derive(StateMachineFuture)]
pub enum Fsm {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Middle1, Middle2, End))]
    Begin,

    #[state_machine_future(transitions(End))]
    Middle1,

    #[state_machine_future(transitions(End))]
    Middle2,

    #[state_machine_future(ready)]
    #[state_machine_future(error)]
    End(()),
}

impl PollFsm for Fsm {
    fn poll_begin<'a>(
        _: &'a mut RentToOwn<'a, Begin>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterBegin, ()>> {
        unimplemented!()
    }

    fn poll_middle1<'a>(
        _: &'a mut RentToOwn<'a, Middle1>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterMiddle1, ()>> {
        unimplemented!()
    }

    fn poll_middle2<'a>(
        _: &'a mut RentToOwn<'a, Middle2>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterMiddle2, ()>> {
        unimplemented!()
    }
}

fn convert<S: Into<FsmStates>>(_state: S) {}

#[test]
fn convert_states() {
    convert(Begin);
    convert(Middle1);
    convert(Middle2);
    convert(End(()));
}
