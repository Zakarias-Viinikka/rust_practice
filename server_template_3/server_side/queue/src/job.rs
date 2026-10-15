use server_protocol::UnbuiltRequest;
use tokio::sync::oneshot;

pub enum Priority {
    Important,
    Normal,
}

pub struct Job {
    pub unbuilt: UnbuiltRequest,
    pub id: usize,
    pub reply: oneshot::Sender<Result<String, String>>,
}