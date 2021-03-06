// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;
use std::error::Error;
use maskerad_memory_allocators::allocation_error::AllocationError as MaskeradMemAllocError;

#[derive(Debug)]
pub enum AllocationError {
    StackError(String, MaskeradMemAllocError),
}

unsafe impl Send for AllocationError {}
unsafe impl Sync for AllocationError {}

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AllocationError::StackError(ref desc, _) => {
                write!(f, "Memory stack error: {}", desc)
            },
        }
    }
}

impl Error for AllocationError {
    fn description(&self) -> &str {
        match self {
            &AllocationError::StackError(_, _) => {
                "StackError"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &AllocationError::StackError(_, ref stack_error) => {
                Some(stack_error)
            },
        }
    }
}

pub type AllocationResult<T> = Result<T, AllocationError>;

impl From<MaskeradMemAllocError> for AllocationError {
    fn from(error: MaskeradMemAllocError) -> Self {
        AllocationError::StackError(String::from("Error while allocating data in a memory stack."), error)
    }
}