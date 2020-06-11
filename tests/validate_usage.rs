use kurisu::arg::Error;
use kurisu::*;
use std::path::PathBuf;

fn vec_to_string(args: Vec<&str>) -> Vec<String> {
    let mut strings = Vec::new();
    for arg in args {
        strings.push(arg.to_string());
    }

    strings
}

#[test]
fn no_args() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(Vec::new());
    let error = kurisu::validate_usage(&yargs);
    assert_eq!(error.unwrap(), Error::NoArgs);
}

#[test]
fn exit_arg_version() {
    #[derive(Debug, Kurisu)]
    struct Yargs {}

    let info = Yargs::get_info_instance(vec_to_string(vec!["--version"])).lock().unwrap();
    let exit_code = kurisu::exit_args(&info, Some);
    assert_eq!(exit_code.unwrap(), ExitCode::OK.into());
}

#[test]
fn exit_arg_help() {
    #[derive(Debug, Kurisu)]
    struct Yargs {}

    let info = Yargs::get_info_instance(vec_to_string(vec!["--help"])).lock().unwrap();
    let exit_code = kurisu::exit_args(&info, Some);
    assert_eq!(exit_code.unwrap(), ExitCode::USAGE.into());
}

#[test]
fn exit_arg_custom() {
    fn exit_func() -> i32 {
        ExitCode::NOINPUT.into()
    }

    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(exit = "exit_func")]
        exit_plz: bool,
    }

    let info = Yargs::get_info_instance(vec_to_string(vec!["--exit-plz"])).lock().unwrap();
    let exit_code = kurisu::exit_args(&info, Some);
    assert_eq!(exit_code.unwrap(), ExitCode::NOINPUT.into());
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
        // If we have a default the value should not generate a usage error
        #[kurisu(default = "something")]
        a_flag: String,
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "--long"]));
    let long = {
        let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
        info.args.iter().find(|a| a.name == "long").unwrap().clone()
    };

    let error = kurisu::validate_usage(&yargs);
    assert!(error.is_some(), "We should get an Error::RequiresValue");
    assert_eq!(error.unwrap(), Error::RequiresValue(long));
}

#[test]
fn value_required_vs_occurrence() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short, nolong)]
        short: usize,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s"]));
    let short = {
        let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
        info.args.iter().find(|a| a.name == "short").unwrap().clone()
    };

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

#[test]
fn required_if_args() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        atest: Option<String>,
        #[kurisu(short, required_if = "atest")]
        btest: Option<String>,
        #[kurisu(short)]
        ctest: Option<String>,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-a=test"]));
    let error = kurisu::validate_usage(&yargs);

    let atest;
    let btest;
    {
        let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
        atest = info.args.iter().find(|a| a.name == "atest").unwrap().clone();
        btest = info.args.iter().find(|a| a.name == "btest").unwrap().clone();
    }

    assert_eq!(error.unwrap(), Error::RequiresValueIf(atest, btest));
}

#[test]
fn positional_missing_value() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(pos = 2)]
        file: PathBuf,
        #[kurisu(pos = 1)]
        operation: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["delete"]));
    let error = kurisu::validate_usage(&yargs);

    let arg = {
        let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
        info.args.iter().find(|a| a.name == "file").unwrap().clone()
    };

    assert_eq!(error.unwrap(), Error::RequiresPositional(arg));
}
