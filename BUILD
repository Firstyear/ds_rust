To build:

* This assumes you have built DS into /opt/dirsrv

Right now, we haven't integrated the rust parts into the autotools build,
so you will need to build and install manually.

You will need rust-binary: It is your responsibility to find this.

This is tested on EL7 with rust 1.7.0

===================================================================

cd slapi_r_plugin; cargo build
cd plugins/hellorust; cargo build
sudo cp target/debug/libhellorust.so /opt/dirsrv/lib/dirsrv/plugins

===================================================================

The config in dse.ldif to make this plugin work:

dn: cn=hellorust,cn=plugins,cn=config
objectClass: top
objectClass: nsSlapdPlugin
objectClass: extensibleObject
cn: hellorust
nsslapd-pluginPath: libhellorust
nsslapd-pluginInitfunc: slapi_r_plugin_init_fn
nsslapd-pluginType: postoperation
nsslapd-pluginEnabled: on
nsslapd-plugin-depends-on-type: database
nsslapd-pluginId: HelloRust
nsslapd-pluginVersion: 1.3.5.1
nsslapd-pluginVendor: 389 Project
nsslapd-pluginDescription: Hello from Rust!

===================================================================

You can (try) to build with clippy code quality checker:

cargo rustc --features clippy -- -Z no-trans -Z extra-plugins=clippy
