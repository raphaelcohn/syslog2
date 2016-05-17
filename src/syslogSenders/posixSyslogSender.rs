// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


extern crate process;
extern crate time;
extern crate string_utilities;
use std::io::Result;
use syslog2Senders::Rfc3164Facility;
use syslog2Senders::SyslogSender;
use Severity;
use syslog2_cstr_withFacility;
use self::string_utilities::to_cstr_best_effort;
use rfc5424::StructuredData;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct PosixSyslogSender
{
}

impl PosixSyslogSender
{
	/// Assumes the syslog2 is already open
	#[allow(dead_code)]
	fn new() -> Result<PosixSyslogSender>
	{
		Ok(PosixSyslogSender
		{	
		})
	}
}

impl SyslogSender for PosixSyslogSender
{
	fn send(&self, rfc3164Facility: Rfc3164Facility, severity: Severity, structured_data_elements: &StructuredData, message: &str) -> Result<()>
	{
		let (cStringMessage, errorOption) = to_cstr_best_effort(message);
		
		syslog2_cstr_withFacility(severity, &cStringMessage, rfc3164Facility.toFacilityMappingSolarisToDaemon());
		
		match errorOption
		{
			None => Ok(()),
			Some(error) => Err(error),
		}
	}
}
