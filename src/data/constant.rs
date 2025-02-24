///! This module provides key constants for interacting with APIs

use super::types::IMAGE_DIRECTORY_ENTRY;

pub const IMAGE_NT_SIGNATURE: u32 = 17744u32;
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: IMAGE_DIRECTORY_ENTRY = 0u16;
pub const EXCEPTION_SINGLE_STEP: i32 = 0x80000004_u32 as _;
pub const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
pub const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
pub const CONTEXT_DEBUG_REGISTERS_AMD64: u32 = 1048592u32;
pub const CONTEXT_DEBUG_REGISTERS_X86: u32 = 65552u32;
pub const STANDARD_RIGHTS_REQUIRED: u32 = 0x000F0000;
pub const SYNCHRONIZE: u32 = 0x00100000;
pub const EVENT_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x3;
pub const WT_EXECUTEONLYONCE: u32 = 0x00000008;
pub const WT_EXECUTEINWAITTHREAD: u32 = 0x00000004;
pub const WT_EXECUTEINTIMERTHREAD: u32 = 0x00000020;