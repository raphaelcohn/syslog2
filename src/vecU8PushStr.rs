// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


use std::io::Write;


pub trait VecU8PushStr
{
    fn push_str(&mut self, value: &str);
}

impl VecU8PushStr for Vec<u8>
{
    fn push_str(&mut self, value: &str)
    {
        self.write(value.as_bytes());
    }
}
