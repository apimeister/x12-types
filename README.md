# X12-types
[![Latest Version](https://img.shields.io/crates/v/x12-types.svg)](https://crates.io/crates/x12-types)

This library provides bindings for the ANSI X12 standard.

## Usage

The types should be used inconjuction with a X12 serializer.

We do recommend the `serde_x12` crate, since we also using this serializer as testing dependency.

## Supported Bindings

* 004010
  * 315 - Status Details (Ocean)

Something missing? Please open an issue.

## Contributions

Since the X12 is fairly huge, we only implement types on demand. So if you are missing some types, please open an issue or merge request.
