ACLOCAL_AMFLAGS = -I m4

# Need to work out how to get this from rustc. May not need for .so
# hello_cargo_LDFLAGS = -ldl -lpthread -lgcc_s -lc -lm -lrt -lutil

# Can we do this without over-riding libdir?
libdir = ${exec_prefix}/lib/dirsrv/plugins

lib_LTLIBRARIES = libhellorust.la

am_libhellorust_la_OBJECTS = hellorust.o
libhellorust_la_SOURCES = ""
libhellorust_la_LIBADD = libhellorust.a

hellorust.o:
	cd $(srcdir)/plugins/hellorust; cargo rustc -- --emit obj=$(abs_builddir)/$@

libhellorust.a:
	cd $(srcdir)/plugins/hellorust; cargo rustc -- --crate-type=staticlib --emit link -o $(abs_builddir)/$@

clean-local:
	cd $(srcdir)/plugins/hellorust; cargo clean

