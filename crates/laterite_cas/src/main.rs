// %%%%%%%%%%%%%%%%%%%% || %%%%%%%%%%%%%%%%%%%%
// %%                LATERITE                %%
// %%               ----++----               %%
// %%   Laterite is an open-source computer  %%
// %% algebra system, written in Rust, and   %%
// %% licensed to you under the terms of the %%
// %% GNU GPLv3 license (only).              %%
// %%                                        %%
// %% You may redistribute it or reuse the   %%
// %% provided code in accordance with that  %%
// %% license.                               %%
// %%               ----++----               %%
// %% USAGE:                                 %%
// %%   Laterite is intended to be used      %%
// %% through the REPL provided in this      %%
// %% crate, laterite-cas.                   %%
// %%   The computation is handled by the    %%
// %% other crate in this repository, called %%
// %% laterite-core.                         %%
// %%                                        %%
// %%   Assuming you have Rust installed,    %%
// %% you can run Laterite with the command  %%
// %% ```shell                               %%
// %% cargo run --release                    %%
// %% ```                                    %%
// %% for optimal compilation.               %%
// %%               ----++----               %%
// %%           C O P Y R I G H T            %%
// %%              ETHAN  BARRY              %%
// %%                  2024                  %%
// %%%%%%%%%%%%%%%%%%%% || %%%%%%%%%%%%%%%%%%%%

mod parser;

use ansi_term::Style;
use chumsky::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::parser::parser;

fn main() -> Result<()> {
    println!(
        "Laterite Computer Algebra System\nVersion: {}",
        env!("CARGO_PKG_VERSION")
    );
    println!("This program is free software under the GPLv3 license.");
    println!(
        "Enter {} for help, or type {} to quit, and {} to terminate.",
        Style::new().bold().paint("?"),
        Style::new().bold().paint("Ctrl+D"),
        Style::new().bold().paint("Ctrl+C"),
    );

    let p = parser();

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
                dbg!(p.parse(line).expect("Did not parse."));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
