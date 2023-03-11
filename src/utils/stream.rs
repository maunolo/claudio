use std::sync::{Arc, Mutex};

pub enum StreamStatus {
    Stopped,
    Running,
}

pub type StreamStatusHandle = Arc<Mutex<StreamStatus>>;

pub trait StreamStatusImpl {
    fn new_handle(status: StreamStatus) -> Self;

    fn play(&self);

    fn pause(&self);

    fn is_running(&self) -> bool;
}

impl StreamStatusImpl for StreamStatusHandle {
    fn new_handle(status: StreamStatus) -> Self {
        Arc::new(Mutex::new(status))
    }

    fn play(&self) {
        if let Ok(mut guard) = self.try_lock() {
            let stream_status = &mut *guard;
            if matches!(stream_status, StreamStatus::Stopped) {
                *stream_status = StreamStatus::Running;
            }
        }
    }

    fn pause(&self) {
        if let Ok(mut guard) = self.lock() {
            let stream_status = &mut *guard;
            if matches!(stream_status, StreamStatus::Running) {
                *stream_status = StreamStatus::Stopped;
            }
        }
    }

    fn is_running(&self) -> bool {
        if let Ok(guard) = self.lock() {
            let stream_status = &*guard;
            matches!(stream_status, StreamStatus::Running)
        } else {
            false
        }
    }
}
