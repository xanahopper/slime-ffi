use std::mem::MaybeUninit;
use num_enum::{TryFromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CallStatus {
    Unknown = 0,
    Success = 1,
    Error = 2,
    Panic = 3,
}

#[repr(C)]
pub struct CallResult<T> {
    pub code: u8,
    pub data: MaybeUninit<T>
}

impl<T> CallResult<T> {
    pub fn new() -> CallResult<T> {
        CallResult {
            code: CallStatus::Unknown.into(),
            data: MaybeUninit::uninit(),
        }
    }

    pub fn success(data: Option<T>) -> CallResult<T> {
        CallResult {
            code: CallStatus::Success.into(),
            data: match data {
                Some(t) => MaybeUninit::new(t),
                _ => MaybeUninit::zeroed(),
            }
        }
    }

    pub fn error<R : Into<T>>(message: R) -> CallResult<T> {
        CallResult {
            code: CallResult::Error.into(),
            data: MaybeUninit::new(message.into()),
        }
    }

    pub fn panic<R: Into<T>>(info: R) -> CallResult<T> {
        CallResult {
            code: CallStatus::Panic.into(),
            data: MaybeUninit::new(info.into()),
        }
    }
}