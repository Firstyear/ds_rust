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

use slapi_r_plugin::log;
use slapi_r_plugin::error;
use slapi_r_plugin::constants;
use slapi_r_plugin::plugin::Slapi_R_Plugin_Manager;
use slapi_r_plugin::pblock::Slapi_R_PBlock;
use slapi_r_plugin::entry::Slapi_R_Entry;

/// Definition of the plugin subsystem for logging
const SUBSYSTEM: &'static str = "plugins::hellorust";

/// A start callback, that allows the plugin to initialise and start any required
/// datastructures, etc.
extern fn hellorust_start( pb: &Slapi_R_PBlock ) -> Result<(), error::PluginOperationError> {
    match log::slapi_r_log_error(constants::LogLevel::FATAL, SUBSYSTEM, format!("Calling the hellorust start callback \n") ) {
        Ok(_) => {},
        Err(_) => return Err(error::PluginOperationError::LoggingError),
    };
    Ok(())
}

/// A close callback, that allows the plugin to destroy any structuse made in
/// the start callback
extern fn hellorust_close( pb: &Slapi_R_PBlock ) -> Result<(), error::PluginOperationError> {
    match log::slapi_r_log_error(constants::LogLevel::FATAL, SUBSYSTEM, format!("Calling the hellorust close callback \n") ) {
        Ok(_) => {},
        Err(_) => return Err(error::PluginOperationError::LoggingError),
    };
    Ok(())
}

///
/// This is a post search operation plugin handler for the Hello Rust plugin.
/// The post search logs a message to the error log and may in the future add
/// a value to the result of a search.
///
/// You should never call this directly! It will be called by Directory Server
/// as part of a plugin callback.
///
extern fn hellorust_post_op( pb: &Slapi_R_PBlock ) -> Result<(), error::PluginOperationError> {
    match log::slapi_r_log_error(constants::LogLevel::FATAL, SUBSYSTEM, format!("Calling the actual rust post_op!! \n") ) {
        Ok(_) => {},
        Err(_) => return Err(error::PluginOperationError::LoggingError),
    };

    // Check if internal operation

    // Get the search results
    match pb.get_search_result_entry() {
        Some(e) => match log::slapi_r_log_error(constants::LogLevel::FATAL, SUBSYSTEM, 
                    format!("Retrieved entry {:?} \n", e) )
            {
                Ok(_) => {},
                Err(_) => return Err(error::PluginOperationError::LoggingError),
            },
        None => {},
    };
    // Display them? 

    // Inject the rust crab

    Ok(())
}

///
/// This is the initialisation function for the Hello Rust plugin.
///
/// This function will assign all the callbacks from this function into
/// Directory Server.
///
extern fn hellorust_init( pb: Slapi_R_PBlock ) -> Result<(), error::PluginRegistrationError> {
    match log::slapi_r_log_error(constants::LogLevel::FATAL, SUBSYSTEM, format!("Hello rust!\n")) {
        Ok(_) => {},
        Err(_) => return Err(error::PluginRegistrationError::LoggingError),
    };

    // Build the R_Plugin_Manager. It will do the magic for us.
    let mut p_manager: Slapi_R_Plugin_Manager = Slapi_R_Plugin_Manager::new();

    p_manager.functions.start = Some(hellorust_start);
    p_manager.functions.close = Some(hellorust_close);
    p_manager.functions.post_search = Some(hellorust_post_op);

    match p_manager.register(pb) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// This is the magic that links a static no_mangle fn into the .so, and the rust
// init types
slapi_r_plugin_init!(hellorust_init);

