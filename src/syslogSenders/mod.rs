// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


extern crate string_utilities;
extern crate libc;

pub use self::rfc3164Facility::Rfc3164Facility;
mod rfc3164Facility;

pub use self::syslog2Sender::SyslogSender;
mod syslog2Sender;

pub use self::insecureBlockingUdpSyslogSender::InsecureBlockingUdpSyslogSender;
mod insecureBlockingUdpSyslogSender;

pub use self::insecureThreadUnsafeBlockingTcpSyslogSender::InsecureThreadUnsafeBlockingTcpSyslogSender;
mod insecureThreadUnsafeBlockingTcpSyslogSender;

pub use self::posixSyslogSender::PosixSyslogSender;
mod posixSyslogSender;

// #[test]
// fn format_message_rfc3164_test()
// {
// 	let overlongHostName = "0123456789012345678901234567890123456789";
// 	let process = Process
// 	{
// 		hostName: overlongHostName.to_owned(),
// 		hostNameWithoutDomain: "macpro".to_owned(),
// 		programName: "myprogram".to_owned(),
// 		pid: 5
// 	};
// 	let rfc3164Facility = Rfc3164Facility::user;
// 	let severity = Severity::LOG_ERR;
// 	let message = "HelloWorld";
// 	let time = Tm { tm_sec: 19, tm_min: 23, tm_hour: 14, tm_mday: 8, tm_mon: 4, tm_year: 116, tm_wday: 0, tm_yday: 128, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 854377000 };
// 	let result = format_message_rfc3164(&process.hostName, &process.programName, &process.pid, time, rfc3164Facility, severity, message);
// 	assert_eq!(result, "<3>May 08 14:23:19 01234567890123456789012345678901 myprogram[5]: HelloWorld");
// }
