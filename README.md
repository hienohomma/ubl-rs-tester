# ubl-rs-tester

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Create example invoice using bells and whistles from [ubl-rs](https://github.com/hienohomma/ubl-rs) crate.

## Description

Recreates [example invoice trivial](https://docs.oasis-open.org/ubl/UBL-2.1-JSON/v1.0/cnd02/json/UBL-Invoice-2.1-Example-Trivial.json) from UBL documentation using [ubl-rs](https://github.com/hienohomma/ubl-rs) crate.

Both the [invoice example](json_examples/UBL-Invoice-2.1-Example-Trivial.json) and the generated invoice are printed as oneliners next to [invoice example](json_examples/UBL-Invoice-2.1-Example-Trivial.json) into a file named `UBL-Invoice-2.1-Example-Trivial-comparison.json` where you can easily compare the results.

At the time of writing the only differences are in field ordering and decimal length (1.00 example vs. 1.0 generated)

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Installation

You need to [install rust](https://www.rust-lang.org/tools/install) and cargo to compile the binary.  
Only tested on linux, if binary compiles / program works on windows or mac that's completely unintentional.

## Usage

Clone this project and navigate to it's parent directory.  
Execute binary and see if it teminates with exitcode 0.  
Compare the generated invoice to the original example on [json_examples](json_examples/) directory

```bash
cd ubl-rs-tester
cargo run
nano json_examples/UBL-Invoice-2.1-Example-Trivial-comparison.json
```

## Contributing

Pull requests, reported issues, improvements in documentation etc. are always welcome.  
Try to behave while at it.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Contact

- Email: <opensource@hienohomma.fi>
- GitHub: [hienohomma](https://github.com/hienohomma)
