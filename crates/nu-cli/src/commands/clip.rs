use crate::command_registry::CommandRegistry;
use crate::commands::WholeStreamCommand;
use crate::prelude::*;
use futures::stream::StreamExt;
use nu_errors::ShellError;
use nu_protocol::{Signature, Value};

use clipboard::{ClipboardContext, ClipboardProvider};

pub struct Clip;

#[async_trait]
impl WholeStreamCommand for Clip {
    fn name(&self) -> &str {
        "clip"
    }

    fn signature(&self) -> Signature {
        Signature::build("clip")
    }

    fn usage(&self) -> &str {
        "Copy the contents of the pipeline to the copy/paste buffer"
    }

    async fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        clip(args, registry).await
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Save text to the clipboard",
                example: "echo 'secret value' | clip",
                result: None,
            },
            Example {
                description: "Save numbers to the clipboard",
                example: "random integer 10000000..99999999 | clip",
                result: None,
            },
        ]
    }
}

pub async fn clip(
    args: CommandArgs,
    _registry: &CommandRegistry,
) -> Result<OutputStream, ShellError> {
    let input = args.input;
    let name = args.call_info.name_tag.clone();
    let values: Vec<Value> = input.collect().await;

    if let Ok(clip_context) = ClipboardProvider::new() {
        let mut clip_context: ClipboardContext = clip_context;
        let mut new_copy_data = String::new();

        if !values.is_empty() {
            let mut first = true;
            for i in values.iter() {
                if !first {
                    new_copy_data.push('\n');
                } else {
                    first = false;
                }

                let string: String = i.convert_to_string();
                if string.is_empty() {
                    return Err(ShellError::labeled_error(
                        "Unable to convert to string",
                        "Unable to convert to string",
                        name,
                    ));
                }

                new_copy_data.push_str(&string);
            }
        }

        match clip_context.set_contents(new_copy_data) {
            Ok(_) => {}
            Err(_) => {
                return Err(ShellError::labeled_error(
                    "Could not set contents of clipboard",
                    "could not set contents of clipboard",
                    name,
                ));
            }
        }
    } else {
        return Err(ShellError::labeled_error(
            "Could not open clipboard",
            "could not open clipboard",
            name,
        ));
    }
    Ok(OutputStream::empty())
}

#[cfg(test)]
mod tests {
    use super::Clip;
    use super::ShellError;

    #[test]
    fn examples_work_as_expected() -> Result<(), ShellError> {
        use crate::examples::test as test_examples;

        Ok(test_examples(Clip {})?)
    }
}
