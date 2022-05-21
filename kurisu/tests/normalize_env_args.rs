use kurisu::Arg;
use kurisu::*;

fn vec_to_string(args: Vec<&str>) -> Vec<String> {
    let mut strings = Vec::new();
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

// When we pass -t "This is a long string value" we get the following from std:env:args()
// Args {
//     inner: [
//         "target/debug/main",
//         "-t",
//         "This is a long string value",
//     ],
// }
#[test]
fn string_value_double_quoted() {
    let kurisu_args = vec![
        Arg {
            name: "test",
            value_type: "String",
            long: Some("test"),
            ..Default::default()
        },
        Arg {
            name: "short",
            value_type: "String",
            short: Some("s"),
            ..Default::default()
        },
    ];

    let args = vec_to_string(vec!["--test", "This is a long string value", "-s", "This is a short string value"]);
    let expected = vec_to_string(vec!["--test=This is a long string value", "-s=This is a short string value"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn empty_value_double_quoted() {
    let kurisu_args = vec![
        Arg {
            name: "test",
            value_type: "String",
            long: Some("test"),
            ..Default::default()
        },
        Arg {
            name: "short",
            value_type: "String",
            short: Some("s"),
            ..Default::default()
        },
    ];

    let args = vec_to_string(vec!["--test", "", "-s", ""]);
    let expected = vec_to_string(vec!["--test=", "-s="]);
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
fn multiple_comma_values() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "Vec < String >",
            long: Some("mul"),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "Vec < isize >",
            short: Some("m"),
            ..Default::default()
        },
    ];

    let args = vec_to_string(vec!["-m=-1,-2", "-m", "3,-4", "--mul", "test1,test2", "--mul=test3,test4", "-m", "5,-6"]);

    let expected = vec_to_string(vec![
        "-m=-1",
        "-m=-2",
        "-m=3",
        "-m=-4",
        "--mul=test1",
        "--mul=test2",
        "--mul=test3",
        "--mul=test4",
        "-m=5",
        "-m=-6",
    ]);

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

#[test]
fn positional_value_dash_only() {
    let kurisu_args = vec![Arg {
        name: "dash",
        value_type: "String",
        short: Some("d"),
        ..Default::default()
    }];

    let args = vec_to_string(vec!["-"]);
    let expected = vec_to_string(vec!["-"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn only_positional_values_follow() {
    let kurisu_args = vec![
        Arg {
            name: "test1",
            value_type: "Vec < String >",
            short: Some("a"),
            position: Some(0),
            ..Default::default()
        },
        Arg {
            name: "test2",
            value_type: "String",
            short: Some("b"),
            position: Some(2),
            ..Default::default()
        },
        Arg {
            name: "test3",
            value_type: "bool",
            short: Some("c"),
            ..Default::default()
        },
    ];

    let args = vec_to_string(vec!["-c", "--", "-a", "-b", "test", "-d"]);
    let expected = vec_to_string(vec!["-c", "--", "-a", "-b=test", "-d"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}

#[test]
fn short_flag_no_space_value() {
    let kurisu_args = vec![Arg {
        name: "test1",
        value_type: "String",
        short: Some("i"),
        ..Default::default()
    }];

    let args = vec_to_string(vec!["-iC4"]);
    let expected = vec_to_string(vec!["-i=C4"]);
    let norm_args = normalize_env_args(&args, &kurisu_args);
    assert_eq!(norm_args, expected);
}
