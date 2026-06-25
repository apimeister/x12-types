//! X12 data elements 0900-0999.

crate::num_element!(
    /// **954** Percent
    ///
    /// - Data element: 954
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Percent.
    E954
);

crate::code_enum!(
    /// **935** Measurement Significance Code
    ///
    /// - Data element: 935
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Measurement Significance Code. Code values verified against the Stedi X12 reference.
    E935 {
        /// Where Air = 1
        "01" => N01,
        /// Where Butyl Acetate = 1
        "02" => N02,
        /// Approximately
        "03" => N03,
        /// Equal to
        "04" => N04,
        /// Greater than or equal to
        "05" => N05,
        /// Greater than
        "06" => N06,
        /// Less than
        "07" => N07,
        /// Less than or equal to
        "08" => N08,
        /// Where H2O = 1 or Water = 1
        "09" => N09,
        /// Not equal to
        "10" => N10,
        /// Corrected to 60 Degrees Fahrenheit
        "11" => N11,
        /// Where Toluene = 1
        "12" => N12,
        /// Vapor in Air
        "13" => N13,
        /// Vapor in Other Than Air
        "14" => N14,
        /// Standard Temperature and Pressure
        "15" => N15,
        /// Conditions Other Than Standard Temperature and Pressure
        "16" => N16,
        /// In Ethyl Alcohol
        "17" => N17,
        /// In Ethyl Ether
        "18" => N18,
        /// In Water
        "19" => N19,
        /// At 1 Atmosphere Pressure
        "20" => N20,
        /// Where Ether = 1
        "21" => N21,
        /// Actual
        "22" => N22,
        /// Predicted
        "23" => N23,
        /// Air-dried Basis
        "24" => N24,
        /// As-received Basis
        "25" => N25,
        /// Dry Basis
        "26" => N26,
        /// Equilibrium Basis
        "27" => N27,
        /// Moisture and Ash-Free Basis
        "28" => N28,
        /// Oxidizing Atmosphere
        "29" => N29,
        /// Reducing Atmosphere
        "30" => N30,
        /// Calculated
        "31" => N31,
        /// Scaled Weight
        "32" => N32,
        /// Ratchet
        "34" => N34,
        /// Saturated Vapor
        "35" => N35,
        /// Unconditional
        "36" => N36,
        /// Short-term
        "37" => N37,
        /// Time-weighted
        "38" => N38,
        /// Corrected
        "39" => N39,
        /// Uncorrected
        "40" => N40,
        /// Off Peak
        "41" => N41,
        /// On Peak
        "42" => N42,
        /// Intermediate
        "43" => N43,
        /// Average
        "44" => N44,
        /// Per Gallon
        "45" => N45,
        /// Estimated
        "46" => N46,
        /// Minimum
        "47" => N47,
        /// Mist
        "49" => N49,
        /// Predominant
        "50" => N50,
        /// Total
        "51" => N51,
        /// Cost
        "52" => N52,
        /// Tenant
        "53" => N53,
        /// Owner
        "54" => N54,
        /// For Sale
        "55" => N55,
        /// Real Estate Owned or Corporate Owned
        "56" => N56,
        /// Boarded or Blocked Up
        "57" => N57,
        /// Planned
        "58" => N58,
        /// Completed
        "59" => N59,
        /// Sold
        "60" => N60,
        /// Rented
        "61" => N61,
        /// Current
        "62" => N62,
        /// Current List
        "63" => N63,
        /// Effective
        "64" => N64,
        /// List When Sold
        "65" => N65,
        /// Sales
        "66" => N66,
        /// Final List
        "67" => N67,
        /// As Is
        "68" => N68,
        /// As Repaired or Improved
        "69" => N69,
        /// Instantaneous
        "70" => N70,
        /// Low
        "71" => N71,
        /// Low to Good
        "72" => N72,
        /// Low to High
        "73" => N73,
        /// Low to Medium
        "74" => N74,
        /// Low to Moderate
        "75" => N75,
        /// Medium
        "76" => N76,
        /// Medium to Good
        "77" => N77,
        /// Medium to High
        "78" => N78,
        /// Moderate
        "79" => N79,
        /// Moderate to Good
        "80" => N80,
        /// Moderate to High
        "81" => N81,
        /// Moderate to Medium
        "82" => N82,
        /// Good
        "83" => N83,
        /// Good to High
        "84" => N84,
        /// High
        "85" => N85,
        /// Budgeted
        "86" => N86,
        /// Forecast
        "87" => N87,
        /// Adjusted
        "88" => N88,
        /// Allocated
        "89" => N89,
        /// Increasing
        "90" => N90,
        /// Stable
        "91" => N91,
        /// Declining
        "92" => N92,
        /// Previous
        "93" => N93,
        /// Potential
        "94" => N94,
        /// Modeled
        "95" => N95,
        /// Measured
        "96" => N96,
        /// Maximum
        "97" => N97,
        /// Regulated
        "98" => N98,
        /// Spring
        "99" => N99,
        /// Summer On-peak
        "AA" => Aa,
        /// Summer Mid-peak
        "AB" => Ab,
        /// Summer Off-peak
        "AC" => Ac,
        /// Summer Super On-peak
        "AD" => Ad,
        /// Summer Super Off-peak
        "AE" => Ae,
        /// Winter On-peak
        "AF" => Af,
        /// Winter Mid-peak
        "AG" => Ag,
        /// Winter Off-peak
        "AH" => Ah,
        /// Winter Super On-peak
        "AI" => Ai,
        /// Winter Super Off-peak
        "AJ" => Aj,
        /// Summer Day
        "AK" => Ak,
        /// Summer Night
        "AL" => Al,
        /// Winter Day
        "AM" => Am,
        /// Winter Night
        "AN" => An,
        /// Summer
        "AO" => Ao,
        /// Winter
        "AP" => Ap,
        /// Day
        "AQ" => Aq,
        /// Night
        "AR" => Ar,
        /// Peak-2
        "AS" => As,
        /// Peak-3
        "AT" => At,
        /// Peak-4
        "AU" => Au,
        /// Shoulder
        "AV" => Av,
        /// Non Time Related Demand
        "AW" => Aw,
        /// Fall
        "AX" => Ax,
        /// Summer On Peak-2
        "AY" => Ay,
        /// Winter On Peak-2
        "AZ" => Az,
        /// Probable Contamination
        "BA" => Ba,
        /// Not Confirmed
        "BB" => Bb,
        /// Tentative Identification
        "BC" => Bc,
        /// Failed
        "BD" => Bd,
        /// Summer Mid Peak-2
        "BE" => Be,
        /// Winter Mid Peak-2
        "BF" => Bf,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **936** Measurement Attribute Code
    ///
    /// - Data element: 936
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Measurement Attribute Code. Code values verified against the Stedi X12 reference.
    E936 {
        /// Clear
        "01" => N01,
        /// Hazy
        "02" => N02,
        /// Excess
        "03" => N03,
        /// Some
        "04" => N04,
        /// Undetectable
        "05" => N05,
        /// Trace
        "06" => N06,
        /// Yes
        "07" => N07,
        /// Closed
        "08" => N08,
        /// Pass
        "09" => N09,
        /// Present
        "10" => N10,
        /// Gel
        "11" => N11,
        /// OK
        "12" => N12,
        /// Slight
        "13" => N13,
        /// No Good
        "14" => N14,
        /// Marginal
        "15" => N15,
        /// Nil
        "16" => N16,
        /// Oil Free
        "17" => N17,
        /// Open
        "18" => N18,
        /// Free
        "19" => N19,
        /// No
        "20" => N20,
        /// Checked
        "21" => N21,
        /// Fail
        "22" => N22,
        /// Absent
        "23" => N23,
        /// Good
        "24" => N24,
        /// Fair
        "25" => N25,
        /// Poor
        "26" => N26,
        /// Excellent
        "27" => N27,
        /// Bright
        "28" => N28,
        /// To Be Determined
        "29" => N29,
        /// High
        "30" => N30,
        /// Negative
        "31" => N31,
        /// Partial
        "32" => N32,
        /// Variable
        "33" => N33,
        /// Balance
        "40" => N40,
        /// Complete
        "41" => N41,
        /// Low
        "42" => N42,
        /// Not Applicable
        "44" => N44,
        /// Not Determined
        "45" => N45,
        /// Negligible
        "46" => N46,
        /// Moderate
        "48" => N48,
        /// Appreciable
        "49" => N49,
        /// Not Available
        "50" => N50,
        /// Conforming
        "51" => N51,
        /// Non-conforming
        "52" => N52,
        /// Probable Contamination
        "53" => N53,
        /// Tentative Identification
        "54" => N54,
        /// Detected; Not Quantified
        "56" => N56,
        /// Backer
        "BA" => Ba,
        /// Full
        "FL" => Fl,
        /// Not Analyzed
        "NA" => Na,
        /// Not Detected
        "ND" => Nd,
        /// Not Sampled
        "NS" => Ns,
        /// Present and Not Counted
        "PR" => Pr,
        /// First Quality
        "Q1" => Q1,
        /// Second Quality
        "Q2" => Q2,
        /// Too Numerous to Count
        "TA" => Ta,
        /// New
        "TB" => Tb,
        /// Washcoat
        "WS" => Ws,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **955** Tax Jurisdiction Code Qualifier
    ///
    /// - Data element: 955
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Tax Jurisdiction Code Qualifier. Code values verified against the Stedi X12 reference.
    E955 {
        /// Customer defined
        "CD" => Cd,
        /// Taxing Authority Code
        "MB" => Mb,
        /// State or Province
        "SP" => Sp,
        /// State Defined
        "ST" => St,
        /// Vendor defined
        "VD" => Vd,
        /// Vertex
        "VE" => Ve,
    }
);

crate::code_enum!(
    /// **963** Tax Type Code
    ///
    /// - Data element: 963
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Tax Type Code. Code values verified against the Stedi X12 reference.
    E963 {
        /// Stadium Tax
        "AA" => Aa,
        /// Surtax
        "AB" => Ab,
        /// Departure
        "AC" => Ac,
        /// Accommodations Tax
        "AD" => Ad,
        /// Ad Valorem Tax
        "AE" => Ae,
        /// Alcoholic Beverage Tax
        "AF" => Af,
        /// Coin Operated Device Tax
        "AG" => Ag,
        /// Corporate Income Tax
        "AH" => Ah,
        /// Employment Tax
        "AI" => Ai,
        /// Estate Tax
        "AJ" => Aj,
        /// Fee-in-Lieu
        "AK" => Ak,
        /// Gaming Tax
        "AL" => Al,
        /// Gift Tax
        "AM" => Am,
        /// Highway (Fuel) Use Tax
        "AN" => An,
        /// International Fuel Tax Agreement (IFTA) Tax
        "AO" => Ao,
        /// Individual Income Tax
        "AP" => Ap,
        /// Inheritance Tax
        "AQ" => Aq,
        /// Motor Fuel Tax
        "AR" => Ar,
        /// Personal Property Tax
        "AS" => As,
        /// Assessment
        "AT" => At,
        /// Real Property Tax
        "AU" => Au,
        /// Severance Tax
        "AV" => Av,
        /// Solid Waste Tax
        "AW" => Aw,
        /// Tobacco Tax
        "AX" => Ax,
        /// Tourism Tax
        "AY" => Ay,
        /// Transit Tax
        "AZ" => Az,
        /// Unemployment Tax
        "BA" => Ba,
        /// Use Tax
        "BB" => Bb,
        /// Withholding Tax
        "BC" => Bc,
        /// Worker's Compensation Tax
        "BD" => Bd,
        /// Harmonized Sales Tax
        "BE" => Be,
        /// Business Privilege Tax
        "BP" => Bp,
        /// City Tax
        "CA" => Ca,
        /// Threshold Tax
        "CB" => Cb,
        /// Federal Value-added Tax (GST) on Goods
        "CG" => Cg,
        /// City Rental Tax
        "CI" => Ci,
        /// County/Parish Sales Tax
        "CP" => Cp,
        /// County Rental Tax
        "CR" => Cr,
        /// City Sales Tax
        "CS" => Cs,
        /// County Tax
        "CT" => Ct,
        /// Federal Value-added Tax (GST) on Services
        "CV" => Cv,
        /// Default Labor Tax
        "DL" => Dl,
        /// Equipment Tax
        "EQ" => Eq,
        /// Energy Tax
        "ET" => Et,
        /// Environmental Tax
        "EV" => Ev,
        /// FICA Tax
        "F1" => F1,
        /// FICA Medicare Tax
        "F2" => F2,
        /// FICA Social Security Tax
        "F3" => F3,
        /// Federal Tax
        "FD" => Fd,
        /// Fuel Super Fund Tax
        "FF" => Ff,
        /// Federal Income Tax Withholding
        "FI" => Fi,
        /// Fuel L.U.S.T. Tax (Leaking Underground Storage Tank)
        "FL" => Fl,
        /// Franchise Tax
        "FR" => Fr,
        /// Fuel Spill Tax
        "FS" => Fs,
        /// Federal Excise Tax
        "FT" => Ft,
        /// Gross Receipts Tax
        "GR" => Gr,
        /// Goods and Services Tax
        "GS" => Gs,
        /// Public Health and Education Tax
        "HS" => Hs,
        /// Handicap Tax
        "HT" => Ht,
        /// Hazardous Waste Tax
        "HZ" => Hz,
        /// Labor By Trade Tax
        "LB" => Lb,
        /// Local Tax (Not Sales Tax)
        "LO" => Lo,
        /// State and Local Sales Tax
        "LS" => Ls,
        /// Local Sales Tax (All Applicable Sales Taxes by Taxing Authority Below the State Level)
        "LT" => Lt,
        /// Leaky Underground Storage Tank (LUST) Tax (federal)
        "LU" => Lu,
        /// Leaky Underground Storage Tank (LUST) Tax (state)
        "LV" => Lv,
        /// Material Tax
        "MA" => Ma,
        /// Minimum Tax
        "MN" => Mn,
        /// Municipal Tax
        "MP" => Mp,
        /// Miscellaneous State Tax
        "MS" => Ms,
        /// Metropolitan Transit Tax
        "MT" => Mt,
        /// Other Taxes
        "OH" => Oh,
        /// Occupational Tax
        "OT" => Ot,
        /// State or Provincial Tax on Goods
        "PG" => Pg,
        /// State or Provincial Tax on Services
        "PS" => Ps,
        /// State or Provincial Fuel Tax
        "SA" => Sa,
        /// Secondary Percentage Tax
        "SB" => Sb,
        /// School Tax
        "SC" => Sc,
        /// State Excise Tax
        "SE" => Se,
        /// Superfund Tax
        "SF" => Sf,
        /// State and Local Tax
        "SL" => Sl,
        /// State/Provincial Tax
        "SP" => Sp,
        /// State Rental Tax
        "SR" => Sr,
        /// State Tax on Specific Labor
        "SS" => Ss,
        /// State Sales Tax
        "ST" => St,
        /// Sales and Use Tax
        "SU" => Su,
        /// Enhanced 911 - State Excise Tax
        "SX" => Sx,
        /// Pre-threshold Tax
        "T1" => T1,
        /// Post Threshold Tax
        "T2" => T2,
        /// Telecommunications Device for the Deaf (TDD) Service Excise Tax
        "TD" => Td,
        /// Telecommunications Tax
        "TT" => Tt,
        /// All Taxes
        "TX" => Tx,
        /// License Tax
        "UL" => Ul,
        /// Utility Users' Tax
        "UT" => Ut,
        /// Value Added Tax
        "VA" => Va,
        /// Well Service
        "WS" => Ws,
        /// 911-City Tax
        "ZA" => Za,
        /// 911-County Tax
        "ZB" => Zb,
        /// 911-Excise Tax
        "ZC" => Zc,
        /// 911-State Tax
        "ZD" => Zd,
        /// 911-Tax
        "ZE" => Ze,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **983** Hazardous Class Qualifier
    ///
    /// - Data element: 983
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Hazardous Class Qualifier. Code values verified against the Stedi X12 reference.
    E983 {
        /// Primary
        "P" => P,
        /// Secondary
        "S" => S,
    }
);

crate::code_enum!(
    /// **984** Hazardous Material Shipping Name Qualifier
    ///
    /// - Data element: 984
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Hazardous Material Shipping Name Qualifier. Code values verified against the Stedi X12 reference.
    E984 {
        /// Canadian Shipping Name
        "C" => C,
        /// Domestic (United States) Shipping Name
        "D" => D,
        /// International Shipping Name
        "I" => I,
    }
);

crate::code_enum!(
    /// **985** N.O.S. Indicator Code
    ///
    /// - Data element: 985
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// N.O.S. Indicator Code. Code values verified against the Stedi X12 reference.
    E985 {
        /// N.O.S. Regulatory Requirements Apply
        "NOS" => Nos,
    }
);

crate::code_enum!(
    /// **986** Special Commodity Indicator Code
    ///
    /// - Data element: 986
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Special Commodity Indicator Code. Code values verified against the Stedi X12 reference.
    E986 {
        /// Positive Indicator
        "S" => S,
    }
);
