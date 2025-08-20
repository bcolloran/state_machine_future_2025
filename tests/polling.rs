//! Test that we get the expected poll results.

#[macro_use]
extern crate state_machine_future;

mod util;

use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Poll};

#[derive(StateMachineFuture)]
pub enum Machine {
    /// Choose which next state to go into depending on what start value is
    /// given.
    #[state_machine_future(start)]
    #[state_machine_future(transitions(NeverReady, AlwaysReady, Error))]
    Start(Result<Box<AfterStart>, usize>),

    /// This always returns NotReady.
    #[state_machine_future(transitions(Error))]
    NeverReady,

    /// This always returns Ready.
    #[state_machine_future(transitions(Ready))]
    AlwaysReady,

    #[state_machine_future(ready)]
    Ready(usize),

    #[state_machine_future(error)]
    Error(usize),
}

impl PollMachine for Machine {
    fn poll_start<'a>(
        start: &'a mut RentToOwn<'a, Start>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterStart, usize>> {
        match start.take().0 {
            Ok(next) => Poll::Ready(Ok(*next)),
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    fn poll_never_ready<'a>(
        _: &'a mut RentToOwn<'a, NeverReady>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterNeverReady, usize>> {
        Poll::Pending
    }

    fn poll_always_ready<'a>(
        _: &'a mut RentToOwn<'a, AlwaysReady>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterAlwaysReady, usize>> {
        Poll::Ready(Ok(AfterAlwaysReady::Ready(Ready(1))))
    }
}

#[test]
fn direct_error() {
    let mut machine = Machine::start(Err(42));
    assert_eq!(util::poll(&mut machine), Poll::Ready(Err(42)));

    // And it's fused: never ready when polling again after we finished.
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
}

#[test]
fn indirect_error() {
    let mut machine = Machine::start(Ok(Box::new(AfterStart::Error(Error(42)))));
    assert_eq!(util::poll(&mut machine), Poll::Ready(Err(42)));

    // And it's fused: never ready when polling again after we finished.
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
}

#[test]
fn never_ready() {
    let mut machine = Machine::start(Ok(Box::new(AfterStart::NeverReady(NeverReady))));
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
}

#[test]
fn always_ready() {
    let mut machine = Machine::start(Ok(Box::new(AfterStart::AlwaysReady(AlwaysReady))));
    assert_eq!(util::poll(&mut machine), Poll::Ready(Ok(1)));

    // And it's fused: never ready when polling again after we finished.
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
    assert!(util::poll(&mut machine).is_pending());
}
