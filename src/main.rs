use rand::Rng;
use std::fs;

fn main() {
    let mut rng = rand::thread_rng();
    
    let errors = vec![
        "Syntax error: unexpected token",
        "LLVM error: undefined reference",
        "Type error: incompatible types",
        "Segmentation fault in virtual compilation",
        "Unexpected end of file",
    ];

    if rng.gen_bool(0.5) {
        let random_error = errors[rng.gen_range(0..errors.len())];
        panic!("Compilation failed: {}", random_error);
    } else {
        let random_result: i32 = rng.gen_range(0..1000);
        fs::write("output.ll", format!("define i32 @main() {{ ret i32 {} }}", random_result))
            .expect("Failed to write output");
        println!("Compilation succeeded: result written to output.ll");
    }
}
