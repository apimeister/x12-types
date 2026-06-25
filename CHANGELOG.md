# 0.12.0 2026-06-25

* typed data elements: segment fields that were raw `String`/`Option<String>` now use a typed element wherever X12 defines one, across v003030/v004010/v005010/v005030. Every element type preserves the raw text and renders/round-trips byte-for-byte.
  * interchange-control I-series (I01–I16) wired into `ISA`/`IEA` — qualifiers/indicators as enums, interchange date/time/control numbers as typed date/time/numeric elements; free-text fields and the component separator remain `String`
  * numeric, date and time elements modeled with `num_element`/`date_element`/`time_element`, exposing typed views (`as_f64()`/`as_i64()`, `date()`, `time()`) while preserving the original text
  * ID code-list elements modeled as enums via `code_enum` with an `Unknown(String)` catch-all so unpublished codes still round-trip; all code values verified against the Stedi X12 reference
  * 416 of 578 referenced data elements are now typed; the remainder stay `String` by design — free-text (AN) elements and open ID registries with no enumerated code list (e.g. 140 Standard Carrier Alpha Code)
  * note: the I-series and ID code-list fields are no longer `String`, so constructing these segments uses the element types (e.g. `I14::Production`, `E373::from_x12("20240115")`) — see the updated README and `examples/`

# 0.11.1 2026-06-24

* fix JSON round-trip: pair `#[serde(default)]` with every `#[serde(skip_serializing_if = ...)]` field (455 fields) so that omitted optional segments/loops deserialize back to `None`/empty instead of erroring with `missing field ...`

# 0.11.0 2026-06-23

* add support for
  * 005010/811 - Consolidated Service Invoice/Statement
  * 005010/857 - Shipment and Billing Notice
  * 005010/866 - Production Sequence
  * 005010/845 - Price Authorization Acknowledgment/Status
  * 005010/888 - Item Maintenance
  * 005010/889 - Promotion Announcement
  * 005010/212 - Motor Carrier Delivery Trailer Manifest
  * 005010/216 - Motor Carrier Shipment Pickup Notification
  * 005010/217 - Motor Carrier Loading and Route Guide
  * 005010/823 - Lockbox
  * 005010/821 - Financial Information Reporting
  * 005010/822 - Account Analysis
  * 005010/309 - Customs Manifest
  * 005010/353 - Customs Events Advisory Details
  * 005010/404 - Rail Carrier Shipment Information
  * 005010/417 - Rail Carrier Waybill Interchange
  * 005010/163 - Transportation Appointment Schedule Information
  * 005010/425 - Rail Waybill Request
  * 005010/869 - Order Status Inquiry
  * 005010/148 - Report of Injury, Illness or Incident
  * 005010/211 - Motor Carrier Bill of Lading
  * 005010/300 - Reservation (Booking Request) (Ocean)
  * 005010/301 - Confirmation (Ocean)
  * 005010/303 - Booking Cancellation (Ocean)
  * 005010/304 - Shipping Instructions
  * 005010/350 - Customs Status Information
  * 005010/867 - Product Transfer and Resale Report
* completeness/compliance fixes
  * 005010/274 - rebuilt to the correct HL-based spec: the prior version used BGN with a flat N1/LX layout and could not parse a 005010 274. It now uses BHT/DTM/PER and the 2000 HL loop with a 2100 NM1 loop carrying demographic detail (N2/PER/DMG/AMT/API/DEG/IND/LUI/DTP/MTX/QTY/WS/CRC/HSD/BCI/PDI/HAD) and its NX1 (2110), LQ (2120), HPL (2130), REF (2140) and EMS (2150) sub-loops; added segments API/DEG/IND/BCI/PDI/HAD/HPL
  * 005010/832 - rebuilt the stub to the full Price/Sales Catalog spec (the library's deepest set): heading CTP/REF/YNQ/PER/DTM/CTB/CUR/ITD/LDT/SAC/TD1-5/FOB/PKG/TXI/AAA/MTX/PWK plus N1, LM, N9 and G93 loops; the LIN item loop gained PO1/G53/SI/CRD/CTB/PID/MEA/PKG/PO4/carrier/ITD/LDT/SAC/FOB/AAA/TC2/TXI/MTX/G55/G54 and its CTP price-tier loop (with a G40 cost loop and an LS-bracketed LM/N1 party section), a LIN party (N1) loop, a G39 manufacturer-item loop, a PKL pack loop, an LFG hazardous-material loop (with a CRC condition loop), an LM/LQ/PID code loop, an SLN subline loop (with LM and N1 sub-loops) and an N9 loop; added segments G93/G26/G36/G39/G40/G43/G54/G55/PKL/LFG/CRD
  * 005010/880 - rebuilt the stub to the full grocery spec: G01 beginning, heading N9/G61/G62/NTE/CAD/G23/G25 plus N1 and G72/G73 allowance loops; the detail is the G17 item loop (not LIN) with G69/G19/G20/N9/G23/G25 and its G72/G73 allowance loop, plus the ENT entity loop (N2/N3/N4/N9) whose REF sub-loop carries QTY/AMT/G72 and a nested G17/G19 item sub-loop; the summary is G31/G33
  * 005010/875 - rebuilt the stub to the full grocery spec: G50 beginning, heading N9/G61/G62/NTE/G66/G23 and N1 + G72/G73 allowance loops; the detail is the G68 line-item loop (not LIN) with G69/G70/N9/G23, its G72 allowance loop, an N1 party loop (with QTY) and an SLN subline loop, plus the G76 total
  * 005010/843 - expanded the stub to the full spec: heading CPR/PCT plus SAC/N9/N1/AMT/ADV/LM/LDT loops; the PO1 quote-line loop gained the full segment set and its PID, SAC, QTY, SCH, cost-analysis (CST), SLN (with QTY/CST/N9/N1), pricing-data (PD), LDT, N9, party (N1 with lead-time) and AMT sub-loops
  * 005010/840 - expanded the stub to the full spec: heading SAC/LDT/N9/N1/SPI(+N1,CB1)/PCT/ADV/LM loops plus the full segment set; the PO1 loop gained CN1/PO3/CTP/PAM/CTB/PWK/PKG/PO4/IT8/CSH/ITD/FST/carrier/RRA/MTX/SPI and its PID, SAC, QTY, SCH, LDT, SLN, N9, party (N1) and PCT sub-loops
  * 005010/865 - expanded the stub to the full spec: heading SAC/LDT/N9/N1/AMT/ADV/LM loops plus the full segment set; the POC line-change loop gained the ACK line-item-acknowledgment loop, the PD pricing-data loop, and PID/SAC/QTY/SCH/LDT/N9/party(N1)/SLN sub-loops
  * 005010/860 - expanded the stub to the full spec: heading PAM/SI/PWK/PKG/carrier/CTB/G53/TXI/PCT plus SAC, LDT, AMT (with FA1), N9, N1, LM, SPI (with N1 and CB1) and ADV loops; the POC line-change loop gained LIN/SI/CN1/PO3/CTP/PAM/MEA/PWK/PKG/PO4/IT8/CSH/ITD/carrier/TC2/CTB/TXI/SPI/MTX and its PID, SAC, QTY, SCH, N9, LDT, party (N1 with lead-time), SLN, AMT and LM sub-loops
  * 005010/943 - corrected the detail/summary structure: the detail item is the W04 loop (W04/G69/N9/W20), not an LX/W07 loop, and the summary is W03 (not W14); added the mandatory W27 carrier segment and W28/W10
  * 005010/753 - rebuilt to the full spec: added the BGN beginning segment and PER, the heading N1 loop, the detail LX and the routing-stop loop (N1/L11/G62/USI with an OID/CMC order loop)
  * 005010/754 - rebuilt to the full spec: added BGN/PER and the heading N1 loop; the detail LX loop now carries L11/BLR/SMD/OID/G62/MSI, a QTY loop (with AT9) and a destination N1 loop
  * 005010/864 - corrected the heading reference segment (N9 -> DTM), added REF/PER to the party loop, and added the nested N1 loop inside the MIT message loop
  * 005010/812 - rebuilt to the full spec: the detail is the CDD adjustment loop (not IT1); added heading CUR/PER/ITD/FOB/SHD/SAC, the N1 loop's N9/AMT, and LM/FA1 loops; the CDD loop gained LIN/PO4/N9/DTM and its SAC, LM, N11 store (with party sub-loop) and FA1 sub-loops
  * 005010/180 - rebuilt to the full spec: the detail item is the BLI loop (not LIN); added heading RDR/PRF/PER/SAC/G38/PKG/TD1/TD5/NTE and LM loop, and the BLI loop's N9/PID/RDR/SAC/AMT/MEA/CRC/NTE/PRF/DD/GF/TD5/SDQ plus its LM, N1, QTY (with party and an LX sub-loop) and FA1 sub-loops
  * 005010/315 - fixed V9 cardinality (Option -> Vec; the spec allows up to 10 event-detail segments)
  * 005010/870 - expanded to the full spec: heading TD3/TD4/TD5 and REF/N1/LM loops; the HL loop gained the ISR status loop, REF/N1/LM loops and the PO1 line-item loop (CUR/SLN/PO3/PID/MEA/PKG) with its own ISR loop (party + carrier detail) and LX loop (with code-source sub-loop)
  * 005010/210 - rebuilt to the full spec: corrected the heading/detail reference segment from N9 to L11, added C3/ITD/G62/R3/H3/K1 and the heading N1/N7/OID loops, the detail S5 stop-off loop (with its OID and party/equipment loops), and expanded the LX loop with L5/H1/H2/L1/L4/L7/K1, its OID loop and a party loop carrying a CD3 carton loop (L11/H6/L9/POD/G62)
  * 005010/824 - expanded the OTI loop to the full spec: added NM1, the TED loop's CTX/NTE/RED, and the LM loop with its nested LQ (with RED) sub-loop
  * 005010/944 - expanded to the full spec: heading N1 loop plus N9/G61/G62/NTE/W08/W18/G08/TD1; detail LX loop gained MAN/PAL and the W07 item loop (G69/N9/W20) with its nested W13 exception sub-loop
  * 005010/830 - expanded to the full spec: heading XPO/CUR/TAX/FOB/CTP/SAC/CSH/ITD/PID/MEA/PWK/PKG/carrier/MAN and LM loop; LIN loop gained UIT/CUR/PO3/CTP/PID/MEA/PWK/PKG/PO4/PRS/SAC/ITD/TAX/FOB/LDT/QTY/ATH/carrier/MAN/DD and the SLN, item-N1, LM, FST (with QTY/SDQ/LM), SDP (with FST) and SHP sub-loops
  * 005010/852 - expanded to the full spec: heading XPO/N9 and the N1 loop's carrier/date detail; LIN loop gained CTP/SAC/PO4/N9/AMT/PAL/QTY and the ZA reporting loop with its QTY/CTP/SDQ and G95 performance sub-loop
  * 005010/861 - expanded to the full spec: heading carrier (TD1-4)/MEA and LM/FA1 loops; RCD loop gained SN1/CUR/LIN/PID/PO4/carrier/SAC/MAN and the LM, SLN (with NM1 + LM) and detail N1/FA1 sub-loops
  * 005010/862 - expanded to the full spec: LIN loop gained the mandatory UIT plus PKG/PO4/PRS/QTY/SDP and carrier detail, the FST loop gained its JIT sub-loop, and the SHP loop was added
  * 005010/846 - expanded the detail LIN loop to the full spec: added the missing line-item segments (PKG/CTP/CUR/SAC/SDQ/MAN/UIT/CS/DD/G53/PCT/LDT/...) and the LM, SLN (with MAN sub-loop), QTY (with SCH/LM and LS-bracketed REF sub-loops) and item-level N1 sub-loops; QTY is now a proper loop rather than a flat field
  * 005010/816 - corrected the beginning segment from BGN to BHT (005010 is HL-based) and added the missing detail HL loop with its N1 sub-loop (QTY/DTM/LQ/ASI); the prior version was modeled on the older non-hierarchical layout and could not parse a 005010 816
  * 005010/856 - rebuilt the HL detail loop to the full spec: added the missing item/packaging/carrier segments (LIN/SN1/SLN/PO4/PID/PWK/PKG/TD4/...) and the TD3, hazardous-material (LH1), CLD, party (N1), SAC, LM and V1 sub-loops. Parties are now a proper N1 sub-loop, so multi-party shipments round-trip (the previous flat model dropped them)
  * 005010/275 - deepened to the full Patient Information / claim-attachment structure (NM1 loop now carries IN1 and a nested NX1 loop; added the detail LX loop with the DTP -> EFI -> BIN binary attachment payload)
  * fixed segment definitions B1 (element requiredness + added elements 05/06), and corrected the Y2 (Container Details) and Y6 (Authentication) documentation to their real ocean element layouts

# 0.10.0 2026-06-22

* add support for
  * 005010/945 - Warehouse Shipping Advice
  * 005010/824 - Application Advice
  * 005010/846 - Inventory Inquiry/Advice
  * 005010/860 - Purchase Order Change Request - Buyer Initiated
  * 005010/210 - Motor Carrier Freight Details and Invoice
  * 005010/180, 274, 275, 310, 315, 753, 754, 812, 816, 830, 832, 840, 843, 852, 861, 862, 864, 865, 870, 875, 880, 943, 944, 990
* v005010 now also carries 810, 856, 997, 940, 204, 214 (previously v004010-only), so all common business, supply-chain, transport and healthcare sets are available under one version
* 837P/I/D (Professional/Institutional/Dental) are handled by the generic 005010/837 parser
* fix 945 (v004010 + v005010): removed a non-standard nested `LX` (and `LS`/`LE`/`FA1`) from the `W12` detail loop that caused multi-line-item shipments to collapse into a single detail loop
* transaction-set parsers are now generated by the `ParseX12` derive (x12-types-macros is a workspace member); per-version module layout standardized; version feature flags are now independent

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