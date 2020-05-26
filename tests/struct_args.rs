extern crate kurisu;

use kurisu::*;
use std::path::PathBuf;
use std::str::FromStr;

fn vec_to_string(args: Vec<&str>) -> Vec<String> {
    let mut strings = vec![];
    for arg in args {
        strings.push(arg.to_string());
    }

    strings
}

#[test]
fn struct_args_builtins() {
    #[derive(Debug, Kurisu)]
    struct Yargs {}

    Yargs::from_args(vec![]);
    let info = Yargs::get_info_instance(vec![]).lock().unwrap();
    assert_eq!(2, info.args.len());

    let version = info.args.iter().find(|a| a.name == "version");
    assert!(version.is_some());
    assert_eq!(version.unwrap().name, "version");
    assert_eq!(version.unwrap().short, None);
    assert_eq!(version.unwrap().long, Some("version"));

    let usage = info.args.iter().find(|a| a.name == "usage");
    assert!(usage.is_some());
    assert_eq!(usage.unwrap().name, "usage");
    assert_eq!(usage.unwrap().short, Some("h"));
    assert_eq!(usage.unwrap().long, Some("help"));
}

#[test]
fn struct_args_default_long() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        #[kurisu(short, nolong)]
        short: bool,
        #[kurisu(long = "very_long-test")]
        more: bool,
        long_arg: bool,
    }

    Yargs::from_args(vec![]);
    let info = Yargs::get_info_instance(vec![]).lock().unwrap();
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
fn struct_args_default_mandatory_values() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        string: String,
        path_buf: PathBuf,
        usize: usize,
        isize: isize,
        bool: bool,
        vec: Vec<String>,
    }

    let yargs = Yargs::from_args(vec![]);
    let info = Yargs::get_info_instance(vec![]).lock().unwrap();
    assert_eq!(8, info.args.len());

    let arg = info.args.iter().find(|a| a.name == "version");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from("false"));

    let arg = info.args.iter().find(|a| a.name == "usage");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from("false"));

    let arg = info.args.iter().find(|a| a.name == "string");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(yargs.string, String::default());

    let arg = info.args.iter().find(|a| a.name == "path_buf");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(yargs.path_buf, PathBuf::from_str("").unwrap());

    let arg = info.args.iter().find(|a| a.name == "usize");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(yargs.usize, usize::default());

    let arg = info.args.iter().find(|a| a.name == "isize");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(yargs.isize, isize::default());

    let arg = info.args.iter().find(|a| a.name == "bool");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(yargs.bool, bool::default());

    let arg = info.args.iter().find(|a| a.name == "vec");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    let default_vec: Vec<String> = Vec::new();
    assert_eq!(yargs.vec, default_vec);
}

#[test]
fn struct_args_values() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        string: String,
        path_buf: PathBuf,
        usize: usize,
        isize: isize,
        bool: bool,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "--string=mystring",
        "--path-buf=/dir/file.txt",
        "--usize=42",
        "--isize",
        "-42",
        "--bool=true",
    ]));

    assert_eq!(yargs.string, String::from("mystring"));
    assert_eq!(yargs.usize, 42);
    assert_eq!(yargs.isize, -42);
    assert_eq!(yargs.bool, true);
    assert_eq!(yargs.path_buf, PathBuf::from_str("/dir/file.txt").unwrap());
}

#[test]
fn struct_args_positional() {
    #[derive(Debug, Kurisu)]
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
fn struct_args_multiple_values() {
    #[derive(Debug, Kurisu)]
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
fn struct_args_positional_infinite() {
    #[derive(Debug, Kurisu)]
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
    ]));

    assert_eq!(yargs.second_file, String::from("/dir/file2.txt"));
    assert_eq!(yargs.fifth_file, String::from("/dir/file5.txt"));
    assert_eq!(
        yargs.files,
        vec_to_string(vec![
            "/dir/file1.txt",
            "/dir/file2.txt",
            "/dir/file3.txt",
            "/dir/file4.txt",
            "/dir/file5.txt",
        ])
    );
}
