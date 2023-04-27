use std::thread::{current, ThreadId};
use crate::runtime::FrontendRuntime;

pub struct CommonContext {
    pub library_id: u64,
    pub thread_id: ThreadId,
}

impl CommonContext {
    pub fn new(library_id: u64) -> Self {
        Self {
            library_id,
            thread_id: current().id()
        }
    }

    pub fn is_current_thread(&self) -> bool {
        current().id() == self.thread_id
    }
}

pub struct CommonRuntime {
    pub ctx: CommonContext,
}

pub struct CommonDialectData {

}

pub fn create_common_runtime(library_id: u64, dialect_data: &CommonDialectData) -> FrontendRuntime {
    FrontendRuntime::C(CommonRuntime {
        ctx: CommonContext::new(library_id),
    })
}