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

crate::num_element!(
    /// **117** Prepaid Amount
    ///
    /// - Data element: 117
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 9
    ///
    /// Prepaid Amount.
    E117
);

crate::num_element!(
    /// **118** Rate
    ///
    /// - Data element: 118
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Rate.
    E118
);

crate::date_element!(
    /// **135** Sailing/Flight Date Estimated
    ///
    /// - Data element: 135
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Sailing/Flight Date Estimated.
    E135
);

crate::date_element!(
    /// **137** Sales Reference Date
    ///
    /// - Data element: 137
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Sales Reference Date.
    E137
);

crate::num_element!(
    /// **148** Lading Value
    ///
    /// - Data element: 148
    /// - Type: Numeric (R)
    /// - Length: min 2, max 9
    ///
    /// Lading Value.
    E148
);

crate::num_element!(
    /// **165** Stop Sequence Number
    ///
    /// - Data element: 165
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Stop Sequence Number.
    E165
);

crate::num_element!(
    /// **167** Tare Weight
    ///
    /// - Data element: 167
    /// - Type: Numeric (N0)
    /// - Length: min 3, max 8
    ///
    /// Tare Weight.
    E167
);

crate::num_element!(
    /// **183** Volume
    ///
    /// - Data element: 183
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Volume.
    E183
);

crate::num_element!(
    /// **186** Waybill Number
    ///
    /// - Data element: 186
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Waybill Number.
    E186
);

crate::num_element!(
    /// **189** Width
    ///
    /// - Data element: 189
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Width.
    E189
);

crate::num_element!(
    /// **191** Advances
    ///
    /// - Data element: 191
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 9
    ///
    /// Advances.
    E191
);

crate::code_enum!(
    /// **184** Volume Unit Qualifier
    ///
    /// - Data element: 184
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Volume Unit Qualifier. Code values verified against the Stedi X12 reference.
    E184 {
        /// Barge
        "B" => B,
        /// Cubic Centimeters
        "C" => C,
        /// Cord
        "D" => D,
        /// Cubic Feet
        "E" => E,
        /// 100 Board Feet
        "F" => F,
        /// Gallons
        "G" => G,
        /// Hundreds of Measurement Tons
        "H" => H,
        /// Load
        "L" => L,
        /// Cubic Decimeters
        "M" => M,
        /// Cubic Inches
        "N" => N,
        /// Car
        "R" => R,
        /// Measurement Ton
        "S" => S,
        /// Container
        "T" => T,
        /// Volumetric Unit
        "U" => U,
        /// Liter
        "V" => V,
        /// Cubic Meters
        "X" => X,
    }
);

crate::code_enum!(
    /// **107** Payment Method Type Code
    ///
    /// - Data element: 107
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Payment Method Type Code. Code values verified against the Stedi X12 reference.
    E107 {
        /// Graduated
        "1" => N1,
        /// Income Sensitive
        "2" => N2,
        /// Income Contingent
        "3" => N3,
        /// Level (Sum Constant)
        "4" => N4,
        /// Credit Account
        "8" => N8,
        /// Private Netting Agreement, Pay by Wire Transfer
        "A" => A,
        /// Bank Draft
        "AA" => Aa,
        /// Cash
        "AB" => Ab,
        /// Credit Card
        "AC" => Ac,
        /// Direct Deposit
        "AD" => Ad,
        /// Money Order
        "AE" => Ae,
        /// Special Account
        "AF" => Af,
        /// Travelers Check
        "AG" => Ag,
        /// Compensation by Balance
        "B" => B,
        /// Pay By Check
        "C" => C,
        /// Debited
        "D" => D,
        /// Electronic Payment System
        "E" => E,
        /// Freight Payment Bank
        "F" => F,
        /// CCD (NACHA Cash Concentration/Disbursement - Funds Transacted without Remittance Information)
        "G" => G,
        /// CCD+(NACHA Cash Concentration/Disbursement - Funds Transacted Plus an 80 Record Remittance Detail)
        "H" => H,
        /// Invoiced Separately
        "I" => I,
        /// CTX (NACHA Corporate Trade Exchange - Transaction Plus Remittance Detail in ANSI Standard Flexible Format)
        "J" => J,
        /// Private Netting, Pay by Check
        "K" => K,
        /// Letter of Credit
        "L" => L,
        /// Deduction from Film Rental
        "M" => M,
        /// Billing Account
        "N" => N,
        /// CTP (NACHA Corporate Trade Payment - Transaction Plus Remittance Detail in Fixed Format)
        "O" => O,
        /// Previously Charged
        "P" => P,
        /// Petroleum Clearinghouse Bank
        "Q" => Q,
        /// Related Detail Account
        "R" => R,
        /// Summary Account
        "S" => S,
        /// Wire Transfer
        "T" => T,
        /// Direct Pay to Others
        "U" => U,
        /// Lock Box
        "V" => V,
        /// Waived
        "W" => W,
        /// In Kind Payment
        "X" => X,
        /// Credit
        "Y" => Y,
    }
);

crate::code_enum!(
    /// **108** Pickup or Delivery Code
    ///
    /// - Data element: 108
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Pickup or Delivery Code. Code values verified against the Stedi X12 reference.
    E108 {
        /// Airport Terminal
        "AT" => At,
        /// Canadian Business Delivery
        "BC" => Bc,
        /// Carrier Advance
        "CA" => Ca,
        /// Business Delivery (Contracted)
        "CB" => Cb,
        /// Convenience Center
        "CC" => Cc,
        /// Customer Counter Pickup
        "CP" => Cp,
        /// Residential Delivery (Contracted)
        "CR" => Cr,
        /// Contract Shipment Delivery
        "CS" => Cs,
        /// City Terminal
        "CT" => Ct,
        /// Drop Box
        "DB" => Db,
        /// Drop Trailer
        "DT" => Dt,
        /// Sunday and Holiday Pickup Service
        "H" => H,
        /// International Business Delivery
        "IB" => Ib,
        /// International Residential Delivery
        "IR" => Ir,
        /// Saturday Pickup Service
        "J" => J,
        /// Live Delivery
        "LD" => Ld,
        /// Live Load
        "LL" => Ll,
        /// Multiple Shipment
        "M" => M,
        /// Other
        "O" => O,
        /// On-call Pickup
        "OC" => Oc,
        /// Other Carrier's Terminal
        "OT" => Ot,
        /// Cut Flowers
        "P" => P,
        /// Private Box
        "PB" => Pb,
        /// Business Delivery (Regular)
        "RB" => Rb,
        /// Canadian Residential Delivery
        "RC" => Rc,
        /// Residential Door
        "RD" => Rd,
        /// Regular Pickup
        "RG" => Rg,
        /// Residential Delivery (Regular)
        "RR" => Rr,
        /// Special Pickup Service
        "S" => S,
        /// Shippers Door
        "SD" => Sd,
        /// Container Shipment
        "U" => U,
        /// High Value Shipment
        "V" => V,
    }
);

crate::code_enum!(
    /// **121** Rate Class Code
    ///
    /// - Data element: 121
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Rate Class Code. Code values verified against the Stedi X12 reference.
    E121 {
        /// Alternate Rating
        "A" => A,
        /// Backhaul Rate
        "BHR" => Bhr,
        /// Specific Commodity Rate
        "C" => C,
        /// Contract Rate
        "CTR" => Ctr,
        /// Demurrage Period 1
        "DMA" => Dma,
        /// Demurrage Period 2
        "DMB" => Dmb,
        /// Demurrage Period 3
        "DMC" => Dmc,
        /// Weight in Excess of Pivot Weight and Applicable Rate
        "E" => E,
        /// Econo Rate
        "ECR" => Ecr,
        /// Overflow
        "F" => F,
        /// Charter
        "H" => H,
        /// Class Not Identifiable
        "I" => I,
        /// Class Rate
        "L" => L,
        /// Minimum
        "M" => M,
        /// Normal Under 45 KG Rate
        "N" => N,
        /// Column Commodity Rate
        "O" => O,
        /// Quantity 45 KG Over Rate
        "Q" => Q,
        /// Quoted Rate
        "QUO" => Quo,
        /// Class Rate (Less than Normal Rate)
        "R" => R,
        /// Class Rate (More than Normal Rate)
        "S" => S,
        /// Sender Rate
        "T" => T,
        /// Pivot Weight and Applicable Pivot Weight Charge
        "U" => U,
        /// Excess Rate
        "V" => V,
        /// IATA Container or Unit Load Device (ULD)
        "X" => X,
        /// Exception Rating
        "Y" => Y,
    }
);

crate::code_enum!(
    /// **122** Rate/Value Qualifier
    ///
    /// - Data element: 122
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Rate/Value Qualifier. Code values verified against the Stedi X12 reference.
    E122 {
        /// Percent Of Amount Advanced
        "AA" => Aa,
        /// Accelerated
        "AB" => Ab,
        /// Percent Of Amount Of Collection
        "AC" => Ac,
        /// Addition
        "AD" => Ad,
        /// Percent of Outstanding Principal Balance
        "AE" => Ae,
        /// Percent of Original Principal Balance
        "AF" => Af,
        /// Percentage
        "AG" => Ag,
        /// Effective Payment Rate
        "AI" => Ai,
        /// Fixed Dollar Amount
        "AJ" => Aj,
        /// Ad Valorum (Per Cent of Value)
        "AV" => Av,
        /// Per Board Feet
        "B0" => B0,
        /// Per 100 Board Feet
        "B1" => B1,
        /// Per 1,000 Board Feet
        "BF" => Bf,
        /// Bill of Lading Declared Value
        "BL" => Bl,
        /// Per Barrel
        "BR" => Br,
        /// Per 50 Cubic Feet
        "C5" => C5,
        /// Commodity Based
        "CB" => Cb,
        /// Per Car Per Day
        "CD" => Cd,
        /// Ceiling
        "CE" => Ce,
        /// Per 40 Cubic Feet (Measurement Ton)
        "CF" => Cf,
        /// Per Cubic Meter
        "CM" => Cm,
        /// Cost per Unit
        "CO" => Co,
        /// Computer Units
        "CP" => Cp,
        /// Per Special Charge
        "CS" => Cs,
        /// Charge or Credit Based on Percentage of Total
        "CT" => Ct,
        /// Per Hundred Weight
        "CW" => Cw,
        /// Department of Defense Unique Codes
        "DD" => Dd,
        /// Decrease
        "DE" => De,
        /// Per Hundred Dollars
        "DH" => Dh,
        /// Divisor
        "DI" => Di,
        /// Flat Division
        "DO" => Do,
        /// Per $1000
        "DP" => Dp,
        /// Per Dromedary Service Shipment
        "DR" => Dr,
        /// Per Hundredweight Per Mile Per Dromedary Service Shipment
        "DS" => Ds,
        /// Per Day Per Vehicle
        "DV" => Dv,
        /// Ex Parte Increase
        "EI" => Ei,
        /// Per Each Request
        "ER" => Er,
        /// Floor
        "FA" => Fa,
        /// Full
        "FB" => Fb,
        /// Flat Charge
        "FC" => Fc,
        /// Percent Of Tariff Rate
        "FF" => Ff,
        /// First
        "FI" => Fi,
        /// Per Flat Bed
        "FL" => Fl,
        /// Per Two Weeks
        "FN" => Fn,
        /// Flat Rate
        "FR" => Fr,
        /// Per Foot
        "FT" => Ft,
        /// Loaded to Full Visible Capacity
        "FV" => Fv,
        /// Per Gross Ton
        "GT" => Gt,
        /// Per Hundredweight Per Dromedary Service Shipment
        "HD" => Hd,
        /// Per Hour Per Load
        "HL" => Hl,
        /// Rate Per Hundred Weight Per Mile
        "HM" => Hm,
        /// Per Half Month
        "HN" => Hn,
        /// Hundredweight Per Day
        "HX" => Hx,
        /// Per Half Year
        "HY" => Hy,
        /// Increase
        "IA" => Ia,
        /// Intermodal Unit
        "IM" => Im,
        /// Per Inch
        "IN" => In,
        /// Per Kilograms
        "KG" => Kg,
        /// Per Kiloliter
        "KL" => Kl,
        /// Per Kilometer
        "KP" => Kp,
        /// Per Kilotons
        "KT" => Kt,
        /// Per Pound Per Article
        "LA" => La,
        /// Per Pound
        "LB" => Lb,
        /// Per Loaded 436L Pallet
        "LF" => Lf,
        /// Liability per Pound per Piece
        "LI" => Li,
        /// Life of Loan
        "LL" => Ll,
        /// Per Label
        "LP" => Lp,
        /// Per Litre
        "LR" => Lr,
        /// Lump Sum
        "LS" => Ls,
        /// Per Long Ton
        "LT" => Lt,
        /// Per Pound Per Vehicle
        "LV" => Lv,
        /// Minimum Per Person
        "M1" => M1,
        /// Minimum per Service
        "MA" => Ma,
        /// Per Mile per Service
        "MB" => Mb,
        /// Minimum Per Car
        "MC" => Mc,
        /// Per Man Per Day
        "MD" => Md,
        /// Multiple Equipment
        "ME" => Me,
        /// Maximum
        "MF" => Mf,
        /// Miles Per Week Per Driver
        "MG" => Mg,
        /// Per Man Per Hour
        "MH" => Mh,
        /// Negative Charge
        "MI" => Mi,
        /// Per Metric Ton (Tonne)
        "MM" => Mm,
        /// Minimum
        "MN" => Mn,
        /// Per Month
        "MO" => Mo,
        /// Maximum Per Shipment
        "MP" => Mp,
        /// Per Mile Per Vehicle Used Per Round Trip
        "MR" => Mr,
        /// Minimum Per Shipment
        "MS" => Ms,
        /// Per Permit
        "MT" => Mt,
        /// Multiplier
        "MU" => Mu,
        /// Per Mile Per Vehicle (Rail Car) Moved
        "MV" => Mv,
        /// Minimum Per Vehicle
        "MW" => Mw,
        /// Mixed Shipment Rule
        "MX" => Mx,
        /// Per Mile Per Shipment
        "MZ" => Mz,
        /// Negative
        "NA" => Na,
        /// Nonamortizing
        "NB" => Nb,
        /// Minimum Per Driver
        "ND" => Nd,
        /// Net Package Charge
        "NE" => Ne,
        /// Per Mile Per Person
        "NM" => Nm,
        /// Minimum Per Day Per Person
        "NP" => Np,
        /// Minimum Per Day Per Vehicle
        "NV" => Nv,
        /// Optional Value
        "OP" => Op,
        /// Per Season
        "OS" => Os,
        /// One-Time Charge
        "OT" => Ot,
        /// Per Year per Square Foot
        "P0" => P0,
        /// Per Advancement
        "P1" => P1,
        /// Per Person Per Night
        "P2" => P2,
        /// Per Car Including Special Equipment Charges
        "P3" => P3,
        /// Per Hundred Weight Including Special Equipment Charges
        "P4" => P4,
        /// Potential
        "P8" => P8,
        /// Partial
        "P9" => P9,
        /// Per Container
        "PA" => Pa,
        /// Per Barge
        "PB" => Pb,
        /// Per Car
        "PC" => Pc,
        /// Per Day
        "PD" => Pd,
        /// Per 20 Foot Equivalent (TEU)
        "PE" => Pe,
        /// Per Cubic Foot
        "PF" => Pf,
        /// Per Gallon
        "PG" => Pg,
        /// Per Hundred (of Basic Unit)
        "PH" => Ph,
        /// Hourly Rate Per Vehicle
        "PI" => Pi,
        /// Projected
        "PJ" => Pj,
        /// Per Cord
        "PK" => Pk,
        /// Per Load
        "PL" => Pl,
        /// Per Mile
        "PM" => Pm,
        /// Per Night
        "PN" => Pn,
        /// Positive
        "PO" => Po,
        /// Per Piece
        "PP" => Pp,
        /// Per Period
        "PQ" => Pq,
        /// Per Hour
        "PR" => Pr,
        /// Per Shipment
        "PS" => Ps,
        /// Per Net Ton
        "PT" => Pt,
        /// Per Unit
        "PU" => Pu,
        /// Per Vehicle
        "PV" => Pv,
        /// Percentage of Charges
        "PW" => Pw,
        /// Payment
        "PX" => Px,
        /// Per Gallon Per Mile
        "PY" => Py,
        /// Per Package Charge
        "PZ" => Pz,
        /// Per Quarter Year
        "QY" => Qy,
        /// Rate per Thousand
        "RA" => Ra,
        /// Rate per Hundred
        "RB" => Rb,
        /// Rate
        "RC" => Rc,
        /// Per Relocation
        "RL" => Rl,
        /// Percent Of Rate
        "RP" => Rp,
        /// Per Vehicle Used Per Round Trip
        "RT" => Rt,
        /// Per 1000 Square Feet
        "S0" => S0,
        /// Per 100 Square Feet
        "S1" => S1,
        /// Subtraction
        "SA" => Sa,
        /// Subsequent
        "SB" => Sb,
        /// Per Stencil
        "SC" => Sc,
        /// Shipper's Export Declaration Value
        "SD" => Sd,
        /// Second
        "SE" => Se,
        /// Per Square Feet
        "SF" => Sf,
        /// Stated
        "SG" => Sg,
        /// Scheduled
        "SH" => Sh,
        /// Per Stop
        "SP" => Sp,
        /// Per Short Ton
        "ST" => St,
        /// Per Stack Car Unit
        "SU" => Su,
        /// Per Vehicle Per Stop
        "SV" => Sv,
        /// Square Yard
        "SY" => Sy,
        /// Per Day Per Shipment
        "SZ" => Sz,
        /// Per 2 Trailers Same Day
        "TB" => Tb,
        /// Per 3 Trailers Same Day
        "TC" => Tc,
        /// Per 4 Trailers Same Day
        "TD" => Td,
        /// Per Mile Per Ton
        "TM" => Tm,
        /// Per Train Rate
        "TN" => Tn,
        /// Per Tag
        "TP" => Tp,
        /// Per Trailer (Per Train)
        "TR" => Tr,
        /// Per Vehicle Moved
        "VA" => Va,
        /// Per Vehicle Used
        "VH" => Vh,
        /// Volume
        "VM" => Vm,
        /// Maximum Per Vehicle
        "VP" => Vp,
        /// Per Rail Car Used
        "VR" => Vr,
        /// Various
        "VS" => Vs,
        /// Per Mile per Vehicle
        "VT" => Vt,
        /// Per Mile Per Vehicle (Rail Car) Used
        "VU" => Vu,
        /// Per Vehicle per State
        "VV" => Vv,
        /// Per Week
        "WK" => Wk,
        /// Weight or Measurement
        "WM" => Wm,
        /// Maximum Per Person
        "XP" => Xp,
        /// Per Year
        "YR" => Yr,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **139** Sales Terms Code
    ///
    /// - Data element: 139
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Sales Terms Code. Code values verified against the Stedi X12 reference.
    E139 {
        /// Cash
        "CH" => Ch,
        /// Date Draft
        "DD" => Dd,
        /// Guarantor
        "GA" => Ga,
        /// Interim Funding
        "IF" => If,
        /// Letter of Credit
        "LC" => Lc,
        /// Mortgage Backed Security
        "MB" => Mb,
        /// No Charge
        "NC" => Nc,
        /// Open Account
        "OA" => Oa,
        /// Stratification
        "SA" => Sa,
        /// Standard Delivery
        "SB" => Sb,
        /// Sight Draft
        "SD" => Sd,
        /// Stock Transfer
        "ST" => St,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **150** Special Charge or Allowance Code
    ///
    /// - Data element: 150
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Special Charge or Allowance Code. Code values verified against the Stedi X12 reference.
    E150 {
        /// Pump Out Charge
        "000" => N000,
        /// Renewal
        "002" => N002,
        /// Account Number Correction Charge
        "003" => N003,
        /// Dividend
        "004" => N004,
        /// Activation of Carnet
        "005" => N005,
        /// Overpayment
        "006" => N006,
        /// Suspense
        "007" => N007,
        /// Balance Forward
        "008" => N008,
        /// Dividend Interest
        "009" => N009,
        /// Add on - Destination
        "010" => N010,
        /// Loan Interest
        "011" => N011,
        /// Premium Waiver
        "012" => N012,
        /// Add on - Origin
        "015" => N015,
        /// Dividend Adjustment
        "016" => N016,
        /// Interest
        "017" => N017,
        /// Address Correction
        "020" => N020,
        /// Advance Destination Amount
        "025" => N025,
        /// Integrated Business Services (IBS) Service Charge
        "026" => N026,
        /// Special Packaging
        "027" => N027,
        /// Box Liners
        "028" => N028,
        /// Product Personalization
        "029" => N029,
        /// Advance Destination Fee
        "030" => N030,
        /// Tape Charges
        "031" => N031,
        /// Subject to Tax
        "032" => N032,
        /// Advance Origin Amount
        "035" => N035,
        /// Advance Origin Fee
        "040" => N040,
        /// Advance Fee
        "045" => N045,
        /// Agent Disbursement - Destination
        "050" => N050,
        /// Agent Disbursement Fee - Destination
        "055" => N055,
        /// Agent Disbursement - Origin
        "060" => N060,
        /// Agent Disbursement Fee - Origin
        "065" => N065,
        /// Air Export Certificate
        "070" => N070,
        /// Air Express Charge
        "075" => N075,
        /// Air Transportation Charge
        "080" => N080,
        /// Airline Opening Fee
        "085" => N085,
        /// Airport Tax - Destination
        "090" => N090,
        /// Airport Tax - Origin
        "095" => N095,
        /// Airport Terminal Handling Charge
        "100" => N100,
        /// Acknowledgment of Delivery Fee (AOD)
        "105" => N105,
        /// Amending Export Documentation
        "110" => N110,
        /// Assembly Fee
        "115" => N115,
        /// Banking Drafts
        "120" => N120,
        /// Cables (sending of)
        "135" => N135,
        /// Call Tag
        "140" => N140,
        /// Canadian C.Q.Customs Clearance
        "145" => N145,
        /// Canadian Currency Exchange
        "150" => N150,
        /// Canadian Import Termination Fee
        "155" => N155,
        /// Canadian Reconsignment Fee
        "160" => N160,
        /// Canadian Remanifest Fee
        "165" => N165,
        /// Certificate of Origin
        "170" => N170,
        /// Certificate of Registration
        "175" => N175,
        /// Chamber of Commerce Service Charge
        "180" => N180,
        /// Change of Airbill - Service Fee
        "185" => N185,
        /// Chemical Milling Charge
        "186" => N186,
        /// City Terminal Charge
        "190" => N190,
        /// Collect Surcharge
        "205" => N205,
        /// Constant Surveillance Service
        "210" => N210,
        /// Consular Legalization Service
        "215" => N215,
        /// Consularization Fee
        "220" => N220,
        /// Constant Surveillance Service - Armed
        "225" => N225,
        /// Credit
        "230" => N230,
        /// Customer Account Identification
        "235" => N235,
        /// Customs Broker Fee
        "240" => N240,
        /// Customs Invoice
        "245" => N245,
        /// Customs Invoice - Additional Page
        "250" => N250,
        /// Data/Drawing Charge
        "255" => N255,
        /// Delivery Surcharge
        "260" => N260,
        /// Development Charge
        "265" => N265,
        /// Discount - Drop Box/Convenience Ctr.
        "270" => N270,
        /// Discount - Incentive
        "275" => N275,
        /// Discount - Multiple Shipment
        "280" => N280,
        /// Discount - Service Option (Delivery)
        "285" => N285,
        /// Discount - Service Option (Pickup)
        "290" => N290,
        /// Discount - Special
        "295" => N295,
        /// Contingency Credit Charge
        "297" => N297,
        /// Distribution Fee
        "300" => N300,
        /// Dry Ice
        "310" => N310,
        /// Duty Charge
        "315" => N315,
        /// Endorsement Fee
        "320" => N320,
        /// Eur1 Presentation Fee
        "325" => N325,
        /// Excise Tax - Destination
        "335" => N335,
        /// Excise Tax - Origin
        "340" => N340,
        /// Expedited One Day Consular Service
        "345" => N345,
        /// Expedited Shipments
        "350" => N350,
        /// Extra Copies and Mailings
        "355" => N355,
        /// Export Customs Clearance
        "360" => N360,
        /// Export Declarations - Automated
        "365" => N365,
        /// Export Declarations - U.S. Shippers
        "370" => N370,
        /// Export License Application
        "375" => N375,
        /// Extra Service - Counter-to-Counter
        "380" => N380,
        /// Facsimile Charges
        "385" => N385,
        /// Facsimile Charges - Additional Pages
        "390" => N390,
        /// Failed Lamp Panel Charge
        "392" => N392,
        /// First Article Charge
        "393" => N393,
        /// Free Domicile Shipment Processing
        "395" => N395,
        /// Freight
        "400" => N400,
        /// Fuel Surcharge
        "405" => N405,
        /// Government Warehouse Fee - Destination
        "410" => N410,
        /// Government Warehouse Fee - Origin
        "415" => N415,
        /// Grain Flow Charge
        "416" => N416,
        /// Hazardous Materials Handling Fee - Domestic
        "420" => N420,
        /// Hazardous Materials Handling Fee - International
        "425" => N425,
        /// Heat Treat Charge
        "426" => N426,
        /// IATA Airbill Preparation
        "430" => N430,
        /// IATA Fee
        "435" => N435,
        /// Import Service Fee
        "440" => N440,
        /// Insurance Fee
        "445" => N445,
        /// Inland Transportation
        "450" => N450,
        /// Insurance Premium
        "455" => N455,
        /// International Door-to-Door Handling Fee
        "460" => N460,
        /// Incorrect Billing Account Charge
        "462" => N462,
        /// Italian Release Charge
        "465" => N465,
        /// Letter of Credit Processing
        "470" => N470,
        /// Mailing - Postage Cost
        "475" => N475,
        /// Mailing - Service Fee
        "480" => N480,
        /// Messenger Service
        "485" => N485,
        /// Minimum Air Transportation Charge
        "490" => N490,
        /// Miscellaneous - Destination
        "495" => N495,
        /// Miscellaneous - Origin
        "500" => N500,
        /// Missing Account NBR Charge
        "505" => N505,
        /// Offshore - Alaska/Hawaii
        "510" => N510,
        /// On Hand Service
        "515" => N515,
        /// Oversized Premium
        "520" => N520,
        /// Passing Shippers Export Entry
        "525" => N525,
        /// Pickup - Out of Area
        "535" => N535,
        /// Pickup Surcharge
        "540" => N540,
        /// Pre-Positioned Inventory Service
        "545" => N545,
        /// Preparation of Air Waybill - Origin
        "550" => N550,
        /// Preparation of Canadian Customs Invoice
        "555" => N555,
        /// Preparation of Commercial Invoice
        "560" => N560,
        /// Preparation of Export Entry
        "565" => N565,
        /// Preparation of Insurance Certificate
        "570" => N570,
        /// Priority Service
        "580" => N580,
        /// Preparation of U.S. Export Documentation
        "585" => N585,
        /// Processing Charge
        "586" => N586,
        /// Rebilled Drayage - Destination
        "590" => N590,
        /// Re-Bill Charge
        "593" => N593,
        /// Proforma Invoice
        "595" => N595,
        /// Recipient Address Correction
        "600" => N600,
        /// Rebilled Drayage - Origin
        "605" => N605,
        /// Record/Filing
        "610" => N610,
        /// Recovery Fee
        "615" => N615,
        /// Recrating/Recoopering - Destination
        "620" => N620,
        /// Recrating/Recoopering - Origin
        "625" => N625,
        /// Registration of Export Shipments
        "635" => N635,
        /// Registration of Export for Reentry
        "640" => N640,
        /// Reliability Charge
        "641" => N641,
        /// Restricted Article Fee
        "645" => N645,
        /// Repickup
        "650" => N650,
        /// Saturday Delivery
        "665" => N665,
        /// Saturday Pickup
        "670" => N670,
        /// Security Signature Service
        "675" => N675,
        /// Service Upgrade
        "680" => N680,
        /// Special Delivery
        "685" => N685,
        /// Special Handling Service
        "690" => N690,
        /// Special Pickup
        "695" => N695,
        /// Special Test Equipment Charge
        "696" => N696,
        /// Special Tooling Charge
        "697" => N697,
        /// Special Vehicle Rent
        "700" => N700,
        /// Stamp Fee
        "705" => N705,
        /// Straightening Charge
        "706" => N706,
        /// Telephone - Destination
        "720" => N720,
        /// Telephone - Origin
        "725" => N725,
        /// Terminal Service Fee
        "730" => N730,
        /// Test/Qualification Charge
        "731" => N731,
        /// Tooling Rework Charge
        "732" => N732,
        /// Tracing Inbound Via Other Carriers
        "735" => N735,
        /// Tracing Service Fee
        "736" => N736,
        /// Transfer of Lading Charge
        "740" => N740,
        /// Valuation Fee
        "745" => N745,
        /// Value Added Tax (VAT)
        "750" => N750,
        /// Waybill and Invoice Distribution
        "760" => N760,
        /// Written Proof of Delivery
        "761" => N761,
        /// X-ray Charge
        "762" => N762,
        /// Auto Towing Charge
        "763" => N763,
        /// Late Return Charge
        "764" => N764,
        /// One Way Drop Off Charge
        "765" => N765,
        /// Business Center Charge
        "766" => N766,
        /// Gift Shop Charge
        "767" => N767,
        /// Health Club Charge
        "768" => N768,
        /// Laundry and Dry Cleaning Charge
        "769" => N769,
        /// In-room Mini-bar Charge
        "770" => N770,
        /// In-room Movie Charge
        "771" => N771,
        /// Passenger Facility Charge
        "772" => N772,
        /// Prepaid Expenses
        "773" => N773,
        /// Other (See related description)
        "999" => N999,
        /// Advertising Allowance
        "AAA" => Aaa,
        /// Adjustments
        "AAJ" => Aaj,
        /// Additional Material
        "AAM" => Aam,
        /// Allowance Non-performance
        "AAN" => Aan,
        /// Allowance Advance
        "AAO" => Aao,
        /// Attendants Accompanying
        "AAS" => Aas,
        /// Handling Charge Tax
        "AAT" => Aat,
        /// Alcoholic Beverage Report Charge
        "ABC" => Abc,
        /// Attachments to Bill of Lading Charge
        "ABL" => Abl,
        /// Allegheny County, PA Delivery Charge
        "ACD" => Acd,
        /// Access Charge - Federal
        "ACF" => Acf,
        /// Access Charges
        "ACH" => Ach,
        /// Actual Labor Charge
        "ACL" => Acl,
        /// Access Charge - State
        "ACS" => Acs,
        /// Advance Charges Handling
        "ADH" => Adh,
        /// Advance Loading Charge
        "ADL" => Adl,
        /// Advances
        "ADV" => Adv,
        /// Additional Copies of Freight Bill
        "AFB" => Afb,
        /// Collect on Delivery Alteration Charge
        "AFC" => Afc,
        /// Aircraft Ordered But Not Used
        "AFN" => Afn,
        /// Armed Guard Service
        "AGS" => Ags,
        /// Additional Injection/Blending Service Charge
        "AIB" => Aib,
        /// Air Freight - Consolidation
        "AIC" => Aic,
        /// Air Freight
        "AIR" => Air,
        /// Advance Lading Charge
        "ALC" => Alc,
        /// Use of Alternate Port
        "ALP" => Alp,
        /// Adjustment for Maximum Charges Billing
        "AMB" => Amb,
        /// Absolute Minimum Charge
        "AMC" => Amc,
        /// Adjustment for Minimum Average Time Requirement Billing
        "AMP" => Amp,
        /// Adjustment for Minimum Charges Billing
        "ANB" => Anb,
        /// Anchoring and Unanchoring
        "ANC" => Anc,
        /// Anodizing Charge
        "ANS" => Ans,
        /// Appliance Servicing
        "APL" => Apl,
        /// Appointment (Notification)
        "APT" => Apt,
        /// Arbitrary (In Addition to Through Rates and Charges)
        "ARB" => Arb,
        /// Air Conditioning Disconnect and Connect
        "ARC" => Arc,
        /// Rail Armed Guard Service
        "ARG" => Arg,
        /// Air Ride Tractor Service Charge
        "ARR" => Arr,
        /// Assembly Charge
        "ASC" => Asc,
        /// Auxiliary Service
        "AUX" => Aux,
        /// Ad Valorem
        "AVA" => Ava,
        /// Beaming Charge
        "BAA" => Baa,
        /// Brokerage or Duty
        "BAB" => Bab,
        /// Buyers Car Allowance
        "BAC" => Bac,
        /// Bad Debt
        "BAD" => Bad,
        /// Both-Flat
        "BAF" => Baf,
        /// Broken Package Charge
        "BAP" => Bap,
        /// Base Charge
        "BAS" => Bas,
        /// Break Bulk Surface Charge
        "BBK" => Bbk,
        /// Border Crossing Fee
        "BCF" => Bcf,
        /// Bordeaux Arbitraries
        "BDX" => Bdx,
        /// Beyond Freight Charges
        "BEY" => Bey,
        /// Bedding/Feeding/Disinfecting
        "BFD" => Bfd,
        /// Bulky Article
        "BKA" => Bka,
        /// Bill of Lading Attendancy
        "BLA" => Bla,
        /// Bill of Lading Charge
        "BLC" => Blc,
        /// Billed Demand
        "BLD" => Bld,
        /// Blocking and Bracing Charge
        "BLK" => Blk,
        /// Blower Charge
        "BLW" => Blw,
        /// Bond Charges
        "BND" => Bnd,
        /// Bobtail Charges
        "BOB" => Bob,
        /// Bop Sheet Charge
        "BOP" => Bop,
        /// Basic Reorder Allowance
        "BRA" => Bra,
        /// Bridge Toll
        "BRD" => Brd,
        /// Aqua Train
        "BRG" => Brg,
        /// Bunker Surcharge
        "BSC" => Bsc,
        /// Broker Selection Surcharge
        "BSS" => Bss,
        /// Bi-level, Tri-level Charges
        "BTC" => Btc,
        /// Bunker Adjustment - 20 Foot Container
        "BU2" => Bu2,
        /// Bunker Adjustment - 40 Foot Container
        "BU4" => Bu4,
        /// Bunker Adjustment
        "BUA" => Bua,
        /// Bureau Report Charge
        "BUR" => Bur,
        /// Beyond Charge
        "BYD" => Byd,
        /// Currency Adjustment - Break Bulk
        "CA1" => Ca1,
        /// Currency Adjustment - 20 Foot Container
        "CA2" => Ca2,
        /// Currency Adjustment - 40 Foot Container
        "CA4" => Ca4,
        /// Cancellation Charge
        "CAA" => Caa,
        /// Cash Discount
        "CAC" => Cac,
        /// Certification Fee
        "CAD" => Cad,
        /// Co-manufacturing Discount
        "CAE" => Cae,
        /// Competitive Allowance
        "CAF" => Caf,
        /// Competitive Car Allowance
        "CAG" => Cag,
        /// Compressor Charge
        "CAH" => Cah,
        /// Crafting
        "CAJ" => Caj,
        /// Customer Equipment Allowance
        "CAK" => Cak,
        /// Cutting Charge
        "CAL" => Cal,
        /// Co-op Credit
        "CAO" => Cao,
        /// Car Loading
        "CAP" => Cap,
        /// Contract Escalation
        "CAQ" => Caq,
        /// Car Rental
        "CAR" => Car,
        /// Container Deposits
        "CAS" => Cas,
        /// Contract Allowance
        "CAV" => Cav,
        /// Cooperative Advertising/Merchandising Allowance (Performance)
        "CAW" => Caw,
        /// Claims Commercial Auto Report Charge
        "CAZ" => Caz,
        /// Copy of Bill of Lading Charge
        "CBL" => Cbl,
        /// Cents Off
        "CBO" => Cbo,
        /// Competitive Price
        "CBP" => Cbp,
        /// Carrier
        "CBR" => Cbr,
        /// Container Allowance
        "CBW" => Cbw,
        /// City Sales Tax (Only)
        "CBX" => Cbx,
        /// Carrier Credit Allowance
        "CCA" => Cca,
        /// Certification Charge
        "CCH" => Cch,
        /// Claims Commercial Property Report Charge
        "CCP" => Ccp,
        /// Concession Credit
        "CCR" => Ccr,
        /// Carrier Caboose Charge
        "CCS" => Ccs,
        /// Carrier Debit Allowance
        "CDA" => Cda,
        /// Corrosion Additive Service Charge
        "CDD" => Cdd,
        /// Cancelled Order, Heavy Duty Flatcar
        "CDF" => Cdf,
        /// Copy of Delivery Receipt Charge
        "CDR" => Cdr,
        /// Container Service Charge UK/EUR
        "CER" => Cer,
        /// Customs Fees - Container Level
        "CFC" => Cfc,
        /// Customs Fees - Lift Level
        "CFL" => Cfl,
        /// Carrier Guard Car Charge
        "CGC" => Cgc,
        /// Canada Great Lakes Additionals
        "CGL" => Cgl,
        /// Return Carrier Guard Car Charge
        "CGR" => Cgr,
        /// Cargo Taxes
        "CGT" => Cgt,
        /// Chassis Equipment Lease Charge
        "CHE" => Che,
        /// Charges Forward/Advance Charge
        "CHG" => Chg,
        /// Chain and Binders
        "CHN" => Chn,
        /// Special Circus Trains
        "CIR" => Cir,
        /// Constant Surveillance
        "CIS" => Cis,
        /// Chicago Loop Charge
        "CLC" => Clc,
        /// Container Loss/Damage
        "CLD" => Cld,
        /// Cleaning Charge
        "CLN" => Cln,
        /// Container Leasing
        "CLS" => Cls,
        /// Concession Money
        "CMC" => Cmc,
        /// City maintenance fee
        "CMF" => Cmf,
        /// Continuous Mileage
        "CMI" => Cmi,
        /// Camp Arbitrary
        "CMP" => Cmp,
        /// Consolidation
        "CNS" => Cns,
        /// Converting
        "CNV" => Cnv,
        /// Commission Amount
        "COA" => Coa,
        /// Connect Charge
        "COC" => Coc,
        /// COD Amount
        "COD" => Cod,
        /// Ocean Freight
        "COF" => Cof,
        /// Fee for Collecting COD Charge
        "COL" => Col,
        /// Combination
        "COM" => Com,
        /// Congestion Surcharge
        "CON" => Con,
        /// Port Changes
        "COP" => Cop,
        /// Core Charge
        "COR" => Cor,
        /// Consignee Unload
        "COU" => Cou,
        /// Claims Personal Auto Report Charge
        "CPA" => Cpa,
        /// Copilot Service Charge
        "CPC" => Cpc,
        /// Computer Processing Expense
        "CPE" => Cpe,
        /// Claims Personal Property Report Charge
        "CPP" => Cpp,
        /// Cost recovery/adjustment
        "CRA" => Cra,
        /// Cost Recovery Factor
        "CRF" => Crf,
        /// Court Reporter Charge
        "CRP" => Crp,
        /// Credit Report Charge
        "CRR" => Crr,
        /// Courier Services
        "CRS" => Crs,
        /// Closing & Sealing
        "CSA" => Csa,
        /// Contract Service Charge
        "CSC" => Csc,
        /// Customs Entry
        "CSE" => Cse,
        /// Customs Formalities
        "CSF" => Csf,
        /// Government Caboose Charge
        "CSP" => Csp,
        /// Conservation research fee
        "CSR" => Csr,
        /// Cassette
        "CST" => Cst,
        /// Container/Trailer Allowance
        "CTA" => Cta,
        /// Container Service Charge USA/Canada
        "CTC" => Ctc,
        /// Customer Required Special Truck at Destination
        "CTD" => Ctd,
        /// Court or Trial Expense
        "CTE" => Cte,
        /// Chassis Transfer
        "CTF" => Ctf,
        /// Cartage Charge
        "CTG" => Ctg,
        /// Controlled Atmosphere
        "CTL" => Ctl,
        /// Customer Required Special Truck at Origin
        "CTO" => Cto,
        /// Circuitous Routing Charge
        "CTR" => Ctr,
        /// Customs Exams (Intensive, Tailgate)
        "CTX" => Ctx,
        /// Currency Adjustment
        "CUA" => Cua,
        /// Currency Discount
        "CUD" => Cud,
        /// Currency Adjustment Factor
        "CUF" => Cuf,
        /// Customer Paid Deductible
        "CUP" => Cup,
        /// Customs Charge
        "CUS" => Cus,
        /// Deficit Freight
        "DAA" => Daa,
        /// Deposit
        "DAB" => Dab,
        /// Distributor Discount/Allowance
        "DAC" => Dac,
        /// Drum Up Charge
        "DAD" => Dad,
        /// Damaged Merchandise
        "DAM" => Dam,
        /// Dockage - Boat Detention
        "DBD" => Dbd,
        /// Double Wide Separate and Reassemble
        "DBL" => Dbl,
        /// Delivery of Fuel from Barge to Pipeline Charge
        "DBP" => Dbp,
        /// Damage to Carrier Equipment
        "DCE" => Dce,
        /// Disconnect charge
        "DCS" => Dcs,
        /// City Delivery
        "DCT" => Dct,
        /// Damage to Carrier Vessel
        "DCV" => Dcv,
        /// Defective Allowance
        "DDA" => Dda,
        /// Drum Cost
        "DDC" => Ddc,
        /// Drum Deposit
        "DDD" => Ddd,
        /// Dowel Pin Charge
        "DDF" => Ddf,
        /// Dual Driver with National Agency Check
        "DDN" => Ddn,
        /// Dual Driver Protectice Service
        "DDP" => Ddp,
        /// Deaf and Disabled Surcharge
        "DDS" => Dds,
        /// Drayage at Port of Debarkation (Zone Rate)
        "DDZ" => Ddz,
        /// Demurrage - Average Agreement
        "DEA" => Dea,
        /// Detention: Vehicle with Power Unit (Bulk Petroleum Product Shipments)
        "DEB" => Deb,
        /// Deductible
        "DED" => Ded,
        /// Delivery Charge
        "DEL" => Del,
        /// Demurrage
        "DEM" => Dem,
        /// Detention of Power Units
        "DEP" => Dep,
        /// Derrick Charge
        "DER" => Der,
        /// Demurrage - Special
        "DES" => Des,
        /// Detention of Trailers
        "DET" => Det,
        /// Texas Rail Commission Deviation Charge
        "DEV" => Dev,
        /// Detention Without Power Unit
        "DEW" => Dew,
        /// Drayage at Port of Embarkation (Zone Rate)
        "DEZ" => Dez,
        /// Keep from Freezing Percent Differential
        "DFD" => Dfd,
        /// 410 Dromedary with Mechanical Restraining Devices Charge
        "DFM" => Dfm,
        /// 410 Dromedary
        "DFS" => Dfs,
        /// Delay Furnishing Destination Weights
        "DFW" => Dfw,
        /// Damage to Government Equipment
        "DGE" => Dge,
        /// Dangerous Goods Surcharge
        "DGS" => Dgs,
        /// Diversion Charge
        "DIC" => Dic,
        /// Direct Repair
        "DIR" => Dir,
        /// Distribution Service
        "DIS" => Dis,
        /// Diversion and Reconsignment
        "DIV" => Div,
        /// Drayage/Line Haul
        "DLH" => Dlh,
        /// Delivery of Fuel from Rail Tank Car to Pipeline Charge
        "DLP" => Dlp,
        /// Deadhead Mileage Charge
        "DMC" => Dmc,
        /// Demand charge
        "DMD" => Dmd,
        /// Dunnage Allowance
        "DNA" => Dna,
        /// Documentation Charge
        "DOC" => Doc,
        /// Deposit in Lieu of Order
        "DON" => Don,
        /// Container Diversion
        "DOV" => Dov,
        /// Delivery of Fuel from Pipeline to Barge Charge
        "DPB" => Dpb,
        /// Drayage at Port of Debarkation
        "DPD" => Dpd,
        /// Drayage at Port of Embarkation
        "DPE" => Dpe,
        /// Delivery of Fuel from Pipeline to Rail Tank Car Charge
        "DPL" => Dpl,
        /// Depreciation
        "DPR" => Dpr,
        /// Delivery of Fuel from Pipeline to Tank Truck or Trailer Charge
        "DPT" => Dpt,
        /// Detention with Power Units (30 minute periods) Charge
        "DPU" => Dpu,
        /// Drayage
        "DRC" => Drc,
        /// Deramping
        "DRP" => Drp,
        /// Driver License Record Report Charge
        "DRV" => Drv,
        /// Dryer Charge
        "DRY" => Dry,
        /// Discount
        "DSC" => Dsc,
        /// Detention - Special Type Flat Car
        "DSF" => Dsf,
        /// Dromedary with Mechanical Restraining Devices Charge
        "DSM" => Dsm,
        /// Dromedary Service Charge
        "DSR" => Dsr,
        /// Container Destuffing
        "DST" => Dst,
        /// Diversion to Air Charge
        "DTA" => Dta,
        /// Detention (Labor)
        "DTB" => Dtb,
        /// Destination Charge
        "DTC" => Dtc,
        /// Destination Duty
        "DTD" => Dtd,
        /// Destination Inland Freight
        "DTF" => Dtf,
        /// Detention Loading
        "DTL" => Dtl,
        /// Delivery of Fuel from Tank Truck or Trailer to Pipeline Charge
        "DTP" => Dtp,
        /// Detention Unloading
        "DTU" => Dtu,
        /// Detention (Vehicle)
        "DTV" => Dtv,
        /// Driver's Wages
        "DWC" => Dwc,
        /// Detention with Power Units (60 minute periods) Charge
        "DWP" => Dwp,
        /// Exchange Access Credit
        "EAC" => Eac,
        /// Extra Axles
        "EAX" => Eax,
        /// Exhibition Delivery Charge
        "EBD" => Ebd,
        /// Exhibition Pickup Charge
        "EBP" => Ebp,
        /// Will Call Charge
        "ECC" => Ecc,
        /// Escort/Courier Service Charge
        "ECR" => Ecr,
        /// Empty Railcar Ordered But Not Used Charge
        "ECS" => Ecs,
        /// Equipment Hose at Destination Charge
        "EDD" => Edd,
        /// Equipment Hose at Origin Charge
        "EDO" => Edo,
        /// Early Buy Allowance
        "EEA" => Eea,
        /// Early Payment Allowance
        "EEB" => Eeb,
        /// Escalation
        "EEC" => Eec,
        /// Expediting Fee
        "EEF" => Eef,
        /// One Time Engineering Charge
        "EEG" => Eeg,
        /// Engineering Charge
        "EEH" => Eeh,
        /// Expediting Premium
        "EEP" => Eep,
        /// Export Shipping Charge
        "EEX" => Eex,
        /// Export/Import Charge
        "EIC" => Eic,
        /// Extra Lights
        "ELS" => Els,
        /// Emergency Response Service
        "EMR" => Emr,
        /// Emergency Surcharge
        "EMS" => Ems,
        /// Empty Movement
        "EMT" => Emt,
        /// Energy charge
        "ENC" => Enc,
        /// Energy Surcharge (Fuel Adjustment Factor)
        "ENS" => Ens,
        /// Emergency Port Charge
        "EPC" => Epc,
        /// Environmental Protection Service
        "EPS" => Eps,
        /// Empty Return
        "ERS" => Ers,
        /// Satisfactory Service Standards Charge
        "ERT" => Ert,
        /// Early Ship Allowance
        "ESA" => Esa,
        /// Emergency Service
        "ESC" => Esc,
        /// Estimated Customs Duty (Dutypaid - Charge)
        "ESD" => Esd,
        /// External Service Expense
        "ESE" => Ese,
        /// Empty Trailer Returned Charge
        "ETR" => Etr,
        /// European Port Charges
        "EUC" => Euc,
        /// Excessive Value Charge
        "EVC" => Evc,
        /// Exclusive Use Charge
        "EXC" => Exc,
        /// Extra Driver
        "EXD" => Exd,
        /// Extra Length
        "EXL" => Exl,
        /// Excess Mileage Charge
        "EXM" => Exm,
        /// Expedited Service Charge
        "EXP" => Exp,
        /// Excess Periods
        "EXS" => Exs,
        /// Excess Weight
        "EXW" => Exw,
        /// Expando Remove and Install
        "EXZ" => Exz,
        /// F.E.T. Federal Excise Tax
        "FAB" => Fab,
        /// F.E.T. (Percent)
        "FAC" => Fac,
        /// F.E.T. (Dollar Value)
        "FAD" => Fad,
        /// Fabrication Charge
        "FAE" => Fae,
        /// F.E.T. Tires
        "FAF" => Faf,
        /// Freight Equalization
        "FAG" => Fag,
        /// Freight Surcharge
        "FAH" => Fah,
        /// Barge Freight All Kinds Service
        "FAK" => Fak,
        /// Freight, Based on Dollar Minimum
        "FBD" => Fbd,
        /// Freight Charges to Border
        "FCB" => Fcb,
        /// Freight Charges to Destination
        "FCD" => Fcd,
        /// Freight Charges Inbound and Outbound
        "FCI" => Fci,
        /// Furnishing Chassis
        "FCS" => Fcs,
        /// Flat Deck Delivery
        "FDD" => Fdd,
        /// Food and Lodging
        "FDL" => Fdl,
        /// Financial Document Surcharge
        "FDS" => Fds,
        /// Fuel Filters Furnished by Carrier Charge
        "FFC" => Ffc,
        /// Finance Charge
        "FFI" => Ffi,
        /// Freshness/Leaker Allowance
        "FFL" => Ffl,
        /// Special Finish Charge
        "FFN" => Ffn,
        /// Freight Passthrough
        "FFP" => Ffp,
        /// Flat Rate
        "FFR" => Ffr,
        /// Fuel Filters Furnished by Shipper Charge
        "FFS" => Ffs,
        /// Fire Report
        "FIR" => Fir,
        /// Flatrack Surcharge
        "FLS" => Fls,
        /// Ferry Service
        "FLT" => Flt,
        /// Foreign Military Sales (FMS) Rental
        "FMR" => Fmr,
        /// Foreign Military Sales (FMS) Special Charge
        "FMS" => Fms,
        /// Franchise fee
        "FRC" => Frc,
        /// Federal Transfer Surcharge
        "FTC" => Ftc,
        /// Filtration Service Charge
        "FTR" => Ftr,
        /// Fuel Charge
        "FUE" => Fue,
        /// Forwarding Agent Commission
        "FWA" => Fwa,
        /// Forwarding Charge
        "FWC" => Fwc,
        /// Texas Rail Commission Fixed Charge
        "FXE" => Fxe,
        /// Garment District
        "GAR" => Gar,
        /// Gate Inspection Charge (Intermodal)
        "GAT" => Gat,
        /// Grain Doors
        "GDR" => Gdr,
        /// Glaze Allowance
        "GGA" => Gga,
        /// Gold Factor
        "GGF" => Ggf,
        /// Gasket
        "GKT" => Gkt,
        /// Garment Surcharge
        "GMS" => Gms,
        /// Government-owned Containers
        "GOC" => Goc,
        /// Gulf Port Delivery Charge
        "GPD" => Gpd,
        /// Groupage Discount
        "GRD" => Grd,
        /// Gross Receipts Surcharge
        "GRS" => Grs,
        /// Government Guard Car Charge
        "GSP" => Gsp,
        /// Greater Security Service
        "GSS" => Gss,
        /// Goods and Services Tax Charge
        "GST" => Gst,
        /// Handling Charges on Distribution Freight Forwarded Beyond
        "HAN" => Han,
        /// Hazardous Cargo Charge
        "HAZ" => Haz,
        /// Harbor Dues
        "HBD" => Hbd,
        /// Heavy Duty Flat Car Charge
        "HDF" => Hdf,
        /// Holding Charge
        "HDG" => Hdg,
        /// Shipment Holdover Charge for Holidays
        "HDH" => Hdh,
        /// Shipment Holdover Charge for Weekends
        "HDW" => Hdw,
        /// Heat in Transit Charges
        "HET" => Het,
        /// Handling Freight At Positions Not Immediately Adjacent To Vehicle Charge
        "HFA" => Hfa,
        /// Hauling and Hoisting to be Direct Billed
        "HHA" => Hha,
        /// Handling
        "HHB" => Hhb,
        /// Household Goods Pickup or Delivery
        "HHG" => Hhg,
        /// Highway Interchange
        "HIC" => Hic,
        /// Home Line Freight Charge
        "HLF" => Hlf,
        /// Accessible Hazardous Material
        "HMA" => Hma,
        /// Inaccessible Hazardous Material
        "HMI" => Hmi,
        /// Hook-up charge
        "HOC" => Hoc,
        /// Sunday or Holiday Pickup or Delivery
        "HOL" => Hol,
        /// Hose Charge
        "HOS" => Hos,
        /// Hose Charge Special
        "HOX" => Hox,
        /// Heater or Refrigeration
        "HRS" => Hrs,
        /// High Security Red In-bond Seal Charge
        "HSC" => Hsc,
        /// Heavy Lift
        "HUL" => Hul,
        /// Hazardous Materials Surcharge Charge
        "HZC" => Hzc,
        /// Hazardous Cargo on Deck
        "HZD" => Hzd,
        /// Hazardous Storage
        "HZS" => Hzs,
        /// Industry Price Allowance
        "IAA" => Iaa,
        /// Income Freight (Manufacturing to Shipping Point)
        "IAB" => Iab,
        /// Inspection Fee
        "IAC" => Iac,
        /// Cooling Service
        "ICE" => Ice,
        /// Idler Car Charge
        "IDC" => Idc,
        /// Improper Documentation
        "IDD" => Idd,
        /// Inside Delivery
        "IDL" => Idl,
        /// Interdivision Profit
        "IDP" => Idp,
        /// Inbound Freight Charges
        "IFC" => Ifc,
        /// Interstate/Highway Toll
        "IHT" => Iht,
        /// Invoice Adjustment
        "IIA" => Iia,
        /// Icing Inhibitor Charge
        "IIH" => Iih,
        /// Item Percentage
        "IIP" => Iip,
        /// Item-Unit
        "IIU" => Iiu,
        /// Island Delivery Charge
        "ILD" => Ild,
        /// Initial License Fee
        "ILF" => Ilf,
        /// Island Pickup Charge
        "ILP" => Ilp,
        /// Impactographs
        "IMP" => Imp,
        /// Intermodal Shipment Service Charge
        "IMS" => Ims,
        /// Insurance Surcharge
        "INC" => Inc,
        /// Interplant Charge
        "INP" => Inp,
        /// Interest on refund
        "INR" => Inr,
        /// Insurance
        "INS" => Ins,
        /// Interpreter Expense
        "INT" => Int,
        /// Intra-plant Charge
        "IPC" => Ipc,
        /// Inside Pickup
        "IPU" => Ipu,
        /// Irish Arbitraries
        "IRA" => Ira,
        /// Interest on security deposit
        "ISD" => Isd,
        /// Intermodal Storage (Origin)
        "ISO" => Iso,
        /// Intermodal Storage (Destination)
        "IST" => Ist,
        /// Insulated Tank Charge
        "ITC" => Itc,
        /// Interline Transfer Charge
        "ITS" => Its,
        /// Junction Settlement Charge
        "JST" => Jst,
        /// Glass Kit
        "KIT" => Kit,
        /// Labor Charges
        "LAA" => Laa,
        /// Extra Labor (Helper Service)
        "LAB" => Lab,
        /// Lading Adjustment Charge
        "LAC" => Lac,
        /// Labor (Repair and Return Orders)
        "LAD" => Lad,
        /// One-Time License Fee
        "LAE" => Lae,
        /// Labor Adjustment Allowance
        "LAL" => Lal,
        /// Commingling/Loss Allowance Charge
        "LAS" => Las,
        /// License and Title
        "LAT" => Lat,
        /// Layover Charges
        "LAY" => Lay,
        /// Light Bar Service Charge
        "LBR" => Lbr,
        /// Land Currency Adjustment Factor - 20 Foot Container
        "LC2" => Lc2,
        /// Land Currency Adjustment Factor - 40 Foot Container
        "LC4" => Lc4,
        /// Late Order Charge
        "LCG" => Lcg,
        /// Percent Differential - Less than Container
        "LCL" => Lcl,
        /// Labor Cost of Removal
        "LCR" => Lcr,
        /// Loading Allowance
        "LDA" => Lda,
        /// Loading
        "LDG" => Ldg,
        /// Unloading Allowance
        "LDL" => Ldl,
        /// Locomotive Delayed in Switching Service
        "LDS" => Lds,
        /// Less than Container
        "LEC" => Lec,
        /// Lift Charge (Intermodal)
        "LFC" => Lfc,
        /// Linehaul from Port of Debarkation
        "LFD" => Lfd,
        /// Lift Gate (Truck) or Forklift Service at Pickup/Delivery
        "LFT" => Lft,
        /// Lodging
        "LGD" => Lgd,
        /// Linehaul Service
        "LHS" => Lhs,
        /// Recurring License Fee
        "LID" => Lid,
        /// Liability of Carrier Charge
        "LIE" => Lie,
        /// Limited Liability
        "LLB" => Llb,
        /// Lot Charge
        "LLC" => Llc,
        /// Lead Factor
        "LLD" => Lld,
        /// Loan Fee
        "LLF" => Llf,
        /// Local Sales Tax (All Applicable Sales Taxes by Taxing Authorities Below the State Level)
        "LLS" => Lls,
        /// Labor, Modify
        "LMC" => Lmc,
        /// Liner Terms at Port of Debarkation
        "LMD" => Lmd,
        /// Liner Terms at Port of Embarkation
        "LME" => Lme,
        /// Labor, No Trouble Found
        "LNT" => Lnt,
        /// Loading (Labor Charges)
        "LOA" => Loa,
        /// Local Delivery/Drayage
        "LOC" => Loc,
        /// Late payment charge
        "LPC" => Lpc,
        /// Linehaul Percent Differential
        "LPD" => Lpd,
        /// Laboratory Pack Fee
        "LPF" => Lpf,
        /// Liquidated Damages
        "LQD" => Lqd,
        /// Labor Service
        "LSC" => Lsc,
        /// Lashing
        "LSH" => Lsh,
        /// Lifeline Surcharge
        "LSS" => Lss,
        /// Labor, Test and Calibrate
        "LTC" => Ltc,
        /// Linehaul to Port of Embarkation
        "LTE" => Lte,
        /// Lubricant Charge
        "LUB" => Lub,
        /// Locomotive Under Own Power
        "LUP" => Lup,
        /// Leaking underground storage tax (LUST)
        "LUS" => Lus,
        /// Layover Service Charge
        "LYC" => Lyc,
        /// Metals Surcharge
        "MAA" => Maa,
        /// Mileage or Travel
        "MAB" => Mab,
        /// Mileage Fee (For Repair and Return)
        "MAC" => Mac,
        /// Minimum Order/Minimum Billing Charge
        "MAD" => Mad,
        /// Monthly Rental
        "MAE" => Mae,
        /// Marriage Rule
        "MAR" => Mar,
        /// Modified Atmosphere
        "MAT" => Mat,
        /// Machining Charge
        "MCC" => Mcc,
        /// Molding
        "MDG" => Mdg,
        /// Mount/Demount
        "MDM" => Mdm,
        /// Meals or Lodging Charge
        "MEA" => Mea,
        /// Escort Service with Overnight Subsistence
        "MEN" => Men,
        /// Escort Service
        "MES" => Mes,
        /// Escort Service (Telephone)
        "MET" => Met,
        /// Manifest Charge
        "MFC" => Mfc,
        /// Manufacturing
        "MFG" => Mfg,
        /// Message Rate Adjustment
        "MGA" => Mga,
        /// Message Charge
        "MGC" => Mgc,
        /// Minimum Charge
        "MIC" => Mic,
        /// Special Mileage Movements
        "MIL" => Mil,
        /// Minimum Guarantee
        "MIN" => Min,
        /// Markup Charge
        "MKU" => Mku,
        /// Minimum Bill of Lading Charge
        "MLB" => Mlb,
        /// Meals
        "MLS" => Mls,
        /// Minimum/Maximum Charge
        "MMC" => Mmc,
        /// Mill Freight
        "MMF" => Mmf,
        /// Market Development Funds
        "MMS" => Mms,
        /// Metropolitan Transit Tax
        "MMT" => Mmt,
        /// Notify Consignee
        "MNC" => Mnc,
        /// Motor Surveillance Service
        "MNS" => Mns,
        /// Miscellaneous Parts Charge
        "MPC" => Mpc,
        /// Marking or Tagging Charge
        "MRK" => Mrk,
        /// Medical Report Charge
        "MRP" => Mrp,
        /// Other Accessorial Service Charge
        "MSC" => Msc,
        /// Miscellaneous Charge
        "MSG" => Msg,
        /// Meter Charge
        "MTR" => Mtr,
        /// Municipal Surcharge
        "MUS" => Mus,
        /// Motor Vehicle Report (MVR) Charge
        "MVR" => Mvr,
        /// Special Motor Surveillance Charge
        "MVS" => Mvs,
        /// Venting Instructions
        "MVT" => Mvt,
        /// Non Generated Freight
        "NAA" => Naa,
        /// New Store Allowance
        "NAB" => Nab,
        /// Nozzle Charge
        "NAL" => Nal,
        /// Order Notify Charge
        "NCH" => Nch,
        /// Next Day Air Service
        "NDA" => Nda,
        /// Non-document Surcharge
        "NDS" => Nds,
        /// Carrier Notification Charge
        "NFY" => Nfy,
        /// N.H.D. Wharfage
        "NHB" => Nhb,
        /// New Store Discount
        "NSD" => Nsd,
        /// New Warehouse Discount
        "NWD" => Nwd,
        /// New York Delivery Charge
        "NYD" => Nyd,
        /// New York Pickup Charge
        "NYP" => Nyp,
        /// O.T.O. Charge
        "OAA" => Oaa,
        /// Overrun Charge
        "OAB" => Oab,
        /// Overtime Loading
        "OAC" => Oac,
        /// Ocean Charges -- Hazardous
        "OCH" => Och,
        /// Over Height Container
        "OCN" => Ocn,
        /// On Call Pickup Service
        "OCP" => Ocp,
        /// Collect on Delivery Deletion Charge
        "ODF" => Odf,
        /// Official Report Charge
        "OFR" => Ofr,
        /// Fumigation
        "OFU" => Ofu,
        /// On Carriage
        "ONC" => Onc,
        /// Option Charge (Color Fabric Office Furniture)
        "OOC" => Ooc,
        /// On Deck Break Bulk Differential
        "OOD" => Ood,
        /// Order-Flat
        "OOF" => Oof,
        /// Optional Charge
        "OPC" => Opc,
        /// Operator Credit
        "ORC" => Orc,
        /// Out of Route Miles
        "ORM" => Orm,
        /// Receipt/Issue Overtime Normal Business Hours Charge
        "ORS" => Ors,
        /// Outside Charge
        "OSC" => Osc,
        /// Optional Software Support for Operational Support Systems
        "OSO" => Oso,
        /// Optional Software Support for Switching Systems
        "OSS" => Oss,
        /// Out of Zone Pickup or Delivery
        "OUT" => Out,
        /// Overnight Service
        "OVN" => Ovn,
        /// Over Dimension
        "OVR" => Ovr,
        /// Over Width Container
        "OWC" => Owc,
        /// Percent of Product
        "PAA" => Paa,
        /// Pump Air Charge
        "PAC" => Pac,
        /// Premium Charge
        "PAD" => Pad,
        /// Premium Transportation
        "PAE" => Pae,
        /// Price Deviation
        "PAF" => Paf,
        /// Professional Fees
        "PAG" => Pag,
        /// Promotional Allowance
        "PAH" => Pah,
        /// Promotional Discount
        "PAI" => Pai,
        /// Pump Charge
        "PAJ" => Paj,
        /// Preparation and Delivery
        "PAK" => Pak,
        /// Parts Adjustment Allowance
        "PAL" => Pal,
        /// Parts Charge
        "PAR" => Par,
        /// Priced Parts Charge
        "PAT" => Pat,
        /// Pickup of Shipments on Saturday, Sunday, and/or Holidays Requiring Absolute Next Day Delivery Charge
        "PAV" => Pav,
        /// Prior billing amount
        "PBA" => Pba,
        /// Permits Bonds Escort Attendant
        "PBE" => Pbe,
        /// Pier Charges Other Than Wharfage
        "PBL" => Pbl,
        /// Protective Service Charge
        "PCH" => Pch,
        /// City Pickup
        "PCT" => Pct,
        /// Prior Delivery Of Bill Charge
        "PDB" => Pdb,
        /// Preloading Charge
        "PDC" => Pdc,
        /// Pickup and Delivery from Storage in Transit
        "PDS" => Pds,
        /// Pickup and Delivery Beyond Service Area Charge
        "PDY" => Pdy,
        /// Pallet Exchange Charge
        "PEC" => Pec,
        /// Penalty Charge
        "PEN" => Pen,
        /// Permit Charge
        "PER" => Per,
        /// Power Factor Adjustment
        "PFA" => Pfa,
        /// Photocopy
        "PHC" => Phc,
        /// Photographs
        "PHG" => Phg,
        /// Pier Pickup and/or Delivery
        "PIR" => Pir,
        /// Packing Surcharge
        "PKS" => Pks,
        /// Insurance Placement Cost Charge
        "PLC" => Plc,
        /// Pallets/Skids/Platforms
        "PLT" => Plt,
        /// Prior Month Credit
        "PMC" => Pmc,
        /// Paint and Materials
        "PMR" => Pmr,
        /// Pickup of Shipments Requiring Same Day Delivery Service Charge
        "PMS" => Pms,
        /// Pickup of Shipments Requiring Same Day Delivery Service and/or Delivery at a Specified Time Charge
        "PMT" => Pmt,
        /// Normal Pump Charge
        "PMU" => Pmu,
        /// Special Pump Charge
        "PMX" => Pmx,
        /// Prior Period Net Adjustment
        "PNA" => Pna,
        /// Piano/Organ Carry
        "POC" => Poc,
        /// Proof of Delivery
        "POD" => Pod,
        /// Police Report Charge
        "POL" => Pol,
        /// Positioning at Origin
        "POS" => Pos,
        /// Per Pound Charge
        "PPC" => Ppc,
        /// Pickup and Delivery Service for Perishables Charge
        "PPD" => Ppd,
        /// Per Item Charge
        "PPH" => Pph,
        /// Pick/Up Allowance
        "PPI" => Ppi,
        /// Precious Metal Content
        "PPM" => Ppm,
        /// Pallet Charge
        "PPN" => Ppn,
        /// Per Order Charge
        "PPO" => Ppo,
        /// Performance Award
        "PPR" => Ppr,
        /// Placement and/or Removal Charge
        "PPS" => Pps,
        /// Parish/County Sales Tax (only)
        "PPT" => Ppt,
        /// Prepaid Usage Allowance
        "PPU" => Ppu,
        /// Prior Balance
        "PRB" => Prb,
        /// Pre-carriage
        "PRC" => Prc,
        /// Pre-carriage Excess
        "PRE" => Pre,
        /// Parking
        "PRK" => Prk,
        /// Prelodge Charge
        "PRL" => Prl,
        /// Premise Use
        "PRM" => Prm,
        /// Prior Period Rebook
        "PRP" => Prp,
        /// Prior Period Reversal
        "PRV" => Prv,
        /// Protective Service - Cold
        "PSC" => Psc,
        /// Preparation of Special Documents
        "PSD" => Psd,
        /// Protective Service Security with Armed Guards
        "PSG" => Psg,
        /// Protective Service - Heat
        "PSH" => Psh,
        /// Protective Service Security
        "PSS" => Pss,
        /// Postage
        "PST" => Pst,
        /// Tobacco Products Report Charge
        "PTC" => Ptc,
        /// Protective Tarp for Security Purposes
        "PTS" => Pts,
        /// Pickup Charge
        "PUC" => Puc,
        /// Pickup and Delivery
        "PUD" => Pud,
        /// Pack and Unpack
        "PUK" => Puk,
        /// Bonded Privately Owned Vehicle Charge
        "PVB" => Pvb,
        /// Detention of Privately Owned Vehicle Charge
        "PVD" => Pvd,
        /// Inoperable Privately Owned Vehicle Charge
        "PVI" => Pvi,
        /// Loading/Unloading of Privately Owned Vehicle Charge
        "PVL" => Pvl,
        /// Privately Owned Vehicle Processing
        "PVP" => Pvp,
        /// Stop-offs for Privately Owned Vehicle Shipment Charge
        "PVS" => Pvs,
        /// Privately Owned Vehicle in Truckaway Service Charge
        "PVT" => Pvt,
        /// Pier Charges - Wharfage
        "PWH" => Pwh,
        /// Pallet Allowance
        "PWT" => Pwt,
        /// Priority Service Charge
        "PYS" => Pys,
        /// Quantity Surcharge
        "QAA" => Qaa,
        /// Quantity Discount
        "QQD" => Qqd,
        /// Rebate
        "RAA" => Raa,
        /// Reclamation, Federal
        "RAB" => Rab,
        /// Reclamation, State
        "RAC" => Rac,
        /// Recovery Allowance
        "RAD" => Rad,
        /// Redistribution Allowance
        "RAE" => Rae,
        /// Rental Deduction
        "RAF" => Raf,
        /// Repack Charge
        "RAG" => Rag,
        /// Retainer
        "RAH" => Rah,
        /// Resellers Discount
        "RAI" => Rai,
        /// Restocking Charge
        "RAJ" => Raj,
        /// Royalties
        "RAK" => Rak,
        /// Roll Rebate
        "RAL" => Ral,
        /// Ramping
        "RAM" => Ram,
        /// No Return Credit Allowance
        "RCA" => Rca,
        /// Repair at buyers expense charge
        "RCB" => Rcb,
        /// Reconsignment Charge
        "RCC" => Rcc,
        /// Reconsign Consignee Charge
        "RCD" => Rcd,
        /// Repair at customer expense charge
        "RCE" => Rce,
        /// Repair at government expense charge
        "RCG" => Rcg,
        /// Redelivery
        "RCL" => Rcl,
        /// Recoopering (at Owner's or Shipper's Expense)
        "RCP" => Rcp,
        /// Reconnect charge
        "RCS" => Rcs,
        /// Reconsign Delivery Charge
        "RDC" => Rdc,
        /// Research & development fee
        "RDF" => Rdf,
        /// Railhead Handling
        "RDH" => Rdh,
        /// Return Cargo Charge
        "REC" => Rec,
        /// Regulatory Fee
        "REE" => Ree,
        /// Refrigeration
        "REF" => Ref,
        /// Regulatory tax
        "REG" => Reg,
        /// Requested Labor Charge
        "REL" => Rel,
        /// Residential Pickup
        "REP" => Rep,
        /// Request Via Canada
        "REQ" => Req,
        /// Residential Delivery
        "RES" => Res,
        /// Returned Load
        "RET" => Ret,
        /// Refund
        "RFD" => Rfd,
        /// Reefer Maintenance
        "RFM" => Rfm,
        /// Regain Allowance
        "RGA" => Rga,
        /// Regain Charge
        "RGC" => Rgc,
        /// Recurring Hardware Maintenance Charge
        "RHM" => Rhm,
        /// Riding Attendant Charge
        "RID" => Rid,
        /// Released Value Charge in Excess of Carrier Maximum Liability Charge
        "RIE" => Rie,
        /// Rail Inspection Service
        "RIS" => Ris,
        /// Retail Loss Allowance
        "RLA" => Rla,
        /// Rents and Leases
        "RLC" => Rlc,
        /// Relinquishment Charge
        "RLQ" => Rlq,
        /// Relocation of Vehicle
        "RLS" => Rls,
        /// Rocky Mountain Bureau 583 Item 1100 Arbitrary Charge
        "RMB" => Rmb,
        /// Return of Empty Container Charge
        "RMC" => Rmc,
        /// Refrigeration/Mechanical Detention
        "RMD" => Rmd,
        /// Return Movement of Pallet Charge
        "RMP" => Rmp,
        /// Rail Surveillance
        "RMS" => Rms,
        /// Roll Out Adjustment
        "ROC" => Roc,
        /// Reduction Prepalletized Cargo
        "RPC" => Rpc,
        /// Reefer Cargo Percent Differential
        "RPD" => Rpd,
        /// Reel Cable
        "RRC" => Rrc,
        /// Reel Deposit
        "RRD" => Rrd,
        /// Refurbishing Charge
        "RRF" => Rrf,
        /// Rental Charge
        "RRN" => Rrn,
        /// Repair Charge
        "RRP" => Rrp,
        /// Regulatory required refund
        "RRR" => Rrr,
        /// Recurring Software Maintenance Charge
        "RSM" => Rsm,
        /// Reshipment
        "RSP" => Rsp,
        /// Restricted Speeds
        "RSS" => Rss,
        /// Respotting
        "RST" => Rst,
        /// Reservations
        "RSV" => Rsv,
        /// Rate Code
        "RTC" => Rtc,
        /// Subject to Cooperative Advertising Allowance
        "SAA" => Saa,
        /// Shipping and Handling
        "SAB" => Sab,
        /// Service Charge (with Cash Discount)
        "SAC" => Sac,
        /// Scrap Allowance
        "SAD" => Sad,
        /// Shrink-Wrap Charge
        "SAE" => Sae,
        /// Special Credit
        "SAF" => Saf,
        /// State Motor Fuel
        "SAG" => Sag,
        /// Stenciling Charge
        "SAH" => Sah,
        /// Super Fund Excise Tax
        "SAI" => Sai,
        /// Surcharge (Dollar Value)
        "SAJ" => Saj,
        /// Surcharge (Percentage)
        "SAK" => Sak,
        /// Stopcharge
        "SAM" => Sam,
        /// State Sales Charge
        "SAN" => San,
        /// Service Assistance Program Surcharge
        "SAP" => Sap,
        /// Shipment Holdover on Weekends Charge
        "SAS" => Sas,
        /// Saturday Pickup or Delivery Charge
        "SAT" => Sat,
        /// Standby Charge
        "SBC" => Sbc,
        /// Sublet
        "SBL" => Sbl,
        /// Special Seal Charge
        "SCC" => Scc,
        /// Special Containers
        "SCD" => Scd,
        /// Small Order Charge
        "SCG" => Scg,
        /// Scale Charge
        "SCL" => Scl,
        /// Second Day Service
        "SCN" => Scn,
        /// Scale Charge Unloading
        "SCU" => Scu,
        /// Special Detention Charge
        "SDC" => Sdc,
        /// Split Delivery
        "SDL" => Sdl,
        /// Special Dromedary Service
        "SDS" => Sds,
        /// Second Day Hundredweight Service
        "SDW" => Sdw,
        /// Special Equipment Charge
        "SEC" => Sec,
        /// Stairs, Elevator, Excess Carry
        "SEE" => See,
        /// Segregating (Sorting)
        "SEG" => Seg,
        /// Self Unloader
        "SEL" => Sel,
        /// Ship Exact Quantity Charge
        "SEQ" => Seq,
        /// Service Charge
        "SER" => Ser,
        /// Security Escort Vehicle Service
        "SEV" => Sev,
        /// Single Factor Origination/Destination
        "SFB" => Sfb,
        /// Stuffing Charge
        "SFC" => Sfc,
        /// Single Factor Origination/Port of Debarkation
        "SFD" => Sfd,
        /// Single Factor Port of Embarkation/Destination
        "SFE" => Sfe,
        /// Special Train Movement
        "SFT" => Sft,
        /// Single Pickup
        "SGL" => Sgl,
        /// Shipment Holdover on Holidays Charge
        "SHH" => Shh,
        /// Shipper Load
        "SHL" => Shl,
        /// State Hazardous Substance Tax
        "SHS" => Shs,
        /// Shipment Holdover on Weekdays Charge
        "SHW" => Shw,
        /// Skirting and Unskirting
        "SKT" => Skt,
        /// Street lamps charge
        "SLC" => Slc,
        /// Slip Sheet Charge
        "SLP" => Slp,
        /// State/Metropolitan Transit Authority Surcharge
        "SMS" => Sms,
        /// Satellite Surveillance Service
        "SNS" => Sns,
        /// Shipment from Non-temp Storage
        "SNT" => Snt,
        /// Stop-off Charge
        "SOC" => Soc,
        /// Stop-off at Pier Charge
        "SOP" => Sop,
        /// Special Allowance
        "SPA" => Spa,
        /// Special Buy
        "SPB" => Spb,
        /// Special Permits
        "SPC" => Spc,
        /// Spreader Charge
        "SPD" => Spd,
        /// Spool Charge
        "SPL" => Spl,
        /// Split Pickup at Pier Charge
        "SPP" => Spp,
        /// Special Freight Supplements
        "SPS" => Sps,
        /// Spotting of Trailer
        "SPT" => Spt,
        /// Split Pickup
        "SPU" => Spu,
        /// Storage
        "SRG" => Srg,
        /// Surveying Routes
        "SRS" => Srs,
        /// Salvage
        "SSA" => Ssa,
        /// Super Bag Charge
        "SSB" => Ssb,
        /// Stripping, Sorting, and Consolidation
        "SSC" => Ssc,
        /// Single Shipment Fee
        "SSF" => Ssf,
        /// Select Charge
        "SSL" => Ssl,
        /// Pole, Wood-service Charge
        "SSO" => Sso,
        /// Shipside Pickup
        "SSP" => Ssp,
        /// Safe Haven Secure Holding Refusal
        "SSR" => Ssr,
        /// Software Support Service
        "SSS" => Sss,
        /// Sales Tax (State and Local)
        "SST" => Sst,
        /// Pole Lashing Equipment (PLE) Surcharge
        "SSU" => Ssu,
        /// Sales and Use Tax
        "SSX" => Ssx,
        /// Conductivity/Anti-static Additive
        "STA" => Sta,
        /// State Surcharge
        "STC" => Stc,
        /// Stop-off at Destination
        "STD" => Std,
        /// Container Stuffing
        "STF" => Stf,
        /// Standard Ground Service
        "STG" => Stg,
        /// Standard Labor Charge
        "STL" => Stl,
        /// Steaming Charge
        "STM" => Stm,
        /// Stowage Charge
        "STO" => Sto,
        /// Stopping in Transit
        "STP" => Stp,
        /// Stop-off at Origination
        "STQ" => Stq,
        /// Storage in Transit
        "STR" => Str,
        /// Special tooling rework charge
        "STW" => Stw,
        /// Special Use
        "SUC" => Suc,
        /// Sufferance Warehouse Charge (Export or Import)
        "SUF" => Suf,
        /// Supervisor Charge
        "SUP" => Sup,
        /// Surcharge
        "SUR" => Sur,
        /// Single Invoice Allowance
        "SVA" => Sva,
        /// Manual Surveillance of Shipment
        "SVL" => Svl,
        /// Storage of Vehicles
        "SVS" => Svs,
        /// Switching Charge
        "SWC" => Swc,
        /// Telephone Charge
        "TAA" => Taa,
        /// Tank Rental
        "TAB" => Tab,
        /// Temporary Allowance
        "TAC" => Tac,
        /// Tax on Transportation
        "TAD" => Tad,
        /// Temporary Voluntary Allowance
        "TAE" => Tae,
        /// Terminal Differential
        "TAF" => Taf,
        /// Testing Charge
        "TAG" => Tag,
        /// Tool Charge
        "TAH" => Tah,
        /// Testing Allowance
        "TAI" => Tai,
        /// Trade In
        "TAJ" => Taj,
        /// Transportation and Setup
        "TAK" => Tak,
        /// Truckload Discount
        "TAL" => Tal,
        /// Tarping Charge
        "TAR" => Tar,
        /// Tax Charge
        "TAX" => Tax,
        /// Governmental Tax
        "TAY" => Tay,
        /// Telegram Chargeback
        "TCB" => Tcb,
        /// Transportation Charge (Minimum Rate)
        "TCM" => Tcm,
        /// Truck Detention
        "TDT" => Tdt,
        /// Terminal Charge
        "TER" => Ter,
        /// Trimming Charge
        "TLC" => Tlc,
        /// Multi-Tank Surveillance Service
        "TMS" => Tms,
        /// Tendering of Multiple Vehicles
        "TMV" => Tmv,
        /// Total Assessorial Charges
        "TOA" => Toa,
        /// TOFC Service Charge
        "TOC" => Toc,
        /// Motor Tow Away Service
        "TOW" => Tow,
        /// Carrier Equipment Pool Charge
        "TPA" => Tpa,
        /// Throughput Container Charge
        "TPC" => Tpc,
        /// Third-Party Service
        "TPS" => Tps,
        /// Travel Charge
        "TRA" => Tra,
        /// Trailer Rental Charge
        "TRC" => Trc,
        /// Travel Expense
        "TRE" => Tre,
        /// Transfer Charge
        "TRF" => Trf,
        /// Termination
        "TRM" => Trm,
        /// Transit
        "TRN" => Trn,
        /// Process in Transit Privilege
        "TRP" => Trp,
        /// Transferred Charges
        "TRS" => Trs,
        /// Thruway Charge
        "TRU" => Tru,
        /// Testing Services Charge
        "TSC" => Tsc,
        /// Tank Surveillance Service
        "TSS" => Tss,
        /// Track Storage
        "TST" => Tst,
        /// Tank Car Allowance
        "TTA" => Tta,
        /// Transportation-Direct Billing
        "TTB" => Ttb,
        /// Trade Discount
        "TTD" => Ttd,
        /// Local Tax
        "TTL" => Ttl,
        /// Tax on Miscellaneous Charges
        "TTM" => Ttm,
        /// Third Party Allowance
        "TTP" => Ttp,
        /// Throughput Allowance
        "TTR" => Ttr,
        /// State Tax
        "TTS" => Tts,
        /// Transportation - Third Party Billing
        "TTT" => Ttt,
        /// Tire Repair and Replace
        "TTU" => Ttu,
        /// Transportation - Vendor Provided
        "TTV" => Ttv,
        /// Turning Charge
        "TUR" => Tur,
        /// Two Door Pickup
        "TWO" => Two,
        /// Under Carriage Furnished By Carrier Charge
        "UFC" => Ufc,
        /// Unloading
        "UND" => Und,
        /// Unloading (Labor Charges)
        "UNL" => Unl,
        /// Usage Plan Detail Charge
        "UPD" => Upd,
        /// Unpacking
        "UPK" => Upk,
        /// Unloading/Reloading Charge
        "URC" => Urc,
        /// Use - Special Type Flat Car
        "USF" => Usf,
        /// U.S. Vehicles
        "USV" => Usv,
        /// Unabsorbed Switching
        "USW" => Usw,
        /// Utilities Disconnect and Connect
        "UTL" => Utl,
        /// Use charge tooling/personnel
        "UTP" => Utp,
        /// Up Charge
        "UUC" => Uuc,
        /// Unsalable Merchandise Allowance
        "UUM" => Uum,
        /// Use Tax
        "UUT" => Uut,
        /// Vendor Freight
        "VAA" => Vaa,
        /// Volume Discount
        "VAB" => Vab,
        /// Van Cleaning
        "VCL" => Vcl,
        /// Voluntary contribution charge
        "VCR" => Vcr,
        /// Excess Mileage for Stop-off Delivery of Personal Vehicles Charge
        "VEX" => Vex,
        /// Vehicles Furnished But Not Used
        "VFN" => Vfn,
        /// Vehicles Inoperable
        "VIS" => Vis,
        /// Virgin Island Transfer Charge
        "VIT" => Vit,
        /// Vehicle Ordered but Not Used
        "VOR" => Vor,
        /// Stop-off Delivery of Personal Vehicles Charge
        "VSO" => Vso,
        /// Vehicles in Truckway
        "VTS" => Vts,
        /// Vehicle Load Allowance
        "VVL" => Vvl,
        /// Vehicle Prep Charge (courtesy delivery)
        "VVP" => Vvp,
        /// War Risk Surcharge
        "WAR" => War,
        /// Wide Area Telephone Service (WATS) Usage Credit
        "WAT" => Wat,
        /// Wharfage - Breakbulk
        "WBB" => Wbb,
        /// Wharfage - Container
        "WCT" => Wct,
        /// Waterfront Delivery Charge
        "WDS" => Wds,
        /// Weather Protection
        "WEA" => Wea,
        /// Weighing Charge (Intermodal)
        "WEI" => Wei,
        /// Wharfage
        "WFG" => Wfg,
        /// Wharfage & Handling
        "WFH" => Wfh,
        /// Wasted/Futile Trip
        "WFT" => Wft,
        /// Warehouse Charge
        "WHC" => Whc,
        /// War Risk Crew Bonus
        "WRB" => Wrb,
        /// Load Weighing Charge
        "WRC" => Wrc,
        /// Empty Weighing Charge
        "WRE" => Wre,
        /// War Risk Insurance
        "WRI" => Wri,
        /// Warehouse Allowance
        "WSA" => Wsa,
        /// Waiting Time
        "WTG" => Wtg,
        /// Waiting Time Service Charge
        "WTM" => Wtm,
        /// Weight Verification Charge
        "WTV" => Wtv,
        /// Wharfage Charge
        "WWC" => Wwc,
        /// Protective Service Rule 25
        "Z01" => Z01,
        /// Protective Service Rule 27
        "Z02" => Z02,
        /// Protective Service Rule 37
        "Z03" => Z03,
        /// Protective Service Rule 75
        "Z04" => Z04,
        /// Protective Service Rule 95
        "Z05" => Z05,
        /// Protective Service Rule 140
        "Z06" => Z06,
        /// Protective Service Rule 160
        "Z07" => Z07,
        /// Protective Service Rule 165
        "Z08" => Z08,
        /// Protective Service Rule 500
        "Z09" => Z09,
        /// Protective Service Rule 510
        "Z10" => Z10,
        /// Protective Service Rule 518
        "Z11" => Z11,
        /// Protective Service Rule 530
        "Z12" => Z12,
        /// Protective Service Rule 531
        "Z13" => Z13,
        /// Protective Service Rule 545
        "Z14" => Z14,
        /// Protective Service Rule 565
        "Z15" => Z15,
        /// Protective Service Rule 570
        "Z16" => Z16,
        /// Protective Service Rule 580
        "Z17" => Z17,
        /// Protective Service Rule 581
        "Z18" => Z18,
        /// Protective Service Rule 705
        "Z19" => Z19,
        /// Protective Service Rule 710
        "Z20" => Z20,
        /// Protective Service Rule 711
        "Z21" => Z21,
        /// Protective Service Rule 712
        "Z22" => Z22,
        /// Protective Service Rule 716
        "Z23" => Z23,
        /// Protective Service Rule 720
        "Z24" => Z24,
        /// Protective Service Rule 725
        "Z25" => Z25,
        /// Protective Service Rule 727
        "Z26" => Z26,
        /// Protective Service Rule 735
        "Z27" => Z27,
        /// Protective Service Rule 740
        "Z28" => Z28,
        /// Protective Service Rule 760
        "Z29" => Z29,
        /// Protective Service Rule 815
        "Z30" => Z30,
        /// Quality Differential
        "Z31" => Z31,
        /// Protective Service Rule 26
        "Z32" => Z32,
        /// Protective Service Rule 715
        "Z33" => Z33,
        /// Protective Service Rule 745
        "Z34" => Z34,
        /// Protective Service Rule 755
        "Z35" => Z35,
        /// First Flight Out
        "ZFF" => Zff,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::code_enum!(
    /// **160** Status Report Request Code
    ///
    /// - Data element: 160
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Status Report Request Code. Code values verified against the Stedi X12 reference.
    E160 {
        /// Not Required
        "N" => N,
        /// Proof of delivery document required
        "P" => P,
        /// Automatic Status Report Requested
        "R" => R,
        /// Automatic proof of delivery document requested
        "S" => S,
    }
);

crate::num_element!(
    /// **170** Tariff Item Part
    ///
    /// - Data element: 170
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 2
    ///
    /// Tariff Item Part.
    E170
);
