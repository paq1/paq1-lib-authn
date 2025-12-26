use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};

use actix_web::http::header;
use futures::future::{LocalBoxFuture, Ready, ready};
use paq1_authn_core::apikey::services::context_provider::ContextProvider;
use paq1_lib_error_handler::prelude::Error::Failure;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct ApiKeyAuth {
    pub context_provider: Arc<dyn ContextProvider>,
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddleware {
            service: Rc::new(service),
            context_provider: self.context_provider.clone(),
        }))
    }
}

pub struct ApiKeyMiddleware<S> {
    pub service: Rc<S>,
    pub context_provider: Arc<dyn ContextProvider>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(|v| v.to_string());

        let context_provider = self.context_provider.clone();
        let service = self.service.clone();

        Box::pin(async move {
            if let Some(bearer_token) = auth_header {
                if let Some(token) = bearer_token.strip_prefix("Bearer ").map(|s| s.trim()) {
                    match context_provider.build_context(&token).await {
                        Ok(context) => {
                            req.extensions_mut().insert(context);
                            service.call(req).await
                        }
                        Err(Failure(_e)) => {
                            // let message = format!("Invalid api key : {}", _e.title);
                            Err(actix_web::error::InternalError::from_response(
                                "Unauthorized",
                                HttpResponse::Unauthorized()
                                    .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                                    .insert_header((header::ACCESS_CONTROL_ALLOW_HEADERS, "*"))
                                    .insert_header((header::ACCESS_CONTROL_ALLOW_METHODS, "*"))
                                    .finish(),
                            )
                            .into())
                        }
                    }
                } else {
                    Err(actix_web::error::InternalError::from_response(
                        "Unauthorized",
                        HttpResponse::Unauthorized()
                            .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                            .insert_header((header::ACCESS_CONTROL_ALLOW_HEADERS, "*"))
                            .insert_header((header::ACCESS_CONTROL_ALLOW_METHODS, "*"))
                            .finish(),
                    )
                    .into())
                }
            } else {
                Err(actix_web::error::InternalError::from_response(
                    "Unauthorized",
                    HttpResponse::Unauthorized()
                        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                        .insert_header((header::ACCESS_CONTROL_ALLOW_HEADERS, "*"))
                        .insert_header((header::ACCESS_CONTROL_ALLOW_METHODS, "*"))
                        .finish(),
                )
                .into())
            }
        })
    }
}
