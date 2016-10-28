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


DEBUGGING = True

from . import topology

from rs389 import RoreplicaPlugin

def test_setup_ds_minimal(topology):
    # Make sure we can start stop.

    rorp = RoreplicaPlugin(topology.standalone)
    rorp.create()
    topology.standalone.stop()
    topology.standalone.start()
    assert(len(topology.standalone.ds_error_log.match('.*ro_replica started.*')) > 0)

    # Now try and add something ....
    # It should fail. Check the log!

    # Try and mod the basedn. Should also fail!
    # Check the log.

