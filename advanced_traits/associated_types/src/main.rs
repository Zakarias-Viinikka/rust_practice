#![warn(unused)]
//use std::fmt::format;
mod animation1;
use animation1::*;

fn main() {
    Animation::Animation1(Animation1Data {
        param1: "()".into(),
        param2: true,
    })
    .do_animation();
}

enum Animation {
    Animation1(Animation1Data), //{param1: string, param2: bool},
                                //Animation2(Animation2Data),// {water: i32, tree: i64},
}

/*impl DoAnimation for Animation1 {
    type params = (String, String);
    fn do_animation(&self, params: Self::params) -> String {
        format!("param 1 and 2: {}, {}", params.0, params.1)
    }
}*/
