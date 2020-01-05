Name: rust-rpm-demo
Version: v1.0.1
Release: 1
Summary: a rust web demo
License: GPL
BuildArch: x86_64
%description
rust web demo
%prep
cp %{_sourcedir}/ -rf %{buildroot}
%build
echo building
%install
mkdir -p %{buildroot}/usr/bin/
mkdir -p %{buildroot}/usr/lib/systemd/system
install -m 755 %{_sourcedir}/rust-rpm-demo %{buildroot}/usr/bin/rust-rpm-demo
install -m 755 %{_sourcedir}/rust-rpm-demo.service %{buildroot}/usr/lib/systemd/system/rust-rpm-demo.service

%files
/usr/bin/rust-rpm-demo
/usr/lib/systemd/system/rust-rpm-demo.service
