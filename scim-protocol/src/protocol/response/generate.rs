#[macro_export]
macro_rules! generate_response {
    ($resp:ident, $resource:ty, $extensions:ty) => {
        $crate::generate_response_struct!($resp, $resource, $extensions);
        $crate::generate_response_deserialize!($resp);
        // $crate::generate_response_serialize($resp);
    };
}
