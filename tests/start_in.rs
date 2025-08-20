//! Test that we can start the state machine in any state.

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
    Middle1 { number: u32 },

    #[state_machine_future(transitions(End))]
    Middle2 { string: String },

    #[state_machine_future(ready)]
    #[state_machine_future(error)]
    End(()),
}

#[allow(unused_must_use)]
#[test]
fn can_start_in_all_states() {
    Fsm::start_in(Begin);
    Fsm::start_in(Middle1 { number: 10 });
    Fsm::start_in(Middle2 {
        string: String::from("Hello!"),
    });
    Fsm::start_in(End(()));
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
