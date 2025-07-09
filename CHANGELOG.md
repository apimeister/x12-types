# 0.9.1 2025-07-09

* add support for 
  * 005010/270 - Eligibility, Coverage or Benefit Inquiry
  * 005010/271 - Eligibility, Coverage or Benefit Information
  * 005010/276 - Health Claim Status Request
  * 005010/820 - Payment Order/Remittance Advice
  * 005010/999 - Implementation Acknowledgment

# 0.9.0 2025-07-07

* update to nom 8
* add support for 005010/277 (thanks, [Dave Spadea](https://github.com/dspadea)), 005010/278

# 0.8.5 2025-01-07

* dependencies

# 0.8.4 2024-04-21

* dependencies

# 0.8.3 2023-11-11

* parser fixes on 004010/997, 004010/322, 004010/214
* added 004010/810 - Invoice

# 0.8.2 2023-11-05

* add parsing for 00303/998
* added 005010/835 - Health Care Claim Payment/Advice
* use parser macro

# 0.8.1 2023-11-04

* fix broken cargo toml

# 0.8.0 2023-11-04

* implement display trait
* use display macros

# 0.7.6 2023-10-06

* added 005010/837 - Health Care Claim

# 0.7.5 2023-10-01

* 004010/315 refine parsing
* 004010/301 refine parsing
* 004010/404 refine parsing
* added 005030/404 - Rail Carrier Shipment Information

# 0.7.4 2023-09-25

* fixed 004010 refine parsing

# 0.7.3 2023-09-24

* added 005010/834 - Benefit Enrollment and Maintenance
* fixed 004010/204 - L11 cardinality

# 0.7.2 2023-09-22

* remove hard serde dependency (only used for testing)
* more nom sgment parsing

# 0.7.0 2022-12-04

* update serde

# 0.6.0 2022-11-18

* some clippy changes
* serde rename '_'-fields

# 0.5.0 2022-11-18

* added
  * 004010/309
  * 004010/310
  
# 0.4.0 2022-08-27

* some more documentation
* add segments as generic types
* added
  * 004010/214
  * 003030/998
* add reflection for 004010/315

# 0.3.0 2022-05-28

* make loops into struct instead of seuqences
* added
  * 004010/322
  * 004010/204

# 0.2.0 2022-05-27

* added
  * 004010/404
  * 004010/997
  * 004010/998
  * 004010/315

# 0.1.0 2022-05-26

* initial version