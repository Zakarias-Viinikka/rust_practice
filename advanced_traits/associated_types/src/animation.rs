pub trait AnimationData {
    type animation_data;
    async fn do_animation(animation_data: Self::animation_data);
}
