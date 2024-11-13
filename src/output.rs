use std::io::{stdout, Write};
use serde::ser;

use crate::commands::CommandResult;

pub(super) fn write_command_stdout_as_json<T>(result: &Box<dyn CommandResult<T>>)  
    where T: ser::Serialize { 
        let data = result.get_result();
        let data_json = serde_json::to_string_pretty(&data).unwrap();

        stdout().write_all(data_json.as_bytes()).unwrap();
}