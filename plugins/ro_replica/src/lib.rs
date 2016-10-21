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

use slapi_r_plugin::constants;
use slapi_r_plugin::constants::LogLevel;
use slapi_r_plugin::entry::Slapi_R_Entry;
use slapi_r_plugin::error::PluginOperationError;
use slapi_r_plugin::error::PluginRegistrationError;
use slapi_r_plugin::log::slapi_r_log_error;
use slapi_r_plugin::plugin::Slapi_R_Plugin_Manager;
use slapi_r_plugin::plugin::Slapi_Plugin_V3;
use slapi_r_plugin::pblock::Slapi_PBlock_V3;
use slapi_r_plugin::pblock::Slapi_PBlock_Init_V3;


struct RoReplicaPlugin {}

impl Slapi_Plugin_V3 for RoReplicaPlugin {
    ///
    /// This is the initialisation function for the Ro Replica Rust plugin.
    ///
    /// This function will assign all the callbacks from this function into
    /// Directory Server.
    ///
    fn init<T: Slapi_PBlock_Init_V3>( pb: T ) -> Result<(), PluginRegistrationError> {

        match slapi_r_log_error(LogLevel::FATAL, SUBSYSTEM, format!("ro_replica started\n")) {
            Ok(_) => {},
            Err(_) => return Err(PluginRegistrationError::LoggingError),
        };

        let mut p_manager: Slapi_R_Plugin_Manager = Slapi_R_Plugin_Manager::new();

        p_manager.functions.start = Some(RoReplicaPlugin::start);
        p_manager.functions.close = Some(RoReplicaPlugin::close);

        match p_manager.register(pb) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }

    }

    /// A start callback, that allows the plugin to initialise and start any required
    /// datastructures, etc.
    fn start<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::FATAL, SUBSYSTEM, format!("Calling the ro_replica start callback \n") );
        Ok(())
    }

    /// A close callback, that allows the plugin to destroy any structuse made in
    /// the start callback
    fn close<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::FATAL, SUBSYSTEM, format!("Calling the ro_replica close callback \n") );
        Ok(())
    }

}

// This is the magic that links a static no_mangle fn into the .so, and the rust
// init types
slapi_r_plugin_init!(RoReplicaPlugin);


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
