#[macro_export]
macro_rules! generate_endpoint {
    (path = $path:literal, endpoint_type = $endpoint:ident, request = $req:ident, response = $resp:ident, resource = $resource:ty, extensions = $extensions:ty $(,)?) => {
        $crate::generate_request!($req, $resource, $extensions);
        $crate::generate_response!($resp, $resource, $extensions);

        pub struct $endpoint;
        impl $crate::protocol::ScimEndpoint for $endpoint {
            const ENDPOINT: &'static str = $path;

            type Resource = $resource;
            type Extensions = $extensions;

            type Request = $req;
            type Response = $resp;
        }
    };
}
