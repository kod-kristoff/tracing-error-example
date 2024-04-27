use std::env;
use std::error::Error;

use tac::tac;
use tracing_error::ErrorLayer;
// use tracing_error::ErrorSubscriber;
use tracing_error::ExtractSpanTrace as _;
use tracing_subscriber::prelude::*;

fn main() {
    let subscriber = tracing_subscriber::Registry::default()
        // any number of other subscriber layers may be added before or
        // after the `ErrorSubscriber`...
        .with(ErrorLayer::default());

    // set the subscriber as the default for the application
    let _ = tracing::subscriber::set_global_default(subscriber);

    let f = env::args_os().nth(1).unwrap();
    if let Err(err) = tac(f) {
        print_naive_spantraces(&err);
        println!("===");
        print_extracted_spantraces(&err);
    }
}

fn print_extracted_spantraces(error: &(dyn Error + 'static)) {
    let mut error = Some(error);
    let mut ind = 0;

    eprintln!("Error:");

    while let Some(err) = error {
        if let Some(spantrace) = err.span_trace() {
            eprintln!("found a spantrace:\n{}", spantrace);
        } else {
            eprintln!("{:>4}: {}", ind, err);
        }

        error = err.source();
        ind += 1;
    }
}

fn print_naive_spantraces(error: &(dyn Error + 'static)) {
    let mut error = Some(error);
    let mut ind = 0;

    eprintln!("Error:");

    while let Some(err) = error {
        eprintln!("{:>4}: {}", ind, err);
        error = err.source();
        ind += 1;
    }
}
