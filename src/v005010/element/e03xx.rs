//! X12 data elements 0300-0399.

crate::code_enum!(
    /// **363** Note Reference Code
    ///
    /// - Data element: 363
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Code identifying the functional area or purpose for which the note applies
    /// (NTE-01, MTX-01). Common codes named below; any other round-trips as `Unknown`.
    E363 {
        /// Entire Transaction Set
        "GEN" => Gen,
        /// Bill of Lading Note
        "BOL" => Bol,
        /// Access Instructions
        "ACC" => Acc,
        /// Insurance
        "INS" => Ins,
        /// Location
        "LOC" => Loc,
        /// Certification
        "CER" => Cer,
        /// Description
        "DGN" => Dgn,
        /// Reference
        "REF" => Ref,
    }
);

crate::code_enum!(
    /// **305** Transaction Handling Code
    ///
    /// - Data element: 305
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code designating the action to be taken by all parties (BPR-01).
    E305 {
        /// Off Invoice (Deduction from Original Invoice)
        "A" => A,
        /// Debit/Credit Advice No Remittance Detail
        "B" => B,
        /// Payment Accompanies Remittance Advice
        "C" => C,
        /// Make Payment Only
        "D" => D,
        /// Debit/Credit Advice with Remittance Detail
        "E" => E,
        /// Notification Only
        "H" => H,
        /// Remittance Information Only
        "I" => I,
        /// Prenotification of Future Transfers
        "P" => P,
        /// Split Payment and Remittance
        "U" => U,
        /// Handling Party's Option to Split Payment and Remittance
        "X" => X,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **346** Application Type Code
    ///
    /// - Data element: 346
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying an application. Common codes named below; any other
    /// round-trips as `Unknown`.
    E346 {
        /// Financial Institution Account Statement
        "BD" => Bd,
        /// Accounts Reconciliation
        "BR" => Br,
        /// Balance and Transaction Reporting
        "BT" => Bt,
        /// Cash Letter
        "CL" => Cl,
        /// Deposit List
        "DL" => Dl,
        /// Bill of Lading
        "BL" => Bl,
        /// Booking
        "BN" => Bn,
        /// Delivery Order
        "DO" => Do,
        /// Freight Tender
        "FR" => Fr,
        /// Load Tender - Truckload (TL) Carrier Only
        "LT" => Lt,
        /// Manifest
        "MF" => Mf,
        /// Single Shipment Invoice
        "SI" => Si,
        /// Multiple Shipment Invoice
        "SP" => Sp,
        /// Trailer Manifest
        "TM" => Tm,
        /// Commercial Invoice
        "CI" => Ci,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **349** Item Description Type
    ///
    /// - Data element: 349
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code indicating the format of a description (PID-01).
    E349 {
        /// Free-form
        "F" => F,
        /// Structured (From Industry Code List)
        "S" => S,
        /// Semi-structured (Code and Text)
        "X" => X,
    }
);

crate::code_enum!(
    /// **306** Action Code
    ///
    /// - Data element: 306
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code indicating type of action.
    E306 {
        /// Add
        "1" => N1,
        /// Change (Update)
        "2" => N2,
        /// Delete
        "3" => N3,
        /// Verify
        "4" => N4,
        /// Send
        "5" => N5,
        /// Pending
        "W" => W,
        /// Reject
        "U" => U,
    }
);

crate::code_enum!(
    /// **309** Location Qualifier
    ///
    /// - Data element: 309
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code identifying type of location.
    E309 {
        /// Origin (Shipping Point)
        "OR" => Or,
        /// Destination (Shipping)
        "DE" => De,
        /// Delivery Location
        "DL" => Dl,
        /// Mailing Address
        "M" => M,
        /// Office Address
        "O" => O,
        /// Home Address
        "H" => H,
        /// Plant
        "PL" => Pl,
        /// Warehouse
        "WH" => Wh,
        /// Distribution Center Number
        "DC" => Dc,
        /// Terminal
        "TM" => Tm,
        /// Port of Arrival
        "PA" => Pa,
        /// Port of Discharge
        "PB" => Pb,
        /// Port of Entry
        "PE" => Pe,
        /// Factory
        "FA" => Fa,
        /// Place of Business
        "BS" => Bs,
    }
);

crate::code_enum!(
    /// **336** Terms Type Code
    ///
    /// - Data element: 336
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying type of payment terms.
    E336 {
        /// Basic
        "01" => N01,
        /// End of Month (EOM)
        "02" => N02,
        /// Discount Not Applicable
        "05" => N05,
        /// Basic Discount Offered
        "08" => N08,
        /// Previously Agreed Upon
        "14" => N14,
        /// Fixed Date
        "18" => N18,
        /// Cash Discount Terms Apply
        "22" => N22,
    }
);

crate::code_enum!(
    /// **353** Transaction Set Purpose Code
    ///
    /// - Data element: 353
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying purpose of transaction set.
    E353 {
        /// Original
        "00" => N00,
        /// Cancellation
        "01" => N01,
        /// Add
        "02" => N02,
        /// Delete
        "03" => N03,
        /// Change
        "04" => N04,
        /// Replace
        "05" => N05,
        /// Confirmation
        "06" => N06,
        /// Duplicate
        "07" => N07,
        /// Status
        "08" => N08,
        /// Information Copy
        "22" => N22,
    }
);

crate::code_enum!(
    /// **355** Unit or Basis for Measurement Code
    ///
    /// - Data element: 355
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the units in which a value is being expressed. The published
    /// list has 844 codes; the common ones are named below, the rest round-trip as
    /// `Unknown`.
    E355 {
        /// Each
        "EA" => Ea,
        /// Pound
        "LB" => Lb,
        /// Gallon
        "GA" => Ga,
        /// Cubic Feet
        "CF" => Cf,
        /// Case
        "CA" => Ca,
        /// Carton
        "CT" => Ct,
        /// Box
        "BX" => Bx,
        /// Dozen
        "DZ" => Dz,
        /// Hours
        "HR" => Hr,
        /// Days
        "DA" => Da,
        /// Months
        "MO" => Mo,
        /// Years
        "YR" => Yr,
        /// Gram
        "GR" => Gr,
        /// Kilogram
        "KG" => Kg,
        /// Milliliter
        "ML" => Ml,
        /// Centimeter
        "CM" => Cm,
        /// Foot
        "FT" => Ft,
        /// Inch
        "IN" => In,
        /// Ounce - Av
        "OZ" => Oz,
        /// Pint
        "PT" => Pt,
        /// Quart
        "QT" => Qt,
        /// Set
        "ST" => St,
        /// Liter
        "LT" => Lt,
        /// Meter
        "MR" => Mr,
        /// Pallet/Unit Load
        "PL" => Pl,
    }
);

crate::code_enum!(
    /// **374** Date/Time Qualifier
    ///
    /// - Data element: 374
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Code specifying type of date or time, or both date and time (gates the
    /// meaning of the date/time that follows, e.g. in DTM, DTP and G62).
    ///
    /// The variants below are the common codes; the published list has ~115. Any
    /// other code round-trips as `Unknown`.
    E374 {
        /// Cancel After
        "001" => CancelAfter,
        /// Delivery Requested
        "002" => DeliveryRequested,
        /// Invoice
        "003" => Invoice,
        /// Purchase Order
        "004" => PurchaseOrder,
        /// Effective
        "007" => Effective,
        /// Process
        "009" => Process,
        /// Requested Ship
        "010" => RequestedShip,
        /// Shipped
        "011" => Shipped,
        /// Promotion Start
        "015" => PromotionStart,
        /// Estimated Delivery
        "017" => EstimatedDelivery,
        /// Delivered
        "035" => Delivered,
        /// Expiration
        "036" => Expiration,
        /// Ship Not Before
        "037" => ShipNotBefore,
        /// Ship No Later
        "038" => ShipNoLater,
        /// Received
        "050" => Received,
        /// Do Not Deliver After
        "063" => DoNotDeliverAfter,
        /// Do Not Deliver Before
        "064" => DoNotDeliverBefore,
        /// Current Schedule Delivery
        "067" => CurrentScheduleDelivery,
        /// Current Schedule Ship
        "068" => CurrentScheduleShip,
        /// Promised for Delivery
        "069" => PromisedForDelivery,
        /// Created
        "085" => Created,
        /// Transaction Creation
        "097" => TransactionCreation,
        /// Issue
        "102" => Issue,
        /// Required By
        "106" => RequiredBy,
        /// Beginning of Period
        "107" => BeginningOfPeriod,
        /// End of Period
        "108" => EndOfPeriod,
        /// Pickup
        "110" => Pickup,
        /// Manifest/Ship Notice
        "111" => ManifestShipNotice,
        /// Requested Pickup
        "118" => RequestedPickup,
        /// Estimated
        "139" => Estimated,
        /// Service Period Start
        "150" => ServicePeriodStart,
        /// Service Period End
        "151" => ServicePeriodEnd,
        /// Invoice Period Start
        "186" => InvoicePeriodStart,
        /// Invoice Period End
        "187" => InvoicePeriodEnd,
        /// Start
        "196" => Start,
        /// End
        "197" => End,
    }
);

crate::code_enum!(
    /// **365** Communication Number Qualifier
    ///
    /// - Data element: 365
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the type of communication number (PER-03/05/07).
    E365 {
        /// Electronic Mail
        "EM" => Em,
        /// Facsimile
        "FX" => Fx,
        /// Telephone
        "TE" => Te,
        /// Telephone Extension
        "EX" => Ex,
        /// Uniform Resource Locator (URL)
        "UR" => Ur,
        /// Cellular Phone
        "CP" => Cp,
        /// Telex
        "TL" => Tl,
    }
);

crate::code_enum!(
    /// **366** Contact Function Code
    ///
    /// - Data element: 366
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the major duty or responsibility of the person or group
    /// named (PER-01).
    E366 {
        /// Information Contact
        "IC" => Ic,
        /// Buyer Name or Department
        "BD" => Bd,
        /// Carrier Contact
        "CR" => Cr,
        /// Customer Service
        "CS" => Cs,
        /// Delivery Contact
        "DC" => Dc,
        /// Expediter
        "EX" => Ex,
        /// Order Contact
        "OC" => Oc,
        /// Receiving Contact
        "RC" => Rc,
        /// Sales Representative or Department
        "SR" => Sr,
        /// Shipper Contact
        "SH" => Sh,
    }
);

crate::time_element!(
    /// **337** Time
    ///
    /// - Data element: 337
    /// - Type: Time (TM)
    /// - Length: min 4, max 8
    ///
    /// Time expressed as `HHMM`, `HHMMSS`, or `HHMMSSdd` (24-hour clock). `time()`
    /// yields a [`chrono::NaiveTime`]; the raw text is preserved for byte-exact
    /// rendering.
    E337
);

crate::date_element!(
    /// **373** Date
    ///
    /// - Data element: 373
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Date expressed as `CCYYMMDD`. `date()` yields a [`chrono::NaiveDate`]; the
    /// raw text is preserved for byte-exact rendering.
    E373
);

crate::num_element!(
    /// **330** Quantity Ordered
    ///
    /// - Data element: 330
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Quantity Ordered.
    E330
);

crate::num_element!(
    /// **332** Percent, Decimal Format
    ///
    /// - Data element: 332
    /// - Type: Numeric (R)
    /// - Length: min 1, max 6
    ///
    /// Percent, Decimal Format.
    E332
);

crate::num_element!(
    /// **338** Terms Discount Percent
    ///
    /// - Data element: 338
    /// - Type: Numeric (R)
    /// - Length: min 1, max 6
    ///
    /// Terms Discount Percent.
    E338
);

crate::num_element!(
    /// **342** Percent of Invoice Payable
    ///
    /// - Data element: 342
    /// - Type: Numeric (R)
    /// - Length: min 1, max 5
    ///
    /// Percent of Invoice Payable.
    E342
);

crate::num_element!(
    /// **343** Installment Total Invoice Amount Due
    ///
    /// - Data element: 343
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 10
    ///
    /// Installment Total Invoice Amount Due.
    E343
);

crate::num_element!(
    /// **351** Terms Discount Days Due
    ///
    /// - Data element: 351
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Terms Discount Days Due.
    E351
);

crate::num_element!(
    /// **356** Pack
    ///
    /// - Data element: 356
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Pack.
    E356
);

crate::num_element!(
    /// **357** Size
    ///
    /// - Data element: 357
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Size.
    E357
);

crate::num_element!(
    /// **358** Quantity Invoiced
    ///
    /// - Data element: 358
    /// - Type: Numeric (R)
    /// - Length: min 1, max 15
    ///
    /// Quantity Invoiced.
    E358
);

crate::num_element!(
    /// **359** Allowance or Charge Rate
    ///
    /// - Data element: 359
    /// - Type: Numeric (R)
    /// - Length: min 1, max 15
    ///
    /// Allowance or Charge Rate.
    E359
);

crate::num_element!(
    /// **360** Allowance or Charge Total Amount
    ///
    /// - Data element: 360
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 15
    ///
    /// Allowance or Charge Total Amount.
    E360
);

crate::num_element!(
    /// **362** Terms Discount Amount
    ///
    /// - Data element: 362
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 10
    ///
    /// Terms Discount Amount.
    E362
);

crate::date_element!(
    /// **370** Terms Discount Due Date
    ///
    /// - Data element: 370
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Terms Discount Due Date.
    E370
);

crate::num_element!(
    /// **382** Number of Units Shipped
    ///
    /// - Data element: 382
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Number of Units Shipped.
    E382
);

crate::num_element!(
    /// **383** Quantity Difference
    ///
    /// - Data element: 383
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Quantity Difference.
    E383
);

crate::num_element!(
    /// **384** Gross Weight per Pack
    ///
    /// - Data element: 384
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Gross Weight per Pack.
    E384
);

crate::num_element!(
    /// **385** Gross Volume per Pack
    ///
    /// - Data element: 385
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Gross Volume per Pack.
    E385
);

crate::num_element!(
    /// **386** Terms Net Days
    ///
    /// - Data element: 386
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Terms Net Days.
    E386
);

crate::num_element!(
    /// **390** Amount Subject to Terms Discount
    ///
    /// - Data element: 390
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 10
    ///
    /// Amount Subject to Terms Discount.
    E390
);

crate::num_element!(
    /// **391** Discounted Amount Due
    ///
    /// - Data element: 391
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 10
    ///
    /// Discounted Amount Due.
    E391
);

crate::num_element!(
    /// **395** Unit Weight
    ///
    /// - Data element: 395
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Unit Weight.
    E395
);

crate::num_element!(
    /// **398** Order Sizing Factor
    ///
    /// - Data element: 398
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Order Sizing Factor.
    E398
);

crate::code_enum!(
    /// **308** Damage Exception Indicator
    ///
    /// - Data element: 308
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Damage Exception Indicator. Code values verified against the Stedi X12 reference.
    E308 {
        /// Damage Being Reported
        "Y" => Y,
    }
);

crate::code_enum!(
    /// **311** Shipment Type Code
    ///
    /// - Data element: 311
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Shipment Type Code. Code values verified against the Stedi X12 reference.
    E311 {
        /// Inventory Adjustment (+)
        "AD" => Ad,
        /// Inventory Adjustment (-)
        "AM" => Am,
        /// Customer Shipment
        "CS" => Cs,
        /// Rejected Shipment
        "RJ" => Rj,
        /// Stock Transfer
        "ST" => St,
    }
);

crate::code_enum!(
    /// **312** Special Indicator Code
    ///
    /// - Data element: 312
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Special Indicator Code. Code values verified against the Stedi X12 reference.
    E312 {
        /// In Bond
        "I" => I,
        /// Local
        "L" => L,
        /// Order Notify
        "N" => N,
        /// Origin Common Point (OCP)
        "O" => O,
        /// Through
        "T" => T,
    }
);

crate::code_enum!(
    /// **313** Authority Identifier Code
    ///
    /// - Data element: 313
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Authority Identifier Code. Code values verified against the Stedi X12 reference.
    E313 {
        /// Billing Clerk
        "BC" => Bc,
        /// Carrier
        "CA" => Ca,
        /// Forwarder
        "FN" => Fn,
        /// Lending Official
        "LO" => Lo,
        /// Non-Recourse
        "NR" => Nr,
        /// Proposer
        "PP" => Pp,
        /// Rate Clerk
        "RC" => Rc,
        /// Release Value
        "RV" => Rv,
        /// Shipper
        "SH" => Sh,
        /// School Official
        "SO" => So,
    }
);

crate::code_enum!(
    /// **322** Load/Empty Status Code
    ///
    /// - Data element: 322
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Load/Empty Status Code. Code values verified against the Stedi X12 reference.
    E322 {
        /// Empty
        "E" => E,
        /// Empty Requiring Census Reporting
        "F" => F,
        /// Loaded
        "L" => L,
        /// Load Requiring Census Reporting
        "M" => M,
        /// Total
        "T" => T,
    }
);

crate::code_enum!(
    /// **331** Allowance or Charge Method of Handling Code
    ///
    /// - Data element: 331
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Allowance or Charge Method of Handling Code. Code values verified against the Stedi X12 reference.
    E331 {
        /// Bill Back
        "01" => N01,
        /// Off Invoice
        "02" => N02,
        /// Vendor Check to Customer
        "03" => N03,
        /// Credit Customer Account
        "04" => N04,
        /// Charge to be Paid by Vendor
        "05" => N05,
        /// Charge to be Paid by Customer
        "06" => N06,
        /// Optional
        "07" => N07,
        /// Off Gross Quantity Invoiced
        "08" => N08,
        /// Allowance To Be Issued by Vendor
        "09" => N09,
        /// Allowance To Be Issued by Reseller
        "10" => N10,
        /// Charge Denied by Vendor
        "11" => N11,
        /// Cancel Allowance
        "12" => N12,
        /// Provide Amount
        "13" => N13,
        /// Information Only
        "15" => N15,
        /// Non-Payable Tax
        "18" => N18,
        /// Accrual Fund
        "20" => N20,
        /// Flat Fund
        "21" => N21,
        /// Cash in Advance
        "25" => N25,
        /// Calculate and Add to Invoice
        "CA" => Ca,
        /// Collect
        "CC" => Cc,
        /// Prepaid
        "PP" => Pp,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **333** Terms Basis Date Code
    ///
    /// - Data element: 333
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Terms Basis Date Code. Code values verified against the Stedi X12 reference.
    E333 {
        /// Ship Date
        "1" => N1,
        /// Delivery Date
        "2" => N2,
        /// Invoice Date
        "3" => N3,
        /// Specified Date
        "4" => N4,
        /// Invoice Receipt Date
        "5" => N5,
        /// Anticipated Delivery Date
        "6" => N6,
        /// Effective Date
        "7" => N7,
        /// Invoice Transmission Date
        "8" => N8,
        /// Purchase Order Date
        "09" => N09,
        /// Invoice Verification Date
        "10" => N10,
        /// Completion And Acceptance Date
        "11" => N11,
        /// Approval of Sample Date
        "12" => N12,
        /// Approval of Gage Date
        "13" => N13,
        /// Quality Approval Date
        "14" => N14,
        /// Receipt of Goods
        "15" => N15,
        /// Quarter Start Date
        "16" => N16,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **335** Transportation Terms Code
    ///
    /// - Data element: 335
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Transportation Terms Code. Code values verified against the Stedi X12 reference.
    E335 {
        /// Cost and Freight
        "CFR" => Cfr,
        /// Cost, Insurance, and Freight
        "CIF" => Cif,
        /// Carriage and Insurance Paid To
        "CIP" => Cip,
        /// Carriage Paid To
        "CPT" => Cpt,
        /// Delivered at Frontier
        "DAF" => Daf,
        /// Delivered Duty Paid
        "DDP" => Ddp,
        /// Deliver Duty Unpaid
        "DDU" => Ddu,
        /// Delivered Ex Quay
        "DEQ" => Deq,
        /// Delivered Ex Ship
        "DES" => Des,
        /// Domestically Supplied
        "DOM" => Dom,
        /// Delivered; Duty Unpaid
        "DUP" => Dup,
        /// Ex Quay
        "EXQ" => Exq,
        /// Ex Ship
        "EXS" => Exs,
        /// Ex Works
        "EXW" => Exw,
        /// Free Alongside Ship
        "FAS" => Fas,
        /// Free Carrier
        "FCA" => Fca,
        /// Freight Carriage and Insurance Paid To
        "FCI" => Fci,
        /// Freight Carriage Paid To
        "FCP" => Fcp,
        /// Free on Board
        "FOB" => Fob,
        /// Free on Rail
        "FOR" => For,
        /// Free on Truck
        "FOT" => Fot,
        /// Non-privileged Foreign
        "NPF" => Npf,
        /// Privileged Foreign
        "PPF" => Ppf,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::code_enum!(
    /// **340** Allowance or Charge Code
    ///
    /// - Data element: 340
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Allowance or Charge Code. Code values verified against the Stedi X12 reference.
    E340 {
        /// Free Goods
        "1" => N1,
        /// Shrink Allowance
        "2" => N2,
        /// Count and Recount
        "3" => N3,
        /// Fuel Allowance
        "4" => N4,
        /// Allowance Non-Performance
        "5" => N5,
        /// Pallet Charge
        "6" => N6,
        /// Ocean Freight Charge
        "7" => N7,
        /// Drayage
        "10" => N10,
        /// Blast Freezing
        "11" => N11,
        /// Communication Expense
        "12" => N12,
        /// Handling In
        "13" => N13,
        /// Handling Out
        "14" => N14,
        /// Labor
        "15" => N15,
        /// Minimum Charge
        "16" => N16,
        /// Physical Inventory
        "17" => N17,
        /// Pick Rate
        "18" => N18,
        /// Postage
        "19" => N19,
        /// Slow Freezing
        "20" => N20,
        /// Storage
        "21" => N21,
        /// Supplies
        "22" => N22,
        /// Taking Weights
        "23" => N23,
        /// Telephone, Telex, Fax
        "24" => N24,
        /// United States Department of Agriculture (USDA) Inspection
        "25" => N25,
        /// Unloading
        "26" => N26,
        /// Withdrawal Line Item Rate
        "27" => N27,
        /// Direct Product Handling (DPC)
        "28" => N28,
        /// Price Adjustment Percent (PCT)
        "29" => N29,
        /// Post Damaged Handling (PDC)
        "30" => N30,
        /// Reclamation Center Handling (Chute)
        "31" => N31,
        /// Reclamation Shared Responsibility (SRS)
        "32" => N32,
        /// Maximum Price Percent (MAX)
        "33" => N33,
        /// Minimum Price Percent (MIN)
        "34" => N34,
        /// Conversion Allowance
        "35" => N35,
        /// Slip Sheet Allowance
        "40" => N40,
        /// Terms Allowance
        "41" => N41,
        /// Central Buy
        "42" => N42,
        /// Display Allowance
        "43" => N43,
        /// Early Buy Allowance
        "44" => N44,
        /// New Discount
        "45" => N45,
        /// New Warehouse
        "46" => N46,
        /// Competitive Marketing Allowance
        "47" => N47,
        /// Special Buy
        "48" => N48,
        /// Lump Sum
        "50" => N50,
        /// Trade Discount
        "51" => N51,
        /// Quantity Discount
        "52" => N52,
        /// Freight Allowance
        "53" => N53,
        /// Pickup Allowance
        "54" => N54,
        /// Warehouse Allowance
        "55" => N55,
        /// Vehicle Load Allowance
        "57" => N57,
        /// Unsaleable Merchandise Allowance
        "58" => N58,
        /// Label Allowance
        "60" => N60,
        /// Handling Allowance
        "61" => N61,
        /// Freshness/Leaker Allowance
        "62" => N62,
        /// Floor Stock Protection
        "63" => N63,
        /// Truckload Allowance
        "64" => N64,
        /// New Item Allowance
        "65" => N65,
        /// Slotting Allowance
        "66" => N66,
        /// New Distribution Allowance
        "67" => N67,
        /// Scanner Allowance
        "68" => N68,
        /// Allowance for Consignment Merchandise
        "70" => N70,
        /// New Store Allowance
        "75" => N75,
        /// Combination Performance and Non-performance
        "80" => N80,
        /// Direct Plant Ship Allowance
        "81" => N81,
        /// Mutually Defined
        "88" => N88,
        /// Performance Allowance
        "90" => N90,
        /// Glaze Allowance
        "91" => N91,
        /// In Transit Price Protection
        "92" => N92,
        /// COOP Credit
        "93" => N93,
        /// Cigarette Stamping
        "94" => N94,
        /// Swell
        "95" => N95,
        /// Grouped Items
        "96" => N96,
        /// Cents Off
        "97" => N97,
        /// Advertising Allowance
        "100" => N100,
        /// Voluntary Price Reduction
        "101" => N101,
        /// Pallet Allowance
        "105" => N105,
        /// Show Allowance
        "106" => N106,
        /// Indirect Customer Allowance
        "107" => N107,
        /// Adjustment
        "110" => N110,
        /// Check Adjustment
        "111" => N111,
        /// Funds Return
        "112" => N112,
        /// Generic Supply Charge
        "113" => N113,
        /// Hardware Maintenance Charge
        "114" => N114,
        /// Media Charge
        "116" => N116,
        /// Per Claim Charge
        "117" => N117,
        /// Per Item Media Charge
        "118" => N118,
        /// Programming Charge
        "119" => N119,
        /// Service Adjustment
        "120" => N120,
        /// Software Maintenance Charge
        "121" => N121,
        /// Goods and Services Credit Allowance
        "490" => N490,
        /// Tax Credit Allowance
        "491" => N491,
        /// Other Allowance
        "499" => N499,
        /// Taxes
        "501" => N501,
        /// Carrier
        "502" => N502,
        /// Special Handling
        "503" => N503,
        /// Freight
        "504" => N504,
        /// Insurance
        "505" => N505,
        /// Railcar Loading
        "506" => N506,
        /// Switch Charge
        "507" => N507,
        /// USDA Inspected, Stamping Certification
        "508" => N508,
        /// Labeling
        "509" => N509,
        /// Koshering
        "510" => N510,
        /// Warehouse
        "511" => N511,
        /// Palletizing
        "512" => N512,
        /// Enclosure
        "513" => N513,
        /// Surcharge
        "514" => N514,
        /// Stopcharge
        "515" => N515,
        /// Delivery Charge
        "516" => N516,
        /// Demurrage
        "517" => N517,
        /// Service Charge
        "518" => N518,
        /// Less Than Truckload Charge
        "519" => N519,
        /// Cut
        "520" => N520,
        /// Paralleling
        "521" => N521,
        /// Broken Lot
        "522" => N522,
        /// Special Packaging
        "523" => N523,
        /// Reel
        "524" => N524,
        /// Deposit Charge - Resale Item
        "525" => N525,
        /// Beverage Tax
        "526" => N526,
        /// Environmental Handling Charge
        "527" => N527,
        /// State or Province Tax
        "537" => N537,
        /// Deposit Charge - Non-Resale Item
        "550" => N550,
        /// Equipment Rental Charge
        "560" => N560,
        /// Equipment Service Charge
        "561" => N561,
        /// Inter-warehouse Freight Charge
        "562" => N562,
        /// Inbound Postage
        "565" => N565,
        /// Outbound Postage
        "566" => N566,
        /// Goods and Services Charge
        "990" => N990,
        /// Other Charges
        "999" => N999,
    }
);

crate::code_enum!(
    /// **344** Unit of Time Period or Interval
    ///
    /// - Data element: 344
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Unit of Time Period or Interval. Code values verified against the Stedi X12 reference.
    E344 {
        /// More Than One Year
        "AA" => Aa,
        /// First Six-Month Period
        "AB" => Ab,
        /// Second Six-Month Period
        "AC" => Ac,
        /// Average Daily
        "AD" => Ad,
        /// Under One Year
        "AE" => Ae,
        /// Next Three Months
        "AF" => Af,
        /// Four Month Period
        "AG" => Ag,
        /// Average Monthly
        "AM" => Am,
        /// Annual
        "AN" => An,
        /// Academic Period
        "AP" => Ap,
        /// Average Year
        "AY" => Ay,
        /// Buyer's Manufacturing Days
        "BD" => Bd,
        /// Bimonthly
        "BM" => Bm,
        /// Bi-weekly
        "BW" => Bw,
        /// Cycles
        "CC" => Cc,
        /// Calendar Year
        "CY" => Cy,
        /// Calendar Days
        "DA" => Da,
        /// Work Days
        "DW" => Dw,
        /// Day
        "DY" => Dy,
        /// Calendar Year-to-Date
        "EA" => Ea,
        /// Campaign-to-Date
        "EB" => Eb,
        /// Election Cycle
        "EC" => Ec,
        /// This Period
        "ED" => Ed,
        /// To Date
        "EE" => Ee,
        /// Option Period at End of Lease
        "EL" => El,
        /// Fiscal Year Plus One Year
        "F1" => F1,
        /// Fiscal Year Plus Two Years
        "F2" => F2,
        /// Fiscal Year
        "FY" => Fy,
        /// Hours
        "HR" => Hr,
        /// Inception to Date
        "ID" => Id,
        /// Maximum Calendar Days
        "KK" => Kk,
        /// As Required
        "KL" => Kl,
        /// Lease Termination Notification Period
        "LN" => Ln,
        /// Lease Term
        "LT" => Lt,
        /// Month to Date
        "MD" => Md,
        /// Minutes
        "MI" => Mi,
        /// Month
        "MO" => Mo,
        /// Mean Time Between Stops
        "MS" => Ms,
        /// Mean Time Between Failure
        "MT" => Mt,
        /// Next Six Months
        "NX" => Nx,
        /// Plan Year to Date
        "PA" => Pa,
        /// Period to Date
        "PD" => Pd,
        /// Preceding 12 Months
        "PM" => Pm,
        /// Preceding Six Months
        "PR" => Pr,
        /// First Quarter
        "Q1" => Q1,
        /// Second Quarter
        "Q2" => Q2,
        /// Third Quarter
        "Q3" => Q3,
        /// Fourth Quarter
        "Q4" => Q4,
        /// Quarter of a Year
        "QY" => Qy,
        /// Semiannual
        "SA" => Sa,
        /// Seller's Manufacturing Days
        "SD" => Sd,
        /// Shift
        "SH" => Sh,
        /// Semimonthly
        "SM" => Sm,
        /// Summer Period
        "SP" => Sp,
        /// Tax Year
        "TY" => Ty,
        /// Weeks
        "WK" => Wk,
        /// Work Week
        "WW" => Ww,
        /// Weekly
        "WY" => Wy,
        /// Year to Date
        "YD" => Yd,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **368** Shipment/Order Status Code
    ///
    /// - Data element: 368
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Shipment/Order Status Code. Code values verified against the Stedi X12 reference.
    E368 {
        /// Multiple Pickup Same Destination
        "AA" => Aa,
        /// Available to Ship - Billed Quantity
        "AB" => Ab,
        /// Available Now - No Shipping Schedule
        "AN" => An,
        /// Allocation
        "AP" => Ap,
        /// Available Now - Scheduled to Ship (date)
        "AS" => As,
        /// Available to Ship - Unbilled Quantity
        "AU" => Au,
        /// Available
        "AV" => Av,
        /// Back Ordered from Previous Order
        "BK" => Bk,
        /// Backordered, Manufacturer, Out-of-Stock
        "BM" => Bm,
        /// Back Ordered
        "BO" => Bo,
        /// Shipment Partial, Back Order to Ship on (Date)
        "BP" => Bp,
        /// Billed total
        "BT" => Bt,
        /// Billed week-to-date
        "BW" => Bw,
        /// Customer Inquiry - All Items
        "CA" => Ca,
        /// Components Missing
        "CB" => Cb,
        /// Shipment Complete on (Date)
        "CC" => Cc,
        /// Consolidated Freight
        "CD" => Cd,
        /// Shipment Includes Extra Items to Meet Price Break
        "CE" => Ce,
        /// Consolidated Load
        "CF" => Cf,
        /// Customer Inquiry - Shipped Items Only
        "CI" => Ci,
        /// Cancelled from Previous Order
        "CK" => Ck,
        /// Complete
        "CL" => Cl,
        /// Shipment Complete with Additional Quantity
        "CM" => Cm,
        /// Next Carrier Move on (Date)
        "CN" => Cn,
        /// Customer Inquiry - Unshipped Items Only
        "CO" => Co,
        /// Partial Shipment on (Date), Considered No Backorder
        "CP" => Cp,
        /// Shipment Complete with Substitution
        "CS" => Cs,
        /// Combination
        "CT" => Ct,
        /// Cancelled Line Item
        "CU" => Cu,
        /// Due for Assortment
        "DA" => Da,
        /// Delivered to Destination on (Date)
        "DD" => Dd,
        /// Deleted Order
        "DE" => De,
        /// Discontinued
        "DI" => Di,
        /// Diverted Order
        "DO" => Do,
        /// Dispose
        "DP" => Dp,
        /// Dating Requirements
        "DR" => Dr,
        /// Out Of Stock Condition
        "DS" => Ds,
        /// Equipment Capacity
        "EC" => Ec,
        /// Expect to Ship By (Date)
        "ED" => Ed,
        /// Expect To Ship Week of (Date)
        "EW" => Ew,
        /// Expect to Deliver by
        "EX" => Ex,
        /// Units Not Shipped By Agent - To Be Shipped From Factory
        "FS" => Fs,
        /// Held Awaiting Qualification
        "HQ" => Hq,
        /// Item Canceled
        "IC" => Ic,
        /// Insufficient Information
        "ID" => Id,
        /// In Process
        "IN" => In,
        /// Inquiry by Purchase Order
        "IP" => Ip,
        /// Item Represents Substitution from Original Order
        "IS" => Is,
        /// Shipment late
        "LM" => Lm,
        /// Last Shipment on (Date)
        "LS" => Ls,
        /// Number of Late Weeks
        "LW" => Lw,
        /// Missing Components Furnished
        "MC" => Mc,
        /// Not Yet Published
        "NF" => Nf,
        /// Not in Process - No Shipping Schedule
        "NN" => Nn,
        /// Not In Process, Scheduled to Ship on (Date)
        "NS" => Ns,
        /// No Shipping Schedule
        "NY" => Ny,
        /// Out of Bill & Hold Goods
        "OB" => Ob,
        /// Order Sent to Factory for Production on (Date)
        "OF" => Of,
        /// Out of Print
        "OP" => Op,
        /// Temporarily Out of Stock - Reorder
        "OR" => Or,
        /// Purchase Order Inquiry - All Items
        "PA" => Pa,
        /// Production
        "PC" => Pc,
        /// Purchase Order Complete
        "PD" => Pd,
        /// Product On Hold
        "PH" => Ph,
        /// Purchase Order Inquiry - Shipped Items Only
        "PI" => Pi,
        /// Packed-to-Date as of (Date)
        "PK" => Pk,
        /// Part Lot, Stop Off
        "PL" => Pl,
        /// In Process - No Shipping Schedule
        "PN" => Pn,
        /// Purchase Order Inquiry - Unshipped Items Only
        "PO" => Po,
        /// Purchase Order Inquiry - Specific Items
        "PP" => Pp,
        /// Partial Shipment
        "PR" => Pr,
        /// In Process, Scheduled to Ship On (Date)
        "PS" => Ps,
        /// Part Lot
        "PT" => Pt,
        /// Quantity Net Due
        "QN" => Qn,
        /// Quantity Past Due
        "QP" => Qp,
        /// Released to Carrier (Date)
        "RC" => Rc,
        /// Revised Expect to Ship By (Date)
        "RD" => Rd,
        /// Recall
        "RI" => Ri,
        /// Received Total
        "RT" => Rt,
        /// Revised Expect to Ship Week of (Date)
        "RW" => Rw,
        /// Shipment Quantity Increase
        "SA" => Sa,
        /// Seconds Available to Ship - Billed Quantity
        "SB" => Sb,
        /// Shipment Quantity Decrease
        "SC" => Sc,
        /// Shipped Damaged
        "SD" => Sd,
        /// Replacement Shipment
        "SE" => Se,
        /// Shipped and Held in Bond at Contractor's Plant
        "SF" => Sf,
        /// Shipped and Held as Government-Furnished Property
        "SG" => Sg,
        /// Shipped (Date)
        "SH" => Sh,
        /// Shipment Late, Scheduled to Ship on (Date)
        "SI" => Si,
        /// Shipped or Performed as Indicated
        "SJ" => Sj,
        /// Shipment Underrun Quantity
        "SK" => Sk,
        /// Shipped - to - Date through (Date)
        "SL" => Sl,
        /// Scheduled for Production at Factory on (Date)
        "SP" => Sp,
        /// Scheduled to ship (Summary quantity)
        "SQ" => Sq,
        /// Split Shipment
        "SS" => Ss,
        /// Stop Off
        "ST" => St,
        /// Seconds Available to Ship - Unbilled Quantity
        "SU" => Su,
        /// Unbilled Quantity Balance
        "UB" => Ub,
        /// Unavailable
        "UN" => Un,
        /// Unsolicited Report
        "UR" => Ur,
        /// Item Invoiced - Shipment to Follow
        "WS" => Ws,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **372** Lading Liability Code
    ///
    /// - Data element: 372
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Lading Liability Code. Code values verified against the Stedi X12 reference.
    E372 {
        /// Full Liability
        "F" => F,
        /// Limited Liability
        "L" => L,
    }
);

crate::code_enum!(
    /// **375** Tariff Service Code
    ///
    /// - Data element: 375
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Tariff Service Code. Code values verified against the Stedi X12 reference.
    E375 {
        /// Domestic Motor Van Door-to-Door Interstate
        "1A" => N1A,
        /// Domestic Motor Van Door-to-Door Intrastate
        "1B" => N1B,
        /// Domestic Container Van Door-to-Door Interstate
        "2A" => N2A,
        /// Domestic Container Van Door-to-Door Intrastate
        "2B" => N2B,
        /// Unaccompanied Baggage for Domestic Commercial Air Transport
        "B1" => B1,
        /// Unaccompanied Baggage for Domestic Transport Not Otherwise Identified
        "B2" => B2,
        /// Unaccompanied Baggage for Intertheater Military Air Transport
        "B3" => B3,
        /// Unaccompanied Baggage for Intratheater Military Water Transport
        "B4" => B4,
        /// Unaccompanied Baggage for Intratheater Commercial Motor Transport
        "B5" => B5,
        /// Unaccompanied Baggage for Domestic Commercial Motor Transport
        "BA" => Ba,
        /// Barge-to-Barge
        "BB" => Bb,
        /// Unaccompanied Baggage for Domestic Military Air Transport
        "BC" => Bc,
        /// Barge-to-Door
        "BD" => Bd,
        /// Unaccompanied Baggage for International Military Water Transport
        "BE" => Be,
        /// Unaccompanied Baggage for International Commercial Air Transport
        "BF" => Bf,
        /// Unaccompanied Baggage for International Military Air Transport
        "BG" => Bg,
        /// Unaccompanied Baggage for International Transport Not Otherwise Identified
        "BH" => Bh,
        /// Unaccompanied Baggage for Intertheater Military Water Transport
        "BK" => Bk,
        /// Unaccompanied Baggage for Intertheater Commercial Air Transport
        "BL" => Bl,
        /// Barge-to-Motor
        "BM" => Bm,
        /// Unaccompanied Baggage for Intertheater Transport Not Otherwise Identified
        "BN" => Bn,
        /// Barge-to-Terminal
        "BO" => Bo,
        /// Barge-to-Pier
        "BP" => Bp,
        /// Barge-to-Rail
        "BR" => Br,
        /// Unaccompanied Baggage for Intratheater Commercial Air Transport
        "BW" => Bw,
        /// Unaccompanied Baggage for Intratheater Military Air Transport
        "BX" => Bx,
        /// Unaccompanied Baggage for Intratheater Transport Not Otherwise Identified
        "BY" => By,
        /// Door-to-Barge
        "DB" => Db,
        /// Door-to-Door
        "DD" => Dd,
        /// Door-to-Motor
        "DM" => Dm,
        /// Door-to-Terminal
        "DO" => Do,
        /// Door-to-Pier
        "DP" => Dp,
        /// Door-to-Rail
        "DR" => Dr,
        /// Rate Applies for Economy Transportation Service
        "EC" => Ec,
        /// Household Goods for International Transport Not Otherwise Identified
        "H1" => H1,
        /// Household Goods for Intratheater Military Water Transport
        "H2" => H2,
        /// Household Goods for Domestic Commercial Motor Transport
        "HA" => Ha,
        /// Household Goods for Domestic Commercial Air Transport
        "HB" => Hb,
        /// Household Goods for Domestic Military Air Transport
        "HC" => Hc,
        /// Household Goods for Domestic Transport Not Otherwise Identified
        "HD" => Hd,
        /// Household Goods for International Military Water Transport
        "HE" => He,
        /// Household Goods for International Commercial Air Transport
        "HF" => Hf,
        /// Household Goods for International Military Air Transport
        "HG" => Hg,
        /// House-to-House
        "HH" => Hh,
        /// Household Goods for Intertheater Military Water Transport
        "HK" => Hk,
        /// Household Goods for Intertheater Commercial Air Transport
        "HL" => Hl,
        /// Household Goods for Intertheater Military Air Transport
        "HM" => Hm,
        /// Household Goods for Intertheater Transport Not Otherwise Identified
        "HN" => Hn,
        /// House-to-Pier
        "HP" => Hp,
        /// Household Goods for Intratheater Commercial Motor Transport
        "HR" => Hr,
        /// Household Goods for Intratheater Commercial Air Transport
        "HW" => Hw,
        /// Household Goods for Intratheater Military Air Transport
        "HX" => Hx,
        /// Household Goods for Intratheater Transport Not Otherwise Identified
        "HY" => Hy,
        /// Motor-to-Barge
        "MB" => Mb,
        /// Motor-to-Door
        "MD" => Md,
        /// Rate applies for Mini-Landbridge
        "ML" => Ml,
        /// Motor-to-Motor
        "MM" => Mm,
        /// Motor-to-Terminal
        "MO" => Mo,
        /// Motor-to-Pier
        "MP" => Mp,
        /// Motor-to-Rail
        "MR" => Mr,
        /// Rate Applies for Next Day Transportation Service
        "ND" => Nd,
        /// Rate applies for Overland Common Point
        "OC" => Oc,
        /// Terminal-to-Door
        "OD" => Od,
        /// Terminal-to-Motor
        "OM" => Om,
        /// Terminal-to-Rail
        "OR" => Or,
        /// Terminal-to-Inland
        "OT" => Ot,
        /// Rate Applies for Overnight Transportation Service
        "OV" => Ov,
        /// Pier-to-Barge
        "PB" => Pb,
        /// Pier-to-Door
        "PD" => Pd,
        /// Pier-to-House
        "PH" => Ph,
        /// Pier-to-Motor
        "PM" => Pm,
        /// Pier-to-Pier
        "PP" => Pp,
        /// Pier-to-Rail
        "PR" => Pr,
        /// Rail-to-Barge
        "RB" => Rb,
        /// Rail-to-Door
        "RD" => Rd,
        /// Rail-to-Motor
        "RM" => Rm,
        /// Rail-to-Terminal
        "RO" => Ro,
        /// Rail-to-Pier
        "RP" => Rp,
        /// Rail-to-Rail
        "RR" => Rr,
    }
);

crate::code_enum!(
    /// **378** Allowance/Charge Percent Qualifier
    ///
    /// - Data element: 378
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Allowance/Charge Percent Qualifier. Code values verified against the Stedi X12 reference.
    E378 {
        /// Gross
        "0" => N0,
        /// Item List Cost
        "1" => N1,
        /// Item Net Cost
        "2" => N2,
        /// Discount/Gross
        "3" => N3,
        /// Discount/Net
        "4" => N4,
        /// Base Price per Unit
        "5" => N5,
        /// Base Price Amount
        "6" => N6,
        /// Base Price Amount Less Previous Discount
        "7" => N7,
        /// Net Monthly On All Invoices Past Due
        "8" => N8,
        /// Late Payment Charge Base Amount
        "9" => N9,
        /// Fuel Rate
        "A" => A,
        /// Item Total Amount
        "B" => B,
        /// Item Unit Price
        "C" => C,
        /// Order Total Amount
        "D" => D,
        /// Hourly Rate
        "H" => H,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **381** Price Reason Code
    ///
    /// - Data element: 381
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Price Reason Code. Code values verified against the Stedi X12 reference.
    E381 {
        /// Bid
        "A" => A,
        /// Back Order
        "B" => B,
        /// Cents Off Label
        "C" => C,
        /// Distress Merchandise
        "D" => D,
        /// Reorder
        "E" => E,
        /// Intro Offering
        "F" => F,
        /// Grand Opening
        "G" => G,
        /// Unmatched Manufacturer ID
        "H" => H,
        /// Unmatched Coupon Family Code
        "I" => I,
        /// Purchase Requirement Not Satisfied
        "J" => J,
        /// Consumer Presented More Coupons Than Allowed
        "K" => K,
        /// Coupon Value Exceeds Retail Price
        "L" => L,
        /// Provision Sales to Non-Provision Account
        "P" => P,
        /// Price Quote
        "Q" => Q,
        /// Temporary Shipping Interruption
        "S" => S,
        /// Price Protection
        "T" => T,
        /// Miscellaneous
        "X" => X,
    }
);

crate::code_enum!(
    /// **399** Pallet Exchange Code
    ///
    /// - Data element: 399
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Pallet Exchange Code. Code values verified against the Stedi X12 reference.
    E399 {
        /// No Exchange/No Return
        "1" => N1,
        /// Exchange Pallets
        "2" => N2,
        /// Return Pallets
        "3" => N3,
        /// Pallets to be Purchased by Customer
        "4" => N4,
        /// Third-Party Pallet Exchange
        "5" => N5,
    }
);

crate::num_element!(
    /// **315** Compensation Paid
    ///
    /// - Data element: 315
    /// - Type: Numeric (R)
    /// - Length: min 2, max 5
    ///
    /// Compensation Paid.
    E315
);

crate::num_element!(
    /// **317** Total Compensation Amount
    ///
    /// - Data element: 317
    /// - Type: Numeric (N0)
    /// - Length: min 3, max 10
    ///
    /// Total Compensation Amount.
    E317
);
