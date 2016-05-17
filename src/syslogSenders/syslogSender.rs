// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


use syslog2Senders::Rfc3164Facility;
use Severity;
use std::io::Result;
use rfc5424::StructuredData;


pub trait SyslogSender
{
	fn send(&self, rfc3164Facility: Rfc3164Facility, severity: Severity, structured_data_elements: &StructuredData, message: &str) -> Result<()>;
}
