#[cfg(feature = "parser_extras")]
use kurisu::*;

#[cfg(feature = "parser_extras")]
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[cfg(feature = "parser_extras")]
use url::Url;

#[cfg(feature = "parser_extras")]
fn vec_to_string(args: Vec<&str>) -> Vec<String> {
    let mut strings = Vec::new();
    for arg in args {
        strings.push(arg.to_string());
    }

    strings
}

#[cfg(feature = "parser_extras")]
#[test]
fn default_mandatory_values() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        ip: IpAddr,
        url: Url,
        query_string: Url,
    }

    let yargs = Yargs::from_args(Vec::new());
    let info = Yargs::get_info_instance(Vec::new()).lock().unwrap();

    let arg = info.args.iter().find(|a| a.name == "ip");
    assert!(arg.is_some());
    assert_eq!(arg.unwrap().default, String::from(""));
    assert_eq!(arg.unwrap().occurrences, 0);
    assert_eq!(yargs.ip, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    assert_eq!(yargs.url, Url::parse("data:text/plain,").unwrap());
    assert_eq!(yargs.query_string, Url::parse("data:text/plain,").unwrap());
}

#[cfg(feature = "parser_extras")]
#[test]
fn values() {
    #[derive(Debug, Kurisu)]
    struct Yargs {
        ipv4: IpAddr,
        ipv6: IpAddr,
        url: Url,
        query_string: Url,
    }

    let yargs = Yargs::from_args(vec_to_string(vec![
        "--ipv4=127.0.0.1",
        "--ipv6=::1",
        "--url",
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open",
        "--query-string",
        "labels=E-easy&state=open",
    ]));

    assert_eq!(yargs.ipv4, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(yargs.ipv6, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
    assert_eq!(yargs.query_string, Url::parse("data:text/plain,?labels=E-easy&state=open").unwrap());
    assert_eq!(
        yargs.url,
        Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open").unwrap()
    );
}
