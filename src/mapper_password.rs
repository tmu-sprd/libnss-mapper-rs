//! Implements C functions for NSS passwd database.
//!
//! # Methods implementing following functions (begins with `_nss_mapper`):
//! * `get_all_entries`: `_setpwent`, `_endpwent`, `_getpwent_r`
//! ** Returns always `NotFound`, because there is nothing to iterate over.
//! * `get_entry_by_uid`: `_getpwuid_r`
//! * `get_entry_by_name`: `_getpwnam_r`

use crate::search_entry::search_entry;

use libnss::interop::Response;
use libnss::passwd::Passwd;
use libnss::passwd::PasswdHooks;
use std::env;
#[cfg(feature = "syslog")]
use syslog::{Facility, Formatter3164};

struct MapperPasswd;
libnss_passwd_hooks!(mapper, MapperPasswd);

impl PasswdHooks for MapperPasswd {
    fn get_all_entries() -> Response<Vec<Passwd>> {
        Response::NotFound
    }

    fn get_entry_by_uid(uid: libc::uid_t) -> Response<Passwd> {
        #[cfg(feature = "syslog")]
        let formatter = Formatter3164 {
            facility: Facility::LOG_AUTH,
            hostname: None,
            process: "libnss_mapper".into(),
            pid: std::process::id(),
        };

        let name = match env::var("LOGNAME") {
            Ok(value) => value,
            #[allow(unused_variables)]
            Err(error) => {
                #[cfg(feature = "syslog")]
                match syslog::unix(formatter) {
                    Err(e) => println!("impossible to connect to syslog: {e:?}"),
                    Ok(mut writer) => {
                        writer.err(format!(
                            "Environment variable LOGNAME not found or not valid Unicode.\n Error: {:?}",
                            error.to_string()
                        )).expect("could not write error message");
                    }
                }

                return Response::Unavail;
            }
        };

        if !name.contains('_') {
            return Response::NotFound;
        }

        let (prefix, _) = name.split_once('_').expect("Delimiter not found at split.");
        let result = search_entry(prefix, &name);

        match result {
            Ok(Some(found)) => {
                if uid == found.uid {
                    Response::Success(found)
                } else {
                    Response::NotFound
                }
            }
            Ok(None) => Response::NotFound,
            #[allow(unused_variables)]
            Err(error) => {
                #[cfg(feature = "syslog")]
                match syslog::unix(formatter) {
                    Err(e) => println!("impossible to connect to syslog: {e:?}"),
                    Ok(mut writer) => {
                        writer.err(error).expect("could not write error message");
                    }
                }

                Response::Unavail
            }
        }
    }

    fn get_entry_by_name(name: String) -> Response<Passwd> {
        if !name.contains('_') {
            return Response::NotFound;
        }

        #[cfg(feature = "syslog")]
        let formatter = Formatter3164 {
            facility: Facility::LOG_AUTH,
            hostname: None,
            process: "libnss_mapper".into(),
            pid: std::process::id(),
        };

        let (prefix, _) = name.split_once('_').expect("Delimiter not found at split.");
        let result = search_entry(prefix, &name);

        match result {
            Ok(Some(found)) => Response::Success(found),
            Ok(None) => Response::NotFound,
            #[allow(unused_variables)]
            Err(error) => {
                #[cfg(feature = "syslog")]
                match syslog::unix(formatter) {
                    Err(e) => println!("impossible to connect to syslog: {e:?}"),
                    Ok(mut writer) => {
                        writer.err(error).expect("could not write error message");
                    }
                }

                Response::Unavail
            }
        }
    }
}
