#[macro_use]
extern crate float_cmp;

use kurisu::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

// TODO: Centralize function or find a better way to do this
fn vec_to_string(args: Vec<&str>) -> Vec<String> {
    let mut strings = Vec::new();
    for arg in args {
        strings.push(arg.to_string());
    }

    strings
}

#[test]
fn debug_info() {
    #[derive(Kurisu)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    let test = format!("{:?}", info);
    assert!(!test.is_empty());
}

#[test]
fn builtins() {
    #[derive(Kurisu)]
    struct Yargs {}

    Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(2, info.args.len());

    let version = info.args.iter().find(|a| a.name == "version");
    assert!(version.is_some());
    assert_eq!(version.unwrap().name, "version");
    assert_eq!(version.unwrap().short, Some("V"));
    assert_eq!(version.unwrap().long, Some("version"));

    let usage = info.args.iter().find(|a| a.name == "usage");
    assert!(usage.is_some());
    assert_eq!(usage.unwrap().name, "usage");
    assert_eq!(usage.unwrap().short, Some("h"));
    assert_eq!(usage.unwrap().long, Some("help"));
}

#[test]
fn default_long() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(short, nolong)]
        short: bool,
        #[kurisu(long = "very_long-test")]
        more: bool,
        long_arg: bool,
    }

    let yargs = Yargs::from_args(Vec::new());
    assert!(!yargs.short);
    assert!(!yargs.more);
    assert!(!yargs.long_arg);

    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(5, info.args.len());

    let arg = info.args.iter().find(|a| a.name == "long_arg");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().short, None);
    assert_eq!(arg.unwrap().long, Some("long-arg"));

    let arg = info.args.iter().find(|a| a.name == "short");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().short, Some("s"));
    assert_eq!(arg.unwrap().long, None);

    let arg = info.args.iter().find(|a| a.name == "more");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().short, None);
    assert_eq!(arg.unwrap().long, Some("very_long-test"));
}

#[test]
fn auto_shorts() {
    #[derive(Kurisu)]
    #[kurisu(auto_shorts)]
    struct Yargs {
        short: bool,
        more: bool,
        long_arg: bool,
    }

    let yargs = Yargs::from_args(Vec::new());
    assert!(!yargs.short);
    assert!(!yargs.more);
    assert!(!yargs.long_arg);

    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(5, info.args.len());

    let arg = info.args.iter().find(|a| a.name == "long_arg");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().short, Some("l"));
    assert_eq!(arg.unwrap().long, Some("long-arg"));

    let arg = info.args.iter().find(|a| a.name == "short");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().short, Some("s"));
    assert_eq!(arg.unwrap().long, Some("short"));

    let arg = info.args.iter().find(|a| a.name == "more");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().short, Some("m"));
    assert_eq!(arg.unwrap().long, Some("more"));
}

#[test]
fn default_mandatory_values() {
    #[derive(Kurisu)]
    struct Yargs {
        string: String,
        path_buf: PathBuf,
        usize: usize,
        isize: isize,
        f64: f64,
        bool: bool,
        vec: Vec<String>,
        #[kurisu(default = "42")]
        default: usize,
    }

    let yargs = Yargs::from_args(Vec::new());
    assert_eq!(yargs.string, String::default());
    assert_eq!(yargs.path_buf, PathBuf::default());
    assert_eq!(yargs.usize, usize::default());
    assert_eq!(yargs.isize, isize::default());
    assert_eq!(yargs.f64, f64::default());
    assert_eq!(yargs.bool, bool::default());
    assert_eq!(yargs.default, 42usize);

    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();
    assert_eq!(10, info.args.len());

    let arg = info.args.iter().find(|a| a.name == "version");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from("false"));
    assert_eq!(arg.unwrap().occurrences, 0);

    let arg = info.args.iter().find(|a| a.name == "usage");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from("false"));
    assert_eq!(arg.unwrap().occurrences, 0);

    let arg = info.args.iter().find(|a| a.name == "string");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert_eq!(yargs.string, String::default());

    let arg = info.args.iter().find(|a| a.name == "path_buf");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert_eq!(yargs.path_buf, PathBuf::from_str("").unwrap());

    let arg = info.args.iter().find(|a| a.name == "usize");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert_eq!(yargs.usize, usize::default());

    let arg = info.args.iter().find(|a| a.name == "isize");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert_eq!(yargs.isize, isize::default());

    let arg = info.args.iter().find(|a| a.name == "f64");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert!(approx_eq!(f64, yargs.f64, f64::default(), ulps = 2));

    let arg = info.args.iter().find(|a| a.name == "bool");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert_eq!(yargs.bool, bool::default());

    let arg = info.args.iter().find(|a| a.name == "default");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from("42"));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert_eq!(yargs.default, 42);

    let arg = info.args.iter().find(|a| a.name == "vec");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    let default_vec: Vec<String> = Vec::new();
    assert_eq!(yargs.vec, default_vec);
}

#[test]
fn values() {
    #[derive(Kurisu)]
    struct Yargs {
        string: String,
        path_buf: PathBuf,
        usize: usize,
        isize: isize,
        f64: f64,
        bool: bool,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "--string=mystring",
        "--path-buf=/dir/file.txt",
        "--usize=42",
        "--isize",
        "-42",
        "--f64=4.2222",
        "--bool=true",
    ]));

    assert_eq!(yargs.string, String::from("mystring"));
    assert_eq!(yargs.usize, 42);
    assert_eq!(yargs.isize, -42);
    assert!(approx_eq!(f64, yargs.f64, 4.2222, ulps = 2));
    assert!(yargs.bool);
    assert_eq!(yargs.path_buf, PathBuf::from_str("/dir/file.txt").unwrap());
}

#[test]
fn positional() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(pos = 2)]
        file: PathBuf,
        #[kurisu(pos = 1)]
        operation: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["delete", "/dir/file.txt"]));
    assert_eq!(yargs.file, PathBuf::from_str("/dir/file.txt").unwrap());
    assert_eq!(yargs.operation, String::from("delete"));
}

#[test]
fn positional_dash_only() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(pos)]
        dash: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-"]));
    assert_eq!(yargs.dash, String::from("-"));
}

#[test]
fn only_positional_values_follow() {
    #[derive(Kurisu)]
    struct Yargs {
        zero: bool,
        one: bool,
        #[kurisu(pos)]
        test1: Vec<String>,
        #[kurisu(pos = 2)]
        test2: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "--zero", "--", "-t1", "test2", "--test1", "--", "external", "-a", "--test",
    ]));
    assert!(yargs.zero);
    assert!(!yargs.one);
    assert_eq!(yargs.test1, vec_to_string(vec!["-t1", "--test1", "--", "external", "-a", "--test"]));
    assert_eq!(yargs.test2, String::from("test2"));
}

#[test]
fn multiple_values() {
    #[derive(Kurisu)]
    struct Yargs {
        string: Vec<String>,
        path_buf: Vec<PathBuf>,
        usize: Vec<usize>,
        isize: Vec<isize>,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "--string=mystring1",
        "--usize",
        "42",
        "--string=mystring3",
        "--usize",
        "24",
        "--isize",
        "-42",
        "--path-buf=/dir1/file1.txt",
        "--isize",
        "-24",
        "--path-buf=/dir2/file2.txt",
    ]));

    assert_eq!(yargs.string, vec_to_string(vec!["mystring1", "mystring3"]));
    assert_eq!(yargs.usize, vec![42, 24]);
    assert_eq!(yargs.isize, vec![-42, -24]);
    assert_eq!(
        yargs.path_buf,
        vec![
            PathBuf::from_str("/dir1/file1.txt").unwrap(),
            PathBuf::from_str("/dir2/file2.txt").unwrap()
        ]
    );
}

#[test]
fn multiple_comma_values() {
    #[derive(Kurisu)]
    struct Yargs {
        string: Vec<String>,
        path_buf: Vec<PathBuf>,
        usize: Vec<usize>,
        isize: Vec<isize>,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "--string=mystring1,mystring2,mystring3",
        "--usize",
        "42,43,44",
        "--string=mystring4",
        "--usize",
        "45",
        "--isize=-42,-43,-44",
        "--path-buf=/dir1/file1.txt,/dir1/file2.txt,/dir1/file3.txt",
        "--isize",
        "45",
        "--path-buf=/dir1/file4.txt",
    ]));

    assert_eq!(yargs.string, vec_to_string(vec!["mystring1", "mystring2", "mystring3", "mystring4"]));
    assert_eq!(yargs.usize, vec![42, 43, 44, 45]);
    assert_eq!(yargs.isize, vec![-42, -43, -44, 45]);
    assert_eq!(
        yargs.path_buf,
        vec![
            PathBuf::from_str("/dir1/file1.txt").unwrap(),
            PathBuf::from_str("/dir1/file2.txt").unwrap(),
            PathBuf::from_str("/dir1/file3.txt").unwrap(),
            PathBuf::from_str("/dir1/file4.txt").unwrap(),
        ]
    );
}

#[test]
fn positional_infinite() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(pos)]
        files: Vec<String>,
        #[kurisu(pos = 2)]
        second_file: String,
        #[kurisu(pos = 5)]
        fifth_file: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "/dir/file1.txt",
        "/dir/file2.txt",
        "/dir/file3.txt",
        "/dir/file4.txt",
        "/dir/file5.txt",
        "/dir/file6.txt",
    ]));

    assert_eq!(yargs.second_file, String::from("/dir/file2.txt"));
    assert_eq!(yargs.fifth_file, String::from("/dir/file5.txt"));
    assert_eq!(
        yargs.files,
        vec_to_string(vec!["/dir/file1.txt", "/dir/file3.txt", "/dir/file4.txt", "/dir/file6.txt",])
    );
}

#[test]
fn positional_infinite_and_last() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(pos)]
        files: Vec<String>,
        #[kurisu(pos = -1)]
        last: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "/dir/file1.txt",
        "/dir/file2.txt",
        "/dir/file3.txt",
        "/dir/file4.txt",
        "/dir/file5.txt",
    ]));

    assert_eq!(yargs.last, String::from("/dir/file5.txt"));
    assert_eq!(
        yargs.files,
        vec_to_string(vec!["/dir/file1.txt", "/dir/file2.txt", "/dir/file3.txt", "/dir/file4.txt",])
    );
}

#[test]
fn positional_infinite_optional() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(pos = 1)]
        subcommand: String,
        #[kurisu(pos)]
        args: Vec<String>,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["dmail"]));

    assert_eq!(yargs.subcommand, String::from("dmail"));
    assert!(yargs.args.is_empty());
}

#[test]
fn empty_value_double_quoted() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(short)]
        short: String,
        long: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-s", "", "--long", ""]));
    assert_eq!(yargs.short, String::from(""));
    assert_eq!(yargs.long, String::from(""));
}

#[test]
fn occurrences() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(short, nolong)]
        verbose: u8,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-vvvv"]));
    assert_eq!(yargs.verbose, 4);
}

#[test]
fn not_occurrence() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(short, nolong)]
        test: u8,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-t", "42"]));
    assert_eq!(yargs.test, 42);
}

#[test]
fn short_flag_no_space_value() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(short, nolong)]
        itest: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["-iC4"]));
    assert_eq!(yargs.itest, String::from("C4"));
}

#[test]
fn environment_var_fallback() {
    std::env::set_var("MY_ENV_VAR", "TESTING ENV VARS");

    #[derive(Kurisu)]
    struct Yargs {
        my_env_var: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![]));
    let vars: HashMap<String, String> = std::env::vars().collect();
    assert_eq!(yargs.my_env_var, *vars.get("MY_ENV_VAR").unwrap());
}

#[test]
fn environment_var_fallback_prefix() {
    std::env::set_var("MY_ENV_VAR", "TESTING ENV VARS");

    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(env_prefix = "MY_")]
        env_var: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![]));
    let vars: HashMap<String, String> = std::env::vars().collect();
    assert_eq!(yargs.env_var, *vars.get("MY_ENV_VAR").unwrap());
}

#[test]
fn environment_var_fallback_override() {
    std::env::set_var("MY_ENV_VAR", "TESTING ENV VARS");

    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(env = "MY_ENV_VAR")]
        test: String,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![]));
    let vars: HashMap<String, String> = std::env::vars().collect();
    assert_eq!(yargs.test, *vars.get("MY_ENV_VAR").unwrap());
}

#[test]
fn annotation_parser() {
    #[derive(Kurisu)]
    struct Yargs {
        #[kurisu(parse_with = "capitalize")]
        hello: String,
        #[kurisu(parse_with = "capitalize")]
        test: String,
    }

    pub fn capitalize(name: &str, info: &'_ Info) -> String {
        let arg = info.args.iter().find(|a| name == a.name).expect("Infallible");
        if arg.value.is_empty() {
            return String::from("");
        }

        arg.value[0].to_uppercase()
    }

    let yargs = Yargs::from_args(vec_to_string(vec!["--test", "hello"]));
    assert_eq!(yargs.test, String::from("HELLO"));
    assert_eq!(yargs.hello, String::from(""));
}
