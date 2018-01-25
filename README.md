# K2 creek

[![Latest](https://img.shields.io/github/release/eHealthExperts/k2-creek.svg?label=latest)](https://github.com/eHealthExperts/k2-creek/releases/latest)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/2p4sq7nnsjn1wo95/branch/master?svg=true)](https://ci.appveyor.com/project/ChriFo/k2-creek)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Microsoft Windows [binary](https://github.com/eHealthExperts/k2-creek/releases/latest) that extract data from the electronic health insurance card via [K2](https://k2.ehealthexperts.de/).<br/>
It then extracts relevant fields and writes out some XML files (compatible with WINCrd2XML from Sagem Monetel).

## Requirements
* [K2](https://k2.ehealthexperts.de/) from eHealth Experts GmbH

## Configuration
Locate a file `config.ini` next to the binary to configure the URL to work with.<br/>
The following example shows the default values.

```ini
[settings]
scheme=http
host=localhost
port=5000
path=/k2/public/api/1/carddata
```
