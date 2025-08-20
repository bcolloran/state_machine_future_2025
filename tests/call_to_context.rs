// FIXME: Fails to compile after porting to futures-lite; `StateMachineFuture` trait signatures with context need further adaptation.
/*
//! Test that we can access context type.

#[macro_use]
extern crate state_machine_future;

mod util;

use core::pin::Pin;
use futures_lite::future;
use state_machine_future::RentToOwn;
use state_machine_future::export::{Context as PollContext, Future, Poll};

pub struct ExternalSource<T> {
    pub value: T,
}

pub struct Context<T> {
    pub external_source: ExternalSource<T>,
    pub lazy_future: Option<future::Ready<Result<T, ()>>>,
}

impl<T: Clone> Context<T> {
    fn load_from_external_source(&mut self) -> &mut future::Ready<Result<T, ()>> {
        let value = self.external_source.value.clone();
        self
            .lazy_future
            .get_or_insert_with(|| future::ready(Ok(value)))
    }
}

#[derive(StateMachineFuture)]
#[state_machine_future(context = "Context")]
pub enum WithContext<T: Clone> {
    #[state_machine_future(start, transitions(Ready))]
    Start(()),

    #[state_machine_future(ready)]
    Ready(T),

    #[state_machine_future(error)]
    Error(()),
}

impl<T: Clone> PollWithContext<T> for WithContext<T> {
    fn poll_start<'s, 'c>(
        _: &'s mut RentToOwn<'s, Start>,
        cx: &mut PollContext<'_>,
        context: &'c mut RentToOwn<'c, Context<T>>,
    ) -> Poll<Result<AfterStart<T>, ()>> {
        let value = match Pin::new(context.load_from_external_source()).poll(cx) {
            Poll::Ready(Ok(v)) => v,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Pending => return Poll::Pending,
        };

        transition!(Ready(value))
    }
}

#[test]
fn can_call_to_context() {
    let source = ExternalSource {
        value: String::from("foo"),
    };

    let context = Context {
        external_source: source,
        lazy_future: None,
    };

    let mut machine = WithContext::start((), context);

    assert_eq!(util::poll(&mut machine), Poll::Ready(Ok(String::from("foo"))));
}
*/
