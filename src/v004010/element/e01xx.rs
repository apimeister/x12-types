//! X12 data elements 0100-0199.

crate::num_element!(
    /// **100** Exchange Rate
    ///
    /// - Data element: 100
    /// - Type: Numeric (R)
    /// - Length: min 1, max 15
    ///
    /// Exchange Rate.
    E100
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
    /// **118** Amount
    ///
    /// - Data element: 118
    /// - Type: Numeric (R)
    /// - Length: min 1, max 15
    ///
    /// Amount.
    E118
);

crate::num_element!(
    /// **123** Group Control Number
    ///
    /// - Data element: 123
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 9
    ///
    /// Group Control Number.
    E123
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
    /// **115** Port or Terminal Function Code
    ///
    /// - Data element: 115
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Port or Terminal Function Code. Code values verified against the Stedi X12 reference.
    E115 {
        /// Final Port of Discharge (Operational)
        "1" => N1,
        /// Collect Charge Location (Operational)
        "2" => N2,
        /// Customs Office of Manifest Origin
        "3" => N3,
        /// Customs Office of Manifest Destination
        "4" => N4,
        /// Activity Location (Operational)
        "5" => N5,
        /// Origin Rail Intermodal Terminal
        "6" => N6,
        /// Destination Rail Intermodal Terminal
        "7" => N7,
        /// First Optional Port of Discharge
        "8" => N8,
        /// Second Optional Port of Discharge
        "9" => N9,
        /// Place of Acceptance (Operational)
        "A" => A,
        /// Reconsolidation Point (Operational)
        "B" => B,
        /// De-Consolidation Point (Operational)
        "C" => C,
        /// Port of Discharge (Operational)
        "D" => D,
        /// Place of Delivery (Contractual)
        "E" => E,
        /// Freight Payable At (Contractual)
        "F" => F,
        /// Port of Entry (Operational)
        "G" => G,
        /// Port of Exit (Operational)
        "H" => H,
        /// Interim Point (Operational)
        "I" => I,
        /// Bill of Lading Port of Loading (Contractual)
        "J" => J,
        /// Bill of Lading Port of Discharge (Contractual)
        "K" => K,
        /// Port of Loading (Operational)
        "L" => L,
        /// Destination (Operational)
        "M" => M,
        /// Final Destination (Operational)
        "N" => N,
        /// Origin (Operational)
        "O" => O,
        /// Dispatching Pool (Operational)
        "P" => P,
        /// Bill of Lading Origin of Goods (Contractual)
        "Q" => Q,
        /// Place of Receipt (Contractual)
        "R" => R,
        /// Return Pool (Operational)
        "S" => S,
        /// Transshipment Port (Contractual)
        "T" => T,
        /// Prepaid Charge Location (Operational)
        "V" => V,
        /// Bill of Lading Release Office (Operational)
        "W" => W,
        /// Third Optional Port of Discharge
        "X" => X,
        /// Relay Port (Operational)
        "Y" => Y,
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
    /// **132** Release Code
    ///
    /// - Data element: 132
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Release Code. Code values verified against the Stedi X12 reference.
    E132 {
        /// Not Released - Billing Data Only
        "B" => B,
        /// Hold previously released shipment
        "H" => H,
        /// Released with Billing Data
        "R" => R,
        /// Scheduled future release
        "S" => S,
        /// Released with shipment information/billing data
        "T" => T,
        /// Unscheduled release
        "U" => U,
    }
);

crate::code_enum!(
    /// **143** Transaction Set Identifier Code
    ///
    /// - Data element: 143
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Transaction Set Identifier Code. Code values verified against the Stedi X12 reference.
    E143 {
        /// Insurance Plan Description
        "100" => N100,
        /// Name and Address Lists
        "101" => N101,
        /// Associated Data
        "102" => N102,
        /// Abandoned Property Filings
        "103" => N103,
        /// Air Shipment Information
        "104" => N104,
        /// Business Entity Filings
        "105" => N105,
        /// Motor Carrier Rate Proposal
        "106" => N106,
        /// Request for Motor Carrier Rate Proposal
        "107" => N107,
        /// Response to a Motor Carrier Rate Proposal
        "108" => N108,
        /// Vessel Content Details
        "109" => N109,
        /// Air Freight Details and Invoice
        "110" => N110,
        /// Individual Insurance Policy and Client Information
        "111" => N111,
        /// Property Damage Report
        "112" => N112,
        /// Election Campaign and Lobbyist Reporting
        "113" => N113,
        /// Vehicle Shipping Order
        "120" => N120,
        /// Vehicle Service
        "121" => N121,
        /// Vehicle Damage
        "124" => N124,
        /// Multilevel Railcar Load Details
        "125" => N125,
        /// Vehicle Application Advice
        "126" => N126,
        /// Vehicle Baying Order
        "127" => N127,
        /// Dealer Information
        "128" => N128,
        /// Vehicle Carrier Rate Update
        "129" => N129,
        /// Student Educational Record (Transcript)
        "130" => N130,
        /// Student Educational Record (Transcript) Acknowledgment
        "131" => N131,
        /// Human Resource Information
        "132" => N132,
        /// Educational Institution Record
        "133" => N133,
        /// Student Aid Origination Record
        "135" => N135,
        /// Educational Testing and Prospect Request and Report
        "138" => N138,
        /// Student Loan Guarantee Result
        "139" => N139,
        /// Product Registration
        "140" => N140,
        /// Product Service Claim Response
        "141" => N141,
        /// Product Service Claim
        "142" => N142,
        /// Product Service Notification
        "143" => N143,
        /// Student Loan Transfer and Status Verification
        "144" => N144,
        /// Request for Student Educational Record (Transcript)
        "146" => N146,
        /// Response to Request for Student Educational Record (Transcript)
        "147" => N147,
        /// Report of Injury, Illness or Incident
        "148" => N148,
        /// Notice of Tax Adjustment or Assessment
        "149" => N149,
        /// Tax Rate Notification
        "150" => N150,
        /// Electronic Filing of Tax Return Data Acknowledgment
        "151" => N151,
        /// Statistical Government Information
        "152" => N152,
        /// Unemployment Insurance Tax Claim or Charge Information
        "153" => N153,
        /// Secured Interest Filing
        "154" => N154,
        /// Business Credit Report
        "155" => N155,
        /// Notice of Power of Attorney
        "157" => N157,
        /// Tax Jurisdiction Sourcing
        "158" => N158,
        /// Motion Picture Booking Confirmation
        "159" => N159,
        /// Transportation Automatic Equipment Identification
        "160" => N160,
        /// Train Sheet
        "161" => N161,
        /// Transportation Appointment Schedule Information
        "163" => N163,
        /// Revenue Receipts Statement
        "170" => N170,
        /// Court and Law Enforcement Notice
        "175" => N175,
        /// Court Submission
        "176" => N176,
        /// Environmental Compliance Reporting
        "179" => N179,
        /// Return Merchandise Authorization and Notification
        "180" => N180,
        /// Royalty Regulatory Report
        "185" => N185,
        /// Insurance Underwriting Requirements Reporting
        "186" => N186,
        /// Premium Audit Request and Return
        "187" => N187,
        /// Educational Course Inventory
        "188" => N188,
        /// Application for Admission to Educational Institutions
        "189" => N189,
        /// Student Enrollment Verification
        "190" => N190,
        /// Student Loan Pre-Claims and Claims
        "191" => N191,
        /// Grant or Assistance Application
        "194" => N194,
        /// Federal Communications Commission (FCC) License Application
        "195" => N195,
        /// Contractor Cost Data Reporting
        "196" => N196,
        /// Real Estate Title Evidence
        "197" => N197,
        /// Loan Verification Information
        "198" => N198,
        /// Real Estate Settlement Information
        "199" => N199,
        /// Mortgage Credit Report
        "200" => N200,
        /// Residential Loan Application
        "201" => N201,
        /// Secondary Mortgage Market Loan Delivery
        "202" => N202,
        /// Secondary Mortgage Market Investor Report
        "203" => N203,
        /// Motor Carrier Load Tender
        "204" => N204,
        /// Mortgage Note
        "205" => N205,
        /// Real Estate Inspection
        "206" => N206,
        /// Motor Carrier Freight Details and Invoice
        "210" => N210,
        /// Motor Carrier Bill of Lading
        "211" => N211,
        /// Motor Carrier Delivery Trailer Manifest
        "212" => N212,
        /// Motor Carrier Shipment Status Inquiry
        "213" => N213,
        /// Transportation Carrier Shipment Status Message
        "214" => N214,
        /// Motor Carrier Pickup Manifest
        "215" => N215,
        /// Motor Carrier Shipment Pickup Notification
        "216" => N216,
        /// Motor Carrier Loading and Route Guide
        "217" => N217,
        /// Logistics Service Request
        "219" => N219,
        /// Logistics Service Response
        "220" => N220,
        /// Cartage Work Assignment
        "222" => N222,
        /// Consolidators Freight Bill and Invoice
        "223" => N223,
        /// Motor Carrier Summary Freight Bill Manifest
        "224" => N224,
        /// Response to a Cartage Work Assignment
        "225" => N225,
        /// Trailer Usage Report
        "227" => N227,
        /// Equipment Inspection Report
        "228" => N228,
        /// Motor Carrier Package Status
        "240" => N240,
        /// Data Status Tracking
        "242" => N242,
        /// Product Source Information
        "244" => N244,
        /// Real Estate Tax Service Response
        "245" => N245,
        /// Account Assignment/Inquiry and Service/Status
        "248" => N248,
        /// Animal Toxicological Data
        "249" => N249,
        /// Purchase Order Shipment Management Document
        "250" => N250,
        /// Pricing Support
        "251" => N251,
        /// Insurance Producer Administration
        "252" => N252,
        /// Underwriting Information Services
        "255" => N255,
        /// Periodic Compensation
        "256" => N256,
        /// Residential Mortgage Insurance Explanation of Benefits
        "259" => N259,
        /// Application for Mortgage Insurance Benefits
        "260" => N260,
        /// Real Estate Information Request
        "261" => N261,
        /// Real Estate Information Report
        "262" => N262,
        /// Residential Mortgage Insurance Application Response
        "263" => N263,
        /// Mortgage Loan Default Status
        "264" => N264,
        /// Real Estate Title Insurance Services Order
        "265" => N265,
        /// Mortgage or Property Record Change Notification
        "266" => N266,
        /// Individual Life, Annuity and Disability Application
        "267" => N267,
        /// Annuity Activity
        "268" => N268,
        /// Health Care Benefit Coordination Verification
        "269" => N269,
        /// Eligibility, Coverage or Benefit Inquiry
        "270" => N270,
        /// Eligibility, Coverage or Benefit Information
        "271" => N271,
        /// Property and Casualty Loss Notification
        "272" => N272,
        /// Insurance/Annuity Application Status
        "273" => N273,
        /// Healthcare Provider Information
        "274" => N274,
        /// Patient Information
        "275" => N275,
        /// Health Care Claim Status Request
        "276" => N276,
        /// Health Care Information Status Notification
        "277" => N277,
        /// Health Care Services Review Information
        "278" => N278,
        /// Voter Registration Information
        "280" => N280,
        /// Tax or Fee Exemption Certification
        "283" => N283,
        /// Commercial Vehicle Safety Reports
        "284" => N284,
        /// Commercial Vehicle Safety and Credentials Information Exchange
        "285" => N285,
        /// Commercial Vehicle Credentials
        "286" => N286,
        /// Wage Determination
        "288" => N288,
        /// Cooperative Advertising Agreements
        "290" => N290,
        /// Reservation (Booking Request) (Ocean)
        "300" => N300,
        /// Confirmation (Ocean)
        "301" => N301,
        /// Booking Cancellation (Ocean)
        "303" => N303,
        /// Shipping Instructions
        "304" => N304,
        /// Customs Manifest
        "309" => N309,
        /// Freight Receipt and Invoice (Ocean)
        "310" => N310,
        /// Canada Customs Information
        "311" => N311,
        /// Arrival Notice (Ocean)
        "312" => N312,
        /// Shipment Status Inquiry (Ocean)
        "313" => N313,
        /// Status Details (Ocean)
        "315" => N315,
        /// Delivery/Pickup Order
        "317" => N317,
        /// Terminal Information
        "319" => N319,
        /// Terminal Operations and Intermodal Ramp Activity
        "322" => N322,
        /// Vessel Schedule and Itinerary (Ocean)
        "323" => N323,
        /// Vessel Stow Plan (Ocean)
        "324" => N324,
        /// Consolidation of Goods In Container
        "325" => N325,
        /// Consignment Summary List
        "326" => N326,
        /// Customs Status Information
        "350" => N350,
        /// U.S. Customs Carrier General Order Status
        "352" => N352,
        /// Customs Events Advisory Details
        "353" => N353,
        /// U.S. Customs Automated Manifest Archive Status
        "354" => N354,
        /// U.S. Customs Acceptance/Rejection
        "355" => N355,
        /// U.S. Customs Permit to Transfer Request
        "356" => N356,
        /// U.S. Customs In-Bond Information
        "357" => N357,
        /// Customs Consist Information
        "358" => N358,
        /// Carrier Interchange Agreement (Ocean)
        "361" => N361,
        /// Cargo Insurance Advice of Shipment
        "362" => N362,
        /// Rail Carrier Shipment Information
        "404" => N404,
        /// Rail Carrier Freight Details and Invoice
        "410" => N410,
        /// Trailer or Container Repair Billing
        "412" => N412,
        /// Rail Carhire Settlements
        "414" => N414,
        /// Rail Carrier Waybill Interchange
        "417" => N417,
        /// Rail Advance Interchange Consist
        "418" => N418,
        /// Advance Car Disposition
        "419" => N419,
        /// Car Handling Information
        "420" => N420,
        /// Estimated Time of Arrival and Car Scheduling
        "421" => N421,
        /// Equipment Order
        "422" => N422,
        /// Rail Industrial Switch List
        "423" => N423,
        /// Rail Carrier Services Settlement
        "424" => N424,
        /// Rail Waybill Request
        "425" => N425,
        /// Rail Revenue Waybill
        "426" => N426,
        /// Railroad Retirement Activity
        "429" => N429,
        /// Railroad Station Master File
        "431" => N431,
        /// Rail Deprescription
        "432" => N432,
        /// Railroad Reciprocal Switch File
        "433" => N433,
        /// Railroad Mark Register Update Activity
        "434" => N434,
        /// Standard Transportation Commodity Code Master
        "435" => N435,
        /// Locomotive Information
        "436" => N436,
        /// Railroad Junctions and Interchanges Activity
        "437" => N437,
        /// Shipment Weights
        "440" => N440,
        /// Railroad Event Report
        "451" => N451,
        /// Railroad Problem Log Inquiry or Advice
        "452" => N452,
        /// Railroad Service Commitment Advice
        "453" => N453,
        /// Railroad Parameter Trace Registration
        "455" => N455,
        /// Railroad Equipment Inquiry or Advice
        "456" => N456,
        /// Railroad Price Distribution Request or Response
        "460" => N460,
        /// Rail Rate Reply
        "463" => N463,
        /// Rate Request
        "466" => N466,
        /// Rate Docket Journal Log
        "468" => N468,
        /// Railroad Clearance
        "470" => N470,
        /// Rail Route File Maintenance
        "475" => N475,
        /// Ratemaking Action
        "485" => N485,
        /// Rate Docket Expiration
        "486" => N486,
        /// Rate Group Definition
        "490" => N490,
        /// Miscellaneous Rates
        "492" => N492,
        /// Rail Scale Rates
        "494" => N494,
        /// Medical Event Reporting
        "500" => N500,
        /// Vendor Performance Review
        "501" => N501,
        /// Pricing History
        "503" => N503,
        /// Clauses and Provisions
        "504" => N504,
        /// Requisition
        "511" => N511,
        /// Material Obligation Validation
        "517" => N517,
        /// Income or Asset Offset
        "521" => N521,
        /// Material Due-In and Receipt
        "527" => N527,
        /// Logistics Reassignment
        "536" => N536,
        /// Notice of Employment Status
        "540" => N540,
        /// Contract Abstract
        "561" => N561,
        /// Contract Completion Status
        "567" => N567,
        /// Contract Payment Management Report
        "568" => N568,
        /// U.S. Customs Export Shipment Information
        "601" => N601,
        /// Transportation Services Tender
        "602" => N602,
        /// Excavation Communication
        "620" => N620,
        /// Well Information
        "625" => N625,
        /// Maintenance Service Order
        "650" => N650,
        /// Intermodal Group Loading Plan
        "715" => N715,
        /// Request for Routing Instructions
        "753" => N753,
        /// Routing Instructions
        "754" => N754,
        /// Contract Pricing Proposal
        "805" => N805,
        /// Project Schedule Reporting
        "806" => N806,
        /// Invoice
        "810" => N810,
        /// Consolidated Service Invoice/Statement
        "811" => N811,
        /// Credit/Debit Adjustment
        "812" => N812,
        /// Electronic Filing of Tax Return Data
        "813" => N813,
        /// General Request, Response or Confirmation
        "814" => N814,
        /// Cryptographic Service Message
        "815" => N815,
        /// Organizational Relationships
        "816" => N816,
        /// Commission Sales Report
        "818" => N818,
        /// Joint Interest Billing and Operating Expense Statement
        "819" => N819,
        /// Payment Order/Remittance Advice
        "820" => N820,
        /// Financial Information Reporting
        "821" => N821,
        /// Account Analysis
        "822" => N822,
        /// Lockbox
        "823" => N823,
        /// Application Advice
        "824" => N824,
        /// Tax Information Exchange
        "826" => N826,
        /// Financial Return Notice
        "827" => N827,
        /// Debit Authorization
        "828" => N828,
        /// Payment Cancellation Request
        "829" => N829,
        /// Planning Schedule with Release Capability
        "830" => N830,
        /// Application Control Totals
        "831" => N831,
        /// Price/Sales Catalog
        "832" => N832,
        /// Mortgage Credit Report Order
        "833" => N833,
        /// Benefit Enrollment and Maintenance
        "834" => N834,
        /// Health Care Claim Payment/Advice
        "835" => N835,
        /// Procurement Notices
        "836" => N836,
        /// Health Care Claim
        "837" => N837,
        /// Trading Partner Profile
        "838" => N838,
        /// Project Cost Reporting
        "839" => N839,
        /// Request for Quotation
        "840" => N840,
        /// Specifications/Technical Information
        "841" => N841,
        /// Nonconformance Report
        "842" => N842,
        /// Response to Request for Quotation
        "843" => N843,
        /// Product Transfer Account Adjustment
        "844" => N844,
        /// Price Authorization Acknowledgment/Status
        "845" => N845,
        /// Inventory Inquiry/Advice
        "846" => N846,
        /// Material Claim
        "847" => N847,
        /// Material Safety Data Sheet
        "848" => N848,
        /// Response to Product Transfer Account Adjustment
        "849" => N849,
        /// Purchase Order
        "850" => N850,
        /// Asset Schedule
        "851" => N851,
        /// Product Activity Data
        "852" => N852,
        /// Routing and Carrier Instruction
        "853" => N853,
        /// Shipment Delivery Discrepancy Information
        "854" => N854,
        /// Purchase Order Acknowledgment
        "855" => N855,
        /// Ship Notice/Manifest
        "856" => N856,
        /// Shipment and Billing Notice
        "857" => N857,
        /// Shipment Information
        "858" => N858,
        /// Freight Invoice
        "859" => N859,
        /// Purchase Order Change Request - Buyer Initiated
        "860" => N860,
        /// Receiving Advice/Acceptance Certificate
        "861" => N861,
        /// Shipping Schedule
        "862" => N862,
        /// Report of Test Results
        "863" => N863,
        /// Text Message
        "864" => N864,
        /// Purchase Order Change Acknowledgment/Request - Seller Initiated
        "865" => N865,
        /// Production Sequence
        "866" => N866,
        /// Product Transfer and Resale Report
        "867" => N867,
        /// Electronic Form Structure
        "868" => N868,
        /// Order Status Inquiry
        "869" => N869,
        /// Order Status Report
        "870" => N870,
        /// Component Parts Content
        "871" => N871,
        /// Residential Mortgage Insurance Application
        "872" => N872,
        /// Commodity Movement Services
        "873" => N873,
        /// Commodity Movement Services Response
        "874" => N874,
        /// Grocery Products Purchase Order
        "875" => N875,
        /// Grocery Products Purchase Order Change
        "876" => N876,
        /// Manufacturer Coupon Family Code Structure
        "877" => N877,
        /// Product Authorization/De-authorization
        "878" => N878,
        /// Price Information
        "879" => N879,
        /// Grocery Products Invoice
        "880" => N880,
        /// Manufacturer Coupon Redemption Detail
        "881" => N881,
        /// Direct Store Delivery Summary Information
        "882" => N882,
        /// Market Development Fund Allocation
        "883" => N883,
        /// Market Development Fund Settlement
        "884" => N884,
        /// Retail Account Characteristics
        "885" => N885,
        /// Customer Call Reporting
        "886" => N886,
        /// Coupon Notification
        "887" => N887,
        /// Item Maintenance
        "888" => N888,
        /// Promotion Announcement
        "889" => N889,
        /// Deduction Research Report
        "891" => N891,
        /// Item Information Request
        "893" => N893,
        /// Delivery/Return Base Record
        "894" => N894,
        /// Delivery/Return Acknowledgment or Adjustment
        "895" => N895,
        /// Product Dimension Maintenance
        "896" => N896,
        /// Loss or Damage Claim - General Commodities
        "920" => N920,
        /// Loss or Damage Claim - Motor Vehicle
        "924" => N924,
        /// Claim Tracer
        "925" => N925,
        /// Claim Status Report and Tracer Reply
        "926" => N926,
        /// Automotive Inspection Detail
        "928" => N928,
        /// Warehouse Shipping Order
        "940" => N940,
        /// Warehouse Stock Transfer Shipment Advice
        "943" => N943,
        /// Warehouse Stock Transfer Receipt Advice
        "944" => N944,
        /// Warehouse Shipping Advice
        "945" => N945,
        /// Warehouse Inventory Adjustment Advice
        "947" => N947,
        /// Functional Group Totals
        "980" => N980,
        /// Response to a Load Tender
        "990" => N990,
        /// Secured Receipt or Acknowledgment
        "993" => N993,
        /// File Transfer
        "996" => N996,
        /// Functional Acknowledgment
        "997" => N997,
        /// Set Cancellation
        "998" => N998,
        /// Implementation Acknowledgment
        "999" => N999,
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
