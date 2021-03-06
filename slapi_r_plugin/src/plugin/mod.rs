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

use super::log::slapi_r_log_error;
use super::error::PluginOperationError;
use super::error::PluginRegistrationError;
use super::constants::LogLevel;
use super::constants;
use super::pblock::Slapi_R_PBlock;
use super::pblock::Slapi_PBlock_V3;
use super::pblock::Slapi_PBlock_Init_V3;

const SUBSYSTEM: &'static str = "slapi_r_plugin::plugin::mod";

/// Defines the functions that *must* be implemented by a version 3 compatible
/// plugin for directory server.
#[allow(non_camel_case_types)]
pub trait Slapi_Plugin_V3 {
    /// The function that initialises the plugin. May do internal or other initilasation
    fn init<T: Slapi_PBlock_Init_V3>( pb: T ) -> Result<(), PluginRegistrationError>;
    /// The function that starts plugin operations. After this point, the other callbacks
    /// may be triggered
    fn start<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError>;
    /// The function that stops and cleans up plugin operation. After this is called, no
    /// other callbacks will be called on the plugin.
    fn close<T: Slapi_PBlock_V3>( pb: &T ) -> Result<(), PluginOperationError>;
}

///
/// Type that represents the possible call backs from a plugin.
/// This is passed into slapi_pblock-...->plg_private, so that when our
/// Regisitered proxy functions are called, we are able to look up and access
/// the correct rust function call backs.
///
#[allow(non_camel_case_types)]
pub struct Slapi_R_Plugin_FN {
    /// An option type for a function callback that handles plugin start up.
    pub start: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles plugin close down.
    pub close: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles post search.
    pub post_search: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre bind
    pub pre_bind: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre unbind
    pub pre_unbind: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre search.
    pub pre_search: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre compare.
    pub pre_compare: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre modify.
    pub pre_modify: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre modrdn.
    pub pre_modrdn: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre add.
    pub pre_add: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre delete.
    pub pre_delete: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre abandon.
    pub pre_abandon: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre entry.
    pub pre_entry: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre referal.
    pub pre_referal: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre result.
    pub pre_result: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
    /// An option type for a function callback that handles pre extop.
    pub pre_extop: Option<fn(&Slapi_R_PBlock) -> Result<(), PluginOperationError>>,
}

///
/// Type that represents a plugin that we will register. After registration
/// this instance is destroyed.
///
#[allow(non_camel_case_types)]
pub struct Slapi_R_Plugin_Manager<'a> {
    /// The name of the plugin.
    pub name: &'a str,
    /// The slapi plugin version api that is provided by the plugin. Defaults to 3
    pub version: constants::PluginVersion,
    /// The set of optional callbacks that the plugin registers.
    pub functions: Slapi_R_Plugin_FN,
}

// These are wrappers that we register on a case by case for plugins
// The idea is to mask libslapd complexities from plugins, and give them
// A pure rust api view.


/// This is the internals of how most of the callbacks works. We extract the
/// function pointers, and execute the named pointer from cb_name. It wraps
/// the raw C types, uses the plg_private data to access our rust function
/// pointers, then we dispatch the call to the rust plugin. We then unwrap the
/// plugin result, translate it to an int that Directory Server can understand
/// and returns it.
///
/// This allows us to mask complexities of Directory Server interaction from
/// pure rust plugins, and gives us an avenue of abstraction to create changes
/// and rewrites in the future.
macro_rules! plugin_execute_fn_cb {
    ($cb_name:ident, $slapi_pblock:ident) => {{
        let pb: Slapi_R_PBlock = Slapi_R_PBlock::build($slapi_pblock);

        // Get the plugin private data we have registered to us.
        if pb.get_plugin_private::<Slapi_R_Plugin_FN>().is_none() {
            return PluginOperationError::Unknown.as_ds_isize()
        }

        let fn_ptrs: &Slapi_R_Plugin_FN = pb.get_plugin_private().unwrap();

        let func = match fn_ptrs.$cb_name {
            Some(f) => f,
            None => return PluginOperationError::Unknown.as_ds_isize(),
        };

        let result: Result<(), PluginOperationError> = func(&pb);
        // Unwrap the result, and give it to DS in a way it can understand.
        match result {
            Ok(_) => constants::LDAP_SUCCESS,
            Err(err) => err.as_ds_isize(),
        }
    }};
}


/// A callback wrapper for starting the plugin. This allows the
/// slapi_r_plugin_manager to start it's own internals, as well
/// as allowing the plugin itself to start up and setup any data
/// structures that it may require.
extern fn slapi_r_plugin_start_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(start, slapi_pblock)
}

/// A callback wrapper for stopping the plugin. This allows the
/// slapi_r_plugin_manager to free structures, such as the plugin
/// private data, and allows the plugin itself to close down.
/// This does *not* use the plugin_execute_fn_cb as it requires access to
/// destroy the plugin private data which the macro obscures.
extern fn slapi_r_plugin_close_cb(slapi_pblock: *const libc::c_void) -> isize {
    let pb: Slapi_R_PBlock = Slapi_R_PBlock::build(slapi_pblock);

    // First check if the plugin actually has any call backs to call on close
    let result_f = if !pb.get_plugin_private::<Slapi_R_Plugin_FN>().is_none() {
        let fn_ptrs: &Slapi_R_Plugin_FN = pb.get_plugin_private().unwrap();

        // TODO: Rewrite the other function callers in post search to use this
        match fn_ptrs.close {
            Some(f) => f(&pb),
            None => Err(PluginOperationError::Unknown),
        }
    } else {
        Ok(())
    };


    let result_p = pb.destroy_plugin_private();

    match (result_f, result_p) {
        (Ok(_), Ok(_)) => constants::LDAP_SUCCESS,
        (Ok(_), Err(e)) => e.as_ds_isize(),
        (Err(e), _) => e.as_ds_isize(),
    }
}

/// The callback wrapper for post_search
extern fn slapi_r_plugin_post_search_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(post_search, slapi_pblock)
}

/// The callback wrapper for pre_bind.
extern fn slapi_r_plugin_pre_bind_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_bind, slapi_pblock)
}

/// The callback wrapper for pre_unbind
extern fn slapi_r_plugin_pre_unbind_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_unbind, slapi_pblock)
}

/// The callback wrapper for pre_search_fn
extern fn slapi_r_plugin_pre_search_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_search, slapi_pblock)
}

/// The callback wrapper for pre_compare
extern fn slapi_r_plugin_pre_compare_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_compare, slapi_pblock)
}

/// The callback wrapper for pre_modify
extern fn slapi_r_plugin_pre_modify_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_modify, slapi_pblock)
}

/// The callback wrapper for pre_modrdn
extern fn slapi_r_plugin_pre_modrdn_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_modrdn, slapi_pblock)
}

/// The callback wrapper for pre_add
extern fn slapi_r_plugin_pre_add_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_add, slapi_pblock)
}

/// The callback wrapper for pre_delete
extern fn slapi_r_plugin_pre_delete_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_delete, slapi_pblock)
}

/// The callback wrapper for pre_abandon
extern fn slapi_r_plugin_pre_abandon_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_abandon, slapi_pblock)
}

/// The callback wrapper for pre_entry
extern fn slapi_r_plugin_pre_entry_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_entry, slapi_pblock)
}

/// The callback wrapper for pre_referal
extern fn slapi_r_plugin_pre_referal_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_referal, slapi_pblock)
}

/// The callback wrapper for pre_result
extern fn slapi_r_plugin_pre_result_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_result, slapi_pblock)
}

/// The callback wrapper for pre_extop
extern fn slapi_r_plugin_pre_extop_cb(slapi_pblock: *const libc::c_void) -> isize {
    plugin_execute_fn_cb!(pre_extop, slapi_pblock)
}


impl<'a> Slapi_R_Plugin_Manager<'a> {
    /// Builds a new Slapi_R_Plugin_Manager. The Rust plugin can then set values
    /// on this struct, and finally will call .register() to complete the
    /// plugins initialisation.
    pub fn new() -> Slapi_R_Plugin_Manager<'a> {

        let srpf: Slapi_R_Plugin_FN = Slapi_R_Plugin_FN{
            start: None,
            close: None,
            post_search: None,
            pre_bind: None,
            pre_unbind: None,
            pre_search: None,
            pre_compare: None,
            pre_modify: None,
            pre_modrdn: None,
            pre_add: None,
            pre_delete: None,
            pre_abandon: None,
            pre_entry: None,
            pre_referal: None,
            pre_result: None,
            pre_extop: None,
        };

        Slapi_R_Plugin_Manager {
            name: SUBSYSTEM,
            version: constants::PluginVersion::_03,
            functions: srpf,
        }
    }

    /// Completes the registration to Directory Server of the plugin. This is
    /// the *last* function you call when building a plugin in a plugin init.
    pub fn register<T: Slapi_PBlock_Init_V3>(self, pb: T) -> Result<(), PluginRegistrationError> {

        match slapi_r_log_error(
            LogLevel::FATAL,
            SUBSYSTEM,
            format!("Registering a rust plugin wrapper\n")
        ) {
            Ok(_) => {},
            Err(_) => return Err(PluginRegistrationError::LoggingError),
        };

        // Set the plugin api version
        pb.set_plugin_version(self.version as isize);

        // Set description:
        // I think this is optional ...

        if self.functions.post_search.is_some() {
            pb.set_plugin_post_search_fn(slapi_r_plugin_post_search_cb)
        }

        if self.functions.pre_bind.is_some() {
            pb.set_plugin_pre_bind_fn(slapi_r_plugin_pre_bind_cb)
        }

        if self.functions.pre_unbind.is_some() {
            pb.set_plugin_pre_unbind_fn(slapi_r_plugin_pre_unbind_cb)
        }

        if self.functions.pre_search.is_some() {
            pb.set_plugin_pre_search_fn(slapi_r_plugin_pre_search_cb)
        }

        if self.functions.pre_compare.is_some() {
            pb.set_plugin_pre_compare_fn(slapi_r_plugin_pre_compare_cb)
        }

        if self.functions.pre_modify.is_some() {
            pb.set_plugin_pre_modify_fn(slapi_r_plugin_pre_modify_cb)
        }

        if self.functions.pre_modrdn.is_some() {
            pb.set_plugin_pre_modrdn_fn(slapi_r_plugin_pre_modrdn_cb)
        }

        if self.functions.pre_add.is_some() {
            pb.set_plugin_pre_add_fn(slapi_r_plugin_pre_add_cb)
        }

        if self.functions.pre_delete.is_some() {
            pb.set_plugin_pre_delete_fn(slapi_r_plugin_pre_delete_cb)
        }

        if self.functions.pre_abandon.is_some() {
            pb.set_plugin_pre_abandon_fn(slapi_r_plugin_pre_abandon_cb)
        }

        if self.functions.pre_entry.is_some() {
            pb.set_plugin_pre_entry_fn(slapi_r_plugin_pre_entry_cb)
        }

        if self.functions.pre_referal.is_some() {
            pb.set_plugin_pre_referal_fn(slapi_r_plugin_pre_referal_cb)
        }

        if self.functions.pre_result.is_some() {
            pb.set_plugin_pre_result_fn(slapi_r_plugin_pre_result_cb)
        }

        if self.functions.pre_extop.is_some() {
            pb.set_plugin_pre_extop_fn(slapi_r_plugin_pre_extop_cb)
        }

        // We always register the start and close functions: We have some
        // checks in place to see if the rust plugin actually needs to use them
        // though.
        pb.set_plugin_start_fn(slapi_r_plugin_start_cb);
        pb.set_plugin_close_fn(slapi_r_plugin_close_cb);

        // Finally,we set a private structure of the functions we had registered
        pb.set_plugin_private(self.functions);

        // Only if the plugin implements lots of be_types do we
        // need slapi_plugin_register.

        // For now, a single plugin can register standalone.

        // If something went wrong, return this:
        //Err(error::PluginRegistrationError::Unknown)
        Ok(())
    }

}

/// This macro is imported by plugins to wrap their rust init functions.
/// This allows complete transparency to C types to the rust plugin.
#[macro_export]
macro_rules! slapi_r_plugin_init {
    ( $plugin_type:ident ) => (
        extern crate libc;
        use slapi_r_plugin::pblock::Slapi_R_PBlock;
        /// A static C function exported from the .so that Directory Server can
        /// find to complete plugin registration.
        #[no_mangle]
        pub extern fn slapi_r_plugin_init_fn(slapi_pblock: *mut libc::c_void) -> isize {
            let pb: Slapi_R_PBlock = Slapi_R_PBlock::build(slapi_pblock);
            match <$plugin_type as Slapi_Plugin_V3>::init(pb) {
                Ok(_) => constants::LDAP_SUCCESS,
                Err(e) => return e.as_ds_isize(),
            }
        }
    );
}

