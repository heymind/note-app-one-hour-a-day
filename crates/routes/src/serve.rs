use crate::Endpoint;
use hyper::server::conn::{AddrIncoming, AddrStream};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, StatusCode};
use indexmap::IndexMap;
use regex::{RegexSet, RegexSetBuilder};
use std::convert::Infallible;
use std::fmt::Write;
use std::result::Result;
use std::sync::Arc;
use std::{error::Error, ops::Deref};

struct ContextInner {
    set: RegexSet,
    routes: IndexMap<String, &'static Endpoint>,
}

impl ContextInner {
    fn with_index(self: Arc<Self>, idx: usize) -> Context {
        Context {
            inner: self,
            index: idx,
        }
    }
}

pub struct Context {
    inner: Arc<ContextInner>,
    index: usize,
}

impl Default for ContextInner {
    fn default() -> Self {
        Self {
            set: RegexSet::empty(),
            routes: Default::default(),
        }
    }
}

pub async fn serve(
    endpoint: &'static Endpoint,
    builder: hyper::server::Builder<AddrIncoming>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut ctx = ContextInner::default();
    let mut stack: Vec<(Vec<&'static str>, &'static [&'static Endpoint])> = Default::default();

    match endpoint {
        Endpoint::Single { path, .. } => {
            ctx.routes.insert(path.to_string(), endpoint);
        }
        Endpoint::Group { prefix, endpoints } => {
            stack.push((prefix.split("/").collect(), *endpoints));
        }
    }

    while let Some((segments, endpoints)) = stack.pop() {
        for endpoint in endpoints {
            match endpoint {
                Endpoint::Single { path, .. } => {
                    let mut full_path = "^".to_string();
                    segments
                        .iter()
                        .cloned()
                        .chain(path.split("/"))
                        .filter(|x| !x.is_empty())
                        .for_each(|seg| write!(&mut full_path, "/{}", seg).unwrap());
                    ctx.routes.insert(full_path, endpoint);
                }
                Endpoint::Group { prefix, endpoints } => {
                    stack.push((
                        segments.iter().cloned().chain(prefix.split("/")).collect(),
                        *endpoints,
                    ));
                }
            }
        }
    }


    let set_builder = RegexSetBuilder::new(ctx.routes.keys());

    ctx.set = set_builder.build().unwrap();

    let ctx = Arc::new(ctx);

    let server = builder.serve(make_service_fn(|socket: &AddrStream| {
        let _remote_addr = socket.remote_addr();
        let ctx = ctx.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |mut req: Request<Body>| {
                let ctx = ctx.clone();
                async move {
                    let ContextInner { set, routes } = ctx.as_ref();
                    let path = req.uri().path();
                    let matches = set.matches(path);
                    for idx in matches.iter() {
                        if let Endpoint::Single {
                            methods, handle, ..
                        } = routes[idx]
                        {
                            if methods.iter().any(|method| method == req.method()) {
                                req.extensions_mut().insert(ctx.with_index(idx));
                                return (handle.deref().0)(req).await;
                            }
                        }
                    }
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                }
            }))
        }
    }));
    server
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
}
