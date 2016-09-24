ACLOCAL_AMFLAGS = -I m4

# Extract the versions from cargo for the .so versions.
hellorelease := $(shell $(srcdir)/sh/extract-cargo-release.sh $(srcdir)/plugins/hellorust/Cargo.toml)

# Install the example ldifs for enabling.
exampledir = $(datadir)/dirsrv/examples
helloexampledir = $(exampledir)/hellorust/
# roexampledir = $(exampledir)/ro_replica/

helloexample_DATA = plugins/hellorust/enable.ldif


# Actually build the plugins
serverplugindir = $(libdir)/dirsrv/plugins

serverplugin_LTLIBRARIES = libhellorust.la

am_libhellorust_la_OBJECTS = hellorust.o
libhellorust_la_SOURCES = ""
libhellorust_la_LIBADD = libhellorust.a
libhellorust_la_LDFLAGS = -release $(hellorelease)

hellorust.o:
	cd $(srcdir)/plugins/hellorust; cargo rustc -- --emit obj=$(abs_builddir)/$@

libhellorust.a:
	cd $(srcdir)/plugins/hellorust; cargo rustc -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

clean-local:
	cd $(srcdir)/plugins/hellorust; cargo clean
	cd $(srcdir)/slapi_r_plugin; cargo clean
	rm $(srcdir)/libhellorust.{a,d,so}; true

RPMBUILD ?= $(abs_builddir)/rpmbuild

rpmroot:
	mkdir -p $(RPMBUILD)/BUILD
	mkdir -p $(RPMBUILD)/RPMS
	mkdir -p $(RPMBUILD)/SOURCES
	mkdir -p $(RPMBUILD)/SPECS
	mkdir -p $(RPMBUILD)/SRPMS

rpmsources: rpmroot
	cd $(srcdir); git archive --prefix=$(PACKAGE_NAME)-$(PACKAGE_VERSION)/ HEAD | gzip > $(RPMBUILD)/SOURCES/$(PACKAGE_NAME)-$(PACKAGE_VERSION).tar.gz

rpms: rpmsources
	rpmbuild --define "_topdir $(RPMBUILD)" -ba $(srcdir)/rpm/$(PACKAGE_NAME).spec

