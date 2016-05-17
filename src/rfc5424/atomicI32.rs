// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


// TODO: This is a temporary implementation until std::sync::atomic::AtomicI32 is stable; its size may be either 32 or 64 bits!


use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering;


pub struct AtomicI32(AtomicIsize);

impl AtomicI32
{
	pub const fn new(v: i32) -> AtomicI32
	{
		AtomicI32(AtomicIsize::new(v as isize))
	}
	
	pub fn load(&self, order: Ordering) -> i32
	{
		self.0.load(order) as i32
	}
	
	pub fn compare_and_swap(&self, current: i32, new: i32, order: Ordering) -> i32
	{
		self.0.compare_and_swap(current as isize, new as isize, order) as i32
	}
}
