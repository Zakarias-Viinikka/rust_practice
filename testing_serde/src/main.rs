use testing_serde::shared_data;

//the outside world should be the caller. that's the pov of what the methods should do
fn main() {
    let outside_caller = OutsideCaller {
        outsider_data: shared_data::DataIn {
            data1: "Hello".to_string(),
            data2: " World".to_string(),
        },
    };
    //outsider turns their data into "transport mode"
    let outsider_data = serde_json::to_string(&outside_caller.outsider_data).unwrap();

    //this represents the data stored internally. could be the data inside a db or a textfile but
    // here it's just stored in a variable in main()
    let data_stored_internally: shared_data::DataIn;

    //outsider gives their data to the api
    // and the api turns it into "normal data" and stores it
    data_stored_internally = take_my_data_mr_api(outsider_data);

    //now outside caller wants it back
    // we turn the "normal data" to "transport mode"
    let data_ready_to_be_returned = can_i_have_data_back_mr_api(data_stored_internally);
    println!(
        "data being transported back to caller: {:?}",
        data_ready_to_be_returned
    );
    //outsider has received data and turns it into "normal mode"
    let data_outsider_turns_into_normal =
        serde_json::from_str::<shared_data::DataOut>(&data_ready_to_be_returned).unwrap();
    println!(
        "data arrived at caller and now looks like this: {:?}",
        data_outsider_turns_into_normal
    );
}

fn can_i_have_data_back_mr_api(data_stored_internally: shared_data::DataIn) -> String {
    let data_out = data_in_transformed_to_data_out(data_stored_internally);
    serde_json::to_string(&data_out).unwrap()
}

fn take_my_data_mr_api(data_in_transport_form: String) -> shared_data::DataIn {
    let proper_data = serde_json::from_str::<shared_data::DataIn>(&data_in_transport_form).unwrap();
    proper_data
}

struct OutsideCaller {
    outsider_data: shared_data::DataIn,
}

fn data_in_transformed_to_data_out(data: shared_data::DataIn) -> shared_data::DataOut {
    let (data1, data2) = (data.data1, data.data2);
    shared_data::DataOut {
        data_combined: data1 + &data2,
    }
}
