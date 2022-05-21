![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# kurisu-derive

Provides `derive(Kurisu)` for the [Kurisu](../kurisu/index.html) crate.

The Kurisu derive macro is not meant to do any validation nor display usage for the command line
string values. It will only construct **_[kurisu::Arg](../kurisu/arg/index.html)_**
structs according to a given Rust Struct, and put them within a function
`get_info_instance` wrapped by a [once_cell](https://docs.rs/once_cell) static instance of
a **_[kurisu::Info](../kurisu/struct.Info.html)_** struct.

Example:
```rust
use kurisu::*;

#[derive(Debug, Kurisu)]
struct Yargs {
    knots: usize,
}

fn main() {
    let env_vars: Vec<String> = std::env::args().skip(1).collect();

    // The Derive Macro add `from_args` function to the user's struct,
    // `from_args` will normalize the command line values into kurisu::Info.env_args
    let args = Yargs::from_args(&env_vars);

    let knots;
    // This scope is so the mutex guard gets released after
    // we retrieve the information we need.
    {
        // The Derive Macro also add `get_info_instance` function to the user's struct.
        // The Kurisu:Info struct holds all the parsed information the Derive Macro did.
        let info = Yargs::get_info_instance(&env_vars).lock().unwrap();
        knots = info.args.iter().find(|a| a.name == "knots").unwrap().clone();
    }

    // If we assume the current example was called as follow: `mycli --knots 8`
    // We would find that information within the Kurisu::Info struct
    assert_eq!(info.env_args, vec!["--knots=8".to_string()]);
    assert_eq!(knots.value, vec!["8".to_string()]);
    assert_eq!(args.knots, 8);
}
```

### Rust types with specific meaning

Type          | Description
--------------|---------------------------------------------
bool          | Defines a Flag
u8            | Defines a Flag with occurrence tracking as its value
String        | Defines value type of an Option or Argument
PathBuf       | Defines value type of an Option or Argument
usize         | Defines value type of an Option or Argument
isize         | Defines value type of an Option or Argument
f64           | Defines value type of an Option or Argument
Vec<T>        | Defines a repetitive Option or Argument

### Override default behaviors on main Struct

The struct that drives these behaviors is handled by
*__[kurisu::Info](../kurisu/struct.Info.html)__*, most of the annotation are associated to one
of its field.

Field name    | Default | Annotation | Description
--------------|--------------|-------------------------|---------------------------------------------
name          | "Unknown"    | name = "mycli"          | Change name on usage screen
version       | 0            | version = "0.1.0"       | Change version on usage screen
desc          | None         | desc = "some short text"| Change description on usage screen
doc           | None         | ///                     | Change DISCUSSION text on usage screen
allow_noargs  | false        | allow_noargs            | Does not display usage screen if no command line values
&nbsp;        | &nbsp;       | cargo                   | Will try to fetch name, version & desc from the Cargo.toml, the specific field annotation take precedence on the cargo annotation
&nbsp;        | &nbsp;       | nosort                  | Avoid sorting alphabetically arguments, flags & options on usage screen
&nbsp;        | &nbsp;       | auto_shorts             | Enables the auto generation of short flags / options according to their field's name first letter, by default no short flags / options are generated unless specified on the struct's field annotation

Example:
```rust
    #[derive(Debug, Kurisu)]
    // Only picks the version from Cargo.toml since name & desc are already present
    #[kurisu(name = "yargs", cargo, desc = "some desc here", auto_shorts)]
    /// This text is for the DISCUSSION
    /// section in the use screen
    ///
    /// It can be a very large text
    struct Yargs {}
```

### Override default behaviors on main Struct's fields

The struct that drives these behaviors is handled by
*__[kurisu::Arg](../kurisu/arg/struct.Arg.html)__*, most of the annotation are associated to one
of its field.

Field name    | Default       | Annotation                 | Description
--------------|---------------|----------------------------|---------------------------------------------
vname         | field's name  | vname = "myname"           | Change the name of the <VALUE> in the usage screen
position      | None          | pos **OR** pos = "1"       | Defines the position of an argument, `pos` without a value defines an infinite positional argument
doc           | None          | ///                        | Defines the description of arg on the usage screen
short         | None          | short **OR** short = "b"   | Change the letter used by short flag / option, otherwise it takes the first letter of the struct's field name
long          | field's name  | nolong **OR** long = "myname" | Change or remove the long flag / option
env           | field's name  | env = "MYSQL_HOST"         | By default the field's name is used to look for environment variable match as last resort, this is to change which environment variable is being matched
env_prefix    | None          | env_prefix = "MYSQL_"      | Will take the field's name and prefix it and look for a match environment variable
required_if   | None          | required_if = "fieldname"  | Will make this option required if the other flag/option is present
default       | ""            | default = "42"             | If not present in the command line values, struct field will be assigned this default value instead of default type value
exit          | None          | exit = "my_exit_func"      | Local function that triggers std::process::exit() after being executed. A bit like the usage display, it will stop execution at `from_args`
&nbsp;        | &nbsp;        | parse_with = "my_func"     | Local function that allows custom parsing for this argument/flag/option

Example:
```rust
use std::path::PathBuf;

#[derive(Debug, Kurisu)]
struct Yargs {
    #[kurisu(pos = 1)]
    file: PathBuf,
    #[kurisu(pos)] // Infinite positional argument
    others: String,
    #[kurisu(short, nolong, env_prefix = "MYSQL_")]
    host: String,
    #[kurisu(long = "very-long-flag")]
    more: bool,
    /// description for `crashed` on usage screen
    crashed: bool,
}
```


License: MIT OR Apache-2.0
