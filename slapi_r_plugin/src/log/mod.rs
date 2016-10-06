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

// use libc;
use std::ffi::CString;
use std::os::raw::c_char;

use super::constants;
use super::error;

extern {
    // For now we have to use slapi_log_err ...
    fn slapi_log_err(level: isize, system: *const c_char, message: *const c_char) -> isize;
}


/// Write a message to the Directory Server Error Log.
/// Directory Server performs *no formatting* of this message. You must use
/// format!() to message, to pre-format the message. This is useful anyway,
/// As you likely want to be using Rust formatting types.
///
/// # Failures
/// If a failure occurs, you should wrap this into a correct Error type for your
/// plugin or operation, ie PluginOperationError::LoggingError. This way the
/// error is transmitted correctly.
///
pub fn slapi_r_log_error(level: constants::LogLevel, subsystem: &str, message: String) -> Result<(), error::LoggingError> {
    let res: isize;
    let c_subsystem = CString::new(subsystem).unwrap();
    let c_message = CString::new(message).unwrap();
    unsafe {
        res = slapi_log_err(level as isize, c_subsystem.as_ptr(), c_message.as_ptr());
    }
    match res {
        constants::LDAP_SUCCESS => Ok(()),
        // Ds logging error codes here are a bit meaningless right now
        _ => Err(error::LoggingError::Unknown),
    }
}

