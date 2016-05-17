// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate libc;
use Priority;
use Severity;
use super::syslog2Senders::Rfc3164Facility;
use self::libc::c_int;

/// Fill in for lack of this value in Android bionic's libc
/// On Windows, create a default for this value that matches Linux
#[cfg(any(target_os = "android", target_os = "windows"))] pub const LOG_NFACILITIES: c_int = 24;
#[cfg(not(any(target_os = "android", target_os = "windows")))] pub const LOG_NFACILITIES: c_int = self::libc::LOG_NFACILITIES;

/// `LOG_AUTHPRIV` and `LOG_FTP` are not available on Solaris for local logging
/// `LOG_NTP`, `LOG_SECURITY` and `LOG_CONSOLE` are only available on FreeBSD and DragonFlyBSD for local logging
/// `LOG_NETINFO`, `LOG_REMOTEAUTH`, `LOG_INSTALL`, `LOG_RAS` and `LOG_LAUNCHD` are only available on Mac OS X for local logging
/// `LOG_CRON` differs in value on Solaris, and _should not_ be used to send syslog2 messages on the wire
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(i32)] // We'd like to use c_int here, but the compiler won't let us
#[cfg(not(target_os = "windows"))]
pub enum Facility
{
	/// Do not use this value in calls to syslog2, as it is ignored
	/// If you really, really want to log as the kernel, use this value with openlog and then omit a facility whilst logging, eg
	/// use the Severity-only variants of the API
	LOG_KERN = self::libc::LOG_KERN,
	LOG_USER = self::libc::LOG_USER,
	LOG_MAIL = self::libc::LOG_MAIL,
	LOG_DAEMON = self::libc::LOG_DAEMON,
	LOG_AUTH = self::libc::LOG_AUTH,
	LOG_SYSLOG = self::libc::LOG_SYSLOG,
	LOG_LPR = self::libc::LOG_LPR,
	LOG_NEWS = self::libc::LOG_NEWS,
	LOG_UUCP = self::libc::LOG_UUCP,
	LOG_CRON = self::libc::LOG_CRON,
	#[cfg(not(target_os = "solaris"))] LOG_AUTHPRIV = self::libc::LOG_AUTHPRIV,
	#[cfg(not(target_os = "solaris"))] LOG_FTP = self::libc::LOG_FTP,
	#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] LOG_NTP = self::libc::LOG_NTP,
	#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] LOG_SECURITY = self::libc::LOG_SECURITY,
	#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] LOG_CONSOLE = self::libc::LOG_CONSOLE,
	#[cfg(target_os = "macos")] LOG_NETINFO = self::libc::LOG_NETINFO,
	#[cfg(target_os = "macos")] LOG_REMOTEAUTH = self::libc::LOG_REMOTEAUTH,
	#[cfg(target_os = "macos")] LOG_INSTALL = self::libc::LOG_INSTALL,
	#[cfg(target_os = "macos")] LOG_RAS = self::libc::LOG_RAS,
	LOG_LOCAL0 = self::libc::LOG_LOCAL0,
	LOG_LOCAL1 = self::libc::LOG_LOCAL1,
	LOG_LOCAL2 = self::libc::LOG_LOCAL2,
	LOG_LOCAL3 = self::libc::LOG_LOCAL3,
	LOG_LOCAL4 = self::libc::LOG_LOCAL4,
	LOG_LOCAL5 = self::libc::LOG_LOCAL5,
	LOG_LOCAL6 = self::libc::LOG_LOCAL6,
	LOG_LOCAL7 = self::libc::LOG_LOCAL7,
	#[cfg(target_os = "macos")] LOG_LAUNCHD = self::libc::LOG_LAUNCHD,
}

/// These values are 'fakes' to allow some measure of syslog2 compatibility on Windows
/// Values match those used on Linux
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(i32)] // We'd like to use c_int here, but the compiler won't let us
#[cfg(target_os = "windows")]
pub enum Facility
{
	LOG_KERN = 0,
	LOG_USER = 1 << 3,
	LOG_MAIL = 2 << 3,
	LOG_DAEMON = 3 << 3,
	LOG_AUTH = 4 << 3,
	LOG_SYSLOG = 5 << 3,
	LOG_LPR = 6 << 3,
	LOG_NEWS = 7 << 3,
	LOG_UUCP = 8 << 3,
	LOG_CRON = 9 << 3,
	LOG_AUTHPRIV = 10 << 3,
	LOG_FTP = 11 << 3,
	LOG_LOCAL0 = 16 << 3,
	LOG_LOCAL1 = 17 << 3,
	LOG_LOCAL2 = 18 << 3,
	LOG_LOCAL3 = 19 << 3,
	LOG_LOCAL4 = 20 << 3,
	LOG_LOCAL5 = 21 << 3,
	LOG_LOCAL6 = 22 << 3,
	LOG_LOCAL7 = 23 << 3,
}

impl Facility
{
	#[inline(always)]
	pub fn toPriority(self, severity: Severity) -> Priority
	{
		severity.toPriority(self)
	}
	
	/// Returns `LOG_AUTHPRIV` except on Solaris, where it returns `LOG_AUTH`
	#[inline(always)]
	#[cfg(not(target_os = "solaris"))]
	pub fn bestAuthenticationFacilityForPlatform() -> Facility
	{
		Facility::LOG_AUTHPRIV
	}
	
	/// Returns `LOG_AUTHPRIV` except on Solaris, where it returns `LOG_AUTH`
	#[inline(always)]
	#[cfg(target_os = "solaris")]
	pub fn bestAuthenticationFacilityForPlatform() -> Facility
	{
		Facility::LOG_AUTH
	}
	
	/// Will not match 1:1, as we will not (ab)use effectively private use RFC 3164 facility codes (12 - 15 inclusive and greater than 23)
	/// Solaris cron is mapped to clock (11), not cron (15), for maximum compatibility
	/// FreeBSD and DragonFlyBSD LOG_SECURITY and LOG_CONSOLE are mapped to authpriv, for maximum compatibility and secure handling
	/// FreeBSD and DragonFlyBSD LOG_NTP is mapped to daemon
	/// Mac OS X LOG_NETINFO is mapped to daemon
	/// Mac OS X LOG_REMOTEAUTH is mapped to authpriv
	/// Mac OS X LOG_INSTALL and LOG_RAS are mapped to authpriv (as they would seem to leak private or privleged information)
	/// Mac OS X LOG_LAUNCHD is mapped to daemon
	pub fn toRfc3164Facility(self) -> Rfc3164Facility
	{
		match self
		{
			Facility::LOG_KERN => Rfc3164Facility::kern,
			Facility::LOG_USER => Rfc3164Facility::user,
			Facility::LOG_MAIL => Rfc3164Facility::mail,
			Facility::LOG_DAEMON => Rfc3164Facility::daemon,
			Facility::LOG_AUTH => Rfc3164Facility::auth,
			Facility::LOG_SYSLOG => Rfc3164Facility::syslog2,
			Facility::LOG_LPR => Rfc3164Facility::lpr,
			Facility::LOG_NEWS => Rfc3164Facility::news,
			Facility::LOG_UUCP => Rfc3164Facility::news,
			Facility::LOG_CRON => Rfc3164Facility::clock,
			#[cfg(not(target_os = "solaris"))] Facility::LOG_AUTHPRIV => Rfc3164Facility::authpriv,
			#[cfg(not(target_os = "solaris"))] Facility::LOG_FTP => Rfc3164Facility::ftp,
			#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] Facility::LOG_NTP => Rfc3164Facility::daemon,
			#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] Facility::LOG_SECURITY => Rfc3164Facility::authpriv,
			#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] Facility::LOG_CONSOLE => Rfc3164Facility::authpriv,
			#[cfg(target_os = "macos")] Facility::LOG_NETINFO => Rfc3164Facility::daemon,
			#[cfg(target_os = "macos")] Facility::LOG_REMOTEAUTH => Rfc3164Facility::authpriv,
			#[cfg(target_os = "macos")] Facility::LOG_INSTALL => Rfc3164Facility::authpriv,
			#[cfg(target_os = "macos")] Facility::LOG_RAS => Rfc3164Facility::authpriv,
			Facility::LOG_LOCAL0 => Rfc3164Facility::local0,
			Facility::LOG_LOCAL1 => Rfc3164Facility::local1,
			Facility::LOG_LOCAL2 => Rfc3164Facility::local2,
			Facility::LOG_LOCAL3 => Rfc3164Facility::local3,
			Facility::LOG_LOCAL4 => Rfc3164Facility::local4,
			Facility::LOG_LOCAL5 => Rfc3164Facility::local5,
			Facility::LOG_LOCAL6 => Rfc3164Facility::local6,
			Facility::LOG_LOCAL7 => Rfc3164Facility::local7,
			#[cfg(target_os = "macos")] Facility::LOG_LAUNCHD => Rfc3164Facility::daemon,
		}
	}
}

impl Default for Facility
{
	/// Defaults to `LOG_USER`, as used in Musl libc
	#[inline(always)]
	fn default() -> Facility
	{
		Facility::LOG_USER
	}
}
