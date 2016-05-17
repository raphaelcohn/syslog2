// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate libc;

use Facility;
use Priority;
use LogMask;
use super::syslog2Senders::Rfc3164Facility;
use self::libc::c_int;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(i32)] // We'd like to use c_int here, but the compiler won't let us
#[cfg(not(target_os = "windows"))]
pub enum Severity
{
	LOG_EMERG = self::libc::LOG_EMERG,
	LOG_ALERT = self::libc::LOG_ALERT,
	LOG_CRIT = self::libc::LOG_CRIT,
	LOG_ERR = self::libc::LOG_ERR,
	LOG_WARNING = self::libc::LOG_WARNING,
	LOG_NOTICE = self::libc::LOG_NOTICE,
	LOG_INFO = self::libc::LOG_INFO,
	LOG_DEBUG = self::libc::LOG_DEBUG,
}

/// Windows values are 'fakes' to allow some compatibility
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(i32)] // We'd like to use c_int here, but the compiler won't let us
#[cfg(target_os = "windows")]
pub enum Severity
{
	LOG_EMERG = 0,
	LOG_ALERT = 1,
	LOG_CRIT = 2,
	LOG_ERR = 3,
	LOG_WARNING = 4,
	LOG_NOTICE = 5,
	LOG_INFO = 6,
	LOG_DEBUG = 7,
}

/// Always inlined to replicate the behaviour of a C 'function' macro
#[inline(always)]
const fn LOG_MASK(pri: Severity) -> LogMask
{
	1 << (pri as c_int)
}

/// Always inlined to replicate the behaviour of a C 'function' macro
#[inline(always)]
const fn LOG_UPTO(toppri: Severity) -> LogMask
{
	(1 << ((toppri as c_int) + 1)) - 1
}

impl Severity
{
	#[inline(always)]
	pub const fn toPriorityForCurrentLoggingFacility(self) -> Priority
	{
		self as c_int
	}
	
	/// Similar to LOG_MAKEPRI on all systems except:-
	/// - not defined on OpenBSD, BitRig or Solaris
	/// - only defined in Android bionic from March 2015
	/// - incorrectly defined in musl to ((f as c_int) << 3) | (p as c_int); see the mailing list posts as to why: http://www.openwall.com/lists/musl/2015/10/12/2
	/// - historically differed in glibc
	#[inline(always)]
	pub const fn toPriority(self, facility: Facility) -> Priority
	{
		(facility as c_int) | (self as c_int)
	}

	#[inline(always)]
	pub const fn toPriorityRfc3164(self, rfc3164Facility: Rfc3164Facility) -> Priority
	{
		(rfc3164Facility as c_int) | (self as c_int)
	}

	/// If a message is masked then it is recorded, otherwise it is dropped
	/// Always inlined to replicate the behaviour of a C 'function' macro
	#[inline(always)]
	pub const fn mask(self) -> LogMask
	{
		LOG_MASK(self)
	}

	/// Creates a log mask that includes all priorities upto and including `toppri`
	/// "Upto" in this case means that LOG_EMERG is least and LOG_DEBUG is most
	/// This is a little counter-intuitive - it's the reverse of what many people think of
	/// eg specifying LOG_ERR in the priority logs LOG_EMERG, LOG_CRIT and LOG_ERR
	/// Always inlined to replicate the behaviour of a C 'function' macro
	#[inline(always)]
	pub const fn mask_upto(self) -> LogMask
	{
		LOG_UPTO(self)
	}
	
	pub const fn mask_all_but_self(self) -> LogMask
	{
		!self.mask()
	}
	
	pub const fn mask_and(self, other: Severity) -> LogMask
	{
		self.mask() | other.mask()
	}
}
