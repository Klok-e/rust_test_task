const ENV_API_KEY: &str = "WEATHER_API_KEY";

#[test]
#[ignore]
fn basic_flow() {
    let mut cmd = assert_cmd::Command::cargo_bin(assert_cmd::crate_name!()).unwrap();

    cmd.arg("configure")
        .arg("open-weather")
        .write_stdin(std::env::var(ENV_API_KEY).unwrap())
        .assert()
        .success();

    let mut cmd = assert_cmd::Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
    cmd.arg("get").arg("London").assert().success();
}
