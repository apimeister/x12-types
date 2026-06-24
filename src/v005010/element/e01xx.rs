//! X12 data elements 0100-0199.

crate::code_enum!(
    /// **102** Ownership Code
    ///
    /// - Data element: 102
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code indicating the relationship of equipment to carrier or ownership of
    /// equipment.
    E102 {
        /// Railroad Leased
        "L" => L,
        /// Not Customer Owned or Leased
        "N" => N,
        /// Seller Owned, Returnable
        "R" => R,
        /// Customer Owned or Leased
        "S" => S,
        /// Trip Leased
        "T" => T,
    }
);

crate::code_enum!(
    /// **188** Weight Unit Code
    ///
    /// - Data element: 188
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code specifying the weight unit.
    E188 {
        /// Metric Ton
        "E" => E,
        /// Grams
        "G" => G,
        /// Kilograms
        "K" => K,
        /// Pounds
        "L" => L,
        /// Measurement Ton
        "M" => M,
        /// Ounces
        "O" => O,
        /// Short Ton
        "S" => S,
        /// Long Ton
        "T" => T,
    }
);

crate::code_enum!(
    /// **152** Special Handling Code
    ///
    /// - Data element: 152
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code specifying special transportation handling instructions. Common codes
    /// named below; any other round-trips as `Unknown`.
    E152 {
        /// Hazardous Cargo
        "HAZ" => Haz,
        /// Fragile - Handle with Care
        "FR" => Fr,
        /// Mechanical Refrigeration
        "MR" => Mr,
        /// Heat
        "HT" => Ht,
        /// Do Not Freeze
        "DNF" => Dnf,
        /// Do Not Stack
        "AJ" => Aj,
        /// Ice
        "IC" => Ic,
        /// Blind Shipment
        "BLS" => Bls,
        /// Exclusive Use
        "EED" => Eed,
        /// Sunday or Holiday Pickup or Delivery
        "HOL" => Hol,
        /// Saturday Pickup or Delivery
        "SAT" => Sat,
        /// Rush Order
        "RO" => Ro,
    }
);

crate::code_enum!(
    /// **187** Weight Qualifier
    ///
    /// - Data element: 187
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code defining the type of weight.
    E187 {
        /// Gross Weight
        "G" => G,
        /// Actual Net Weight
        "N" => N,
        /// Tare Weight
        "T" => T,
        /// Billed Weight
        "B" => B,
        /// Legal Weight
        "L" => L,
        /// Dunnage Weight
        "D" => D,
    }
);

crate::code_enum!(
    /// **146** Shipment Method of Payment
    ///
    /// - Data element: 146
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying payment terms for transportation charges.
    E146 {
        /// Collect
        "CC" => Cc,
        /// Prepaid (by Seller)
        "PP" => Pp,
        /// Prepaid but Charged to Customer
        "PC" => Pc,
        /// Prepaid Only
        "PO" => Po,
        /// Advance Collect
        "CA" => Ca,
        /// Advance Prepaid
        "PA" => Pa,
        /// Third Party Pay
        "TP" => Tp,
        /// Service Freight, No Charge
        "NC" => Nc,
    }
);

crate::code_enum!(
    /// **176** Time Qualifier
    ///
    /// - Data element: 176
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code specifying the type of time.
    E176 {
        /// Original Transaction
        "0" => N0,
        /// Must Respond By
        "1" => N1,
        /// Pickup Appointment Scheduled Time
        "2" => N2,
        /// Delivery Appointment Scheduled Time
        "3" => N3,
        /// Pickup Requested Scheduled Time
        "4" => N4,
        /// Delivery Requested Scheduled Time
        "5" => N5,
        /// Actual Pickup Time
        "8" => N8,
        /// Actual Delivery Time
        "9" => N9,
        /// Estimated Arrival Time
        "E" => E,
        /// Effective Time
        "W" => W,
    }
);

crate::code_enum!(
    /// **100** Currency Code
    ///
    /// - Data element: 100
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Code (ISO 4217) for country in whose currency the charges are specified.
    E100 {
        /// US Dollar
        "USD" => Usd,
        /// Canadian Dollar
        "CAD" => Cad,
        /// Euro
        "EUR" => Eur,
        /// Pound Sterling
        "GBP" => Gbp,
        /// Yen
        "JPY" => Jpy,
        /// Australian Dollar
        "AUD" => Aud,
        /// Swiss Franc
        "CHF" => Chf,
        /// Yuan Renminbi
        "CNY" => Cny,
        /// Mexican Peso
        "MXN" => Mxn,
        /// Indian Rupee
        "INR" => Inr,
    }
);

crate::code_enum!(
    /// **133** Routing Sequence Code
    ///
    /// - Data element: 133
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code describing the relationship of a carrier to a specific shipment movement.
    E133 {
        /// 1st Carrier after Origin Carrier
        "1" => N1,
        /// 2nd Carrier after Origin Carrier
        "2" => N2,
        /// 3rd Carrier after Origin Carrier
        "3" => N3,
        /// Origin Carrier, Agent's Routing (Rail)
        "A" => A,
        /// Origin/Delivery Carrier (Any Mode)
        "B" => B,
        /// Delivery (Delivery Switch Carrier)
        "D" => D,
        /// Origin Switch Carrier
        "I" => I,
        /// Origin Carrier (Air, Motor, or Ocean)
        "O" => O,
        /// Origin Carrier, Rule 11 Shipment
        "R" => R,
        /// Origin Carrier, Shipper's Routing (Rail)
        "S" => S,
        /// Intermediate Switch Carrier
        "V" => V,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **128** Reference Identification Qualifier
    ///
    /// - Data element: 128
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code qualifying the Reference Identification (127). The published list has
    /// ~1500 codes; the common ones are named below and any other round-trips as
    /// `Unknown`.
    E128 {
        /// Bill of Lading Number
        "BM" => Bm,
        /// Booking Number
        "BN" => Bn,
        /// Contract Number
        "CT" => Ct,
        /// Customer Order Number
        "CO" => Co,
        /// Department Number
        "DP" => Dp,
        /// Invoice Number
        "IV" => Iv,
        /// Internal Order Number
        "IL" => Il,
        /// Purchase Order Number
        "PO" => Po,
        /// Purchase Order Line Item Identifier (Buyer)
        "P8" => P8,
        /// Promotion/Deal Number
        "PD" => Pd,
        /// Reference Number
        "RE" => Re,
        /// Release Number
        "RL" => Rl,
        /// Shipment Identification Number
        "SI" => Si,
        /// Store Number
        "ST" => St,
        /// Transaction Reference Number
        "TN" => Tn,
        /// Vendor Order Number
        "VN" => Vn,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **156** State or Province Code
    ///
    /// - Data element: 156
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code (Standard State/Province as defined by appropriate government agency).
    /// Common US states / Canadian provinces named below.
    E156 {
        /// California
        "CA" => Ca,
        /// Texas
        "TX" => Tx,
        /// New York
        "NY" => Ny,
        /// Florida
        "FL" => Fl,
        /// Illinois
        "IL" => Il,
        /// Pennsylvania
        "PA" => Pa,
        /// Ohio
        "OH" => Oh,
        /// Georgia
        "GA" => Ga,
        /// New Jersey
        "NJ" => Nj,
        /// Ontario
        "ON" => On,
        /// Quebec
        "QC" => Qc,
        /// British Columbia
        "BC" => Bc,
    }
);

crate::date_element!(
    /// **109** Pickup Date
    ///
    /// - Data element: 109
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Date a shipment is picked up, expressed as `CCYYMMDD`. `date()` yields a
    /// [`chrono::NaiveDate`]; the raw text is preserved for byte-exact rendering.
    E109
);
