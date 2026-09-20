use error_stuff::ServErr;
use server_protocol::{UnbuiltResponse, unbuild_response};

pub fn receive_response(json: &str) -> Result<(UnbuiltResponse, usize), ServErr> {
    unbuild_response(json)
}
