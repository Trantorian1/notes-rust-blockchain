#[allow(dead_code)]
// TODO:
// >
// > Each section of code marked this way will detail an exercise for you to solve.
// >
// > This is only a sample designed to make sure you can compile this activity
// > on your machine. Make sure that works before moving on!
// >
struct Todo {
    a: u32,
    b: u32,
}

// NOTE:
// >
// > Sections marked this way indicate some extra information that is not directly
// > related to the task at hand, but might still prove useful.
// >

impl Todo {
    #[allow(dead_code)]
    fn foo() -> bool {
        true
    }

    #[allow(dead_code)]
    fn bazz() -> bool {
        false
    }
}

// TEST:
// >
// > A few sanity checks will be provided in each activity. Keep in mind it is
// > not enough for you code to pass these to be valid! A full battery of test
// > will be provided in the next section, along with a sample solution.
// >
// > You can run these sanity checks with:
// >
// > `cargo test`
// >
// > Or if you want debug output
// >
// > `RUST_LOG=info cargo test`
// >
// > Run this now to make sure this activity compile on you machine! You should
// > be greeted with the output:
// >
// > `test tests::foo ... ok`
// > `test tests::baz ... ok`
// >
#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn foo() {
        log::info!("Testing foo");
        assert!(Todo::foo());
    }

    #[test_log::test]
    fn baz() {
        log::info!("Testing bazz");
        assert!(!Todo::bazz());
    }
}
