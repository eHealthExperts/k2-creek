# Changelog

## Unrelease

* update dependencies

## 0.7.2

* fix file removing when `force_delete` is enabled
* update dependencies

## 0.7.1

* fix output folder for kvk data files
* update dependencies

## 0.7.0

* reorganise configuration file layout
* add ouptut folder config key
* update dependencies

## 0.6.3

* use config-rs crate for configiration management
* update dependencies

## 0.6.2

* switch to editon 2018
* update dependencies

## 0.6.1

* add missing xml declaration
* update dependencies

## 0.6.0

* complete rewrite
* add `status` and `terminalId` to Result.xml

## 0.5.1

* restore KVK_Daten.bin

## 0.5.0

* human readable KVK data
* update dependencies

## 0.4.2

* adjust error handling
* update dependencies

## 0.4.1

* reactivate 32bit build
* add example config

## 0.4.0

* handle case where no card is found by k2
* enforce clippy compliance
* handle case of existing files of earlier creek runs in workdir
  * add config flag `force_delete` for avoiding interactive prompts

## 0.3.7

* enable all tests

## 0.3.6

* fix return type of errorCode

## 0.3.5

* fix kvkData

## 0.3.4

* fix ci builds

## 0.3.3

* write KVK data as ASN.1 binary file

## 0.3.2

* disable test to enable ci builds

## 0.3.1

* change default port to 8080

## 0.3.0

* encode generated XML file with ISO-8859-15
* add missing file KVK_Daten.xml
* add integration tests

## 0.2.1

* statically linked C runtime

## 0.2.0

* JSON null field tolerance

## 0.1.0

* initial release
