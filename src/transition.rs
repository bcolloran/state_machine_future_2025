/// Auxiliary macro for `poll_state_xy` functions to transition into a new state.
#[macro_export]
macro_rules! transition {
    ( $new_state:expr ) => {
        return ::core::task::Poll::Ready(Ok($new_state.into()));
    };
}
