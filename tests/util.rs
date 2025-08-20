use futures_lite::future::{block_on, poll_once};
use state_machine_future::export::{Future, Poll};

pub fn poll<F: Future + Unpin>(f: &mut F) -> Poll<F::Output> {
    match block_on(poll_once(f)) {
        Some(output) => Poll::Ready(output),
        None => Poll::Pending,
    }
}
