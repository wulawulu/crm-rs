// This file is @generated by prost-build.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(sqlx::FromRow, derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    #[builder(setter(into))]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[builder(setter(into))]
    pub name: ::prost::alloc::string::String,
}
#[derive(sqlx::FromRow, Clone, PartialEq, ::prost::Message)]
pub struct UserWithUnfinished {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, repeated, tag = "3")]
    pub started_but_not_finished: ::prost::alloc::vec::Vec<i32>,
}
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(map = "string, message", tag = "1")]
    #[builder(setter(each(name = "timestamp", into)))]
    pub timestamps: ::std::collections::HashMap<::prost::alloc::string::String, TimeQuery>,
    #[prost(map = "string, message", tag = "2")]
    #[builder(setter(each(name = "id", into)))]
    pub ids: ::std::collections::HashMap<::prost::alloc::string::String, IdQuery>,
}
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawQueryRequest {
    #[prost(string, tag = "1")]
    #[builder(setter(into))]
    pub query: ::prost::alloc::string::String,
}
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TimeQuery {
    #[prost(message, optional, tag = "1")]
    #[builder(setter(into, strip_option))]
    pub lower: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    #[builder(setter(into, strip_option))]
    pub upper: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdQuery {
    #[prost(uint32, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
}
/// Generated client implementations.
pub mod user_stats_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UserStatsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserStatsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UserStatsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UserStatsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                    >,
                >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            UserStatsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::User>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/user_stats.UserStats/Query");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("user_stats.UserStats", "Query"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn raw_query(
            &mut self,
            request: impl tonic::IntoRequest<super::RawQueryRequest>,
        ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::User>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/user_stats.UserStats/RawQuery");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("user_stats.UserStats", "RawQuery"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn query_with_unfinished(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::UserWithUnfinished>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/user_stats.UserStats/QueryWithUnfinished");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "user_stats.UserStats",
                "QueryWithUnfinished",
            ));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_stats_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserStatsServer.
    #[async_trait]
    pub trait UserStats: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the Query method.
        type QueryStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::User, tonic::Status>,
            > + std::marker::Send
            + 'static;
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<Self::QueryStream>, tonic::Status>;
        /// Server streaming response type for the RawQuery method.
        type RawQueryStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::User, tonic::Status>,
            > + std::marker::Send
            + 'static;
        async fn raw_query(
            &self,
            request: tonic::Request<super::RawQueryRequest>,
        ) -> std::result::Result<tonic::Response<Self::RawQueryStream>, tonic::Status>;
        /// Server streaming response type for the QueryWithUnfinished method.
        type QueryWithUnfinishedStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::UserWithUnfinished, tonic::Status>,
            > + std::marker::Send
            + 'static;
        async fn query_with_unfinished(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> std::result::Result<tonic::Response<Self::QueryWithUnfinishedStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UserStatsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> UserStatsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserStatsServer<T>
    where
        T: UserStats,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/user_stats.UserStats/Query" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySvc<T: UserStats>(pub Arc<T>);
                    impl<T: UserStats> tonic::server::ServerStreamingService<super::QueryRequest> for QuerySvc<T> {
                        type Response = super::User;
                        type ResponseStream = T::QueryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { <T as UserStats>::query(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = QuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/user_stats.UserStats/RawQuery" => {
                    #[allow(non_camel_case_types)]
                    struct RawQuerySvc<T: UserStats>(pub Arc<T>);
                    impl<T: UserStats> tonic::server::ServerStreamingService<super::RawQueryRequest>
                        for RawQuerySvc<T>
                    {
                        type Response = super::User;
                        type ResponseStream = T::RawQueryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RawQueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as UserStats>::raw_query(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RawQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/user_stats.UserStats/QueryWithUnfinished" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWithUnfinishedSvc<T: UserStats>(pub Arc<T>);
                    impl<T: UserStats> tonic::server::ServerStreamingService<super::QueryRequest>
                        for QueryWithUnfinishedSvc<T>
                    {
                        type Response = super::UserWithUnfinished;
                        type ResponseStream = T::QueryWithUnfinishedStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserStats>::query_with_unfinished(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = QueryWithUnfinishedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    let mut response = http::Response::new(empty_body());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for UserStatsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "user_stats.UserStats";
    impl<T> tonic::server::NamedService for UserStatsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
