use crate::Cli;
use serde::ser;

pub mod generate;

pub trait CommandExecutionContext {
    fn get_cli(&self) -> &Cli;
}

pub trait CommandExec<T>
where
    T: ser::Serialize,
{
    async fn exec(
        &self,
        context: &impl CommandExecutionContext,
    ) -> Result<Box<dyn CommandResult<T>>, Box<dyn std::error::Error>>;
}

pub trait CommandResult<T>
where
    T: ser::Serialize,
{
    fn get_result(&self) -> &T;
}
