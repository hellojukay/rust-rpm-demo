Summary: a rust web demo.
Name: demo
Version: v1.0.1
Release: 1
License: GPL
Group: Develop/Tools
BuildArch: x86_64
%description
This is a demo 
%prep
no source
%build
echo hello world
%install
mkdir /opt/demo
install -m 755 demo.service /usr/lib/systemd/demo.service
%files
/usr/lib/systemd/demo.service
%changelog
# let`s skip this.
