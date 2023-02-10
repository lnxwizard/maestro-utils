//! The `mount` command allows to mount a filesystem.

use std::env;
use std::ffi::CString;
use std::ffi::c_int;
use std::ffi::c_ulong;
use std::io::Error;
use std::io;
use std::process::exit;
use std::ptr::null;

/// Mount flag: TODO doc
const MS_RDONLY: c_ulong = 1;
/// Mount flag: TODO doc
const MS_NOSUID: c_ulong = 2;
/// Mount flag: TODO doc
const MS_NODEV: c_ulong = 4;
/// Mount flag: TODO doc
const MS_NOEXEC: c_ulong = 8;
/// Mount flag: TODO doc
const MS_SYNCHRONOUS: c_ulong = 16;
/// Mount flag: TODO doc
const MS_REMOUNT: c_ulong = 32;
/// Mount flag: TODO doc
const MS_MANDLOCK: c_ulong = 64;
/// Mount flag: TODO doc
const MS_DIRSYNC: c_ulong = 128;
/// Mount flag: TODO doc
const MS_NOATIME: c_ulong = 1024;
/// Mount flag: TODO doc
const MS_NODIRATIME: c_ulong = 2048;
/// Mount flag: TODO doc
const MS_BIND: c_ulong = 4096;
/// Mount flag: TODO doc
const MS_MOVE: c_ulong = 8192;
/// Mount flag: TODO doc
const MS_REC: c_ulong = 16384;
/// Mount flag: TODO doc
const MS_SILENT: c_ulong = 32768;
/// Mount flag: TODO doc
const MS_POSIXACL: c_ulong = 1 << 16;
/// Mount flag: TODO doc
const MS_UNBINDABLE: c_ulong = 1 << 17;
/// Mount flag: TODO doc
const MS_PRIVATE: c_ulong = 1 << 18;
/// Mount flag: TODO doc
const MS_SLAVE: c_ulong = 1 << 19;
/// Mount flag: TODO doc
const MS_SHARED: c_ulong = 1 << 20;
/// Mount flag: TODO doc
const MS_RELATIME: c_ulong = 1 << 21;
/// Mount flag: TODO doc
const MS_KERNMOUNT: c_ulong = 1 << 22;
/// Mount flag: TODO doc
const MS_I_VERSION: c_ulong = 1 << 23;
/// Mount flag: TODO doc
const MS_STRICTATIME: c_ulong = 1 << 24;
/// Mount flag: TODO doc
const MS_LAZYTIME: c_ulong = 1 << 25;
/// Mount flag: TODO doc
const MS_NOREMOTELOCK: c_ulong = 1 << 27;
/// Mount flag: TODO doc
const MS_NOSEC: c_ulong = 1 << 28;
/// Mount flag: TODO doc
const MS_BORN: c_ulong = 1 << 29;
/// Mount flag: TODO doc
const MS_ACTIVE: c_ulong = 1 << 30;
/// Mount flag: TODO doc
const MS_NOUSER: c_ulong = 1 << 31;
/// Mount flag: TODO doc
const MS_MGC_VAL: c_ulong = 0xc0ed0000;
/// Mount flag: TODO doc
const MS_MGC_MSK: c_ulong = 0xffff0000;

/// Prints the command's usage.
///
/// `bin` is the name of the current binary.
fn print_usage(bin: &str) {
	eprintln!("Usage:");
	eprintln!(" {} [-h]", bin);
	eprintln!(" {} -l", bin);
	eprintln!(" {} -a", bin);
	eprintln!(" {} [device] dir", bin);
	eprintln!();
	eprintln!("Options:");
	eprintln!(" -h:\t\tprints usage");
	eprintln!(" -l:\t\tlists mounted filesystems");
	eprintln!(" -a:\t\tmounts every filesystems specified in the /etc/fstab file");
	eprintln!(" device:\tthe device to mount. If not specified, the command attempts to find the device using the /dev/fstab file");
	eprintln!(" dir:\t\tthe directory on which the filesystem is to be mounted");
}

extern "C" {
	fn mount(
		source: *const i8,
		target: *const i8,
		filesystemtype: *const i8,
		mountflags: c_ulong,
		data: *const u8
	) -> c_int;
}

/// Mounts a filesystem.
///
/// Arguments:
/// TODO
pub fn mount_fs(
	source: &str,
	target: &str,
	fs_type: Option<&str>,
	mountflags: c_ulong,
	data: Option<&[u8]>
) -> io::Result<()> {
	let source_c = CString::new(source).unwrap();
	let target_c = CString::new(target).unwrap();

	let fs_type_c = fs_type.map(|fs_type| CString::new(fs_type).unwrap());
	let fs_type_ptr = fs_type_c
		.as_ref()
		.map(|fs_type| fs_type.as_ptr())
		.unwrap_or(null::<_>());

	let data = data.map(|data| data.as_ptr())
		.unwrap_or(null::<_>());

	let ret = unsafe {
		mount(
			source_c.as_ptr(),
			target_c.as_ptr(),
			fs_type_ptr,
			mountflags,
			data
		)
	};
	if ret < 0 {
		return Err(Error::last_os_error());
	}

	Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

	if args.is_empty() {
		print_usage("mount");
		exit(1);
	}
	let bin = &args[0];

	let a: Vec<&str> = args.iter()
		.map(|s| s.as_str())
		.collect();

	match &a[1..] {
		&[] => {
			print_usage(bin);
			exit(1);
		}

		&["-h"] => {
			print_usage(bin);
			exit(0);
		}

		&["-l"] => {
			// TODO print /etc/mtab to stdout
			todo!();
		}

		&["-a"] => {
			// TODO iterate on entries of /etc/fstab and mount all
			todo!();
		}

		&[device, dir] => {
			mount_fs(device, dir, Some("ext4"), 0, None).unwrap(); // TODO handle error
		}

		&[dir] => {
			// TODO lookup in /etc/fstab to get device, then mount
			todo!();
		}

		_ => {
			print_usage(bin);
			exit(1);
		}
	}
}
