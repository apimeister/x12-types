//! X12 data elements 0000-0099.

crate::num_element!(
    /// **2** Group Control Number
    ///
    /// - Data element: 2
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 9
    ///
    /// Group Control Number.
    E2
);

crate::num_element!(
    /// **28** Group Control Number
    ///
    /// - Data element: 28
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 9
    ///
    /// Group Control Number.
    E28
);

crate::num_element!(
    /// **58** Charge
    ///
    /// - Data element: 58
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 12
    ///
    /// Charge.
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
    /// **96** Number of Included Segments
    ///
    /// - Data element: 96
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 10
    ///
    /// Number of Included Segments.
    E96
);

crate::num_element!(
    /// **97** Number of Transaction Sets Included
    ///
    /// - Data element: 97
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Number of Transaction Sets Included.
    E97
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
