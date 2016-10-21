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

#![warn(missing_docs)]

//! Hello rust is a plugin proof of concept to demonstrate how native rust
//! can be used for the development of plugins to Ns Directory Server.

#[macro_use]
extern crate slapi_r_plugin;

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

/// Definition of the plugin subsystem for logging
const SUBSYSTEM: &'static str = "plugins::hellorust";

struct HellorustPlugin {}

impl HellorustPlugin {
    ///
    /// This is a post search operation plugin handler for the Hello Rust plugin.
    /// The post search logs a message to the error log and may in the future add
    /// a value to the result of a search.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn post_op<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError> {

        slapi_r_log_error_plugin!(LogLevel::FATAL, SUBSYSTEM, format!("Calling the actual rust post_op!! \n"));

        // Check if internal operation

        // Get the search results
        match pb.get_search_result_entry() {
            Some(e) => slapi_r_log_error_plugin!(LogLevel::FATAL, SUBSYSTEM, format!("Retrieved entry {:?} \n", e)),
            None => {},
        };
        // Display them? 

        Ok(())
    }
}

impl Slapi_Plugin_V3 for HellorustPlugin {
    ///
    /// This is the initialisation function for the Hello Rust plugin.
    ///
    /// This function will assign all the callbacks from this function into
    /// Directory Server.
    ///
    fn init<T: Slapi_PBlock_Init_V3>( pb: T ) -> Result<(), PluginRegistrationError> {
        // need to do something better here ...
        match slapi_r_log_error(LogLevel::FATAL, SUBSYSTEM, format!("Hello rust!\n")) {
            Ok(_) => {},
            Err(_) => return Err(PluginRegistrationError::LoggingError),
        };

        // Build the R_Plugin_Manager. It will do the magic for us.
        let mut p_manager: Slapi_R_Plugin_Manager = Slapi_R_Plugin_Manager::new();

        p_manager.functions.start = Some(HellorustPlugin::start);
        p_manager.functions.close = Some(HellorustPlugin::close);
        p_manager.functions.post_search = Some(HellorustPlugin::post_op);

        match p_manager.register(pb) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// A start callback, that allows the plugin to initialise and start any required
    /// datastructures, etc.
    fn start<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::FATAL, SUBSYSTEM, format!("Calling the hellorust start callback \n") );
        Ok(())
    }

    /// A close callback, that allows the plugin to destroy any structuse made in
    /// the start callback
    fn close<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::FATAL, SUBSYSTEM, format!("Calling the hellorust close callback \n") );
        Ok(())
    }

}

// This is the magic that links a static no_mangle fn into the .so, and the rust
// init types
slapi_r_plugin_init!(HellorustPlugin);

