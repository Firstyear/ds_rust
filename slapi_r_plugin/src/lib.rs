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

#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate libc;

///
/// Constants Module
///
/// This module defines the Directory Server library constants that are defined
/// in slapi-public.h and slapi-plugin.h.
///
pub mod constants;
///
/// Error Module
///
/// This module contains definitions of all the possible Error Results that
/// Directory Server or a plugin may return. It is used to facilitate conversion
/// of errors between C and Rust in native types, as well as allowing detailed
/// and specific error types to be created in Rust as Super types.
///
pub mod error;
///
/// Log Modules
///
/// This module wraps the Directory Server logging apis into types that Rust
/// can consume natively.
///
pub mod log;
///
/// Entry module.
///
/// This module wraps and represents entry types that directory server will
/// return as a result of search and other requests.
/// This module contains the struct representation of an entry, as well as an
/// iterator type for use during searches. The iterator is built by pblock.
pub mod entry;
///
/// Operation module
///
/// This module wraps and represents operations that the directory server is
/// currently processing. These are normally retrieved from the pblock, and
/// contain the current state information regarding the actions being performed
/// by this thread. It contains flags such as if the operation is from internal
/// or a replica, it contains the type of operation IE add, mod, search, and 
/// may contain other related data.
pub mod operation;
///
/// Pblock Module
///
/// This module wraps the Directory Server Parameter Block data structure.
/// The Slapi_PBlock is an important datastructure, used for nearly all
/// operations in the Directory Server. Requests and Responses are placed into
/// it. This module abstracts the Slapi_PBlock into Rust types, and wraps the
/// get and set functions into various types. This allows each type to be
/// changed in the future without affecting the stability of plugins.
///
pub mod pblock;
///
/// Plugin Module
/// 
/// This module is responsible for wrapping and proxying the interactions
/// plugins and Directory Server. The motivation is to simplify the design
/// and creation of plugins in Rust, so that they do not require exposure to the
/// details of the C types that Directory Server provides.
///
pub mod plugin;


