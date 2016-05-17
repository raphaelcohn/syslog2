// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.

extern crate time;
use std::io::Write;
use self::time::Tm;
use rfc5424::TruncatedUsAsciiPrintableString;
use rfc5424::StructuredData;
use syslog2Senders::Rfc3164Facility;
use rfc5424::write_structured_data_elements;
use Severity;
use rfc5424::truncatedUsAsciiPrintableString::WriteTruncatedUsAsciiPrintableString;
use VecU8PushStr;


const SyslogProtocolVersion: &'static [u8] = b"1";

lazy_static!
{
	static ref SyslogMonths: Vec<&'static str> = new();
}

fn new() -> Vec<&'static str>
{
	assert_has_not_been_called!("Only one instance can exist");
	
	vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
}

// Duplication of fields seems unpleasant - doubles the enum width
#[derive(Debug)]
pub enum SyslogRfc
{
	Rfc3164
	{
		hostNameWithoutDomain: TruncatedUsAsciiPrintableString,
		appName: TruncatedUsAsciiPrintableString,
		processId: TruncatedUsAsciiPrintableString,
	},
	Rfc5424
	{
		hostName: TruncatedUsAsciiPrintableString,
		appName: TruncatedUsAsciiPrintableString,
		processId: TruncatedUsAsciiPrintableString,
		messageId: TruncatedUsAsciiPrintableString,
	},
}

impl SyslogRfc
{
	// The match in here seems wrong - surely we should prefer dispatch?
	pub fn write(&self, time: Tm, rfc3164Facility: Rfc3164Facility, severity: Severity, structured_data_elements: &StructuredData, message: &str) -> Vec<u8>
	{
		let mut writer: Vec<u8> = Vec::with_capacity(4096);

		writer.push(b'<');
		writer.push_str(&severity.toPriorityRfc3164(rfc3164Facility).to_string());
		writer.push(b'>');
		writer.push(b' ');
		
		match *self
		{
			SyslogRfc::Rfc3164{ref hostNameWithoutDomain, ref appName, ref processId} =>
			{
				write!(&mut writer, "{} {:02} {:04}:{:02}:{:02} ", SyslogMonths[time.tm_mon as usize], time.tm_mday, time.tm_hour, time.tm_min, time.tm_sec);
		
				writer.write_truncated(&hostNameWithoutDomain);
				writer.push(b' ');
		
				writer.write_truncated(&appName);
				writer.push(b'[');
				writer.write_truncated(&processId);
				writer.push(b']');
				writer.push(b' ');
			},
			
			SyslogRfc::Rfc5424{ref hostName, ref appName, ref processId, ref messageId} =>
			{
				writer.write(SyslogProtocolVersion);
				writer.push(b' ');

				// BUG: time.tm_sec can be 60, but RFC 5424 explicitly disallows leap seconds
				// Done this way as more efficient than strftime (we can write! rather than create an intermediate), which also has not got a specifier for microseconds
				let microseconds = time.tm_nsec / 1000;
				write!(&mut writer, "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}Z ", time.tm_year, time.tm_mon, time.tm_mday, time.tm_hour, time.tm_min, time.tm_sec, microseconds);
	
				writer.write_truncated(&hostName);
				writer.push(b' ');
	
				writer.write_truncated(&appName);
				writer.push(b' ');
	
				writer.write_truncated(&processId);
				writer.push(b' ');
	
				writer.write_truncated(&messageId);
				writer.push(b' ');
	
				write_structured_data_elements(&mut writer, structured_data_elements);
				writer.push(b' ');
		
				// UTF-8 "BOM" (silly, but it's a RFC 5424 requirement)
				writer.push(0xEF);
				writer.push(0xBB);
				writer.push(0xBF);
			},
		}

		writer.push_str(message);
		
		writer
	}
}

