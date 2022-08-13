use common::{args, check};
use libtest_mimic::{Test, Conclusion};

#[macro_use]
mod common;


fn tests() -> Vec<Test> {
    vec![
        Test::test("passes", || Ok(())),
        Test::test("panics", || panic!("uh oh")),
    ]
}

#[test]
fn normal() {
    check(args([]), tests, 2,
        Conclusion {
            num_filtered_out: 0,
            num_passed: 1,
            num_failed: 1,
            num_ignored: 0,
            num_measured: 0,
        },
        "
            test passes ... ok
            test panics ... FAILED

            failures:

            ---- panics ----
            test panicked: uh oh


            failures:
                panics
        "
    );
}