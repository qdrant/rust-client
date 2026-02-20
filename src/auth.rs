use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Clone)]
pub struct TokenInterceptor {
    api_key: Option<String>,
}

impl TokenInterceptor {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }
}

impl Interceptor for TokenInterceptor {
    fn call(&mut self, mut req: Request<()>) -> anyhow::Result<Request<()>, Status> {
        if let Some(api_key) = &self.api_key {
            req.metadata_mut().insert(
                "api-key",
                api_key.parse().map_err(|_| {
                    Status::invalid_argument(format!("Malformed API key or token: {api_key}"))
                })?,
            );
        }
        Ok(req)
    }
}

#[derive(Clone)]
pub struct WrappedInterceptor<I: Send + Sync + 'static + Clone + Interceptor = TokenInterceptor> {
    pub inner: Option<I>,
}

impl<I: Send + Sync + 'static + Clone + Interceptor> Default for WrappedInterceptor<I> {
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<I: Send + Sync + 'static + Clone + Interceptor> WrappedInterceptor<I> {
    pub fn new(interceptor: I) -> Self {
        Self {
            inner: Some(interceptor),
        }
    }
}

impl<I: Send + Sync + 'static + Clone + Interceptor> Interceptor for WrappedInterceptor<I> {
    fn call(&mut self, req: Request<()>) -> anyhow::Result<Request<()>, Status> {
        match self.inner.as_mut() {
            Some(inter) => inter.call(req),
            None => Ok(req),
        }
    }
}
