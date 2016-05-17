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
use std::net::UdpSocket;
use self::network_constants::UdpPort;
use self::network_constants::udp::BindToAnyLocalUdpPortAvailable;
use self::network_constants::udp::SyslogPort;
use self::network_constants::ipv4;
use self::network_constants::ipv6;
use syslog2Senders::Rfc3164Facility;
use syslog2Senders::SyslogSender;
use rfc5424::StructuredData;
use SyslogRfc;
use Severity;

#[derive(Debug)]
pub struct InsecureBlockingUdpSyslogSender<S: ToSocketAddrs>
{
	syslog2Rfc: SyslogRfc,
	socket: UdpSocket,
	serverSocketAddress: S,
}

impl <S: ToSocketAddrs> InsecureBlockingUdpSyslogSender<S>
{
	fn new(syslog2Rfc: SyslogRfc, localSocketAddress: S, serverSocketAddress: S) -> Result<InsecureBlockingUdpSyslogSender<S>>
	{
		let socket = try!(UdpSocket::bind(localSocketAddress));
		try!(socket.set_write_timeout(None));
		
		Ok(InsecureBlockingUdpSyslogSender
		{
			syslog2Rfc: syslog2Rfc,
			socket: socket,
			serverSocketAddress: serverSocketAddress,
		})
	}
}

impl InsecureBlockingUdpSyslogSender<(Ipv4Addr, UdpPort)>
{
	fn new_from_localhost_on_any_port_to_localhost_on_514(syslog2Rfc: SyslogRfc) -> Result<InsecureBlockingUdpSyslogSender<(Ipv4Addr, UdpPort)>>
	{
		<InsecureBlockingUdpSyslogSender<(Ipv4Addr, UdpPort)>>::new_from_localhost_on_any_port(syslog2Rfc, (ipv4::localhost(), SyslogPort))
	}
	
	fn new_from_localhost_on_any_port(syslog2Rfc: SyslogRfc, serverSocketAddress: (Ipv4Addr, UdpPort)) -> Result<InsecureBlockingUdpSyslogSender<(Ipv4Addr, UdpPort)>>
	{
		let x = (ipv4::localhost(), BindToAnyLocalUdpPortAvailable);
		InsecureBlockingUdpSyslogSender::new(syslog2Rfc, x, serverSocketAddress)
	}
}

impl InsecureBlockingUdpSyslogSender<(Ipv6Addr, UdpPort)>
{
	fn new_from_localhost_on_any_port_to_localhost_on_514(syslog2Rfc: SyslogRfc) -> Result<InsecureBlockingUdpSyslogSender<(Ipv6Addr, UdpPort)>>
	{
		<InsecureBlockingUdpSyslogSender<(Ipv6Addr, UdpPort)>>::new_from_localhost_on_any_port(syslog2Rfc, (ipv6::localhost(), SyslogPort))
	}
	
	fn new_from_localhost_on_any_port(syslog2Rfc: SyslogRfc, serverSocketAddress: (Ipv6Addr, UdpPort)) -> Result<InsecureBlockingUdpSyslogSender<(Ipv6Addr, UdpPort)>>
	{
		let x = (ipv6::localhost(), BindToAnyLocalUdpPortAvailable);
		InsecureBlockingUdpSyslogSender::new(syslog2Rfc, x, serverSocketAddress)
	}
}

impl <S: ToSocketAddrs> SyslogSender for InsecureBlockingUdpSyslogSender<S>
{
	fn send(&self, rfc3164Facility: Rfc3164Facility, severity: Severity, structured_data_elements: &StructuredData, message: &str) -> Result<()>
	{
		let timeNow = time::now_utc();
		
		let data = self.syslog2Rfc.write(timeNow, rfc3164Facility, severity, structured_data_elements, message);
		
		let bytesLength: usize = data.len();
		let mut bytesWrittenSoFar: usize = 0;
		
		loop
		{
			let result = self.socket.send_to(&data[bytesWrittenSoFar..], &self.serverSocketAddress);
		
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
						ErrorKind::WriteZero => continue, // Hmmm, is this possible for UDP?
						ErrorKind::WouldBlock => continue,
						ErrorKind::TimedOut => continue,
						ErrorKind::Interrupted => continue,
						//ErrorKind::ConnectionAborted => ? reconnect - but this is UDP ...
						_ => return Err(error)
					}
				},
			}
		}
	}
}
