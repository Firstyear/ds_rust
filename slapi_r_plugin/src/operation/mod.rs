//
// BEGIN COPYRIGHT BLOCK
// Copyright (C) 2016 Red Hat, Inc.
// All rights reserved.
//
// License: GPL (version 3 or any later version).
// See LICENSE for details. 
// END COPYRIGHT BLOCK
//
// Author: William Brown <wibrown@redhat.com>
//

#![warn(missing_docs)]

use libc;
use super::constants::*;

// Possible to make this a trait based type?

#[derive(Debug)]
#[allow(non_camel_case_types)]
/// Slapi_R_Operation is a container for the slapi_operation C type
/// This represents the current operation being processed by this thread.
/// By making this opaque, we prevent having to deal with C types in rust.
pub struct Slapi_R_Operation {
    slapi_operation: *const libc::c_void,
}

extern {
    fn operation_is_flag_set(op: *const libc::c_void, flag: isize) -> isize;
}

impl Slapi_R_Operation {
    /// Build a new Slapi_R_Operation from the pointer to the slapi_operation
    /// you should not need to do this yourself, you should be getting the
    /// Slapi_R_Operation from the pblock methods.
    pub fn new(slapi_operation: *const libc::c_void) -> Slapi_R_Operation {
        Slapi_R_Operation {
            slapi_operation: slapi_operation
        }
    }

    /// This is a private "c like" function which handles requesting the flag
    /// from the operation in directory Server.
    fn is_flag_set(&self, flag: isize) -> bool {
        let mut flag_set: isize = 0;
        unsafe {
            flag_set = operation_is_flag_set(self.slapi_operation, flag)
        }
        if flag_set != 0 {
            true
        } else {
            false
        }
    }

    pub fn is_replicated(&self) -> bool {
        self.is_flag_set(OP_FLAG_REPLICATED | OP_FLAG_LEGACY_REPLICATION_DN)
    }

    pub fn is_internal(&self) -> bool {
        self.is_flag_set(OP_FLAG_INTERNAL)
    }
}
