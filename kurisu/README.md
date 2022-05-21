![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# kurisu

Command line arguments parser through custom derive macro

For full documentation on `derive(Kurisu)`, please see [kurisu_derive](../kurisu_derive/index.html).

### Parsing, Validating and Usage display
Kurisu separate these three concepts:
```rust
use kurisu::*;

#[derive(Debug, Kurisu)]
struct Yargs {
    knots: usize,
}

fn main() {
    let env_vars: Vec<String> = std::env::args().skip(1).collect();

    // Will take the string values from the command line and try to parse them and assign
    // them to the struct's field. If the flag or option is not present then
    // its default type value will be assigned to the struct's field.
    // In this case: usize::default()
    let args = Yargs::from_args(env_vars);

    // Returns an Option<kurisu::arg::Error> or None
    let arg_error = kurisu::validate_usage(&args);

    // If an error is present `print_usage_error` will std::process::exit()
    // with kurisu::ExitCode::USAGE(64) as exit code
    mayuri::print_usage_error(&args, arg_error);

    // Assuming the application was called like so: `mycli --knots 8`
    assert_eq!(args.knots, 8);
}
```
You can shorten this to **_kurisu::[valid_exit](fn.valid_exit.html)(&args)_**
which combines **_kurisu::[validate_usage](fn.validate_usage.html)(&args)_** and
**_[mayuri](mayuri/index.html)::[print_usage_error](mayuri/fn.print_usage_error.html)(&args)_**.

Kurisu tries to have sane defaults for the struct, if we take the following struct as example:
```rust
struct Yargs {
    sinking: bool,
    knots: usize,
    pirate_ship_name: String
}
```
The field `pirate_ship_name` will have no short option `-p` and only a long
option `--pirate-ship-name`. The characters `_` of the field's name will be matched
and displayed as `-`. The field `sinking` will be a flag because of its type `bool`.
The defaults can be altered through annotation,
please see [kurisu_derive](../kurisu_derive/index.html) for more information.

Kurisu as specific definitions for Argument, Flag and Option. They are by no mean an official
definition, but this is how they are handled within this library.

### Arguments
A single word in a command line, example: `mycli myargument`. Also refered to as a positional
argument where you can define a specific struct field to a specific argument position.

They are never prefixed by either `-` or `--`. Supported struct field types:
- `String`,
- `PathBuf`,
- `usize`,
- `isize`,
- `f64`,
- `bool`,

It is possible to define an infinite positional argument where that struct field's value will
include all positional arguments (_excluding other defined arguments with specific positions_).
The infinite positional argument struct field type is defined by `Vec<T>` and one of the
supported types.

Arguments are always required. There is no way to make them optional.

### Flags
Prefixed by either `-` or `--`, examples: `mycli --my-flag`, `mycli -f`. Their struct field type
is always a `bool`. They never have a value associated to them, example: `mycli -f value`,
is considered a flag followed by an argument unless `-f` is defined as an option.

It is possible to stack short flags, example: `mycli -fBc`.

It is also possible to have repeating flags with their occurrences counted,
example `mycli -vvv`. In this case the struct field type is a `u8`.

Flags are always optional. There is no way to make them required.

### Options
Prefixed by either `-` or `--` followed by a value, examples: `mycli --my-option=myvalue`,
`mycli -f myvalue`. An option value assignment operator can either be `=` or ` `.
They support the same types as arguments.

It is possible to have repeating options, example: `mycli -f one -f=two -f three`,
in this case their struct field type is `Vec<T>` with a valid type.

Options are always optional by default, but if present their value is always required.
It is possible to have an option be required through the annotation `required_if`,
for more details see [kurisu_derive](../kurisu_derive/index.html).


License: MIT OR Apache-2.0
