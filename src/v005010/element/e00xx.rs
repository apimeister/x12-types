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
