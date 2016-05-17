// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate libc;
use self::libc::c_int;
use Severity;

pub type LogMask = c_int;

#[cfg(debug_assertions)]
#[inline(always)]
pub fn default_log_mask() -> LogMask
{
	Severity::LOG_DEBUG.mask()
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn default_log_mask() -> LogMask
{
	Severity::LOG_INFO.mask()
}


pub trait LogMaskTrait
{
	/// Sets the priority mask, and returns the old mask
	/// If never called, then the default LogMask allows all levels
	fn set_mask(self) -> LogMask;
}

impl LogMaskTrait for LogMask
{
	/// Sets the priority mask, and returns the old mask
	/// If never called, then the default LogMask allows all levels
	/// Always inlined as it is just a wrapper around the C function
	#[inline(always)]
	fn set_mask(self) -> LogMask
	{
		unsafe { self::libc::setlogmask(self) }
	}
}