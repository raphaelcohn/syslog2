// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


// Bug in rust nightly as of May 5th
#![allow(dead_code)]

extern crate time;
extern crate network_constants;
use std::io::Result;
use std::io::ErrorKind;
use std::net::ToSocketAddrs;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::TcpStream;
use std::io::Write;
use std::net::Shutdown;
use self::network_constants::tcp::SyslogPort;
use self::network_constants::ipv4;
use self::network_constants::ipv6;
use syslog2Senders::Rfc3164Facility;
use syslog2Senders::SyslogSender;
use Severity;
use SyslogRfc;
use rfc5424::StructuredData;

#[derive(Debug)]
pub struct InsecureThreadUnsafeBlockingTcpSyslogSender
{
	syslog2Rfc: SyslogRfc,
	stream: TcpStream
}

impl InsecureThreadUnsafeBlockingTcpSyslogSender
{
	fn new<S: ToSocketAddrs>(syslog2Rfc: SyslogRfc, serverSocketAddress: S) -> Result<InsecureThreadUnsafeBlockingTcpSyslogSender>
	{
		let stream = try!(TcpStream::connect(serverSocketAddress));
		try!(stream.set_write_timeout(None));
		try!(stream.shutdown(Shutdown::Read));
		
		Ok(InsecureThreadUnsafeBlockingTcpSyslogSender
		{
			syslog2Rfc: syslog2Rfc,
			stream: stream,
		})
	}
	
	fn new_ipv4_localhost_514(syslog2Rfc: SyslogRfc) -> Result<InsecureThreadUnsafeBlockingTcpSyslogSender>
	{
		InsecureThreadUnsafeBlockingTcpSyslogSender::new(syslog2Rfc, (ipv4::localhost(), SyslogPort))
	}
	
	fn new_ipv4_514(syslog2Rfc: SyslogRfc, serverAddress: Ipv4Addr) -> Result<InsecureThreadUnsafeBlockingTcpSyslogSender>
	{
		InsecureThreadUnsafeBlockingTcpSyslogSender::new(syslog2Rfc, (serverAddress, SyslogPort))
	}
	
	fn new_ipv6_localhost_514(syslog2Rfc: SyslogRfc) -> Result<InsecureThreadUnsafeBlockingTcpSyslogSender>
	{
		InsecureThreadUnsafeBlockingTcpSyslogSender::new(syslog2Rfc, (ipv6::localhost(), SyslogPort))
	}
	
	fn new_ipv6_514(syslog2Rfc: SyslogRfc, serverAddress: Ipv6Addr) -> Result<InsecureThreadUnsafeBlockingTcpSyslogSender>
	{
		InsecureThreadUnsafeBlockingTcpSyslogSender::new(syslog2Rfc, (serverAddress, SyslogPort))
	}
}

impl SyslogSender for InsecureThreadUnsafeBlockingTcpSyslogSender
{
	fn send(&self, rfc3164Facility: Rfc3164Facility, severity: Severity, structured_data_elements: &StructuredData, message: &str) -> Result<()>
	{
		let timeNow = time::now_utc();
		let data = self.syslog2Rfc.write(timeNow, rfc3164Facility, severity, structured_data_elements, message);
		
		let bytesLength: usize = data.len();
		let mut bytesWrittenSoFar: usize = 0;
		
		let mut stream = &self.stream;
		loop
		{
			let result = stream.write(&data[bytesWrittenSoFar..]);
		
			match result
			{
				Ok(bytesSent) =>
				{
					bytesWrittenSoFar += bytesSent;
					if bytesWrittenSoFar == bytesLength
					{
						return Ok(())
					}
					debug_assert!(bytesWrittenSoFar <= bytesLength, "Syscalls to UDP sendto() are broken - they overwrote!");
				},
				Err(error) =>
				{
					match error.kind()
					{
						ErrorKind::WriteZero => continue,
						ErrorKind::WouldBlock => continue,
						ErrorKind::TimedOut => continue,
						ErrorKind::Interrupted => continue,
						//ErrorKind::ConnectionAborted => ? reconnect ?
						_ => return Err(error)
					}
				},
			}
		}
	}
}
