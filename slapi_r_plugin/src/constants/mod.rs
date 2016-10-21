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

/// A successful operation. Most Directory Server fuctions expect this.
pub const LDAP_SUCCESS: isize = 0;


// Constants used for plugin types! These should be an enum one day. This is not complete!

/// PBlock constant for retrieving the next entry of an entry result list.
pub const SLAPI_SEARCH_RESULT_ENTRY: isize = 194;
/// PBlock constant for registering the close function for a plugin.
pub const SLAPI_PLUGIN_CLOSE_FN: isize = 210;
/// PBlock constant for registering the start function for a plugin.
pub const SLAPI_PLUGIN_START_FN: isize = 212;

/// PBlock constant for registering a
pub const SLAPI_PLUGIN_PRE_BIND_FN: isize = 401;
/// PBlock constant for registering a pre UNBIND operation.
pub const SLAPI_PLUGIN_PRE_UNBIND_FN: isize = 402;
/// PBlock constant for registering a pre SEARCH operation.
pub const SLAPI_PLUGIN_PRE_SEARCH_FN: isize = 403;
/// PBlock constant for registering a pre COMPARE operation.
pub const SLAPI_PLUGIN_PRE_COMPARE_FN: isize = 404;
/// PBlock constant for registering a pre MODIFY operation.
pub const SLAPI_PLUGIN_PRE_MODIFY_FN: isize = 405;
/// PBlock constant for registering a pre MODRDN operation.
pub const SLAPI_PLUGIN_PRE_MODRDN_FN: isize = 406;
/// PBlock constant for registering a pre ADD operation.
pub const SLAPI_PLUGIN_PRE_ADD_FN: isize = 407;
/// PBlock constant for registering a pre DELETE operation.
pub const SLAPI_PLUGIN_PRE_DELETE_FN: isize = 408;
/// PBlock constant for registering a pre ABANDON operation.
pub const SLAPI_PLUGIN_PRE_ABANDON_FN: isize = 409;
/// PBlock constant for registering a pre ENTRY operation.
pub const SLAPI_PLUGIN_PRE_ENTRY_FN: isize = 410;
/// PBlock constant for registering a pre REFERAL operation.
pub const SLAPI_PLUGIN_PRE_REFERAL_FN: isize = 411;
/// PBlock constant for registering a pre RESULT operation.
pub const SLAPI_PLUGIN_PRE_RESULT_FN: isize = 412;
/// PBlock constant for registering a pre EXTOP operation.
pub const SLAPI_PLUGIN_PRE_EXTOP_FN: isize = 413;
/// PBlock constant for registering a pre BE_TXN add operation.
pub const SLAPI_PLUGIN_BE_TXN_PRE_ADD_FN: isize = 460;
/// PBlock constant for registering a pre BE_TXN modify operation.
pub const SLAPI_PLUGIN_BE_TXN_PRE_MODIFY_FN: isize = 461;
/// PBlock constant for registering a pre BE_TXN modrdn operation.
pub const SLAPI_PLUGIN_BE_TXN_PRE_MODRDN_FN: isize = 462;
/// PBlock constant for registering a pre BE_TXN delete operation.
pub const SLAPI_PLUGIN_BE_TXN_PRE_DELETE_FN: isize = 463;
/// PBlock constant for registering a pre BE_TXN delete tombstone operatation.
pub const SLAPI_PLUGIN_BE_TXN_PRE_DELETE_TOMBSTONE_FN: isize = 464;
/// PBlock constant for registering a post search operation.
pub const SLAPI_PLUGIN_POST_SEARCH_FN: isize = 503;
/// PBlock constant for registering a post add operation.
pub const SLAPI_PLUGIN_BE_TXN_POST_ADD_FN: isize = 560;
/// PBlock constant for registering a post modify operation.
pub const SLAPI_PLUGIN_BE_TXN_POST_MODIFY_FN: isize = 561;
/// PBlock constant for registering a post modrdn operation.
pub const SLAPI_PLUGIN_BE_TXN_POST_MODRDN_FN: isize = 562;
/// PBlock constant for registering a post delete operation.
pub const SLAPI_PLUGIN_BE_TXN_POST_DELETE_FN: isize = 563;

// Constants that pblock uses for get / set. This is not complete!
/// PBlock constant for accessing private plugin data.
pub const SLAPI_PLUGIN_PRIVATE: isize = 4;
/// PBlock constant for accessing plugin type.
pub const SLAPI_PLUGIN_TYPE: isize = 5;
/// PBlock constant for accessing plugin API version.
pub const SLAPI_PLUGIN_VERSION: isize = 8;
/// PBlock constant for accessing plugin operation return codes.
pub const SLAPI_PLUGIN_OPRETURN: isize = 9;
/// PBlock constant for accessing plugin description information.
pub const SLAPI_PLUGIN_DESCRIPTION: isize = 12;


/// Plugin API versions that can be used for a plugin.
/// You should in most cases use version _03
#[derive(Debug)]
pub enum PluginVersion {
    /// Version 00. Do not use.
    _00,
    /// Version 01.
    _01,
    /// Version 02.
    _02,
    /// Version 03.
    _03,
}

/// Error logging levels that may be used. This is controlled by -d on the
/// ns-slapd commandline.
#[derive(Debug)]
pub enum LogLevel {
    /// Always log messages at this level. Soon to go away, see EMERG, ALERT, CRIT, ERR, WARNING, NOTICE, INFO, DEBUG
    FATAL,
    /// Log detailed messages.
    TRACE,
    /// Log packet tracing.
    PACKETS,
    /// Log argument tracing.
    ARGS,
    /// Log connection tracking.
    CONNS,
    /// Log BER parsing.
    BER,
    /// Log filter processing.
    FILTER,
    /// Log configuration processing.
    CONFIG,
    /// Log .... ???
    SHELL,
    /// Log .... ???
    PARSE,
    /// Log .... ???
    HOUSE,
    /// Log detailed replication information.
    REPL,
    /// Log cache management.
    CACHE,
    /// Log detailed plugin operations.
    PLUGIN,
    /// Log .... ???
    TIMING,
    /// Log backend infomation.
    BACKLDBM,
    /// Log ACL processing.
    ACLSUMMARY,
    /// Log nuncstans processing.
    NUNCSTANS,
    /// Emergency messages. Server is bursting into flame.
    EMERG,
    /// Important alerts, server may explode soon.
    ALERT,
    /// Critical messages, but the server isn't going to explode. Admin should intervene.
    CRIT,
    /// Error has occured, but we can keep going. Could indicate misconfiguration.
    ERR,
    /// Warning about an issue that isn't very important. Good to resolve though.
    WARNING,
    /// Inform the admin of something that they should know about, IE server is running now.
    NOTICE,
    /// Informational messages that are nice to know.
    INFO,
    /// Debugging information from the server.
    DEBUG,
}

