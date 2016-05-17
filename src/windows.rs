// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate process;
extern crate time;
extern crate libc;
use self::libc::c_int;
use self::process::CurrentProcess;
use self::rfc3164::format_message_rfc3164;
use std::ffi::CStr;
use std::ffi::CString;
use Priority;

pub fn syslog2_cstr(priority: Priority, message: &CStr)
{
	log_to_standard_error_for_windows_and_solaris_cstr(priority, message);
}

// Exists because we need byte string constants, and these are for UNSIGNED bytes
pub fn syslog2_bytes(priority: Priority, message: &[u8])
{
	log_to_standard_error_for_windows_and_solaris_bytes(priority, message);
}

pub fn with_open_syslog2<F, R>(programName: &CStr, logToStandardErrorAsWell: bool, defaultFacility: Facility, closure: F) -> R
where F: Fn() -> R
{
	enable_logging_to_standard_error(programName, logToStandardErrorAsWell, defaultFacility);
	closure();
}
