use dash_core::parser::parser::Parser;

mod read;
mod run;
pub use read::read;
pub use run::run;

use crate::deduce::Deduce;
use crate::serialize::Serialize;

fn process(source: &str) {
    // TODO: don't expect()
    let ast = Parser::from_str(&source)
        .expect("Lexer error")
        .parse_all()
        .expect("Parser error");

    for mut stmt in ast {
        stmt.deduce();
        println!("{}", stmt.serialize());
    }
}
