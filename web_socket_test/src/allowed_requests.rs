use db_weebee::mascot::LiveForever;
use shared_types::message_struct;

fn do_request(msg: &message_struct::Message, liver: &LiveForever) -> Vec<u8> {
    let data = msg.content.0;
    match msg.request {
        message_struct::Request::GetData => liver.get_data(data),
        _ => message_struct::i_dont_want_to(msg.message_id, msg.request),
    }
}
