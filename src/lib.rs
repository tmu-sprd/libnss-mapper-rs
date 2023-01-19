//! This crate implements the `libnss_mapper` library to add the `mapper` service
//! to the Name Service Switch in GNU C library (glibc).
//!
//! Documentation for adding a new NSS service to glibc can be found [here].
//!
//! [here]: https://www.gnu.org/software/libc/manual/html_node/Extending-NSS.html

extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;
#[cfg(feature = "syslog")]
extern crate syslog;

/// Constant for path to configuration file.
const MAIN_CONF_FILE: &str = "/etc/nssmapper.conf";

mod mapper_password;
mod mapper_shadow;
mod search_entry;
