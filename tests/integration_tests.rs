
const BIN_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/egk_xml_dump");

#[test]
fn todo_test() {
    run_test(|| {
        assert!(true, "something went wrong");
    })
}

fn setup() {

}

fn teardown() {

}

fn run_test<T>(test: T) -> () where T: FnOnce() -> () + std::panic::UnwindSafe {
    setup();
    //let daemon = Command::new(BIN_PATH).stdout(Stdio::null()).spawn();
    //thread::sleep(time::Duration::from_secs(5));
    let result = std::panic::catch_unwind(|| {
        test()
    });
    //daemon.unwrap().kill().ok();
    teardown();

    assert!(result.is_ok())
}
