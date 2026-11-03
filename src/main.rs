//! StubTHAT enforcement core — minimal skeleton.
//!
//! Candidate for the Language Pie Rust sector (Native/performance engine).
//! Maturity: stubbed_out. Not a proven runtime.
//!
//! The core receives a Bootstrap English ticket and returns a source-bound
//! outpoint. It does not execute arbitrary commands.

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: stubthat-core <ticket>");
        std::process::exit(1);
    }
    let ticket = &args[1];
    match enforce(ticket) {
        Ok(outpoint) => println!("{}", outpoint),
        Err(e) => {
            eprintln!("held: {}", e);
            std::process::exit(2);
        }
    }
}

/// Enforce a ticket. Returns a source-bound outpoint or holds with a reason.
fn enforce(ticket: &str) -> Result<String, String> {
    if ticket.trim().is_empty() {
        return Err("empty ticket".into());
    }
    // Stub: echo a receipt-shaped outpoint. Real enforcement lives later.
    Ok(format!("outpoint:ticket={} status=stubbed", ticket))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_holds() {
        assert!(enforce("").is_err());
    }

    #[test]
    fn non_empty_returns_outpoint() {
        let r = enforce("canvas").unwrap();
        assert!(r.contains("outpoint"));
        assert!(r.contains("canvas"));
    }
}
