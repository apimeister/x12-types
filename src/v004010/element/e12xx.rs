//! X12 data elements 1200-1299.

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
    /// **1264** Delayed Repayment Reason Code
    ///
    /// - Data element: 1264
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Delayed Repayment Reason Code. Code values verified against the Stedi X12 reference.
    E1264 {
        /// Full-Time Student
        "1" => N1,
        /// At Least Half-time Student
        "2" => N2,
        /// Graduate Fellowship
        "3" => N3,
        /// Internship or Residency
        "4" => N4,
        /// Rehabilitation Training
        "5" => N5,
        /// Armed Forces or Public Health Services or National Oceanic and Atmospheric Administration
        "6" => N6,
        /// Peace Corps
        "9" => N9,
        /// Tax-Exempt Organization Volunteer
        "11" => N11,
        /// Teacher Shortage
        "12" => N12,
        /// Temporarily Totally Disabled
        "13" => N13,
        /// Parental Leave
        "14" => N14,
        /// Mother of Preschool Child Re-entering Workforce
        "15" => N15,
        /// Unemployment - 36 months
        "16" => N16,
        /// Unemployment - 24 months
        "17" => N17,
        /// Persian Gulf War Active Duty
        "20" => N20,
        /// Pre-Bankruptcy
        "21" => N21,
        /// Bankruptcy
        "22" => N22,
        /// Special Case
        "23" => N23,
        /// Economic Hardship
        "24" => N24,
        /// Incarceration
        "25" => N25,
        /// Intent to Re-Enroll in School
        "26" => N26,
        /// ACTION Programs
        "27" => N27,
        /// Pre-deferment Delinquency
        "28" => N28,
        /// Late Repayment Start
        "29" => N29,
        /// Medical Problems
        "30" => N30,
        /// Interval between Disbursements
        "31" => N31,
        /// National Emergency (Military or Natural)
        "32" => N32,
        /// Due Diligence Cure
        "33" => N33,
        /// Conform Differing Due Dates
        "34" => N34,
        /// National Community Service
        "35" => N35,
        /// Education Loan Debt Burden
        "36" => N36,
        /// Revoked Deferment
        "37" => N37,
    }
);

crate::code_enum!(
    /// **1265** Interest Payment Code
    ///
    /// - Data element: 1265
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Interest Payment Code. Code values verified against the Stedi X12 reference.
    E1265 {
        /// In-school Interest Paid Monthly by Borrower
        "1" => N1,
        /// In-school Interest Capitalized Quarterly
        "2" => N2,
        /// In-school Interest Paid Quarterly by Borrower
        "3" => N3,
        /// In-school Interest Capitalized Annually
        "4" => N4,
        /// In-school Interest Capitalized at Repayment
        "5" => N5,
        /// In Advance
        "6" => N6,
        /// In Arrears
        "7" => N7,
    }
);

crate::code_enum!(
    /// **1266** Major Course of Study
    ///
    /// - Data element: 1266
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Major Course of Study. Code values verified against the Stedi X12 reference.
    E1266 {
        /// Agriculture
        "1" => N1,
        /// Architecture
        "2" => N2,
        /// Art
        "3" => N3,
        /// Biological Sciences
        "4" => N4,
        /// Business or Commerce
        "5" => N5,
        /// Communications
        "6" => N6,
        /// Community Service
        "7" => N7,
        /// Computer Science
        "8" => N8,
        /// Education
        "9" => N9,
        /// Engineering
        "10" => N10,
        /// Liberal Arts
        "11" => N11,
        /// Health Professions
        "12" => N12,
        /// Physical Sciences
        "13" => N13,
        /// Social Sciences
        "14" => N14,
        /// Trade, Industrial, Technical
        "15" => N15,
        /// Other or Undecided
        "16" => N16,
    }
);

crate::code_enum!(
    /// **1267** Dependency Status Code
    ///
    /// - Data element: 1267
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Dependency Status Code. Code values verified against the Stedi X12 reference.
    E1267 {
        /// Dependent Undergraduate
        "1" => N1,
        /// Independent Undergraduate
        "2" => N2,
        /// Dependent Graduate
        "3" => N3,
        /// Independent Graduate
        "4" => N4,
        /// Dependent (Other)
        "5" => N5,
        /// Independent (Other)
        "6" => N6,
    }
);

crate::code_enum!(
    /// **1268** Applicant Type Code
    ///
    /// - Data element: 1268
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Applicant Type Code. Code values verified against the Stedi X12 reference.
    E1268 {
        /// Student
        "1" => N1,
        /// Parent
        "2" => N2,
        /// Legal Guardian
        "3" => N3,
    }
);

crate::code_enum!(
    /// **1275** Fumigated/Cleaned Indicator
    ///
    /// - Data element: 1275
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Fumigated/Cleaned Indicator. Code values verified against the Stedi X12 reference.
    E1275 {
        /// Both Fumigated and Cleaned
        "B" => B,
        /// Cleaned
        "C" => C,
        /// Fumigated
        "F" => F,
        /// Not Fumigated or Cleaned
        "N" => N,
    }
);

crate::code_enum!(
    /// **1277** Canadian Wheat Board (CWB) Marketing Class Code
    ///
    /// - Data element: 1277
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Canadian Wheat Board (CWB) Marketing Class Code. Code values verified against the Stedi X12 reference.
    E1277 {
        /// Board Grain
        "0" => N0,
        /// Non-Board Grain
        "1" => N1,
        /// Seed Purchase
        "4" => N4,
        /// Contract
        "5" => N5,
    }
);

crate::code_enum!(
    /// **1278** Canadian Wheat Board (CWB) Marketing Class Type Code
    ///
    /// - Data element: 1278
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Canadian Wheat Board (CWB) Marketing Class Type Code. Code values verified against the Stedi X12 reference.
    E1278 {
        /// Consigned
        "C" => C,
        /// Street
        "S" => S,
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
