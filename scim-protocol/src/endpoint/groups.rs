use crate::protocol::NoExtensions;
use crate::resource::group::Group;

use crate::generate_endpoint;

generate_endpoint!(
    path = "/Groups",
    endpoint_type = GroupsEndpoint,
    request = GroupRequest,
    response = GroupResponse,
    resource = Group,
    extensions = NoExtensions,
);
