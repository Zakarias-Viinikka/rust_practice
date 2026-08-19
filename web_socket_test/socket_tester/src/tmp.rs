use shared_types::{byte_serialization::Convert, data_structures, message_struct};

fn pretend_play() {
    let msg_id = 0 as usize;
    let request = message_struct::Request::GetData;
    let table_name = "table name".to_string();
    let arguments = vec![data_structures::SelectArgument::All];
    let columns_to_read = Vec::new();
    let content = data_structures::GetDataIn {
        table_name,
        arguments,
        columns_to_read,
    };
    let content = content.serialize_wrapper();
    let content = message_struct::Base64Bytes(content);

    let msg = message_struct::Message {
        message_id: msg_id,
        request,
        content,
    };

    let msg_json = message_struct::serialize_message(&msg).map_err(|e| e.to_string());
    println!("{:?}", msg_json);
}
