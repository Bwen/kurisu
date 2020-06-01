extern crate kurisu;

use kurisu::*;
use std::fs;
use toml::Value;

#[test]
fn annotation() {
    #[derive(Debug, Kurisu)]
    #[kurisu(name = "yargs", version = "1.0.0", doc = "some doc here")]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(info.name, Some("yargs"));
    assert_eq!(info.version, Some("1.0.0"));
    assert_eq!(info.doc, Some("some doc here"));
}

#[test]
fn cargo() {
    #[derive(Debug, Kurisu)]
    #[kurisu(cargo)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    let cargo_string = fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml = cargo_string.parse::<Value>().unwrap();

    assert_eq!(info.name, Some(cargo_toml["package"]["name"].as_str().unwrap()));
    assert_eq!(info.version, Some(cargo_toml["package"]["version"].as_str().unwrap()));
    assert_eq!(info.doc, Some(cargo_toml["package"]["description"].as_str().unwrap()));
}

#[test]
fn doc() {
    #[derive(Debug, Kurisu)]
    /// line one
    /// line two
    /// line three
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(info.doc, Some("line one line two line three"));
}

#[test]
fn none() {
    #[derive(Debug, Kurisu)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(info.name, None);
    assert_eq!(info.version, None);
    assert_eq!(info.doc, None);
}

#[test]
fn noargs() {
    #[derive(Debug, Kurisu)]
    #[kurisu(allow_noargs)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(info.allow_noargs, true);
}