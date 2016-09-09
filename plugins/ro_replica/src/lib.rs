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

//! ro_replica is a plugin that will prevent external and non-replica changes
//! to the directory server. Allows it to still replicate some changes back (ie
//! nsAccountLock) to replicate back to the cluster, but prevent modifications
//! to the server itself. Could be made to send a referal rather than rejection

#[macro_use]
extern crate slapi_r_plugin;

/// Definition of the plugin subsystem for logging
const SUBSYSTEM: &'static str = "plugins::ro_replica";

use slapi_r_plugin::log;
use slapi_r_plugin::error;
use slapi_r_plugin::constants;
use slapi_r_plugin::plugin::Slapi_R_Plugin_Manager;
use slapi_r_plugin::pblock::Slapi_R_PBlock;


///
/// This is the initialisation function for the Ro Replica Rust plugin.
///
/// This function will assign all the callbacks from this function into
/// Directory Server.
///
extern fn ro_replica_init( pb: Slapi_R_PBlock ) -> Result<(), error::PluginRegistrationError> {

    match log::slapi_r_log_error(constants::LogLevel::FATAL, SUBSYSTEM, format!("Hello rust!\n")) {
        Ok(_) => {},
        Err(_) => return Err(error::PluginRegistrationError::LoggingError),
    };

    let mut p_manager: Slapi_R_Plugin_Manager = Slapi_R_Plugin_Manager::new();

    match p_manager.register(pb) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }

}

// This is the magic that links a static no_mangle fn into the .so, and the rust
// init types
slapi_r_plugin_init!(ro_replica_init);


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
