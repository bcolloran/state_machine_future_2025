//! Test that we handle overlapping start/ready/error states properly.

#[macro_use]
extern crate state_machine_future;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Poll};

#[derive(StateMachineFuture)]
pub enum AllOverlapping {
    #[state_machine_future(start)]
    #[state_machine_future(ready)]
    #[state_machine_future(error)]
    OnlyState(()),
}

#[derive(StateMachineFuture)]
pub enum NotOverlapping {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Ready, Error))]
    Start,
    #[state_machine_future(ready)]
    Ready(()),
    #[state_machine_future(error)]
    Error(()),
}

impl PollNotOverlapping for NotOverlapping {
    fn poll_start<'a>(
        _: &'a mut RentToOwn<'a, Start>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterStart, ()>> {
        unimplemented!()
    }
}

#[derive(StateMachineFuture)]
pub enum ReadyErrorOverlapping {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(ReadyError))]
    Init,

    #[state_machine_future(ready)]
    #[state_machine_future(error)]
    ReadyError(()),
}

impl PollReadyErrorOverlapping for ReadyErrorOverlapping {
    fn poll_init<'a>(
        _: &'a mut RentToOwn<'a, Init>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterInit, ()>> {
        unimplemented!()
    }
}
