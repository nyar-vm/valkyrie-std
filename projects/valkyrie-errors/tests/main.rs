use miette::{NamedSource, Result};
use valkyrie_errors::ValkyrieError;

#[test]
fn ready() {
    println!("it works!")
}

fn this_fails() -> Result<()> {
    Err(ValkyrieError::duplicate_type("Optional".to_string(), (0, 10), (20, 30)))?;
    Ok(())
}

// Now to get everything printed nicely, just return a `Result<()>`
// and you're all set!
//
// Note: You can swap out the default reporter for a custom one using
// `miette::set_hook()`
#[test]
fn pretend_this_is_main() -> Result<()> {
    this_fails()?;
    Ok(())
}
