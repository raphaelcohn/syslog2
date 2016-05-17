// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate libc;
use self::libc::c_int;
use std::mem::transmute;
use Severity;
use Facility;

#[cfg(not(target_os = "windows"))]
const LOG_PRIMASK:c_int = self::libc::LOG_PRIMASK;
#[cfg(target_os = "windows")]
const LOG_PRIMASK:c_int = 7;

#[cfg(not(target_os = "windows"))]
const LOG_FACMASK:c_int = self::libc::LOG_FACMASK;
#[cfg(target_os = "windows")]
const LOG_FACMASK:c_int = 0x3f8;

pub type Priority = c_int;

/// Always inlined to replicate the behaviour of a C 'function' macro
#[inline(always)]
fn LOG_PRI(p: Priority) -> Severity
{
	unsafe { transmute((p) & LOG_PRIMASK) }
}

/// Always inlined to replicate the behaviour of a C 'function' macro
#[inline(always)]
fn LOG_FAC(p: Priority) -> Facility
{
	unsafe { transmute((p & LOG_FACMASK) >> 3) }
}

pub trait PriorityTrait
{
	fn severity(self) -> Severity;
	
	fn facility(self) -> Facility;
}

impl PriorityTrait for Priority
{
	/// Always inlined to replicate the behaviour of a C 'function' macro
	#[inline(always)]
	fn severity(self) -> Severity
	{
		LOG_PRI(self)
	}
	
	/// Always inlined to replicate the behaviour of a C 'function' macro
	#[inline(always)]
	fn facility(self) -> Facility
	{
		LOG_FAC(self)
	}
}
