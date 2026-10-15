use crate::requests;
use db_wrapper::mascot::LiveForever;
use queue::Job;
use tokio::sync::mpsc::UnboundedReceiver;

// runs in the background: takes the next job off the important line, or the normal line
// if important is empty, runs it against the database, and sends the answer back
pub fn start(
    mut important_rx: UnboundedReceiver<Job>,
    mut normal_rx: UnboundedReceiver<Job>,
    liver: LiveForever,
) {
    tokio::spawn(async move {
        loop {
            let job = tokio::select! {
                biased;
                Some(job) = important_rx.recv() => job,
                Some(job) = normal_rx.recv() => job,
                else => break,
            };
            let result = requests::handle_request(&liver, job.unbuilt, job.id);
            let _ = job.reply.send(result);
        }
    });
}
