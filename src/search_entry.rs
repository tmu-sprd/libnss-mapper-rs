//! Searches config file for 'prefix' and returns result.
//!
//! # Arguments
//!
//! * `prefix` - The prefix in the given name. E.g. in '`root_user`' the prefix is 'root'.
//! * `name` - The given name. e.g. '`root_user`'.
//!
//! # Return value
//!
//! Returns a result of options or an error. The options are either Some Passwd struct or None, if not found.

use super::MAIN_CONF_FILE;

use libnss::passwd::Passwd;
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::result::Result;

pub fn search_entry(prefix: &str, name: &str) -> Result<Option<Passwd>, String> {
    let attr = match fs::symlink_metadata(MAIN_CONF_FILE) {
        Ok(stats) => stats,
        Err(error) => {
            return Err(format!(
                "Problem opening file {MAIN_CONF_FILE}:\n Error: {:?}",
                error.to_string()
            ))
        }
    };

    if !attr.is_file() {
        return Err(format!("{MAIN_CONF_FILE} is not a regular file!"));
    }

    if attr.st_uid() != 0 || attr.st_gid() != 0 {
        return Err(format!(
            "{MAIN_CONF_FILE} has wrong file permissions! It must not be owned by root:root (0:0)!"
        ));
    }

    if attr.st_mode() & libc::S_IWGRP != 0 || attr.st_mode() & libc::S_IWOTH != 0 {
        return Err(format!("{MAIN_CONF_FILE} has wrong file permissions! It must not be writable by group or other!"));
    }

    let content = match fs::read_to_string(MAIN_CONF_FILE) {
        Ok(content) => content,
        Err(error) => {
            return Err(format!(
                "Problem opening file {MAIN_CONF_FILE}:\n Error: {:?}",
                error.to_string()
            ))
        }
    };

    let mut found_entry = Passwd {
        name: "NotFound".to_string(),
        passwd: "!".to_string(),
        uid: 9999,
        gid: 9999,
        gecos: String::new(),
        dir: String::new(),
        shell: String::new(),
    };
    let mut found_count = 0;
    for (line_number, line) in content.lines().enumerate() {
        let token_count = line.split(':').count();
        if token_count != 7 {
            return Err(format!(
                "{MAIN_CONF_FILE} has a wrong format!\n Error: \"Wrong number of entries, should be 7 but are {token_count}.\"\n Line {}: {line}",
                line_number + 1
            ));
        }

        let tokens: Vec<&str> = line.split(':').collect();
        if tokens[0] != prefix {
            continue;
        }

        if found_count > 0 {
            found_count += 1;
            continue;
        }

        found_count = 1;
        found_entry = Passwd {
            name: name.to_string(),
            passwd: "!".to_string(),
            uid: match tokens[2].parse::<libc::uid_t>() {
                Ok(uid) => uid,
                Err(error) => return Err(format!(
                    "In {MAIN_CONF_FILE} matching line has a UID entry, which is not a number or out of range!\n Error: {:?}\n Line {}: {line}",
                    error.to_string(),
                    line_number + 1
                )),
            },
            gid: match tokens[3].parse::<libc::gid_t>() {
                Ok(uid) => uid,
                Err(error) => return Err(format!(
                    "In {MAIN_CONF_FILE} matching line has a GID entry, which is not a number or out of range!\n Error: {:?}\n Line {}: {line}",
                    error.to_string(),
                    line_number + 1
                )),
            },
            gecos: tokens[4].to_string(),
            dir: format!("{}/{}",tokens[5], name),
            shell: tokens[6].to_string()
            };
    }

    if found_count > 1 {
        return Err(format!("{MAIN_CONF_FILE} has multiple matching entries for {prefix}!\n Error: There are {found_count} entries, instead of one."));
    }

    if found_entry.name != "NotFound" {
        return Ok(Some(found_entry));
    }

    Ok(None)
}
