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

// This file contains all the error type enums for Dirsrv and wrappers
// to convert them into the INTS that ds expects.

/// This type is used when a plugin is being registered with directory server
/// It represents an error in the plugin that should be returned.
#[derive(Debug)]
pub enum PluginRegistrationError {
    /// An unknown error occured.
    Unknown,
    /// An error occured registering the post search function.
    PostSearchFN,
    /// An error occured attempting to log a message.
    LoggingError,
}

impl PluginRegistrationError {
    /// Convert the Rust error type to an isize that Directory Server can
    /// interpret.
    pub fn as_ds_isize(self) -> isize {
        match self {
            PluginRegistrationError::LoggingError => -1,
            PluginRegistrationError::PostSearchFN => -1,
            PluginRegistrationError::Unknown => -1,
        }
    }
}

/// This type is used by a plugin operation Ie post_search, to represent a
/// failure during the processing. It is returned to Directory Server.
#[derive(Debug)]
pub enum PluginOperationError {
    /// An unknown error occured.
    Unknown,
    /// An error occured attempting to log a message.
    LoggingError,
    /// Unwilling to perform
    UnwillingToPerform,
}

impl PluginOperationError {
    /// Convert the Rust error type to an isize that Directory Server can
    /// interpret.
    pub fn as_ds_isize(self) -> isize {
        match self {
            PluginOperationError::Unknown => -1,
            PluginOperationError::LoggingError => -2,
            // From ldap.h
            PluginOperationError::UnwillingToPerform => 0x35,
        }
    }
}

/// This type represents an error while working with the Slapi_R_PBlock.
/// This error may be from Directory Server and is returned to the Rust code.
#[derive(Debug)]
pub enum PBlockError {
    /// An unknown error occured.
    Unknown,
    /// An error occured setting a value into the C Slapi_PBlock structure
    SetFail,
    /// An error occured getting a value from the C Slapi_PBlock structure
    GetFail,
    /// An error occured destroying a value in the C Slapi_PBlock
    DestroyFail,
}

impl PBlockError {
    /// Convert the Rust error type to an isize that Directory Server can
    /// interpret.
    pub fn as_ds_isize(self) -> isize {
        match self {
            PBlockError::Unknown => -1,
            PBlockError::SetFail => -2,
            PBlockError::GetFail => -3,
            PBlockError::DestroyFail => -4,
        }
    }
}

/// This type represents an error while send a log to Directory server
/// This error may be from Directory Server and is returned to the Rust code.
#[derive(Debug)]
pub enum LoggingError {
    /// An unknown error occured.
    Unknown,
}


