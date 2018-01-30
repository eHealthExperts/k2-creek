# K2 creek

[![Latest](https://img.shields.io/github/release/eHealthExperts/k2-creek.svg?label=latest)](https://github.com/eHealthExperts/k2-creek/releases/latest)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/pax0ee62rrb3bvoc/branch/master?svg=true)](https://ci.appveyor.com/project/ChriFo/k2-creek)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Microsoft Windows [binary](https://github.com/eHealthExperts/k2-creek/releases/latest) that extract data from the electronic health insurance card via [K2](https://k2.ehealthexperts.de/).<br/>
It then extracts relevant fields and writes out some XML files (compatible with WINCrd2XML from Sagem Monetel).

## Requirements
* [K2](https://k2.ehealthexperts.de/) from eHealth Experts GmbH

## Usage

Copy the [binary](https://github.com/eHealthExperts/k2-creek/releases/latest) into a desired folder. After the binary was executed, the card information at the configured K2 path will be written as XML files next to the binary. The following files are created when information is delivered by the card:<br/>

- `eGK_allgemeineVersicherungsdaten.xml`
- `eGK_geschuetzteVersichertendaten.xml`
- `eGK_MFDF_HCA_EF_StatusVD.xml`
- `eGK_MFEFGDO.xml`
- `eGK_PersoenlicheVersichertendaten.xml`
- `eGK_Pruefungsnachweis.xml`
- `KVK_Daten.xml`
- `Result.xml`

For more information of the file content have a look into the WinCrd2XML handbook.


## Configuration
Locate a file `config.ini` next to the binary to configure the URL to work with.<br/>
The following example shows the default values.

```ini
[settings]
scheme=http
host=localhost
port=8080
path=/k2/public/api/1/carddata
```
