// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


extern crate uptime;
use self::uptime::uptime_in_microseconds;
use std::net::IpAddr;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::atomic::Ordering;
use std::cmp::max;
use rfc5424::TruncatedUsAsciiPrintableString;
use rfc5424::StructuredDataParameter;
use rfc5424::truncatedUsAsciiPrintableString::WriteTruncatedUsAsciiPrintableString;
use rfc5424::atomicI32::AtomicI32;

static mut SequenceId: AtomicI32 = AtomicI32::new(1);

lazy_static!
{
	pub static ref timeQualitySdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("timeQuality");
	pub static ref tzKnownSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("tzKnown");
	pub static ref isSyncedSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("isSynced");
	pub static ref syncAccuracySdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("syncAccuracy");
	pub static ref originSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("origin");
	pub static ref enterpriseIdSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("enterpriseId");
	pub static ref softwareSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("software");
	pub static ref softwareVersionSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("softwareVersion");
	pub static ref ipSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("ip");
	pub static ref metaSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("meta");
	pub static ref sequenceIdSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("sequenceId");
	pub static ref sysUpTime: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("sysUpTime");
	pub static ref languageSdName: TruncatedUsAsciiPrintableString = TruncatedUsAsciiPrintableString::new_sd_name("language");
}

pub struct StructuredDataElement<'a>
{
	id: &'a TruncatedUsAsciiPrintableString,
	parameters: Vec<StructuredDataParameter<'a>>
}

impl <'a> StructuredDataElement<'a>
{
	// More at https://www.iana.org/assignments/syslog2-parameters/syslog2-parameters.xhtml
	
	pub fn write(&self, mut writer: &mut Vec<u8>)
	{	
		writer.push(b'[');
		
		writer.write_truncated(self.id);
		
		for parameter in &self.parameters
		{
			parameter.write(&mut writer);
		}
		
		writer.push(b']');
	}
	
	/// syncAccuracy must be 0 if isSynced is false
	pub fn timeQuality(tzKnown: bool, isSynced: bool, syncAccuracy: u64) -> StructuredDataElement<'a>
	{
		debug_assert!(isSynced && syncAccuracy != 0, "syncAccuracy must be 0 (not {}) if isSynced is false", syncAccuracy);
		
		let tzKnownValue = match tzKnown
		{
			true => "1",
			false => "0"
		};
		
		let isSyncedValue = match isSynced
		{
			true => "1",
			false => "0"
		};
		
		let syncAccuracyValue = syncAccuracy.to_string();
		
		StructuredDataElement
		{
			id: &timeQualitySdName,
			parameters: vec![tzKnownSdName.parameter(tzKnownValue.into()), isSyncedSdName.parameter(isSyncedValue.into()), syncAccuracySdName.parameter(syncAccuracyValue.into())]
		}
	}
	
	// /// software must be less than 48 characters
	// /// softwareVersion must be less than 32 characters
	// /// StormMQ's ianaPrivateEnterpriseNumber is 38188, for instance
	// /// We do not support sub-identifiers (we could, using a vec!, I suppose)
	pub fn origin(ianaPrivateEnterpriseNumber: u32, software: &'a str, softwareVersion: &'a str, ipAddresses: &Vec<IpAddr>) -> StructuredDataElement<'a>
	{
		debug_assert!(software.len() <= 48, "software must be less than 48 characters, not {}", software.len());
		debug_assert!(softwareVersion.len() <= 32, "softwareVersion must be less than 32 characters, not {}", softwareVersion.len());

		let mut parameters: Vec<StructuredDataParameter<'a>> = Vec::with_capacity(3 + ipAddresses.len());
		parameters.push(enterpriseIdSdName.parameter(format!("1.3.6.1.4.1.{}", ianaPrivateEnterpriseNumber).into()));
		parameters.push(softwareSdName.parameter(software.into()));
		parameters.push(softwareVersionSdName.parameter(softwareVersion.into()));
		for ipAddress in ipAddresses
		{
			parameters.push(ipSdName.parameter(ipAddress.to_string().into()));
		}

		StructuredDataElement
		{
			id: &originSdName,
			parameters: parameters,
		}
	}

	pub fn meta() -> StructuredDataElement<'a>
	{
		let mut parameters: Vec<StructuredDataParameter<'a>> = Vec::with_capacity(3);
		
		let mut current: i32;
		current = unsafe { SequenceId.load(Ordering::Relaxed) };
		loop
		{
			let next = if current == 2147483647
			{
				1
			}
			else
			{
				current + 1
			};
			let previous = unsafe { SequenceId.compare_and_swap(current, next, Ordering::Relaxed) };
			if previous == current
			{
				break
			}
			current = previous;
		}
		parameters.push(sequenceIdSdName.parameter(current.to_string().into()));
		
		if let Ok(uptime_in_microseconds) = uptime_in_microseconds()
		{
			let uptime_in_hundredths_of_a_second = max(uptime_in_microseconds, 0) / 10_000;
			parameters.push(sysUpTime.parameter(uptime_in_hundredths_of_a_second.to_string().into()));
		}
		
		parameters.push(languageSdName.parameter("en".into()));
				
		StructuredDataElement
		{
			id: &metaSdName,
			parameters: parameters,
		}
	}
}

impl <'a> PartialEq for StructuredDataElement<'a>
{
	#[inline]
	fn eq(&self, other: &StructuredDataElement<'a>) -> bool
	{
		return self.id.eq(&other.id);
	}
}

impl <'a> Eq for StructuredDataElement<'a>
{
}

impl <'a> Hash for StructuredDataElement<'a>
{
	#[inline]
	fn hash<H>(&self, state: &mut H) where H: Hasher
	{
		self.id.hash(state);
	}
}
