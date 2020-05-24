extern crate tuturu_kurisu;

use tuturu_kurisu::Arg;
use tuturu_kurisu::*;

fn vec_to_string(args: Vec<&str>) -> Vec<String> {
    let mut strings = vec![];
    for arg in args {
        strings.push(arg.to_string());
    }

    strings
}

#[test]
fn no_value_flag_next_to_arg() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "bool",
            short: Some("a"),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "String",
            long: Some("test"),
            ..Default::default()
        },
        Arg {
            name: "test3",
            value_type: "usize",
            short: Some("b"),
            ..Default::default()
        },
        Arg {
            name: "test4",
            value_type: "String",
            short: Some("g"),
            ..Default::default()
        },
        Arg {
            name: "test5",
            value_type: "String",
            long: Some("abc"),
            ..Default::default()
        },
    ];
    let args = vec_to_string(vec!["-a", "arg1", "--test", "test1", "-b=23", "-g", "test2", "--abc=test"]);
    let expected = vec_to_string(vec!["-a", "arg1", "--test=test1", "-b=23", "-g=test2", "--abc=test"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn short_flag_stacking() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "bool",
            short: Some("a"),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "String",
            short: Some("z"),
            ..Default::default()
        },
        Arg {
            name: "test3",
            value_type: "bool",
            short: Some("b"),
            ..Default::default()
        },
        Arg {
            name: "test4",
            value_type: "String",
            long: Some("test4"),
            ..Default::default()
        },
        Arg {
            name: "test4",
            value_type: "bool",
            short: Some("c"),
            ..Default::default()
        },
    ];
    let args = vec_to_string(vec!["--test4", "test", "-abc", "arg1"]);
    let expected = vec_to_string(vec!["--test4=test", "-a", "-b", "-c", "arg1"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn optional_value() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "Option < String >",
            short: Some("a"),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "Option < String >",
            long: Some("long"),
            ..Default::default()
        },
    ];

    // No way to tell if -a optional is NOT arg1...
    let args = vec_to_string(vec!["-a", "arg1", "--long"]);
    let expected = vec_to_string(vec!["-a=arg1", "--long"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn ending_flags_with_optional_value() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "Option < String >",
            short: Some("a"),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "bool",
            short: Some("b"),
            ..Default::default()
        },
    ];

    let args = vec_to_string(vec!["arg1", "-a", "-b"]);
    let expected = vec_to_string(vec!["arg1", "-a", "-b"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn long_flag_with_dashes() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "Option < String >",
            long: Some("long"),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "bool",
            long: Some("very-long"),
            ..Default::default()
        },
        Arg {
            name: "test3",
            value_type: "bool",
            long: Some("too-long-carnage"),
            ..Default::default()
        },
    ];

    let args = vec_to_string(vec!["--too-long-carnage", "arg1", "--very-long", "--long", "lval"]);
    let expected = vec_to_string(vec!["--too-long-carnage", "arg1", "--very-long", "--long=lval"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn multiple_values() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "Vec < String >",
            long: Some("mul"),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "Vec < String >",
            short: Some("m"),
            ..Default::default()
        },
    ];

    let args = vec_to_string(vec!["-m=test1", "-m", "test2", "--mul", "test3", "--mul=test4", "-m", "test5"]);
    let expected = vec_to_string(vec!["-m=test1", "-m=test2", "--mul=test3", "--mul=test4", "-m=test5"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn required_value_without_value() {
    let kurisu_args = vec![Arg {
        name: "test1",
        value_type: "String",
        short: Some("a"),
        ..Default::default()
    }];

    let args = vec_to_string(vec!["-a=test1", "-a", "test2", "-a", "-a", "test3"]);
    let expected = vec_to_string(vec!["-a=test1", "-a=test2", "-a", "-a=test3"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}
