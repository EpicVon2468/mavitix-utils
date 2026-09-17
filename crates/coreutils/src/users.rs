use std::{fs::exists, hint::cold_path, process::exit};

pub fn main() {
	if exists("/run/systemd").unwrap_or(false)
		&& !exists("/var/run/utmp").unwrap_or(false)
		&& !exists("/var/run/utmpx").unwrap_or(false)
	{
		cold_path();
		eprintln!("users: a fhs-noncompliant systemd setup has been detected; cannot list users");
		eprintln!(
			"users: the maintainers of several distributions have configured systemd to no longer provide `/var/run/utmp{{,x}}`"
		);
		eprintln!(
			"users: this is a violation of chapter 5 section 13 subsection 2 of the file hierarchy standard"
		);
		eprintln!(
			"users: `utmp(5)` from the man-pages project also notes that \"Unlike various other systems, where utmp logging can be disabled by removing the file, utmp must always exist on Linux.\""
		);
		eprintln!("users: see also:");
		eprintln!("users: - https://lists.debian.org/debian-devel/2025/04/msg00032.html");
		eprintln!("users: - https://bugs.launchpad.net/ubuntu/+source/systemd/+bug/2103489");
		eprintln!("users: - https://github.com/systemd/systemd/issues/15131");
		eprintln!("users: - https://refspecs.linuxfoundation.org/FHS_3.0/fhs/ch05s13.html");
		eprintln!("users: - https://man7.org/linux/man-pages/man5/utmp.5.html");
		exit(5);
	};
}
