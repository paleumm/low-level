// SPDX-License-Identifier: GPL-2.0

//! Template

use kernel::prelude::*;

module! {
    type: Template,
    name: "template",
    license: "GPL",
}

struct Template {
    numbers: Vec<i32>,
}

impl kernel::Module for Template {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("template (init)\n");

        let mut numbers = Vec::new();
        numbers.try_push(72)?;
        numbers.try_push(108)?;
        numbers.try_push(200)?;

        Ok(Template { numbers })
    }
}

impl Drop for Template {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Template sample (exit)\n");
    }
}
