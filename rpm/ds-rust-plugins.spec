Summary:          Rust plugins for 389 Directory Server
Name:             ds-rust-plugins
# Probably shuold make this a template.in later ...
Version:          0.1.0
Release:          1%{?dist}
License:          GPLv3+
URL:              http://github.com/Firstyear/ds-rust-plugins
Group:            Development/Libraries
BuildRequires:    rust
BuildRequires:    cargo
BuildRequires:    389-ds-base-devel
Requires:         389-ds-base

Source0:          https://git.fedorahosted.org/cgit/nunc-stans.git/snapshot/%{name}-%{version}.tar.gz
#Source0:          http://fedorahosted.org/sources/%{name}-%{version}.tar.xz

%description
%{name} is a set of plugins for 389 Directory Server developed in Rust. Rust is
a language that pursues safety, correctness and concurrency. It eliminates
classes of problems such as memory leaks, pointer dereferencing and many other
stability and security issues at compile time. This package contains a
helloworld example and read only database plugin.

%prep
%setup -q

%build
# Is there a macro that does this?
autoreconf -i
%configure
make

%install
%{__make} install DESTDIR=$RPM_BUILD_ROOT INSTALL="%{__install} -p"
# %{__rm} -f $RPM_BUILD_ROOT%{_libdir}/lib*.a
#%{__rm} -f $RPM_BUILD_ROOT%{_libdir}/lib*.la

%clean
make distclean

%files
%defattr(-,root,root,-)
%doc %{_datadir}/dirsrv/
%{_libdir}/dirsrv/plugins/*.so*
%exclude %{_libdir}/dirsrv/plugins/*.la
%exclude %{_libdir}/dirsrv/plugins/*.a

%changelog
* Sat Sep 24 2016 William Brown <wibrown@redhat.com> - 0.1.0-1
- Initial build



