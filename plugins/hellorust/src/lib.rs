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
    fn post_search<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError> {

        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust post_search!! \n"));

        // Get the search results
        match pb.get_search_result_entry() {
            Some(e) => slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Retrieved entry {:?} \n", e)),
            None => {},
        };

        Ok(())
    }

    ///
    /// This is a pre_bind plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_bind<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_bind!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_unbind plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_unbind<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_unbind!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_search plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_search<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_search!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_compare plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_compare<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_compare!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_modify plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_modify<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_modify!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_modrdn plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_modrdn<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_modrdn!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_add plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_add<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_add!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_delete plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_delete<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_delete!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_abandon plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_abandon<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_abandon!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_entry plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_entry<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_entry!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_referal plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_referal<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_referal!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_result plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_result<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_result!! \n"));
        Ok(())
    }

    ///
    /// This is a pre_extop plugin handler that logs the event has occured.
    ///
    /// You should never call this directly! It will be called by Directory Server
    /// as part of a plugin callback.
    ///
    fn pre_extop<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust pre_extop!! \n"));
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
        match slapi_r_log_error(LogLevel::INFO, SUBSYSTEM, format!("Hello rust!\n")) {
            Ok(_) => {},
            Err(_) => return Err(PluginRegistrationError::LoggingError),
        };

        // Build the R_Plugin_Manager. It will do the magic for us.
        let mut p_manager: Slapi_R_Plugin_Manager = Slapi_R_Plugin_Manager::new();

        p_manager.functions.start = Some(HellorustPlugin::start);
        p_manager.functions.close = Some(HellorustPlugin::close);
        p_manager.functions.post_search = Some(HellorustPlugin::post_search);
        p_manager.functions.pre_bind = Some(HellorustPlugin::pre_bind);
        p_manager.functions.pre_unbind = Some(HellorustPlugin::pre_unbind);
        p_manager.functions.pre_search = Some(HellorustPlugin::pre_search);
        p_manager.functions.pre_compare = Some(HellorustPlugin::pre_compare);
        p_manager.functions.pre_modify = Some(HellorustPlugin::pre_modify);
        p_manager.functions.pre_modrdn = Some(HellorustPlugin::pre_modrdn);
        p_manager.functions.pre_add = Some(HellorustPlugin::pre_add);
        p_manager.functions.pre_delete = Some(HellorustPlugin::pre_delete);
        p_manager.functions.pre_abandon = Some(HellorustPlugin::pre_abandon);
        p_manager.functions.pre_entry = Some(HellorustPlugin::pre_entry);
        p_manager.functions.pre_referal = Some(HellorustPlugin::pre_referal);
        p_manager.functions.pre_result = Some(HellorustPlugin::pre_result);
        p_manager.functions.pre_extop = Some(HellorustPlugin::pre_extop);

        match p_manager.register(pb) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// A start callback, that allows the plugin to initialise and start any required
    /// datastructures, etc.
    fn start<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust start callback \n") );
        Ok(())
    }

    /// A close callback, that allows the plugin to destroy any structuse made in
    /// the start callback
    fn close<T: Slapi_PBlock_V3>( _: &T ) -> Result<(), PluginOperationError> {
        slapi_r_log_error_plugin!(LogLevel::INFO, SUBSYSTEM, format!("Calling the hellorust close callback \n") );
        Ok(())
    }

}

// This is the magic that links a static no_mangle fn into the .so, and the rust
// init types
slapi_r_plugin_init!(HellorustPlugin);

