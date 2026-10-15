use db_wrapper::mascot::LiveForever;
use protocol::payload::*;
use protocol::serialization::Convert;
use server_protocol::{
    DbError, GetEntireTableOut, Request, UnbuiltRequest, build_response, ok_serialized,
};

pub fn handle_request(
    liver: &LiveForever,
    unbuilt: UnbuiltRequest,
    id: usize,
) -> Result<String, String> {
    match unbuilt {
        UnbuiltRequest::Ping => {
            build_response(id, Request::Ping, ok_serialized()).map_err(|e| e.to_string())
        }
        UnbuiltRequest::GetEntireTable { table_name } => {
            let payload: Result<GetEntireTableOut, DbError> = liver
                .get_data(GetDataIn {
                    table_name: table_name.clone(),
                    arguments: SelectArguments::Single(SelectArgument::All),
                    columns_to_read: Vec::new(),
                })
                .map(|out| GetEntireTableOut { rows: out.rows });
            build_response(
                id,
                Request::GetEntireTable { table_name },
                payload.to_payload(),
            )
            .map_err(|e| e.to_string())
        }
    }
}