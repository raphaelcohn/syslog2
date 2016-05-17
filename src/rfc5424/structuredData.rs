// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


use std::collections::HashSet;
use rfc5424::StructuredDataElement;


pub type StructuredData<'a> = HashSet<StructuredDataElement<'a>>;

pub fn write_structured_data_elements<'a>(mut writer: &mut Vec<u8>, structured_data_elements: &StructuredData)
{
	if structured_data_elements.is_empty()
	{
		writer.push(b'-');
		return
	}
	
	for structured_data_element in structured_data_elements
	{
		structured_data_element.write(&mut writer);
	}
}
