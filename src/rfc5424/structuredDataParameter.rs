// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


use std::borrow::Cow;
use rfc5424::TruncatedUsAsciiPrintableString;
use rfc5424::truncatedUsAsciiPrintableString::WriteTruncatedUsAsciiPrintableString;
use std::io::Write;
use VecU8PushStr;


pub struct StructuredDataParameter<'a>
{
	name: &'a TruncatedUsAsciiPrintableString,
	value: Cow<'a, str>,
}

impl <'a> StructuredDataParameter<'a>
{
	pub fn new(name: &'a TruncatedUsAsciiPrintableString, value: Cow<'a, str>) -> StructuredDataParameter<'a>
	{
		StructuredDataParameter
		{
			name: name,
			value: value,
		}
	}
	
	pub fn write(&self, mut writer: &mut Vec<u8>)
	{
		writer.push(b' ');
		writer.write_truncated(self.name);
		writer.push_str("=\"");
		for character in self.value.chars()
		{
			match character
			{
				'"' => writer.push_str("\\\""),
				'\\' => writer.push_str("\\\\"),
				']' => writer.push_str("\\["),
				_ => 
				{
					// TODO: This is deeply sub-optimal, but encode_utf8() is not yet stable and we can't use it.
					let mut suboptimalString = String::with_capacity(4);
					suboptimalString.push(character);
					writer.push_str(&suboptimalString);
				}
			}
		}
		writer.push(b'"');
	}
}
