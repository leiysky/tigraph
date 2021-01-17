#![feature(iterator_fold_self)]

mod catalog;
mod core;
mod optimizer;
mod parser;
mod runtime;
mod server;
mod types;
mod util;

use server::run;
use util::{Error, ErrorKind};

extern crate nom;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
