use db_weebee::mascot::LiveForever;
use shared_types::message_struct;

pub fn do_request(
    msg_request: &message_struct::Request,
    msg_content: Vec<u8>,
    liver: &LiveForever,
) -> Vec<u8> {
    match msg_request {
        message_struct::Request::GetData => liver.get_data(msg_content),
        _ => message_struct::i_dont_want_to(),
    }
}
