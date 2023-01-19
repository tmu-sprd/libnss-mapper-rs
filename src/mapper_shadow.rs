use crate::search_entry::search_entry;

use libnss::interop::Response;
use libnss::shadow::Shadow;
use libnss::shadow::ShadowHooks;
#[cfg(feature = "syslog")]
use syslog::{Facility, Formatter3164};

struct MapperShadow;
libnss_shadow_hooks!(mapper, MapperShadow);

impl ShadowHooks for MapperShadow {
    fn get_all_entries() -> Response<Vec<Shadow>> {
        Response::NotFound
    }

    fn get_entry_by_name(name: String) -> Response<Shadow> {
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
            Ok(Some(_)) => Response::Success(Shadow {
                name: name.to_string(),
                passwd: "!".to_string(),
                last_change: 13571,
                change_min_days: 0,
                change_max_days: 99999,
                change_warn_days: 7,
                change_inactive_days: -1,
                expire_date: -1,
                reserved: 0,
            }),
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
