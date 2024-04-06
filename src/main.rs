mod parser;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    let html = ureq::get("https://www.kudosprime.com/fh5/carlist.php?range=2000")
        .call()?
        .into_string()?;
    let cars = parser::parse_carlist(&html)?;
    let json = serde_json::to_string_pretty(&cars)?;
    println!("{json}");

    Ok(())
}
