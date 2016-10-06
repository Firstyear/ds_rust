# --- BEGIN COPYRIGHT BLOCK ---
# Copyright (C) 2016 Red Hat, Inc.
# All rights reserved.
#
# License: GPL (version 3 or any later version).
# See LICENSE for details.
# --- END COPYRIGHT BLOCK ---
#
# Author: William Brown <wibrown@redhat.com>
#

# This contains a set of python libraries that allow usage of the rs plugins
# to enable and configure them.

from lib389.plugins import Plugin

class HellorustPlugin(Plugin):
    def __init__(self, instance, dn="cn=hellorust,cn=plugins,cn=config", batch=False):
        super(HellorustPlugin, self).__init__(instance, dn, batch)
        self._default_properties = {
            'nsslapd-pluginEnabled': 'on',
            'nsslapd-pluginPath' : 'libhellorust',
            'nsslapd-pluginInitfunc' : 'slapi_r_plugin_init_fn',
            'nsslapd-pluginType': 'preoperation',
            'nsslapd-plugin-depends-on-type': 'database',
            'nsslapd-pluginId': 'Hello Rust',
            'nsslapd-pluginVendor': '389 Project',
            'nsslapd-pluginVersion': '0.1.0',
            'nsslapd-pluginDescription': 'Hello Rust!',
        }


    def create(self, rdn="hellorust", properties={}, basedn="cn=plugins,cn=config"):
        self._default_properties.update(properties)
        super(HellorustPlugin, self).create(rdn, self._default_properties, basedn)

class RoreplicaPlugin(Plugin):
    def __init__(self, instance, dn="cn=ro_replica,cn=plugins,cn=config", batch=False):
        super(RoreplicaPlugin, self).__init__(instance, dn, batch)
        self._default_properties = {
            'nsslapd-pluginEnabled': 'on',
            'nsslapd-pluginPath' : 'libro_replica',
            'nsslapd-pluginInitfunc' : 'slapi_r_plugin_init_fn',
            'nsslapd-pluginType': 'preoperation',
            'nsslapd-plugin-depends-on-type': 'database',
            'nsslapd-pluginId': 'ro_replica',
            'nsslapd-pluginVendor': '389 Project',
            'nsslapd-pluginVersion': '0.1.0',
            'nsslapd-pluginDescription': 'Prevents writes to this instance from external sources',
        }


    def create(self, rdn="ro_replica", properties={}, basedn="cn=plugins,cn=config"):
        self._default_properties.update(properties)
        super(RoreplicaPlugin, self).create(rdn, self._default_properties, basedn)
