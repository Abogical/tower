//! Future types

use error::{self, Error};
use futures::{Async, Future, Poll};
use tower_service::Service;
use tower_util::Oneshot;
use std::fmt;

/// Filtered response future
pub struct ResponseFuture<T, S, Request>
where
    S: Service<Request>,
{
    /// Response future state
    state: State<S, Request>,

    /// Predicate future
    check: T,
}

enum State<S, Request>
where
    S: Service<Request>,
{
    Check(S, Request),
    WaitResponse(Oneshot<S, Request>),
    Invalid,
}

impl<T, S, Request> ResponseFuture<T, S, Request>
where
    T: Future<Error = Error>,
    S: Service<Request>,
    S::Error: Into<error::Source>,
{
    pub(crate) fn new(request: Request, check: T, service: S) -> Self {
        ResponseFuture {
            state: State::Check(service, request),
            check,
        }
    }
}

impl<T, S, Request> Future for ResponseFuture<T, S, Request>
where
    T: Future<Error = Error>,
    S: Service<Request>,
    S::Error: Into<error::Source>,
{
    type Item = S::Response;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::State::*;
        use std::mem;

        loop {
            match mem::replace(&mut self.state, Invalid) {
                Check(service, request) => {
                    // Poll predicate
                    match self.check.poll()? {
                        Async::Ready(_) => {
                            self.state = WaitResponse(Oneshot::new(service, request));
                        }
                        Async::NotReady => {
                            self.state = Check(service, request);
                            return Ok(Async::NotReady);
                        }
                    }
                }
                WaitResponse(mut response) => {
                    let ret = response.poll().map_err(Error::inner);

                    self.state = WaitResponse(response);

                    return ret;
                }
                Invalid => {
                    panic!("invalid state");
                }
            }
        }
    }
}

impl<T, S, Request> fmt::Debug for ResponseFuture<T, S, Request>
where
    T: fmt::Debug,
    S: Service<Request> + fmt::Debug,
    S::Future: fmt::Debug,
    S: fmt::Debug,
    Request: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ResponseFuture")
            .field("state", &self.state)
            .field("check", &self.check)
            .finish()
    }
}

impl<S, Request> fmt::Debug for State<S, Request>
where
    S: Service<Request> + fmt::Debug,
    S::Future: fmt::Debug,
    S: fmt::Debug,
    Request: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::State::*;

        match *self {
            Check(ref s, ref req) => {
                fmt.debug_tuple("State::Check")
                    .field(s)
                    .field(req)
                    .finish()
            }
            WaitResponse(ref oneshot) => {
                fmt.debug_tuple("State::WaitResponse")
                    .field(oneshot)
                    .finish()
            }
            _ => unreachable!(),
        }
    }
}
