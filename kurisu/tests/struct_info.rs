use kurisu::*;
use std::fs;
use toml::Value;

#[test]
fn annotation() {
    #[derive(Kurisu)]
    #[kurisu(name = "yargs", version = "1.0.0", desc = "some desc here")]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(info.name, Some("yargs"));
    assert_eq!(info.version, Some("1.0.0"));
    assert_eq!(info.desc, Some("some desc here"));
    assert_eq!(info.desc, Some("some desc here"));
    assert!(!info.allow_noargs);
}

#[test]
fn cargo() {
    #[derive(Kurisu)]
    #[kurisu(cargo)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    let cargo_string = fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml = cargo_string.parse::<Value>().unwrap();

    assert_eq!(info.name, Some(cargo_toml["package"]["name"].as_str().unwrap()));
    assert_eq!(info.version, Some(cargo_toml["package"]["version"].as_str().unwrap()));
    assert_eq!(info.desc, Some(cargo_toml["package"]["description"].as_str().unwrap()));
}

#[test]
fn cargo_only_if_no_value() {
    #[derive(Kurisu)]
    #[kurisu(cargo, name = "yargs", version = "1.2.3", desc = "some desc here")]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();

    assert_eq!(info.name, Some("yargs"));
    assert_eq!(info.version, Some("1.2.3"));
    assert_eq!(info.desc, Some("some desc here"));
}

#[test]
fn doc() {
    #[derive(Kurisu)]
    /// line one
    /// line two
    /// line three
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(info.doc, Some("line one\nline two\nline three"));
}

#[test]
fn none() {
    #[derive(Kurisu)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(info.name, None);
    assert_eq!(info.version, None);
    assert_eq!(info.doc, None);
}

#[test]
fn noargs() {
    #[derive(Kurisu)]
    #[kurisu(allow_noargs)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert!(info.allow_noargs);
}
