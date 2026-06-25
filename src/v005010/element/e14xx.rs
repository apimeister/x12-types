//! X12 data elements 1400-1499.

crate::num_element!(
    /// **1470** Number
    ///
    /// - Data element: 1470
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 9
    ///
    /// Number.
    E1470
);

crate::code_enum!(
    /// **1401** Proposal Data Detail Identifier Code
    ///
    /// - Data element: 1401
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Proposal Data Detail Identifier Code. Code values verified against the Stedi X12 reference.
    E1401 {
        /// Actual Amount
        "AA" => Aa,
        /// Estimated Amount
        "AB" => Ab,
        /// Negotiated Amount
        "AC" => Ac,
        /// Mixed Amount
        "AD" => Ad,
        /// Estimated Quantity
        "AE" => Ae,
        /// Negotiated Quantity
        "AF" => Af,
        /// Actual Rate
        "AG" => Ag,
        /// Estimated Rate
        "AH" => Ah,
        /// Negotiated Rate
        "AI" => Ai,
        /// Mixed Amounts
        "AJ" => Aj,
        /// Mixed Quantities
        "AK" => Ak,
        /// Mixed Rates
        "AL" => Al,
        /// Actual
        "AM" => Am,
        /// Estimated
        "AN" => An,
        /// Negotiated
        "AO" => Ao,
        /// Mixed
        "AP" => Ap,
        /// Cost Share Amount
        "AQ" => Aq,
    }
);

crate::code_enum!(
    /// **1402** Equipment Attribute Code
    ///
    /// - Data element: 1402
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Equipment Attribute Code. Code values verified against the Stedi X12 reference.
    E1402 {
        /// Atmosphere Controlled
        "ATM" => Atm,
        /// Carbon Dioxide Gas
        "C02" => C02,
        /// Clean Exterior
        "CLE" => Cle,
        /// Door Height Requirement
        "DHT" => Dht,
        /// Door Width Requirement
        "DWI" => Dwi,
        /// Use Flatrack as Platform
        "FAP" => Fap,
        /// Genset Required
        "GEN" => Gen,
        /// Liner Attached
        "LIN" => Lin,
        /// Moisture Content Requirement
        "MCP" => Mcp,
        /// Moisture Free
        "MOI" => Moi,
        /// Nitrogen Gas
        "N2" => N2,
        /// Door Off
        "NDO" => Ndo,
        /// Oil Stain Free
        "NOI" => Noi,
        /// Odor Free
        "ODF" => Odf,
        /// Pre-cooled
        "PCO" => Pco,
        /// Pre-mount Required
        "PRE" => Pre,
        /// Pre-set Temperature
        "PSE" => Pse,
        /// Reefer Used As Dry
        "RAD" => Rad,
        /// Rust Free
        "RUS" => Rus,
        /// Slider Chassis
        "SLI" => Sli,
        /// Smoke Test
        "SMO" => Smo,
        /// Steam Cleaned
        "STE" => Ste,
        /// Substitution Allowed
        "SUB" => Sub,
        /// Survey Required
        "SUR" => Sur,
        /// Timber Treated
        "TBT" => Tbt,
        /// Tectrol Gas
        "TEC" => Tec,
        /// Dual Voltage Required
        "VOD" => Vod,
        /// Single Voltage Required
        "VOS" => Vos,
        /// Year of Manufacture Requirement
        "YEA" => Yea,
    }
);

crate::code_enum!(
    /// **1468** Reason Stopped Work Code
    ///
    /// - Data element: 1468
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Reason Stopped Work Code. Code values verified against the Stedi X12 reference.
    E1468 {
        /// Vacation
        "10" => N10,
        /// Paid Leave of Absence
        "11" => N11,
        /// Unpaid Leave of Absence
        "12" => N12,
        /// Lay off Permanent
        "13" => N13,
        /// Lay off Temporary
        "14" => N14,
        /// Personal Day Off
        "15" => N15,
        /// Terminated
        "16" => N16,
        /// Illness
        "17" => N17,
        /// Injury
        "18" => N18,
        /// Holiday
        "19" => N19,
        /// Discharged Failed to Meet Work Standards
        "D1" => D1,
        /// Discharged Not Qualified
        "D2" => D2,
        /// Failed to Report for Assignment
        "F1" => F1,
        /// Fire, Flood or Natural Disaster
        "F2" => F2,
        /// Lay Off Seasonal
        "L1" => L1,
        /// Lay Off End of Assignment or Contract
        "L2" => L2,
        /// Lay Off Reduced Hours or Partial Employment
        "L3" => L3,
        /// Lay Off Workforce Reduction
        "L4" => L4,
        /// Misconduct Insubordination
        "M1" => M1,
        /// Misconduct Excessive Absence or Tardiness
        "M2" => M2,
        /// Misconduct Violated Rules or Regulations
        "M3" => M3,
        /// Misconduct Dishonesty
        "M4" => M4,
        /// Misconduct Failure to Call or Show
        "M5" => M5,
        /// Labor Dispute
        "O1" => O1,
        /// No Record of Employment
        "O2" => O2,
        /// Still Employed Part-time
        "O3" => O3,
        /// Jury Duty
        "O4" => O4,
        /// Company Relocation
        "O5" => O5,
        /// Still Employed or No Separation
        "O6" => O6,
        /// Other
        "O7" => O7,
        /// Quit
        "Q1" => Q1,
        /// Refusal to Provide Service
        "R1" => R1,
        /// Voluntary Accepted Another Job
        "V1" => V1,
        /// Voluntary Personal or Not Job Related
        "V2" => V2,
        /// Voluntary Left Area or Follow Spouse
        "V3" => V3,
        /// Voluntary No Reason Given
        "V4" => V4,
        /// Voluntary Educational Pursuits
        "V5" => V5,
    }
);

crate::code_enum!(
    /// **1469** Affected Area or Section Code
    ///
    /// - Data element: 1469
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Affected Area or Section Code. Code values verified against the Stedi X12 reference.
    E1469 {
        /// Single Family
        "1" => N1,
        /// Multifamily
        "2" => N2,
        /// Individual Sale
        "3" => N3,
        /// Entire Portfolio for Issues
        "4" => N4,
        /// All Mortgages
        "5" => N5,
        /// Partial Serviced Mortgages
        "6" => N6,
    }
);

crate::code_enum!(
    /// **1476** Language Proficiency Indicator
    ///
    /// - Data element: 1476
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Language Proficiency Indicator. Code values verified against the Stedi X12 reference.
    E1476 {
        /// English Only
        "1" => N1,
        /// Fully English Proficient
        "2" => N2,
        /// Limited English Proficient
        "3" => N3,
        /// Non-English Speaking
        "4" => N4,
        /// Status Unknown
        "5" => N5,
        /// Redesignated Fluent English Proficient
        "6" => N6,
        /// Excellent or Fluent
        "A" => A,
        /// Good
        "B" => B,
        /// Fair
        "C" => C,
        /// Poor
        "D" => D,
        /// Unacceptable
        "E" => E,
    }
);
