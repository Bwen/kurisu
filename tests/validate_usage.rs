extern crate kurisu;

use kurisu::arg::Error;
use kurisu::*;

fn vec_to_string(args: Vec<&str>) -> Vec<String> {
    let mut strings = vec![];
    for arg in args {
        strings.push(arg.to_string());
    }

    strings
}

#[test]
fn value_empty() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "", "--long", ""]));
    let error = kurisu::validate_usage(&yargs);
    assert!(error.is_none());
}

#[test]
fn value_required() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "--long"]));
    let short;
    {
        let info = Yargs::get_info_instance(vec![]).lock().unwrap();
        short = info.args.iter().find(|a| a.name == "short").unwrap().clone();
    }

    let error = kurisu::validate_usage(&yargs);
    assert!(error.is_some(), "We should get an Error::RequiresValue");
    assert_eq!(error.unwrap(), Error::RequiresValue(short));
}

#[test]
fn invalid_short() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "test1", "--long", "test2", "-k"]));
    let error = kurisu::validate_usage(&yargs);
    assert_eq!(error.unwrap(), Error::Invalid(String::from("-k")));
}

#[test]
fn invalid_long() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "test1", "--long", "test2", "--test"]));
    let error = kurisu::validate_usage(&yargs);
    assert_eq!(error.unwrap(), Error::Invalid(String::from("--test")));
}

#[test]
fn invalid_arg() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "test1", "--long", "test2", "test"]));
    let error = kurisu::validate_usage(&yargs);
    assert_eq!(error.unwrap(), Error::Invalid(String::from("test")));
}

#[test]
fn invalid_arg_within_args() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
        #[kurisu(pos = 1)]
        source: String,
        #[kurisu(pos = 2)]
        target: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "-s",
        "test1",
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "--long",
        "test2",
    ]));

    let error = kurisu::validate_usage(&yargs);
    assert_eq!(error.unwrap(), Error::Invalid(String::from("file3.txt")));
}

#[test]
fn infinite_pos() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
        #[kurisu(pos)]
        test: Vec<String>,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "file1.txt",
        "-s",
        "test1",
        "file2.txt",
        "--long",
        "test2",
        "file3.txt",
    ]));

    let error = kurisu::validate_usage(&yargs);
    assert!(
        error.is_none(),
        "Should not Error out, infinite pos should be greedy and absorb all positional args"
    );
}

#[test]
fn invalid_arg_last_pos() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
        #[kurisu(pos = -1)]
        test: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "test1", "file1.txt", "--long", "test2", "file2.txt"]));
    let error = kurisu::validate_usage(&yargs);
    assert_eq!(error.unwrap(), Error::Invalid(String::from("file1.txt")));
}
