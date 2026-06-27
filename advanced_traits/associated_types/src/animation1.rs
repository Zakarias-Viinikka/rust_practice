use crate::animation::AnimationData;

pub struct Animation1Data {
    pub param1: String,
    pub param2: bool,
}

impl AnimationData for Animation1Data {
    type animation_data = Animation1Data;
    async fn do_animation(animation_data) {
        //do animation. maybe match "part 1 of animation => do_part1"
    }
}
