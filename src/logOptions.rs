// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate libc;
use std::default::Default;

#[cfg(not(target_os = "windows"))]
bitflags!
{
	pub flags LogOptions: ::libc::c_int
	{
		const LOG_PID = ::libc::LOG_PID,
		const LOG_CONS = ::libc::LOG_CONS,
		const LOG_ODELAY = ::libc::LOG_ODELAY,
		const LOG_NDELAY = ::libc::LOG_NDELAY,
		const LOG_NOWAIT = ::libc::LOG_NOWAIT,
		#[cfg(not(target_os = "solaris"))] const LOG_PERROR = ::libc::LOG_PERROR,
	}
}

/// Windows flags are fakes to allow some compatibility. They are set to match the values on Linux.
#[cfg(target_os = "windows")]
bitflags!
{
	pub flags LogOptions: libc::c_int
	{
		const LOG_PID = 0x01,
		const LOG_CONS = 0x02,
		const LOG_ODELAY = 0x04,
		const LOG_NDELAY = 0x08,
		const LOG_NOWAIT = 0x10,
		const LOG_PERROR = 0x20,
	}
}

impl LogOptions
{
	pub fn clear(&mut self)
	{
		self.bits = 0;
	}
	
	/// Log also to standard error. Not supported on Solaris (will panic if yes is true)
	#[cfg(not(target_os = "solaris"))] 
	pub fn logToStandardErrorAsWell(&mut self, yes: bool)
	{
		if yes
		{
			self.insert(LOG_PERROR);
		}
		else
		{
			self.remove(LOG_PERROR);
		}
	}
}

impl Default for LogOptions
{
	/// Defaults to `LOG_CONS`, `LOG_NDELAY` and `LOG_PID`
	#[cfg(not(target_os = "android"))]
	#[inline(always)]
	fn default() -> LogOptions
	{
		 LOG_CONS | LOG_NDELAY | LOG_PID
	}
	
	/// Android only supports LOG_PID and LOG_PERROR
	#[cfg(target_os = "android")]
	#[inline(always)]
	fn default() -> LogOptions
	{
		 LOG_PID
	}
}
