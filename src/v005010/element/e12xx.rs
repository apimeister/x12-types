//! X12 data elements 1200-1299.

crate::code_enum!(
    /// **1250** Date Time Period Format Qualifier
    ///
    /// - Data element: 1250
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code indicating the date format, time format, or date and time format.
    E1250 {
        /// Date Expressed in Format CCYYMMDD
        "D8" => D8,
        /// Date Expressed in Format YYMMDD
        "D6" => D6,
        /// Date and Time Expressed in Format CCYYMMDDHHMM
        "DT" => Dt,
        /// Range of Dates Expressed in Format CCYYMMDD-CCYYMMDD
        "RD8" => Rd8,
        /// Time Expressed in Format HHMM
        "TM" => Tm,
    }
);

crate::code_enum!(
    /// **1270** Code List Qualifier Code
    ///
    /// - Data element: 1270
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Code identifying a specific industry code list. Common codes named below.
    E1270 {
        /// Document Identification Code
        "0" => N0,
        /// Free On Board Site Code
        "1" => N1,
        /// Kind of Contract Code
        "3" => N3,
        /// Type of Contract Code
        "4" => N4,
        /// Transaction Status Indicator Code
        "10" => N10,
        /// Payment Type Code
        "20" => N20,
        /// Transportation Mode or Method Code
        "39" => N39,
        /// Health Care Provider Taxonomy
        "68" => N68,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1201** Information Status Code
    ///
    /// - Data element: 1201
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Information Status Code. Code values verified against the Stedi X12 reference.
    E1201 {
        /// Partial
        "A" => A,
        /// Low Industry Score
        "B" => B,
        /// Current
        "C" => C,
        /// Medium Industry Score
        "D" => D,
        /// High Industry Score
        "E" => E,
        /// Latest
        "L" => L,
        /// Oldest
        "O" => O,
        /// Prior
        "P" => P,
        /// Second Most Current
        "S" => S,
        /// Third Most Current
        "T" => T,
    }
);

crate::code_enum!(
    /// **1203** Maintenance Reason Code
    ///
    /// - Data element: 1203
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Maintenance Reason Code. Code values verified against the Stedi X12 reference.
    E1203 {
        /// Divorce
        "01" => N01,
        /// Birth
        "02" => N02,
        /// Death
        "03" => N03,
        /// Retirement
        "04" => N04,
        /// Business Name Change
        "4A" => N4A,
        /// Business Name Correction
        "4B" => N4B,
        /// Physical or Mailing Address Correction
        "4C" => N4C,
        /// Adoption
        "05" => N05,
        /// Strike
        "06" => N06,
        /// Termination of Benefits
        "07" => N07,
        /// Termination of Employment
        "08" => N08,
        /// Consolidation Omnibus Budget Reconciliation Act (COBRA)
        "09" => N09,
        /// Consolidation Omnibus Budget Reconciliation Act (COBRA) Premium Paid
        "10" => N10,
        /// Surviving Spouse
        "11" => N11,
        /// Lay Off
        "12" => N12,
        /// Leave of Absence
        "13" => N13,
        /// Voluntary Withdrawal
        "14" => N14,
        /// Primary Care Provider (PCP) Change
        "15" => N15,
        /// Quit
        "16" => N16,
        /// Fired
        "17" => N17,
        /// Suspended
        "18" => N18,
        /// Sabbatical
        "19" => N19,
        /// Active
        "20" => N20,
        /// Disability
        "21" => N21,
        /// Plan Change
        "22" => N22,
        /// Furloughed
        "23" => N23,
        /// Resigned
        "24" => N24,
        /// Change in Identifying Data Elements
        "25" => N25,
        /// Declined Coverage
        "26" => N26,
        /// Pre-Enrollment
        "27" => N27,
        /// Initial Enrollment
        "28" => N28,
        /// Benefit Selection
        "29" => N29,
        /// Discrimination Test
        "30" => N30,
        /// Legal Separation
        "31" => N31,
        /// Marriage
        "32" => N32,
        /// Personnel Data
        "33" => N33,
        /// Investment Elections and Contribution Rates
        "34" => N34,
        /// Loan Repayment
        "35" => N35,
        /// Contribution or Plan Allocation
        "36" => N36,
        /// Leave of Absence with Benefits
        "37" => N37,
        /// Leave of Absence without Benefits
        "38" => N38,
        /// Lay Off with Benefits
        "39" => N39,
        /// Lay Off without Benefits
        "40" => N40,
        /// Re-enrollment
        "41" => N41,
        /// New Entity
        "42" => N42,
        /// Change of Location
        "43" => N43,
        /// Change of Telephone Number
        "44" => N44,
        /// Went Out of Business
        "45" => N45,
        /// Current Customer Information File in Error
        "46" => N46,
        /// Account Balance Reporting
        "47" => N47,
        /// Fees Processing
        "48" => N48,
        /// Interfund Transfer
        "49" => N49,
        /// Loan Request
        "50" => N50,
        /// Enrollment in Subsequent Benefit Plan
        "51" => N51,
        /// Health Care Facility Change
        "52" => N52,
        /// Name Synonym Add
        "53" => N53,
        /// Sub Location Add
        "54" => N54,
        /// Sub Location Change
        "55" => N55,
        /// Sub Location Expire
        "56" => N56,
        /// Buyout
        "57" => N57,
        /// Merger
        "58" => N58,
        /// Non Payment
        "59" => N59,
        /// Coverage Placed Elsewhere
        "60" => N60,
        /// Duplicate Coverage
        "61" => N61,
        /// Change in Ownership
        "62" => N62,
        /// Business Sold
        "63" => N63,
        /// Underwriting Reason
        "64" => N64,
        /// No Employees, Exposure or Operations
        "65" => N65,
        /// Revocation of Voluntary Market Acceptance
        "66" => N66,
        /// Include Primary Business Management
        "67" => N67,
        /// Exclude Primary Business Management
        "68" => N68,
        /// Failure to Pay Deductible
        "69" => N69,
        /// Misrepresented Information
        "70" => N70,
        /// Rewritten
        "71" => N71,
        /// Adding a Jurisdiction
        "72" => N72,
        /// Deleting a Jurisdiction
        "73" => N73,
        /// Occupational Illness
        "75" => N75,
        /// Change Insured Federal Employer Identification Number (FEIN)
        "76" => N76,
        /// Change Employer Federal Employer Identification Number (FEIN)
        "77" => N77,
        /// Change Employer Unemployment Insurance (UI) Code
        "78" => N78,
        /// Change Policy Number
        "79" => N79,
        /// Modification without a Specific Operating Unit Location in Jurisdiction
        "80" => N80,
        /// Change Policy Effective Date
        "81" => N81,
        /// Change Policy Expiration Date
        "82" => N82,
        /// Change Insurer Federal Employer Identification Number (FEIN)
        "83" => N83,
        /// No Eligible Employees
        "84" => N84,
        /// Reinstatement - Canceled in Error
        "85" => N85,
        /// Change in Insured Information
        "86" => N86,
        /// Change in Employer Information
        "87" => N87,
        /// Parent Identification Change
        "88" => N88,
        /// Change to Expiration Date
        "89" => N89,
        /// Phone Verify Only
        "90" => N90,
        /// Name Synonym Delete
        "91" => N91,
        /// Duplicate Entry on Customer Identification File
        "92" => N92,
        /// Removal of the Customer Identification File Merge ID
        "93" => N93,
        /// Removal of the Customer Identification File Buyout ID
        "94" => N94,
        /// Removal of the Customer Identification File in Error ID
        "95" => N95,
        /// Re-activation of an Out-of-Business Customer
        "96" => N96,
        /// Sub-location Reinstatement
        "97" => N97,
        /// Dissatisfaction with Office Staff
        "AA" => Aa,
        /// Dissatisfaction with Medical Care/Services Rendered
        "AB" => Ab,
        /// Inconvenient Office Location
        "AC" => Ac,
        /// Dissatisfaction with Office Hours
        "AD" => Ad,
        /// Unable to Schedule Appointments in a Timely Manner
        "AE" => Ae,
        /// Dissatisfaction with Physician's Referral Policy
        "AF" => Af,
        /// Less Respect and Attention Time Given than to Other Patients
        "AG" => Ag,
        /// Patient Moved to a New Location
        "AH" => Ah,
        /// No Reason Given
        "AI" => Ai,
        /// Appointment Times not Met in a Timely Manner
        "AJ" => Aj,
        /// Algorithm Assigned Benefit Selection
        "AL" => Al,
        /// Member Benefit Selection
        "EC" => Ec,
        /// Became Medical Only
        "XB" => Xb,
        /// Indemnity
        "XI" => Xi,
        /// Became Lost Time
        "XL" => Xl,
        /// Medical Only
        "XM" => Xm,
        /// Notification Only
        "XN" => Xn,
        /// Transfer
        "XT" => Xt,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1254** Immunization Status Code
    ///
    /// - Data element: 1254
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Immunization Status Code. Code values verified against the Stedi X12 reference.
    E1254 {
        /// First Inoculation
        "1" => N1,
        /// Second Inoculation
        "2" => N2,
        /// Third Inoculation
        "3" => N3,
        /// Fourth Inoculation
        "4" => N4,
        /// Fifth Inoculation
        "5" => N5,
        /// Sixth Inoculation
        "6" => N6,
        /// Seventh Inoculation
        "7" => N7,
        /// Eighth Inoculation
        "8" => N8,
        /// Ninth Inoculation
        "9" => N9,
        /// Medical Exemption
        "10" => N10,
        /// Personal Exemption
        "11" => N11,
        /// Religious Exemption
        "12" => N12,
        /// Had the Disease
        "13" => N13,
        /// Has Not Had the Disease
        "14" => N14,
    }
);

crate::code_enum!(
    /// **1257** Special Program Category Code
    ///
    /// - Data element: 1257
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Special Program Category Code. Code values verified against the Stedi X12 reference.
    E1257 {
        /// Learning Disabled
        "1" => N1,
        /// Speech Impaired
        "2" => N2,
        /// Communication Disordered or Impaired
        "3" => N3,
        /// Educable Mentally Retarded
        "4" => N4,
        /// Trainable Mentally Retarded
        "5" => N5,
        /// Severely or Profoundly Mentally Retarded
        "6" => N6,
        /// Emotionally Disturbed
        "7" => N7,
        /// Emotionally Vulnerable
        "8" => N8,
        /// Socially Maladjusted
        "9" => N9,
        /// Behaviorally Disordered
        "10" => N10,
        /// Hard of Hearing or Hearing Impaired
        "11" => N11,
        /// Deaf
        "12" => N12,
        /// Visually Handicapped
        "13" => N13,
        /// Blind
        "14" => N14,
        /// Deaf and Blind
        "15" => N15,
        /// Multi-handicapped
        "16" => N16,
        /// Orthopedically Impaired
        "17" => N17,
        /// Other Health Impaired
        "18" => N18,
        /// Traumatic Brain Injury
        "19" => N19,
        /// Autistic
        "20" => N20,
        /// Early Childhood
        "21" => N21,
        /// Developmentally Delayed
        "22" => N22,
        /// Other Special Education Program
        "27" => N27,
        /// Other Program (not Special Education)
        "28" => N28,
        /// Not Determined
        "99" => N99,
    }
);

crate::code_enum!(
    /// **1262** Loan Type Code
    ///
    /// - Data element: 1262
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Loan Type Code. Code values verified against the Stedi X12 reference.
    E1262 {
        /// Subsidized Federal Stafford
        "1" => N1,
        /// Federal Supplemental Loans for Students (SLS)
        "2" => N2,
        /// Federal PLUS Loan to Parents
        "3" => N3,
        /// Unsubsidized Federal Stafford
        "4" => N4,
        /// Federal Consolidated Loan
        "5" => N5,
        /// Residential Loan
        "6" => N6,
        /// Personal Loan
        "7" => N7,
        /// Automobile
        "8" => N8,
        /// Unsecured
        "9" => N9,
        /// Secured
        "10" => N10,
        /// Partially Secured
        "11" => N11,
        /// Home Improvement
        "12" => N12,
        /// FHA Home Improvement
        "13" => N13,
        /// Installment Sales Contract
        "14" => N14,
        /// Charge Account
        "15" => N15,
        /// Real Estate, Specific Type Unknown
        "16" => N16,
        /// Secured by Co-signer
        "17" => N17,
        /// Business
        "18" => N18,
        /// Recreational
        "19" => N19,
        /// Educational
        "20" => N20,
        /// Lease
        "21" => N21,
        /// Co-maker
        "22" => N22,
        /// Check Credit or Line of Credit
        "23" => N23,
        /// FHA Co-maker (Not Borrower)
        "24" => N24,
        /// Mobile Home
        "25" => N25,
        /// Credit Card
        "26" => N26,
        /// FHA Real Estate Mortgage
        "27" => N27,
        /// Note Loan
        "28" => N28,
        /// Note Loan with Co-maker
        "29" => N29,
        /// Household Good (Secured)
        "30" => N30,
        /// Household Goods and Other Collateral Auto
        "31" => N31,
        /// Veterans Administration Real Estate Mortgage
        "32" => N32,
        /// Conventional Real Estate Mortgage
        "33" => N33,
        /// Real Estate Mortgage without Other Collateral
        "34" => N34,
        /// Rental Agreement
        "35" => N35,
        /// Summary of Accounts with Same Status
        "36" => N36,
        /// Unknown Loan Type
        "37" => N37,
        /// Debt Counseling Service
        "38" => N38,
        /// Employment
        "39" => N39,
        /// Combined Credit Plan
        "40" => N40,
        /// Debit Card
        "41" => N41,
        /// Credit Line - Secured
        "42" => N42,
        /// Collection Attorney
        "43" => N43,
        /// Insurance Claims
        "44" => N44,
        /// Child Support
        "45" => N45,
        /// Government Unsecured Guarantee Loan
        "46" => N46,
        /// Government Secured Guarantee Loan
        "47" => N47,
        /// Government Secured Direct Loan
        "48" => N48,
        /// Government Grant
        "49" => N49,
        /// Government Overpayment
        "50" => N50,
        /// Government Fine
        "51" => N51,
        /// Government Fee for Service
        "52" => N52,
        /// Government Employee Advance
        "53" => N53,
        /// Government Miscellaneous Debt
        "54" => N54,
        /// Government Benefit
        "55" => N55,
        /// Returned Check
        "56" => N56,
        /// Installment Loan
        "57" => N57,
        /// Fully Amortized First
        "58" => N58,
        /// Balloon First
        "59" => N59,
        /// Subordinate Mortgage
        "60" => N60,
        /// Home Equity Line of Credit
        "61" => N61,
        /// Wrap Around
        "62" => N62,
        /// Tax
        "63" => N63,
        /// Mortgage
        "64" => N64,
        /// Rehabilitation
        "65" => N65,
        /// One to Four Servicing
        "66" => N66,
        /// Special Service
        "67" => N67,
        /// Other
        "90" => N90,
        /// Refinance
        "91" => N91,
        /// Sale
        "92" => N92,
        /// Wholesale
        "93" => N93,
    }
);

crate::code_enum!(
    /// **1280** Direction Identifier Code
    ///
    /// - Data element: 1280
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Direction Identifier Code. Code values verified against the Stedi X12 reference.
    E1280 {
        /// Northeast
        "A" => A,
        /// Northwest
        "B" => B,
        /// Southeast
        "C" => C,
        /// Southwest
        "D" => D,
        /// East
        "E" => E,
        /// North Northwest
        "F" => F,
        /// South Southeast
        "G" => G,
        /// South Southwest
        "H" => H,
        /// North Northeast
        "I" => I,
        /// East Northeast
        "J" => J,
        /// East Southeast
        "K" => K,
        /// West Northwest
        "L" => L,
        /// West Southwest
        "M" => M,
        /// North
        "N" => N,
        /// South
        "S" => S,
        /// West
        "W" => W,
    }
);

crate::code_enum!(
    /// **1292** Returns Disposition Code
    ///
    /// - Data element: 1292
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Returns Disposition Code. Code values verified against the Stedi X12 reference.
    E1292 {
        /// Consumer Return to Vendor
        "CR" => Cr,
        /// Dispose
        "DI" => Di,
        /// Keep with an Allowance
        "KA" => Ka,
        /// Keep and Repair
        "KR" => Kr,
        /// Manufacturer Warranty Service
        "MW" => Mw,
        /// Partially Authorized
        "PA" => Pa,
        /// Return with Authorization Number
        "RA" => Ra,
        /// Request Denied
        "RD" => Rd,
        /// Return for Factory Repair
        "RF" => Rf,
        /// Return without Authorization Number
        "RN" => Rn,
        /// Return Authorization Pending
        "RP" => Rp,
        /// Ship to Third Party
        "RT" => Rt,
        /// Ship to Third Party for Charitable Contribution
        "SC" => Sc,
        /// Ship to Third Party for Disposal
        "SD" => Sd,
    }
);

crate::code_enum!(
    /// **1293** Return Request Reason Code
    ///
    /// - Data element: 1293
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Return Request Reason Code. Code values verified against the Stedi X12 reference.
    E1293 {
        /// Beyond Code Date
        "BC" => Bc,
        /// Customer Ordering Error
        "CO" => Co,
        /// Color Variance
        "CV" => Cv,
        /// Damaged Product
        "DA" => Da,
        /// Discontinued Product
        "DI" => Di,
        /// Defective Merchandise or Store Inspection
        "DM" => Dm,
        /// Defective Packaging
        "DP" => Dp,
        /// Defective Merchandise or Returned by Consumer
        "DR" => Dr,
        /// Damaged In Transit
        "DT" => Dt,
        /// Excess Inventory
        "EI" => Ei,
        /// End of Season
        "EO" => Eo,
        /// Excessive Wear
        "EW" => Ew,
        /// Label Problem
        "LP" => Lp,
        /// Mark Downs
        "MD" => Md,
        /// Not as Expected
        "NA" => Na,
        /// Outdated Packaging
        "OP" => Op,
        /// Price Error
        "PE" => Pe,
        /// Poor Fit
        "PF" => Pf,
        /// Product Recall
        "PR" => Pr,
        /// Product Spoiled
        "PS" => Ps,
        /// Poor Workmanship
        "PW" => Pw,
        /// Recall
        "RE" => Re,
        /// Short-Dated Product
        "SD" => Sd,
        /// Samples
        "SM" => Sm,
        /// Shipped past Cancel Date
        "SP" => Sp,
        /// Stock Reduction Agreement
        "SR" => Sr,
        /// Style Problem
        "ST" => St,
        /// Termination
        "TE" => Te,
        /// Wrong Goods or Not Ordered
        "WG" => Wg,
    }
);

crate::code_enum!(
    /// **1294** Return Response Reason Code
    ///
    /// - Data element: 1294
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Return Response Reason Code. Code values verified against the Stedi X12 reference.
    E1294 {
        /// Excessive Wear and Tear
        "EW" => Ew,
        /// Freight or Retailer Damage
        "FR" => Fr,
        /// Item not Defective
        "IN" => In,
        /// Item as Ordered
        "IO" => Io,
        /// More Information Requested
        "MI" => Mi,
        /// No Record of Original Sale
        "NR" => Nr,
        /// Out of Season or Discontinued Line Item
        "OS" => Os,
        /// Pricing or Cost Difference
        "PC" => Pc,
        /// Picture Requested
        "PR" => Pr,
        /// Quantity Difference
        "QD" => Qd,
        /// Repair or Refurbish
        "RR" => Rr,
        /// Return Time Limit Exceeded or Beyond Warranty Period
        "RT" => Rt,
        /// Sample Requested
        "SR" => Sr,
        /// Unidentifiable Item
        "UI" => Ui,
    }
);
