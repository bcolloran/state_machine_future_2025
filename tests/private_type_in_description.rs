//! Test that we don't leak private types in public API.

#[macro_use]
extern crate state_machine_future;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Poll};

struct PrivateType;

// Should not get this error:
//
// error[E0446]: private type `PrivateType` in public interface
//   --> tests/private_type_in_description.rs:12:10
//    |
// 12 | #[derive(StateMachineFuture)]
//    |          ^^^^^^^^^^^^^^^^^^ can't leak private type
#[derive(StateMachineFuture)]
enum Machine {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Ready))]
    Start(PrivateType),

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
        unimplemented!()
    }
}
