// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


extern crate process;
use self::process::Process;
use rfc5424::TruncatedUsAsciiPrintableString;
use SyslogRfc;


const NILVALUE: &'static str = "-";

#[derive(Debug, Copy, Clone)]
pub enum SyslogRfcConstructor
{
	Rfc3164 = 32,
	Rfc5424 = 48,
}

impl SyslogRfcConstructor
{
	pub fn new(&self, process: &Process, messageId: &str) -> SyslogRfc
	{
		let programName = &process.programName;
		let truncatedPrintableUsAsciiProgramName = TruncatedUsAsciiPrintableString::new(if programName.is_empty()
		{
			NILVALUE
		}
		else
		{
			programName
		}, *self as usize);
		
		let truncatedPrintableProcessId = TruncatedUsAsciiPrintableString::new(&process.pid.to_string(), 128);
		
		match *self
		{
			SyslogRfcConstructor::Rfc3164 =>
			{
				let hostNameWithoutDomain = &process.hostNameWithoutDomain;
				let truncatedPrintableUsAsciiHostNameWithoutDomain = TruncatedUsAsciiPrintableString::new(if hostNameWithoutDomain.is_empty()
				{
					NILVALUE
				}
				else
				{
					hostNameWithoutDomain
				}, 255);
				
				SyslogRfc::Rfc3164
				{
					hostNameWithoutDomain: truncatedPrintableUsAsciiHostNameWithoutDomain,
					appName: truncatedPrintableUsAsciiProgramName,
					processId: truncatedPrintableProcessId,
				}
			},
			SyslogRfcConstructor::Rfc5424 =>
			{
				let hostName = &process.hostName;
				let truncatedPrintableUsAsciiHostName = TruncatedUsAsciiPrintableString::new(if hostName.is_empty()
				{
					NILVALUE
				}
				else
				{
					hostName
				}, 255);
				
				SyslogRfc::Rfc5424
				{
					hostName: truncatedPrintableUsAsciiHostName,
					appName: truncatedPrintableUsAsciiProgramName,
					processId: truncatedPrintableProcessId,
					messageId: TruncatedUsAsciiPrintableString::new(if messageId.is_empty()
					{
						NILVALUE
					}
					else
					{
						messageId
					}, 32),
				}
			},
		}
	}
	
}