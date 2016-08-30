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

#[derive(Debug)]
#[allow(non_camel_case_types)]
/// Slapi_R_Entry is a container for a slapi_entry C type.
/// This represents an entry that has been retrieved from the revelant
/// backend as part of an operation.
/// By making this opaque, we create rewrite and improvement possibilities
pub struct Slapi_R_Entry {
    // Will need to contain a pointer to the real Slapi_Entry
    slapi_entry: *const libc::c_void,
}

// #[derive(Debug)]
// pub struct Slapi_R_Entries {
    // contains the current pointer, and a way to access what is next.
    // Probably is done through stashing the pblock, and calling
    // get search_result_entry?
// }

impl Slapi_R_Entry {
    /// Build a new Slapi_R_Entry from a pointer to the slapi_entry
    /// The slapi_entry is created in a number of paths through DS,
    /// so you should not need to do this yourself.
    pub fn new(slapi_entry: *const libc::c_void) -> Slapi_R_Entry {
        Slapi_R_Entry { slapi_entry: slapi_entry }
    }
}
