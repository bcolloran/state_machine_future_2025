//! Test case taken from
//! <https://github.com/fitzgen/state_machine_future/issues/25>.

#[macro_use]
extern crate state_machine_future;
use state_machine_future::RentToOwn;
use state_machine_future::export::{Context, Future, Poll};

#[derive(StateMachineFuture)]
enum Foo<T>
where
    T: Future<Output = i32> + Unpin,
{
    #[state_machine_future(start, transitions(Finished))]
    Start(T),
    #[state_machine_future(ready)]
    Finished(i32),
    #[state_machine_future(error)]
    Failed(()),
}

impl<T> PollFoo<T> for Foo<T>
where
    T: Future<Output = i32> + Unpin,
{
    fn poll_start<'a>(
        _state: &'a mut RentToOwn<'a, Start<T>>,
        _: &mut Context<'_>,
    ) -> Poll<Result<AfterStart, ()>> {
        panic!()
    }
}
