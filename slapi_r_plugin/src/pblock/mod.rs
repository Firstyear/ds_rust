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
use std::ptr;
use std::mem;
use super::constants::*;
use super::error::PBlockError;
use super::entry::Slapi_R_Entry;
// use std::ops::Drop;

// Wrapper for the pblock in rust.
// By wrapping this, rather than passing libc::c_void around, it makes it opaque
// giving us future re-write / modification options.

/// Slapi_PBlock_Init_V3 defines the set of functions that a version 3 plugin
/// requires access to for correct installation and initialisation of the plugin.
/// This will only be used in the plugins' init function.
#[allow(non_camel_case_types)]
pub trait Slapi_PBlock_Init_V3 {
    /// Get the plugin api version
    fn get_plugin_version(&self) -> Option<isize>;
    /// Set the plugin api version
    fn set_plugin_version(&self, version: isize);
    /// Set the plugin's closing function handler. This is used by init the macros
    fn set_plugin_close_fn(&self, func: extern fn(*const libc::c_void) -> isize);
    /// Set the plugin's start function handler. This is used by init the macros
    fn set_plugin_start_fn(&self, func: extern fn(*const libc::c_void) -> isize);
    /// Set the plugin's post search function handler. This is used by init the macros
    fn set_plugin_post_search_fn(&self, func: extern fn(*const libc::c_void) -> isize);
    /// Set the private data into the plugin.
    fn get_plugin_private<T>(&self) -> Option<&T>;
    /// Get the private data from the plugin.
    fn set_plugin_private<T>(&self, value: T);
    /// Destroy the private data stored in the plugin
    fn destroy_plugin_private(&self) -> Result<(), PBlockError>;
}

/// Slapi_PBlock_V3 defines the set of functions that version 3 plugins expect
/// to be present. This allows us to re-implement the version 3 pblock to plugins
/// very easily, as well as allowing testing to occur.
#[allow(non_camel_case_types)]
pub trait Slapi_PBlock_V3 {
    fn get_search_result_entry(&self) -> Option<Slapi_R_Entry>;
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
/// Slapi_R_PBlock is a container that contains the slapi_pblock C type.
/// Functions of Slapi_R_PBlock wrap and interact with the native C PBlock.
/// By making this opaque, it gives us the ability to rework or improve 
/// the structure without breaking the API of Slapi_R_PBlock.
pub struct Slapi_R_PBlock {
    slapi_pblock: *const libc::c_void,
}

extern {
    fn slapi_pblock_new() -> *const libc::c_void;
    fn slapi_pblock_init(pb: *const libc::c_void);
    fn slapi_pblock_destroy(pb: *const libc::c_void);

    fn slapi_pblock_get(pb: *const libc::c_void, arg: isize, value: *const libc::c_void);
    fn slapi_pblock_set(pb: *const libc::c_void, arg: isize, value: *const libc::c_void);
}


impl Slapi_R_PBlock {
    /// Creates a new Slapi_R_PBlock, with a new C PBlock structure.
    ///
    /// # Panics
    /// Directory Server's pblock functions DO NOT check for null pointer deref
    /// It is possible while using a new Slapi_R_PBlock to trigger a SIGSEGV
    /// as pblock internal structures such as pb_plugin are *not* allocated.
    pub fn new() -> Slapi_R_PBlock {
        unsafe {
            Slapi_R_PBlock { slapi_pblock: slapi_pblock_new() }
        }
    }

    /// Builds a new Slapi_R_PBlock from a Slapi_PBlock C pointer.
    /// It is very likely, that this is the function you want to use if you are
    /// woring on slapi_r_plugin internals.
    pub fn build( slapi_pblock: *const libc::c_void ) -> Slapi_R_PBlock {
        Slapi_R_PBlock { slapi_pblock: slapi_pblock }
    }

    /// This will wipe and reset the internal contents of the Slapi_PBlock C
    /// structure.
    pub fn init(self) {
        unsafe {
            slapi_pblock_init(self.slapi_pblock)
        }
    }

    /// This will free and deallocate all parts o fthe Slapi_PBlock C structure.
    /// It is the *last* thing you should use.
    ///
    /// # Panics
    /// After calling this function, if you access any other function of this
    /// struct, you will likely cause a SIGSEGV
    pub fn destroy(self) {
        unsafe {
            slapi_pblock_destroy(self.slapi_pblock)
        }
    }

    /// This is an internal wrapper allowing the retrival of an isize from the
    /// Slapi_PBlock.
    fn _get_isize(&self, pblock_type: isize) -> Option<isize> {
        let mut value: isize = 0;
        let value_ptr: *const libc::c_void = &mut value as *const _ as *const libc::c_void;
        unsafe {
            // There is potentially a crash here as pblock makes no check if
            // SLAPI_PLUGIN_TYPE has been set ...
            slapi_pblock_get(self.slapi_pblock, pblock_type, value_ptr);
        }
        Some(value)
    }

    /// This is an internal wrapper allowing the setting of an isize from the
    /// Slapi_PBlock.
    fn _set_isize(&self, pblock_type: isize, mut value: isize) {
        // There is a potential crash here as pblock makes no check if pblock->pb_plugin is alloced
        let value_ptr: *const libc::c_void = &mut value as *const _ as *const libc::c_void;
        unsafe {
            // This value is copied as it's an int.
            slapi_pblock_set(self.slapi_pblock, pblock_type, value_ptr);
        }
    }

    /// This is an internal wrapper allowing the retrival of platform struct
    /// pointer from the Slapi_PBlock.
    fn _get_void_ptr(&self, pblock_type: isize) -> Option<*const libc::c_void> {
        let mut value: *mut libc::c_void = ptr::null::<libc::c_void>() as *mut libc::c_void; // = &mut value;
        // Make a pointer to our pointer ....
        let value_ptr: *const libc::c_void = &mut value as *const _ as *const libc::c_void;
        unsafe {
            // There is potentially a crash here as pblock makes no check if
            // SLAPI_PLUGIN_TYPE has been set ...
            slapi_pblock_get(self.slapi_pblock, pblock_type, value_ptr);
        }
        if value_ptr.is_null() || value.is_null() {
            None
        } else  {
            Some(value)
        }
    }

    /// This is an internal wrapper allowing the setting of a platform struct
    /// into the Slapi_PBlock.
    fn _set_void_ptr(&self, pblock_type: isize, value: libc::c_void) {
        let value_ptr: *const libc::c_void = &value;
        unsafe {
            slapi_pblock_set(self.slapi_pblock, pblock_type, value_ptr);
        }
    }

    //  These only accept / return fn that take the pb as a single arg!
    //fn _get_pb_fn_ptr(&self, pblock_type: isize) -> Option<extern fn(*mut libc::c_void) -> isize> {
    //    None
    //}

    /// This is an internal wrapper allowing the setting of a function pointer
    // into the Slapi_PBlock IFP type.
    fn _set_pb_fn_ptr(&self, pblock_type: isize, ptr: extern fn(*const libc::c_void) -> isize) {
        let value_ptr: *const libc::c_void = ptr as *const libc::c_void;
        unsafe {
            slapi_pblock_set(self.slapi_pblock, pblock_type, value_ptr);
        }
    }

    // NOTE: The bellow will probably become part of the v3 interface.

    /// This will retrieve the value of SLAPI_PLUGIN_TYPE, such as BE_TXN,
    /// POST_OP etc.
    pub fn get_plugin_type(&self) -> Option<isize> {
        self._get_isize(SLAPI_PLUGIN_TYPE)
    }

    /// This will set the plugin type value as SLAPI_PLUGIN_TYPE, such as
    /// BE_TXN, POST_OP, etc.
    pub fn set_plugin_type(&self, plugin_type: isize) {
        self._set_isize(SLAPI_PLUGIN_TYPE, plugin_type)
    }

    /// This will get the operation return code as SLAPI_PLUGIN_OPRETURN
    pub fn get_plugin_opreturn(&self) -> Option<isize> {
        self._get_isize(SLAPI_PLUGIN_OPRETURN)
    }

    /// This will set the operation return code as SLAPI_PLUGIN_OPRETURN
    pub fn set_plugin_opreturn(&self, opreturn: isize) {
        self._set_isize(SLAPI_PLUGIN_OPRETURN, opreturn)
    }


}

impl Slapi_PBlock_Init_V3 for Slapi_R_PBlock {
    /// This will get the plugin api version from SLAPI_PLUGIN_VERSION
    /// See also constants::PluginVersion
    fn get_plugin_version(&self) -> Option<isize> {
        self._get_isize(SLAPI_PLUGIN_VERSION)
    }

    /// This will set the plugin api version from SLAPI_PLUGIN_VERSION
    /// See also constants::PluginVersion
    fn set_plugin_version(&self, version: isize) {
        self._set_isize(SLAPI_PLUGIN_VERSION, version)
    }

    /// This will set the close plugin callback handler as
    /// SLAPI_PLUGIN_CLOSE_FN. You should *not* call this directly
    /// as the Slapi_R_Plugin_Manager will handle this for you.
    fn set_plugin_close_fn(&self, func: extern fn(*const libc::c_void) -> isize) {
        self._set_pb_fn_ptr(SLAPI_PLUGIN_CLOSE_FN, func)
    }

    /// This will set the start plugin callback handler as
    /// SLAPI_PLUGIN_START_FN. You should *not* call this directly
    /// as the Slapi_R_Plugin_Manager will handle this for you.
    fn set_plugin_start_fn(&self, func: extern fn(*const libc::c_void) -> isize) {
        self._set_pb_fn_ptr(SLAPI_PLUGIN_START_FN, func)
    }

    /// This will set the post search operation plugin callback handler as
    /// SLAPI_PLUGIN_POST_SEARCH_FN. You should *not* call this directly
    /// as the Slapi_R_Plugin_Manager will handle this for you.
    fn set_plugin_post_search_fn(&self, func: extern fn(*const libc::c_void) -> isize) {
        self._set_pb_fn_ptr(SLAPI_PLUGIN_POST_SEARCH_FN, func)
    }

    /// This will get a pointer to a structure stored in the plugin private data
    /// stash, from Slapi_PBlock.pb_plugin->plg_private. SLAPI_PLUGIN_PRIVATE
    /// You should *never* call this directly, as certain parts of the
    /// Slapi_R_Plugin_Manager rely on this data being un-tampered.
    fn get_plugin_private<T>(&self) -> Option<&T> {
                                            // Should this value here be a ptr::null? 
        let mut value: *mut libc::c_void = 0usize as *mut libc::c_void;
        let value_ptr: *const libc::c_void = &mut value as *const _ as *const libc::c_void;
        unsafe {
            // There is potentially a crash here as pblock makes no check if
            // SLAPI_PLUGIN_TYPE has been set ...
            slapi_pblock_get(self.slapi_pblock, SLAPI_PLUGIN_PRIVATE, value_ptr);
            Some(&**(value_ptr as *const *const T))
        }
    }

    /// This will memory copy and set a pointer to the memory of a generic
    /// Rust type into a 'box' stored in the Slapi_PBlock for this plugin
    /// instance.
    /// Certain parts of Slapi_R_Plugin_Manager rely on this, so you should
    /// *never* call this directly.
    fn set_plugin_private<T>(&self, value: T) {
        unsafe {
            let value_ptr = libc::malloc(mem::size_of::<T>() as libc::size_t) as *mut T;
            assert!(!value_ptr.is_null());
            // This does a memcpy
            ptr::write(value_ptr, value);
            let f_value_ptr: *const libc::c_void = value_ptr as *const libc::c_void;
            slapi_pblock_set(self.slapi_pblock, SLAPI_PLUGIN_PRIVATE, f_value_ptr);
        }
    }

    /// This will destroy (free) the contents of the pointer stored
    /// within the Slapi_PBlock instance.
    /// Certain parts of Slapi_R_Plugin_Manager rely on this, so you should
    /// *never* call this directly.
    fn destroy_plugin_private(&self) -> Result<(), PBlockError> {
        let mut value: *mut libc::c_void = 0usize as *mut libc::c_void;
        let value_ptr: *const libc::c_void = &mut value as *const _ as *const libc::c_void;
        unsafe {
            // There is potentially a crash here as pblock makes no check if
            // SLAPI_PLUGIN_TYPE has been set ...
            slapi_pblock_get(self.slapi_pblock, SLAPI_PLUGIN_PRIVATE, value_ptr);
            if !value_ptr.is_null() {
                libc::free(value);
            }
        }
        Ok(())
    }

}

impl Slapi_PBlock_V3 for Slapi_R_PBlock {
    // pub fn set_search_result_entry()

    /// This will retrieve the next Slapi_R_Entry from the result set
    /// in the pblock.
    fn get_search_result_entry(&self) -> Option<Slapi_R_Entry> {
        match self._get_void_ptr(SLAPI_SEARCH_RESULT_ENTRY) {
            Some(p) => Some(Slapi_R_Entry::new(p)),
            None => None,
        }
    }
}


// error: cannot move out of type `pblock::Slapi_R_PBlock`, which defines the `Drop` trait [E0509]
// impl Drop for Slapi_R_PBlock {
//     fn drop(&mut self) {
//         unsafe {
//             slapi_pblock_destroy(self.slapi_pblock);
//         }
//     }
// }


#[test]
fn test_slapi_r_pblock_new() {
    let pb: Slapi_R_PBlock = Slapi_R_PBlock::new();
    {
        pb.set_plugin_opreturn(SLAPI_PLUGIN_BE_TXN_PRE_ADD_FN);
        let v = pb.get_plugin_opreturn();
        assert_eq!(Some(SLAPI_PLUGIN_BE_TXN_PRE_ADD_FN), v);
    }
    assert!(true);
    pb.destroy();
}


