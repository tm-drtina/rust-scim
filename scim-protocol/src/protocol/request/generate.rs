#[macro_export]
macro_rules! generate_request {
    ($req:ident, $resource:ty, $extensions:ty) => {
        $crate::generate_request_struct!($req, $resource, $extensions);
        $crate::generate_request_deserialize!($req);
        $crate::generate_request_serialize!($req);
    };
}
