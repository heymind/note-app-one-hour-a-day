use futures::future::BoxFuture;
use hyper::{Body, Request, Response};
use spin::Lazy;
use std::error::Error;
use std::result::Result;
pub struct Handle(
    pub  Box<
        dyn Fn(
                Request<Body>,
            )
                -> BoxFuture<'static, Result<Response<Body>, Box<dyn Error + Send + Sync>>>
            + Send
            + Sync,
    >,
);

impl std::fmt::Debug for Handle {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
#[derive(Debug)]
pub enum Endpoint<F: 'static = fn() -> Handle> {
    Single {
        methods: &'static [&'static str],
        path: &'static str,
        handle: Lazy<Handle, F>,
    },
    Group {
        prefix: &'static str,
        endpoints: &'static [&'static Endpoint<F>],
    },
}

impl<F> Endpoint<F> {
    pub const fn single(methods: &'static [&'static str], path: &'static str, handle: F) -> Self {
        Endpoint::Single {
            methods,
            path,
            handle: Lazy::new(handle),
        }
    }
    pub const fn group(prefix: &'static str, endpoints: &'static [&'static Endpoint<F>]) -> Self {
        Self::Group { prefix, endpoints }
    }
}
