// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

use Facility;


/// Clock is also known as cron, and is the standard for cron and at messages
/// Solaris' cron (15) is not supported
/// Use of numbers 12 - 15 inclusive or above 23 is not cross-platform compatible and any message recieved with these codes should, regardless of RFC 3164, be considered private use
/// and recorded or used in a manner compatible with 'secret' (some of these codes on BSD systems are used for LOG_SECURITY and LOG_CONSOLE).
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum Rfc3164Facility
{
	kern = 0,
	user = 1,
	mail = 2,
	daemon = 3,
	auth = 4,
	syslog2 = 5,
	lpr = 6,
	news = 7,
	uucp = 8,
	clock = 9,
	authpriv = 10,
	ftp = 11,
	//cron = 15, // Solaris only; re-used as RAS by Mac OS X
	local0 = 16,
	local1 = 17,
	local2 = 18,
	local3 = 19,
	local4 = 20,
	local5 = 21,
	local6 = 22,
	local7 = 23,
}

impl Default for Rfc3164Facility
{
	fn default() -> Rfc3164Facility
	{
		Rfc3164Facility::user
	}
}

impl Rfc3164Facility
{
	/// All succeed apart from Rfc3164Facility::ftp, for which there is no mapping on Solaris
	/// ie if your discount Solaris, you can always `unwrap()`
	pub fn toFacilityAccommodatingSolaris(self) -> Option<Facility>
	{
		match self
		{
			Rfc3164Facility::kern => Some(Facility::LOG_KERN),
			Rfc3164Facility::user => Some(Facility::LOG_USER),
			Rfc3164Facility::mail => Some(Facility::LOG_MAIL),
			Rfc3164Facility::daemon => Some(Facility::LOG_DAEMON),
			Rfc3164Facility::auth => Some(Facility::LOG_AUTH),
			Rfc3164Facility::syslog2 => Some(Facility::LOG_SYSLOG),
			Rfc3164Facility::lpr => Some(Facility::LOG_LPR),
			Rfc3164Facility::news => Some(Facility::LOG_NEWS),
			Rfc3164Facility::uucp => Some(Facility::LOG_UUCP),
			Rfc3164Facility::clock => Some(Facility::LOG_CRON),
			Rfc3164Facility::authpriv => Some(Facility::bestAuthenticationFacilityForPlatform()),
			Rfc3164Facility::ftp =>
			{
				if cfg!(target_os = "solaris")
				{
					None
				}
				else
				{
					Some(Facility::LOG_FTP)
				}
			},
			Rfc3164Facility::local0 => Some(Facility::LOG_LOCAL0),
			Rfc3164Facility::local1 => Some(Facility::LOG_LOCAL1),
			Rfc3164Facility::local2 => Some(Facility::LOG_LOCAL2),
			Rfc3164Facility::local3 => Some(Facility::LOG_LOCAL3),
			Rfc3164Facility::local4 => Some(Facility::LOG_LOCAL4),
			Rfc3164Facility::local5 => Some(Facility::LOG_LOCAL5),
			Rfc3164Facility::local6 => Some(Facility::LOG_LOCAL6),
			Rfc3164Facility::local7 => Some(Facility::LOG_LOCAL7),
		}
	}
	
	pub fn toFacilityMappingSolarisToDaemon(self) -> Facility
	{
		self.toFacilityAccommodatingSolaris().unwrap_or(Facility::LOG_DAEMON)
	}
	
	/// Use of numbers 12 - 15 inclusive or above 23 is not cross-platform compatible and should, regardless of RFC 3164, be considered private use; we return None
	/// Some of these have been used for security messages, so anything returning None should be used or categorised as 'secret'.
	pub fn from(wire: i32) -> Option<Rfc3164Facility>
	{
		match wire
		{
			0 => Some(Rfc3164Facility::kern),
			1 => Some(Rfc3164Facility::user),
			2 => Some(Rfc3164Facility::mail),
			3 => Some(Rfc3164Facility::daemon),
			4 => Some(Rfc3164Facility::auth),
			5 => Some(Rfc3164Facility::syslog2),
			6 => Some(Rfc3164Facility::lpr),
			7 => Some(Rfc3164Facility::news),
			8 => Some(Rfc3164Facility::uucp),
			9 => Some(Rfc3164Facility::clock),
			10 => Some(Rfc3164Facility::authpriv),
			11 => Some(Rfc3164Facility::ftp),
			12 => None,
			13 => None,
			14 => None,
			15 => None, // Solaris cron, Mac OS X RAS
			16 => Some(Rfc3164Facility::local0),
			17 => Some(Rfc3164Facility::local1),
			18 => Some(Rfc3164Facility::local2),
			19 => Some(Rfc3164Facility::local3),
			20 => Some(Rfc3164Facility::local4),
			21 => Some(Rfc3164Facility::local5),
			22 => Some(Rfc3164Facility::local6),
			23 => Some(Rfc3164Facility::local7),
			24 => None, // Mac OS X LAUNCHD
			_ => None,
		}
	}
}

