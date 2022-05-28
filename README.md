# X12-types
[![Latest Version](https://img.shields.io/crates/v/x12-types.svg)](https://crates.io/crates/x12-types)

This library provides bindings for the ANSI X12 standard.

## Usage

The types should be used inconjuction with a X12 serializer.

We do recommend the `serde_x12` crate, since we are using this serializer for testing.

So far, there is also no validation attached to the structs.

## Supported Bindings

* 003030
  * 998 - Set Cancellation
* 004010
  * 204 - Motor Carrier Load Tender
  * 214 - Transportation Carrier Shipment Status Message
  * 315 - Status Details (Ocean)
  * 322 - Terminal Operations and Intermodal Ramp Activity
  * 404 - Rail Carrier Shipment Information
  * 997 - Functional Acknowledgment
  * 998 - Set Cancellation

Something missing? Please open an issue.

## Contributions

Since the X12 is fairly huge, we only implement types on demand. So if you are missing some types, please open an issue or merge request.
