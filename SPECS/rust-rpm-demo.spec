Name: rust-rpm-demo
Version: v1.0.1
Release: 1
Summary: a rust web demo
License: GPL
BuildArch: x86_64
%description
rust web demo
%build
cargo build
%install
mkdir -p %{buildroot}/usr/bin/
install -m 755 %{_sourcedir}/demo.sh %{buildroot}/usr/bin/demo.sh

%files
/usr/bin/demo.sh
