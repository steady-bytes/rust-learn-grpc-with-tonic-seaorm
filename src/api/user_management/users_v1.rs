//////////////////////
/// ROOT AGGREGATE ///
//////////////////////

/// this is similar to the model of the User aggregate type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub password: ::prost::alloc::string::String,
}
/////////////////////
/// CRUD MESSAGES ///
/////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignupUserRequest {
    #[prost(message, optional, tag="1")]
    pub model: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignupUserResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByIdResponse {
    #[prost(message, optional, tag="1")]
    pub model: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
    /// `id` from the users is used to lookup single data for aggregate to change
    #[prost(message, optional, tag="1")]
    pub model: ::core::option::Option<User>,
    /// all fields in protobuf's are optional, and thus a list of attributes to change
    /// must be provided in updates.
    #[prost(string, repeated, tag="2")]
    pub update_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResponse {
    #[prost(message, optional, tag="1")]
    pub model: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteByIdRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteByIdResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
///////////////
//// EVENTS ///
///////////////
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSignedUp {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRegistered {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/////////////////////////
/// Syncronus Actions ///
/////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloUserRequest {
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloUserResponse {
    #[prost(string, tag="1")]
    pub hello_output: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod users_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Users - All of the public methods that relate to a user.
    #[derive(Debug, Clone)]
    pub struct UsersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UsersClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UsersClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UsersClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UsersClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Create a new user in the system
        pub async fn signup(
            &mut self,
            request: impl tonic::IntoRequest<super::SignupUserRequest>,
        ) -> Result<tonic::Response<super::SignupUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UsersV1.Users/Signup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List all users
        pub async fn get_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserByIdRequest>,
        ) -> Result<tonic::Response<super::GetUserByIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UsersV1.Users/GetById");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a user's details
        pub async fn update_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserRequest>,
        ) -> Result<tonic::Response<super::UpdateUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UsersV1.Users/UpdateById");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a user by their id
        pub async fn delete_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteByIdRequest>,
        ) -> Result<tonic::Response<super::DeleteByIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UsersV1.Users/DeleteById");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SayHello to a user
        pub async fn say_hello(
            &mut self,
            request: impl tonic::IntoRequest<super::HelloUserRequest>,
        ) -> Result<tonic::Response<super::HelloUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/UsersV1.Users/SayHello");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod users_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with UsersServer.
    #[async_trait]
    pub trait Users: Send + Sync + 'static {
        /// Create a new user in the system
        async fn signup(
            &self,
            request: tonic::Request<super::SignupUserRequest>,
        ) -> Result<tonic::Response<super::SignupUserResponse>, tonic::Status>;
        /// List all users
        async fn get_by_id(
            &self,
            request: tonic::Request<super::GetUserByIdRequest>,
        ) -> Result<tonic::Response<super::GetUserByIdResponse>, tonic::Status>;
        /// Update a user's details
        async fn update_by_id(
            &self,
            request: tonic::Request<super::UpdateUserRequest>,
        ) -> Result<tonic::Response<super::UpdateUserResponse>, tonic::Status>;
        /// Delete a user by their id
        async fn delete_by_id(
            &self,
            request: tonic::Request<super::DeleteByIdRequest>,
        ) -> Result<tonic::Response<super::DeleteByIdResponse>, tonic::Status>;
        /// SayHello to a user
        async fn say_hello(
            &self,
            request: tonic::Request<super::HelloUserRequest>,
        ) -> Result<tonic::Response<super::HelloUserResponse>, tonic::Status>;
    }
    /// Users - All of the public methods that relate to a user.
    #[derive(Debug)]
    pub struct UsersServer<T: Users> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Users> UsersServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UsersServer<T>
    where
        T: Users,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/UsersV1.Users/Signup" => {
                    #[allow(non_camel_case_types)]
                    struct SignupSvc<T: Users>(pub Arc<T>);
                    impl<T: Users> tonic::server::UnaryService<super::SignupUserRequest>
                    for SignupSvc<T> {
                        type Response = super::SignupUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignupUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).signup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/UsersV1.Users/GetById" => {
                    #[allow(non_camel_case_types)]
                    struct GetByIdSvc<T: Users>(pub Arc<T>);
                    impl<T: Users> tonic::server::UnaryService<super::GetUserByIdRequest>
                    for GetByIdSvc<T> {
                        type Response = super::GetUserByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/UsersV1.Users/UpdateById" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateByIdSvc<T: Users>(pub Arc<T>);
                    impl<T: Users> tonic::server::UnaryService<super::UpdateUserRequest>
                    for UpdateByIdSvc<T> {
                        type Response = super::UpdateUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/UsersV1.Users/DeleteById" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteByIdSvc<T: Users>(pub Arc<T>);
                    impl<T: Users> tonic::server::UnaryService<super::DeleteByIdRequest>
                    for DeleteByIdSvc<T> {
                        type Response = super::DeleteByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/UsersV1.Users/SayHello" => {
                    #[allow(non_camel_case_types)]
                    struct SayHelloSvc<T: Users>(pub Arc<T>);
                    impl<T: Users> tonic::server::UnaryService<super::HelloUserRequest>
                    for SayHelloSvc<T> {
                        type Response = super::HelloUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).say_hello(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SayHelloSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Users> Clone for UsersServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Users> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Users> tonic::transport::NamedService for UsersServer<T> {
        const NAME: &'static str = "UsersV1.Users";
    }
}
