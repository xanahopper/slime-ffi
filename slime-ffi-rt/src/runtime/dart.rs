use crate::runtime::FrontendRuntime;

pub struct DartRuntime;

pub struct DartInitializeData;

pub fn create_dart_runtime(library_id: u64, dialect_data: &DartInitializeData) -> FrontendRuntime {
    FrontendRuntime::DartRuntime(DartRuntime {})
}