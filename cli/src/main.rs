use std::fs;

use askama::Template;
use yaspec::YASpec;

mod ts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("reading yaspec");
    let yaspec = fs::read_to_string("./cli/examples/todo/yaspec.yaml")?;
    let yaspec = serde_yaml::from_str::<YASpec>(&yaspec)?;

    println!("writing typescript");
    let output = ts::RootTemplate(&yaspec).render()?;
    fs::write("./cli/examples/todo/out/todo.ts", output)?;

    Ok(())
}
