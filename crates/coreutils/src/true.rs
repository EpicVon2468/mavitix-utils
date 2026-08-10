pub mod boolean;

// '/bin/true --version' prints version
// '/bin/true --version foo' prints nothing
// '/bin/true foo --version' prints nothing
// '/bin/true --version --version' prints nothing
// Above applies for '--help' as well
bool_program!("true", 0);
