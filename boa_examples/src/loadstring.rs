//! This example shows how to load a JavaScript string and execute it

use boa::{Context, JsValue};

pub fn main() {
    let js_code = "console.log('Hello World from a JS code string!')";

    // Instantiate the execution context
    let mut context = Context::default();

    // Parse the source code
    let code_block = match context
        .parse(js_code)
        .map(|statement_list| context.compile(&statement_list))
    {
        Ok(res) => res,
        Err(e) => {
            // Pretty print the error
            eprintln!(
                "Uncaught {}",
                context
                    .throw_syntax_error::<String, JsValue>(e.to_string())
                    .expect_err("interpreter.throw_syntax_error() did not return an error")
                    .display()
            );

            return;
        }
    };

    // Execute the JS code read from the source file
    match context.execute(code_block) {
        Ok(v) => println!("{}", v.display()),
        Err(e) => eprintln!("Uncaught {}", e.display()),
    }
}
