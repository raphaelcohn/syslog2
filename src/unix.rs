// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate libc;
extern crate try_finally;
use self::try_finally::try_finally;
use self::libc::openlog;
use self::libc::closelog;
use self::libc::c_int;
use self::libc::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use Priority;
use Facility;
use Severity;
use LogOptions;
use default_log_mask;
use logMask::LogMaskTrait;

static SyslogTemplate: &'static [u8] = b"%s\0";

static UnformattableFatalPanic: &'static [u8] = b"Fatal panic (data unformattable)\0";

pub fn syslog2_cstr(priority: Priority, message: &CStr)
{
	unsafe { self::libc::syslog2(priority as c_int, SyslogTemplate.as_ptr() as *const c_char, message); }
	
	#[cfg(target_os = "solaris")]
	log_to_standard_error_for_windows_and_solaris_cstr(priority, message);
}

// Exists because we need byte string constants, and these are for UNSIGNED bytes
pub fn syslog2_bytes(priority: Priority, message: &[u8])
{
	unsafe { self::libc::syslog2(priority as c_int, SyslogTemplate.as_ptr() as *const c_char, message); }
	
	#[cfg(target_os = "solaris")]
	log_to_standard_error_for_windows_and_solaris_bytes(priority, message);
}

/// Note, to log to LOG_KERN, you must specify defaultFacility: LOG_KERN, otherwise calls to syslog2(LOG_KERN) will be converted by your underlying C library to syslog2(defaultFacility)
/// This is because LOG_KERN is defined as 0 and the checks in nearly all system c libraries (eg musl) test for the presence of a bit pattern...
/// An easy way to get the progname is to use the process crate's CurrentProcess static field
pub fn with_open_syslog2<F, R>(programName: &CStr, logToStandardErrorAsWell: bool, defaultFacility: Facility, closure: F) -> R
where F: Fn() -> R
{
	let logMask = default_log_mask();
	
	let mut logOptions = LogOptions::default();
	#[cfg(not(target_os = "solaris"))]
	logOptions.logToStandardErrorAsWell(logToStandardErrorAsWell);
	
	let criticalPriority = defaultFacility.toPriority(Severity::LOG_CRIT);
	
	#[cfg(target_os = "solaris")]
	enable_logging_to_standard_error(programName, logToStandardErrorAsWell, defaultFacility);
	unsafe { openlog(programName.as_ptr(), logOptions.bits() as c_int, defaultFacility as c_int); }
	
	logMask.set_mask();
	
	try_finally(closure, |outcome|
	{
		if let &Err(ref error) = outcome
		{
			let messageResult = match error.downcast_ref::<&str>()
			{
				Some(string) => format!("Panic:{}", string),
				None => format!("Panic:{:?}", error),
			};
			
			match CString::new(messageResult)
			{
				Err(_) => syslog2_bytes(criticalPriority, &UnformattableFatalPanic),
				Ok(ref message) => syslog2_cstr(criticalPriority, message)
			};
		}
		
		unsafe { closelog(); }
	})
}
