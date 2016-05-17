// This file is part of syslog. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog/master/COPYRIGHT. No part of syslog, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog/master/COPYRIGHT.

#![feature(const_fn)]
#![feature(stmt_expr_attributes)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate once;
extern crate libc;
use std::ffi::CStr;

mod severity;
pub use severity::Severity;

mod facility;
pub use facility::Facility;

mod priority;
pub use priority::Priority;
pub use priority::PriorityTrait;

mod logMask;
pub use logMask::LogMask;
pub use logMask::LogMaskTrait;
pub use logMask::default_log_mask;

mod logOptions;
pub use logOptions::LogOptions;

mod vecU8PushStr;
pub use vecU8PushStr::VecU8PushStr;

pub use syslogRfcConstructor::SyslogRfcConstructor;
mod syslogRfcConstructor;

pub use syslogRfc::SyslogRfc;
mod syslogRfc;

pub mod syslogSenders;

pub mod rfc5424;

// TODO: What are the Windows event log equivalents?
// TODO: Integrate with the log crate...
// TODO: Hand-off thread for SyslogSender, because they block
// TODO: TCP reconnect on failure. Not great, as possible we will have sent a partial message...
// TODO: TCP socket close down on end-of-logging
// TODO: syslog logging macros - like logging or format!() macros, but create a String with a final "\0" on it
	// TODO: format macros for CString (HARD, as compiler built in - examine compiler generated code)
	// TODO: audit formats (use ':' or some other separator (\t would be nice but rsyslog subs by default))
// TODO: syslog RFC 5424 uptime - a bit challenging
// TODO: libtls + TCP syslog

// Extensions: rust collectd, statsd, collectd plugins
	// Hosted graphite using statsd (securely): https://www.hostedgraphite.com/hosted-statsd
	// Sending data to collectd via insecure UDP: https://pythonhosted.org/collectd/

// Loggly uses a variant of this over TCP or UDP (I think needs syslog tokens)
// Loggly uses a variant of this that works using TLS over TCP
// Papertrail uses RFC 3164 or RFC 5424 over TCP or UDP or TLS over TCP
// Loggr == agents
// Logentries (needs syslog tokens, but has support for searches for metrics, threshold alerts and can output to a hosted graphite)
// https://sematext.com/logsene/ (hosted kibana)
// Cloudlytics

pub fn syslog_cstr_currentLoggingFacility(severity: Severity, message: &CStr)
{
	let priority = severity.toPriorityForCurrentLoggingFacility();
	syslog_cstr(priority, message);
}

// Exists because we need byte string constants, and these are for UNSIGNED bytes
pub fn syslog_bytes_currentLoggingFacility(severity: Severity, message: &[u8])
{
	let priority = severity.toPriorityForCurrentLoggingFacility();
	syslog_bytes(priority, message);
}

pub fn syslog_cstr_withFacility(severity: Severity, message: &CStr, facility: Facility)
{
	let priority = severity.toPriority(facility);
	syslog_cstr(priority, message);
}

// Exists because we need byte string constants, and these are for UNSIGNED bytes
pub fn syslog_bytes_withFacility(severity: Severity, message: &[u8], facility: Facility)
{
	let priority = severity.toPriority(facility);
	syslog_bytes(priority, message);
}

#[cfg(any(target_os = "windows", target_os = "solaris"))]
static mut OpenLogProgramName: Option<CString> = None;

#[cfg(any(target_os = "windows", target_os = "solaris"))]
static mut OpenLogDoNotLogToStandardError: bool = true;

#[cfg(any(target_os = "windows", target_os = "solaris"))]
static mut OpenLogDefaultFacility: Facility = Default::default();

#[cfg(any(target_os = "windows", target_os = "solaris"))]
pub fn log_to_standard_error_for_windows_and_solaris_bytes(priority: Priority, message: &[u8])
{
	let message = unsafe { CStr.from_bytes_with_nul_unchecked(message) };
	log_to_standard_error_for_windows_and_solaris_cstr(priority, message);
}

#[cfg(any(target_os = "windows", target_os = "solaris"))]
pub fn enable_logging_to_standard_error(programName: &CStr, logToStandardErrorAsWell: bool, defaultFacility: Facility)
{
	unsafe
	{
		OpenLogProgramName = Some(programName.to_owned());
		OpenLogDoNotLogToStandardError = !logToStandardErrorAsWell;
		OpenLogDefaultFacility = defaultFacility;
	}
}

#[cfg(any(target_os = "windows", target_os = "solaris"))]
pub fn log_to_standard_error_for_windows_and_solaris_cstr(priority: Priority, message: &CStr)
{
	if OpenLogDoNotLogToStandardError
	{
		return;
	}
	
	let message: &str = message.to_string_lossy();
	let timeNow = time::now_utc();
	let messageFacility = priority.facility();
	
	let chosenFacility = match priority.facility()
	{
		Facility::LOG_KERN => OpenLogDefaultFacility,
		everythingElse @ _ => everythingElse,
	};
	
	let message = format_message_rfc3164(&CurrentProcess.hostName, &OpenLogProgramName, &CurrentProcess.pid, timeNow, chosenFacility.toRfc3164Facility(), priority.severity(), message);
	
	// We do not use the writeln! macro here, as it is not clear that it gets the line delimiter correct (and after the breaking changes to the lines() logic in rust, I no longer trust the std library in this area)
	write!(&mut w, "{}\r\n", message).unwrap();
}

cfg_if!
{
	if #[cfg(windows)]
	{
		mod windows;
		pub use windows::*;
	}
	else if #[cfg(unix)]
	{
		mod unix;
		pub use unix::*;
	}
	else
	{
		// Unsupported
	}
}


#[test]
#[should_panic]
fn writesToSyslogAndCleansUpOnPanic()
{
	with_open_syslog(&CString::new("progname").unwrap(), true, Default::default(), ||
	{
		static TestString: &'static [u8] = b"Test of syslog_bytes*\0";
		let cstr = CString::new("Test of syslog_cstr*").unwrap();
		
		syslog_cstr_currentLoggingFacility(Severity::LOG_CRIT, &cstr);
		syslog_bytes_currentLoggingFacility(Severity::LOG_CRIT, &TestString);
		
		syslog_cstr_withFacility(Severity::LOG_CRIT, &cstr, Facility::LOG_LOCAL0);
		syslog_bytes_withFacility(Severity::LOG_CRIT, &TestString, Facility::LOG_LOCAL0);
		
		syslog_cstr(Severity::LOG_CRIT.toPriority(Facility::LOG_LOCAL0), &CString::new("Test of syslog_cstr").unwrap());
		syslog_bytes(Severity::LOG_CRIT.toPriority(Facility::LOG_LOCAL0), &TestString);
		
		panic!("hello");
	});
}
