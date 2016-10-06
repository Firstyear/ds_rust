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

from rs389 import HellorustPlugin

import time

def test_setup_ds_minimal(topology):
    # Make sure we can start stop.

    hrp = HellorustPlugin(topology.standalone)
    hrp.create()
    topology.standalone.stop()
    topology.standalone.start()
    assert(len(topology.standalone.ds_error_log.match('.*Hello rust\!.*')) > 0)


