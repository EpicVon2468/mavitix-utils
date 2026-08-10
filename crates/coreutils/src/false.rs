pub mod boolean;

// '/bin/false --version' prints version
// '/bin/false --version foo' prints nothing
// '/bin/false foo --version' prints nothing
// '/bin/false --version --version' prints nothing
// Above applies for '--help' as well
bool_program!("false", 1);
