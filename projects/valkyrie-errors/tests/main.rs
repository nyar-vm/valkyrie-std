use miette::{NamedSource, Result};
use valkyrie_errors::ValkyrieError;

#[test]
fn ready() {
    println!("it works!")
}

fn this_fails() -> Result<()> {
    // You can use plain strings as a `Source`, or anything that implements
    // the one-method `Source` trait.
    let src = "source\n  text\n    here".to_string();
    let len = src.len();
    Err(ValkyrieError::duplicate_type("type".to_string(), "a".to_string()))?;

    Ok(())
}

// Now to get everything printed nicely, just return a `Result<()>`
// and you're all set!
//
// Note: You can swap out the default reporter for a custom one using
// `miette::set_hook()`
#[test]
fn pretend_this_is_main() -> Result<()> {
    // kaboom~
    this_fails()?;

    Ok(())
}
