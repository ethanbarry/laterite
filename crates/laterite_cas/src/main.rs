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
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
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
                match p.parse(line.clone()) {
                    Ok(tree) => {
                        dbg!(tree);
                    }
                    Err(errs) => {
                        errs.into_iter().for_each(|e| {
                            let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason()
                            {
                                msg.clone()
                            } else {
                                format!(
                                    "{}{}, expected {}",
                                    if e.found().is_some() {
                                        "Unexpected token"
                                    } else {
                                        "Unexpected end of input"
                                    },
                                    if let Some(label) = e.label() {
                                        format!(" while parsing {}", label)
                                    } else {
                                        String::new()
                                    },
                                    if e.expected().len() == 0 {
                                        "something else".to_string()
                                    } else {
                                        e.expected()
                                            .map(|expected| match expected {
                                                Some(expected) => expected.to_string(),
                                                None => "end of input".to_string(),
                                            })
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    },
                                )
                            };

                            let report = Report::build(ReportKind::Error, (), e.span().start)
                                .with_code(3)
                                .with_message(msg)
                                .with_label(
                                    Label::new(e.span())
                                        .with_message(match e.reason() {
                                            chumsky::error::SimpleReason::Custom(msg) => {
                                                msg.clone()
                                            }
                                            _ => format!(
                                                "Unexpected {}",
                                                e.found()
                                                    .map(|c| format!("token {}", c.fg(Color::Red)))
                                                    .unwrap_or_else(|| "end of input".to_string())
                                            ),
                                        })
                                        .with_color(Color::Red),
                                );

                            let report = match e.reason() {
                                chumsky::error::SimpleReason::Unclosed { span, delimiter } => {
                                    report.with_label(
                                        Label::new(span.clone())
                                            .with_message(format!(
                                                "Unclosed delimiter {}",
                                                delimiter.fg(Color::Yellow)
                                            ))
                                            .with_color(Color::Yellow),
                                    )
                                }
                                chumsky::error::SimpleReason::Unexpected => report,
                                chumsky::error::SimpleReason::Custom(_) => report,
                            };

                            report.finish().print(Source::from(&line)).unwrap();
                        });
                    }
                };
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
