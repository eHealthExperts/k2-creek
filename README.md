# K2 creek

[![Latest](https://img.shields.io/github/release/eHealthExperts/k2-creek.svg?label=latest)](https://github.com/eHealthExperts/k2-creek/releases/latest)
[![Build Status](https://github.com/eHealthExperts/k2-creek/workflows/Test/badge.svg)](https://github.com/eHealthExperts/k2-creek/actions)
[![codecov](https://codecov.io/gh/eHealthExperts/k2-creek/branch/master/graph/badge.svg)](https://codecov.io/gh/eHealthExperts/k2-creek)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Microsoft Windows [binary](https://github.com/eHealthExperts/k2-creek/releases/latest) that extract data from the electronic health insurance card via [K2](https://k2.ehealthexperts.de/)

It then extracts relevant fields and writes out some XML/binary files (compatible with WINCrd2XML from Sagem Monetel).

## Requirements

* [K2](https://k2.ehealthexperts.de/) from eHealth Experts GmbH

## Usage

Copy the [binary](https://github.com/eHealthExperts/k2-creek/releases/latest) into a desired folder. After the binary was executed, the card information at the configured K2 path will be written into [some files](tests/writer) next to the executable.

## Configuration

Execute `k2-creek.exe --help` to see the command line configuration options.

Alternatively locate a file `config.ini` next to the binary to configure the URL to work with.

The following example shows the default values.

```ini
[k2]
api=2
timeout= # default is no timeout
url= # no default

[output]
force_delete=false
path=.
```

* `force_delete` is for avoiding interaction when older generated files were found at the target destination. Setting this to `true` causes automatic deletion.
