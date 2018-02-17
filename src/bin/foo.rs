
static PATHS: &'static [&str] = &["X:\\path1", "X:\\path2"];
static ENV_VARS: &'static [&str] = &["MY_ENV", "ABS_PATH_ENV", "ANOTHER_ENV"];
static ENV_VALS: &'static [&str] = &["my env value", "X:\\abs path", "another env value"];

fn main() {
    println!("This is the template file!");
}
