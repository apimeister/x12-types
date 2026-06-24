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
