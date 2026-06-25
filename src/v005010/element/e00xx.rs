//! X12 data elements 0000-0099.

crate::code_enum!(
    /// **56** Type of Service Code
    ///
    /// - Data element: 56
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the extent of transportation service requested.
    E56 {
        /// Airport-to-Airport
        "AA" => Aa,
        /// Transport Mode Change
        "AI" => Ai,
        /// Breakbulk
        "BB" => Bb,
        /// Container Station
        "CS" => Cs,
        /// Container Yard
        "CY" => Cy,
        /// Door-to-Airport of Debarkation
        "DA" => Da,
        /// Door to Door
        "DD" => Dd,
        /// Door to Ramp
        "DR" => Dr,
        /// Haulage (contractual carrier arrangement)
        "HA" => Ha,
        /// House-to-house
        "HH" => Hh,
        /// House-to-pier
        "HP" => Hp,
        /// Less than Trailer/Container Load
        "LT" => Lt,
    }
);

crate::code_enum!(
    /// **88** Marks and Numbers Qualifier
    ///
    /// - Data element: 88
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code specifying the application or source of Marks and Numbers (87).
    E88 {
        /// EAN.UCC Serial Shipping Container Code (SSCC)
        "AA" => Aa,
        /// UCC/EAN-128 Application Identifier (AI) and Data
        "AI" => Ai,
        /// Shipper-Assigned Case Number
        "CA" => Ca,
        /// Carrier-Assigned Package ID Number
        "CP" => Cp,
        /// EAN.UCC SSCC and Application Identifier
        "GM" => Gm,
        /// Line Item Only
        "L" => L,
        /// Master Carton Number
        "MC" => Mc,
        /// Premarked by Buyer
        "PB" => Pb,
        /// Originator Assigned
        "R" => R,
        /// Entire Shipment
        "S" => S,
        /// Shipper Assigned
        "SM" => Sm,
        /// U.P.C. Shipping Container Code
        "UC" => Uc,
        /// U.P.C. Consumer Package Code (1-5-5-1)
        "UP" => Up,
        /// Pallet Number
        "W" => W,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **23** Commodity Code Qualifier
    ///
    /// - Data element: 23
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code identifying the commodity coding system used in Commodity Code (22).
    E23 {
        /// National Drug Code
        "5" => N5,
        /// Harmonized Tariff Schedule of the United States Annotated
        "A" => A,
        /// U.S. Foreign Trade Schedule B
        "B" => B,
        /// Canadian Freight Classification
        "C" => C,
        /// Federal Supply Classification
        "K" => K,
        /// National Motor Freight Classification (NMFC)
        "N" => N,
        /// North American Industrial Classification System (NAICS) Code
        "R" => R,
        /// Standard Transportation Commodity Code (STCC)
        "T" => T,
        /// Uniform Freight Classification (UFC)
        "U" => U,
        /// Standard Industrial Classification (SIC) Code
        "V" => V,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **40** Equipment Description Code
    ///
    /// - Data element: 40
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying type of equipment used for shipment. Common codes named
    /// below; any other round-trips as `Unknown`.
    E40 {
        /// 40 ft. IL Container (Open Top)
        "40" => N40,
        /// Air Ride Van
        "AA" => Aa,
        /// Boxcar
        "BX" => Bx,
        /// Closed Van
        "CV" => Cv,
        /// Refrigerated Container
        "CZ" => Cz,
        /// Flat Bed Trailer
        "FT" => Ft,
        /// High Cube Van
        "HV" => Hv,
        /// Refrigerated (Reefer) Car
        "RC" => Rc,
        /// Flat Car
        "RF" => Rf,
        /// Gondola Car (Open)
        "RO" => Ro,
        /// Trailer, Dry Freight
        "TF" => Tf,
        /// Trailer (not otherwise specified)
        "TL" => Tl,
        /// Tank Car
        "TN" => Tn,
        /// Tractor
        "TR" => Tr,
        /// Truck, Van
        "TV" => Tv,
    }
);

crate::code_enum!(
    /// **91** Transportation Method/Type Code
    ///
    /// - Data element: 91
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code specifying the method or type of transportation for the shipment.
    E91 {
        /// Air
        "A" => A,
        /// Air Express
        "AE" => Ae,
        /// Barge
        "B" => B,
        /// Consolidation
        "C" => C,
        /// Parcel Post
        "D" => D,
        /// Expedited Truck
        "E" => E,
        /// Customer Pickup
        "H" => H,
        /// Contract Carrier
        "L" => L,
        /// Motor (Common Carrier)
        "M" => M,
        /// Containerized Ocean
        "O" => O,
        /// Private Carrier
        "P" => P,
        /// Rail
        "R" => R,
        /// Ocean
        "S" => S,
        /// Best Way (Shipper's Option)
        "T" => T,
        /// Private Parcel Service
        "U" => U,
    }
);

crate::code_enum!(
    /// **26** Country Code
    ///
    /// - Data element: 26
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code identifying the country (ISO 3166 alpha-2). Common codes named below;
    /// any other round-trips as `Unknown`.
    E26 {
        /// United States
        "US" => Us,
        /// Canada
        "CA" => Ca,
        /// Mexico
        "MX" => Mx,
        /// United Kingdom
        "GB" => Gb,
        /// Germany
        "DE" => De,
        /// France
        "FR" => Fr,
        /// Italy
        "IT" => It,
        /// Spain
        "ES" => Es,
        /// Netherlands
        "NL" => Nl,
        /// China
        "CN" => Cn,
        /// Japan
        "JP" => Jp,
        /// Australia
        "AU" => Au,
        /// Brazil
        "BR" => Br,
        /// India
        "IN" => In,
    }
);

crate::code_enum!(
    /// **66** Identification Code Qualifier
    ///
    /// - Data element: 66
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code designating the system/method of code structure used for the
    /// Identification Code (67). Common codes named below.
    E66 {
        /// D-U-N-S Number, Dun & Bradstreet
        "1" => N1,
        /// D-U-N-S+4, D-U-N-S Number with Four Character Suffix
        "9" => N9,
        /// Assigned by Seller or Seller's Agent
        "91" => N91,
        /// Assigned by Buyer or Buyer's Agent
        "92" => N92,
        /// Standard Carrier Alpha Code (SCAC)
        "2" => N2,
        /// Health Industry Number (HIN)
        "20" => N20,
        /// Global Location Number (GLN)
        "UL" => Ul,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **98** Entity Identifier Code
    ///
    /// - Data element: 98
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code identifying an organizational entity, a physical location, property or
    /// an individual. Common codes named below; any other round-trips as `Unknown`.
    E98 {
        /// Provider
        "1P" => N1p,
        /// Receiver
        "40" => N40,
        /// Submitter
        "41" => N41,
        /// Bill-to-Party
        "BT" => Bt,
        /// Buying Party (Purchaser)
        "BY" => By,
        /// Carrier
        "CA" => Ca,
        /// Consignee
        "CN" => Cn,
        /// Party to Receive Drop Ship
        "DD" => Dd,
        /// Message From
        "FR" => Fr,
        /// Insured or Subscriber
        "IL" => Il,
        /// Location of Goods
        "LH" => Lh,
        /// Manufacturer of Goods
        "MF" => Mf,
        /// Payee
        "PE" => Pe,
        /// Payer
        "PR" => Pr,
        /// Patient
        "QC" => Qc,
        /// Ship From
        "SF" => Sf,
        /// Shipper
        "SH" => Sh,
        /// Ship To
        "ST" => St,
        /// Supplier/Manufacturer
        "SU" => Su,
        /// Message To
        "TO" => To,
        /// Selling Party
        "SE" => Se,
        /// Vendor
        "VN" => Vn,
        /// Remit To
        "RI" => Ri,
        /// Party to Receive Commercial Invoice Remittance
        "RE" => Re,
    }
);

crate::date_element!(
    /// **32** Delivery Date
    ///
    /// - Data element: 32
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Delivery Date.
    E32
);

crate::num_element!(
    /// **34** Service Standard
    ///
    /// - Data element: 34
    /// - Type: Numeric (N1)
    /// - Length: min 1, max 4
    ///
    /// Service Standard.
    E34
);

crate::num_element!(
    /// **58** Amount Charged
    ///
    /// - Data element: 58
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 15
    ///
    /// Amount Charged.
    E58
);

crate::num_element!(
    /// **60** Freight Rate
    ///
    /// - Data element: 60
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Freight Rate.
    E60
);

crate::num_element!(
    /// **65** Height
    ///
    /// - Data element: 65
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Height.
    E65
);

crate::num_element!(
    /// **74** Declared Value
    ///
    /// - Data element: 74
    /// - Type: Numeric (N2)
    /// - Length: min 2, max 12
    ///
    /// Declared Value.
    E74
);

crate::date_element!(
    /// **76** Invoice Date
    ///
    /// - Data element: 76
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Invoice Date.
    E76
);

crate::num_element!(
    /// **77** Flashpoint Temperature
    ///
    /// - Data element: 77
    /// - Type: Numeric (N)
    /// - Length: min 1, max 3
    ///
    /// Flashpoint Temperature.
    E77
);

crate::num_element!(
    /// **80** Lading Quantity
    ///
    /// - Data element: 80
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 7
    ///
    /// Lading Quantity.
    E80
);

crate::num_element!(
    /// **81** Weight
    ///
    /// - Data element: 81
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Weight.
    E81
);

crate::num_element!(
    /// **82** Length
    ///
    /// - Data element: 82
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Length.
    E82
);

crate::num_element!(
    /// **95** Number of Containers
    ///
    /// - Data element: 95
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 4
    ///
    /// Number of Containers.
    E95
);

crate::code_enum!(
    /// **9** Late Reason Code
    ///
    /// - Data element: 9
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Late Reason Code. Code values verified against the Stedi X12 reference.
    E9 {
        /// Coverage Lack of Information
        "C1" => C1,
        /// Dispute Concerning Coverage
        "D1" => D1,
        /// Dispute Concerning Compensability in Whole
        "D2" => D2,
        /// Dispute Concerning Compensability in Part
        "D3" => D3,
        /// Dispute Concerning Disability in Whole
        "D4" => D4,
        /// Dispute Concerning Disability in Part
        "D5" => D5,
        /// Dispute Concerning Impairment
        "D6" => D6,
        /// Wrongful Determination of No Coverage
        "E1" => E1,
        /// Error(s) from Employer
        "E2" => E2,
        /// Error(s) from Employee
        "E3" => E3,
        /// Error(s) from State
        "E4" => E4,
        /// Error(s) from Health Care Provider
        "E5" => E5,
        /// Error(s) from Other Claim Administrator, Independent Adjuster or Third Party Administrator
        "E6" => E6,
        /// Late No Excuse
        "L1" => L1,
        /// Late Notification, Employer
        "L2" => L2,
        /// Late Notification, Employee
        "L3" => L3,
        /// Late Notification, State
        "L4" => L4,
        /// Late Notification, Health Care Provider
        "L5" => L5,
        /// Late Notification, Assigned Risk
        "L6" => L6,
        /// Late Investigation
        "L7" => L7,
        /// Technical Processing Delay or Computer Failure
        "L8" => L8,
        /// Manual Processing Delay
        "L9" => L9,
        /// Intermittent Lost Time Prior to First Payment
        "LA" => La,
        /// Awaiting Wage Amount Verification
        "LB" => Lb,
    }
);

crate::code_enum!(
    /// **39** Entitlement Code
    ///
    /// - Data element: 39
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Entitlement Code. Code values verified against the Stedi X12 reference.
    E39 {
        /// Agent
        "A" => A,
        /// Broker
        "B" => B,
        /// Consignee
        "C" => C,
        /// Destination Carrier
        "D" => D,
        /// Forwarder or Agent
        "E" => E,
        /// Issuing Carrier
        "I" => I,
        /// Shipper
        "S" => S,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **72** Type of Service Offered Code
    ///
    /// - Data element: 72
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Type of Service Offered Code. Code values verified against the Stedi X12 reference.
    E72 {
        /// Direct Service
        "D" => D,
        /// Interline with Connecting Line
        "I" => I,
        /// No Service
        "N" => N,
    }
);

crate::code_enum!(
    /// **73** Compensation Qualifier
    ///
    /// - Data element: 73
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Compensation Qualifier. Code values verified against the Stedi X12 reference.
    E73 {
        /// Agency Fees
        "A" => A,
        /// Brokerage
        "B" => B,
        /// Freight Forwarder
        "F" => F,
    }
);

crate::code_enum!(
    /// **78** Container Type Request Code
    ///
    /// - Data element: 78
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Container Type Request Code. Code values verified against the Stedi X12 reference.
    E78 {
        /// Containers with Chassis
        "C" => C,
        /// Containers Only
        "R" => R,
        /// Shipper To Provide - None Requested
        "S" => S,
        /// Chassis Only
        "W" => W,
    }
);

crate::code_enum!(
    /// **83** Code For Licensing, Certification, Registration, or Accreditation Agency
    ///
    /// - Data element: 83
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code For Licensing, Certification, Registration, or Accreditation Agency. Code values verified against the Stedi X12 reference.
    E83 {
        /// Middle States Association of Colleges and Schools, Commission on Higher Education
        "1" => N1,
        /// Middle States Association of Colleges and Schools, Commission on Secondary Schools
        "2" => N2,
        /// New England Association of Schools and Colleges, Commission on Institutions of Higher Education
        "3" => N3,
        /// New England Association of Schools and Colleges, Commission on Vocational, Technical, Career Institutions
        "4" => N4,
        /// North Central Association of Colleges and Schools, Commission on Institutions of Higher Education
        "5" => N5,
        /// North Central Association of Colleges and Schools, Commission on Schools
        "6" => N6,
        /// Northwest Association of Schools and Colleges, Commission on Colleges
        "7" => N7,
        /// Southern Association of Colleges and Schools, Commission on Colleges
        "8" => N8,
        /// Western Association of Schools and Colleges, Accrediting Commission for Community and Junior Colleges
        "9" => N9,
        /// Western Association of Schools and Colleges, Accrediting Commission for Schools
        "10" => N10,
        /// Western Association of Schools and Colleges, Accrediting Commission for Senior Colleges and Universities
        "11" => N11,
        /// Commission for Accreditation for Rehabilitation Facilities (CARF)
        "A" => A,
        /// Joint Commission on Accreditation of Health Organizations (JCAHO)
        "B" => B,
        /// Commercial - Motor Vehicle
        "C" => C,
        /// Driver License - Motor Vehicle
        "D" => D,
        /// Office of Export Administration
        "E" => E,
        /// Federal Aviation Administration (FAA)
        "F" => F,
        /// Department of Agriculture - Tobacco, Seeds & Plants
        "G" => G,
        /// National Committee for Quality Assurance (NCQA)
        "H" => H,
        /// Department of the Interior - U.S. Endangered Native Fish and Wildlife and Migratory Birds
        "I" => I,
        /// Department of Justice - Narcotics and Dangerous Drugs
        "J" => J,
        /// Utilization Review Accreditation Commission (URAC)
        "K" => K,
        /// US Customs Service - Automated Export System
        "L" => L,
        /// Maritime Administration - Watercraft (Other Than Vessels of War), Vessels Exported for Scrapping, Dismantling, Dismembering, or Destroying the Hulls Thereof (Also Controlled by Office of Export Administration)
        "M" => M,
        /// Nuclear Regulatory Commission - Commodities Subject to the Atomic Energy Act
        "N" => N,
        /// Federal Power Commission - Natural Gas and Electric Energy
        "P" => P,
        /// Department of State - Arms, Ammunition, Implements of War; Vessels of War
        "S" => S,
        /// Department of Treasury - Gold
        "T" => T,
    }
);

crate::code_enum!(
    /// **90** Measurement Unit Qualifier
    ///
    /// - Data element: 90
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Measurement Unit Qualifier. Code values verified against the Stedi X12 reference.
    E90 {
        /// Centimeters
        "C" => C,
        /// Feet
        "E" => E,
        /// Inches
        "N" => N,
        /// Meters
        "X" => X,
    }
);

crate::code_enum!(
    /// **92** Purchase Order Type Code
    ///
    /// - Data element: 92
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Purchase Order Type Code. Code values verified against the Stedi X12 reference.
    E92 {
        /// Assortment Against Blanket
        "AB" => Ab,
        /// AOG (Aircraft on Ground) Critical
        "AC" => Ac,
        /// AOG (Aircraft on Ground) Service
        "AO" => Ao,
        /// Bidding
        "BD" => Bd,
        /// Blanket Order/Estimated Quantities (Not firm Commitment)
        "BE" => Be,
        /// Bill and Hold
        "BH" => Bh,
        /// Blanket Order (Quantity Firm)
        "BK" => Bk,
        /// Bailment
        "BL" => Bl,
        /// Budgetary Quote
        "BQ" => Bq,
        /// Buying
        "BY" => By,
        /// Contract Award Notification
        "CA" => Ca,
        /// Cooperative Agreement
        "CB" => Cb,
        /// Change to Contract
        "CC" => Cc,
        /// Assistance Award Loan
        "CD" => Cd,
        /// Undefinitized Contract Action
        "CE" => Ce,
        /// Confirmation
        "CF" => Cf,
        /// Formula Funds Assistance Award
        "CG" => Cg,
        /// Consigned Order
        "CN" => Cn,
        /// Catalog Order
        "CO" => Co,
        /// Change to Purchase Order
        "CP" => Cp,
        /// Change to Release
        "CR" => Cr,
        /// Direct Ship
        "DR" => Dr,
        /// Dropship
        "DS" => Ds,
        /// Emergency Order
        "EO" => Eo,
        /// Formula Funds
        "FF" => Ff,
        /// Fabricate and Hold
        "FH" => Fh,
        /// Information Copy
        "IN" => In,
        /// Job Lot
        "JL" => Jl,
        /// Agreement
        "KA" => Ka,
        /// Blanket Purchase Agreement
        "KB" => Kb,
        /// Contract
        "KC" => Kc,
        /// Basic Agreement
        "KD" => Kd,
        /// Basic Ordering Agreement
        "KE" => Ke,
        /// Grant
        "KG" => Kg,
        /// Indefinite Delivery Indefinite Quantity
        "KI" => Ki,
        /// Purchase Order
        "KN" => Kn,
        /// Close Out
        "KO" => Ko,
        /// Authority to Proceed
        "KP" => Kp,
        /// Indefinite Delivery Definite Quantity
        "KQ" => Kq,
        /// Requirements
        "KR" => Kr,
        /// Letter Contract
        "KS" => Ks,
        /// Task Order
        "KT" => Kt,
        /// Lease (Blanket Agreement)
        "LB" => Lb,
        /// Loan
        "LN" => Ln,
        /// Lease
        "LS" => Ls,
        /// Novation Agreement
        "NA" => Na,
        /// New Order
        "NE" => Ne,
        /// Not for Sale
        "NO" => No,
        /// New Product Introduction
        "NP" => Np,
        /// New Store Opening
        "NS" => Ns,
        /// Special Order
        "OS" => Os,
        /// Promotion
        "PR" => Pr,
        /// Release Against Assortment
        "RA" => Ra,
        /// Retailer Pre-commitment
        "RC" => Rc,
        /// Reorder
        "RE" => Re,
        /// Release or Delivery Order
        "RL" => Rl,
        /// Renewal Order
        "RN" => Rn,
        /// Rush Order
        "RO" => Ro,
        /// Repair and Return
        "RR" => Rr,
        /// Rental
        "RT" => Rt,
        /// Record Update Service
        "RU" => Ru,
        /// Resume Work Order
        "RW" => Rw,
        /// Stand-alone Order
        "SA" => Sa,
        /// Shipped Order
        "SO" => So,
        /// Sample
        "SP" => Sp,
        /// Supply or Service Order
        "SS" => Ss,
        /// Standing Order
        "ST" => St,
        /// Stop Work
        "SW" => Sw,
        /// Toll Conversion Order
        "TC" => Tc,
        /// Time & Materials
        "TM" => Tm,
        /// Termination
        "TR" => Tr,
        /// Unit Down
        "UD" => Ud,
        /// Unit Exchange
        "UE" => Ue,
        /// Urgent Service Request
        "US" => Us,
        /// Warranty Order
        "WO" => Wo,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);
