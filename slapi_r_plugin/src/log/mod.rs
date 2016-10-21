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

use super::error::LoggingError;
use super::constants;
use super::constants::LogLevel;


extern {
    fn slapi_log_error(level: isize, system: *const c_char, message: *const c_char) -> isize;
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
pub fn slapi_r_log_error(level: LogLevel, subsystem: &str, message: String) -> Result<(), LoggingError> {
    let res: isize;
    let c_subsystem = CString::new(subsystem).unwrap();
    let c_message = CString::new(message).unwrap();
    unsafe {
        res = slapi_log_error(level as isize, c_subsystem.as_ptr(), c_message.as_ptr());
    }
    match res {
        constants::LDAP_SUCCESS => Ok(()),
        // Ds logging error codes here are a bit meaningless right now
        _ => Err(LoggingError::Unknown),
    }
}

/// This macro wraps and discards the result of a slapi_r_log_error call. It's good for
/// quick development, but you probably want the checked version that will return a
/// plugin error.
#[macro_export]
macro_rules! slapi_r_log_error_unchecked {
    ( $level:ident, $subsystem:ident, $message:ident ) => (
        match slapi_r_log_error($level, $subsystem, $message) {
            Ok(_) => {},
            Err(_) => {},
        };
    );
}

/// This macro is for plugins which return a PluginOperationError.
/// If the logging fails, we immedately return the error and the plugin stops
/// processing as this is a bad server state!
#[macro_export]
macro_rules! slapi_r_log_error_plugin {
    ( $level:expr, $subsystem:expr, $message:expr ) => (
        match slapi_r_log_error($level, $subsystem, $message) {
            Ok(_) => {},
            Err(_) => return Err(PluginOperationError::LoggingError),
        };
    );
}

