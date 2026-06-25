//! X12 data elements 1100-1199.

crate::code_enum!(
    /// **1104** Name Component Qualifier
    ///
    /// - Data element: 1104
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Name Component Qualifier. Code values verified against the Stedi X12 reference.
    E1104 {
        /// Prefix
        "01" => N01,
        /// First Name
        "02" => N02,
        /// First Middle Name
        "03" => N03,
        /// Second Middle Name
        "04" => N04,
        /// Last Name
        "05" => N05,
        /// First Initial
        "06" => N06,
        /// First Middle Initial
        "07" => N07,
        /// Second Middle Initial
        "08" => N08,
        /// Suffix
        "09" => N09,
        /// Generation
        "10" => N10,
        /// Doing Business As (DBA) or Trading As (T/A)
        "11" => N11,
        /// Combined (Unstructured) Name
        "12" => N12,
        /// Combined Name and Account Number
        "13" => N13,
        /// Name of an agency
        "14" => N14,
        /// Maiden or former name
        "15" => N15,
        /// Composite name (used if the name cannot be broken into separate parts, formatted with last name sent first)
        "16" => N16,
        /// Middle Names
        "17" => N17,
        /// Preferred First Name or Nickname
        "18" => N18,
        /// Corporation
        "19" => N19,
        /// Corporation Suffix
        "20" => N20,
        /// Professional Title
        "21" => N21,
        /// Organization Name
        "22" => N22,
    }
);

crate::code_enum!(
    /// **1106** Address Component Qualifier
    ///
    /// - Data element: 1106
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Address Component Qualifier. Code values verified against the Stedi X12 reference.
    E1106 {
        /// Street Number
        "01" => N01,
        /// Street Name
        "02" => N02,
        /// Prefix Direction
        "03" => N03,
        /// Suffix Direction
        "04" => N04,
        /// P.O. Box Number
        "05" => N05,
        /// Rural Route Number
        "06" => N06,
        /// City Name
        "07" => N07,
        /// State Name
        "08" => N08,
        /// State Postal Code
        "09" => N09,
        /// Zip Code (5 Digit)
        "10" => N10,
        /// Zip Code Addendum (4 Digit)
        "11" => N11,
        /// Building Name
        "12" => N12,
        /// Apartment Number
        "13" => N13,
        /// Suite Number
        "14" => N14,
        /// Unstructured Street Address
        "15" => N15,
        /// Unstructured City and State and Zip Code
        "16" => N16,
        /// Unstructured City and State
        "17" => N17,
        /// Unstructured Mailing Address
        "18" => N18,
        /// Zip Code (9 Digit)
        "19" => N19,
        /// County
        "20" => N20,
        /// Subdivision
        "21" => N21,
        /// Map Coordinates and Directions
        "22" => N22,
        /// Continent
        "23" => N23,
        /// State or Province Abbreviation
        "24" => N24,
        /// Township
        "25" => N25,
        /// Municipality
        "26" => N26,
        /// Sub-barrio
        "27" => N27,
        /// Association Name
        "28" => N28,
        /// Parking Unit Number
        "29" => N29,
        /// Pier
        "30" => N30,
        /// Wing
        "31" => N31,
        /// Floor
        "32" => N32,
        /// Driveway
        "33" => N33,
        /// Lot
        "34" => N34,
        /// Room
        "35" => N35,
        /// Slip
        "36" => N36,
        /// Unit
        "37" => N37,
        /// Property Address
        "38" => N38,
        /// Unstructured Property
        "39" => N39,
        /// Street Suffix
        "40" => N40,
        /// Country
        "41" => N41,
        /// House Number Prefix
        "42" => N42,
        /// House Number Suffix
        "43" => N43,
        /// Drawer Number
        "51" => N51,
        /// Foreign Region or Province
        "52" => N52,
        /// Postal District
        "53" => N53,
        /// Street Number Alpha
        "54" => N54,
        /// Province
        "56" => N56,
        /// Cross Street
        "57" => N57,
        /// International Postal Code
        "58" => N58,
        /// Street Number Low
        "59" => N59,
        /// Street Number High
        "60" => N60,
        /// Street Number Fraction
        "61" => N61,
        /// Street Name Suffix
        "62" => N62,
        /// Secondary Unit Identifier
        "63" => N63,
        /// Secondary Unit Number
        "64" => N64,
        /// ZIP Code Delivery Point Addendum
        "65" => N65,
        /// ZIP Code Delivery Point BarCode (DPBC) Checkdigit
        "66" => N66,
        /// Postal Service Carrier Route
        "67" => N67,
        /// Postal Service Carrier Route Line of Travel (LOT)
        "68" => N68,
        /// Postal Service Address Change Service (ACS) Keyline
        "69" => N69,
        /// Congressional District Code
        "71" => N71,
        /// Zip Code (11 digit)
        "74" => N74,
        /// International Organization for Standardization (ISO) Country Code
        "79" => N79,
        /// Street Name Prefix
        "80" => N80,
        /// Private Mail Box (PMB)
        "83" => N83,
        /// Urbanization
        "85" => N85,
        /// Access Customer Terminal Location (ACTL)
        "90" => N90,
        /// Additional Point of Termination (APOT)
        "91" => N91,
        /// Local Service Termination (LST)
        "92" => N92,
        /// Specific Point of Interface (XPOI)
        "93" => N93,
        /// Section
        "94" => N94,
        /// Range
        "96" => N96,
        /// Geographical Area
        "97" => N97,
        /// High Even Street Number
        "AA" => Aa,
        /// High Odd Street Number
        "AB" => Ab,
        /// Low Even Street Number
        "AC" => Ac,
        /// Low Odd Street Number
        "AD" => Ad,
        /// Secondary Address High
        "AE" => Ae,
        /// Secondary Address Low
        "AF" => Af,
        /// High Even Secondary Address
        "AG" => Ag,
        /// High Odd Secondary Address
        "AH" => Ah,
        /// Low Even Secondary Address
        "AI" => Ai,
        /// Low Odd Secondary Address
        "AJ" => Aj,
        /// Building Number
        "AK" => Ak,
    }
);

crate::code_enum!(
    /// **1107** Name Type Code
    ///
    /// - Data element: 1107
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Name Type Code. Code values verified against the Stedi X12 reference.
    E1107 {
        /// Given Name (Name at Birth)
        "01" => N01,
        /// Current Legal
        "02" => N02,
        /// Alias
        "03" => N03,
        /// Name of Record
        "04" => N04,
        /// Previous Name
        "05" => N05,
        /// Name of Record Requestor
        "06" => N06,
        /// Married Name
        "07" => N07,
        /// Professional Name
        "08" => N08,
        /// Doing Business As (DBA) and Trading As (T/A)
        "11" => N11,
        /// Spouse
        "12" => N12,
        /// Combined Name and Account Number
        "13" => N13,
        /// Beneficiary
        "17" => N17,
        /// Primary Care Provider
        "18" => N18,
        /// Corrected Insured
        "19" => N19,
        /// Prior Incorrect Insured
        "20" => N20,
        /// Corrected Name
        "21" => N21,
    }
);

crate::code_enum!(
    /// **1129** Adjustment Reason Code Characteristic
    ///
    /// - Data element: 1129
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Adjustment Reason Code Characteristic. Code values verified against the Stedi X12 reference.
    E1129 {
        /// Primary Disability - Frozen
        "1" => N1,
        /// Primary Disability - Non-frozen
        "2" => N2,
        /// Dependent Disability - Frozen
        "3" => N3,
        /// Dependent Disability - Non-frozen
        "4" => N4,
        /// Retirement - Frozen
        "5" => N5,
        /// Retirement - Non-frozen
        "6" => N6,
        /// Permanent
        "7" => N7,
        /// Temporary
        "8" => N8,
        /// Canadian
        "9" => N9,
        /// Joint
        "10" => N10,
        /// Deferred
        "11" => N11,
        /// Eligible
        "12" => N12,
        /// Applied
        "13" => N13,
        /// Receiving
        "14" => N14,
        /// Defense Medical Evaluation
        "15" => N15,
        /// Delayed Reporting Penalty (Insured)
        "16" => N16,
        /// Engineering Services
        "17" => N17,
        /// Expert Test (Trial)
        "18" => N18,
        /// Expert Fees and Expenses (excluding tests and testimony)
        "19" => N19,
        /// Fee Schedule Savings
        "22" => N22,
        /// Fatal
        "23" => N23,
        /// Permanent Total
        "24" => N24,
        /// Permanent Total Supplemental
        "25" => N25,
        /// Permanent Partial Scheduled
        "26" => N26,
        /// Permanent Partial Unscheduled
        "27" => N27,
        /// Temporary Total
        "28" => N28,
        /// Temporary Partial
        "29" => N29,
        /// Employer Liability
        "30" => N30,
        /// Permanent Partial Disfigurement
        "31" => N31,
        /// Unspecified Employer Payments
        "32" => N32,
        /// Funeral Expenses
        "34" => N34,
        /// Penalties
        "35" => N35,
        /// Interest
        "36" => N36,
        /// Employer's Legal Expenses
        "37" => N37,
        /// Claimant's Legal Expenses
        "38" => N38,
        /// Payments to Physicians
        "39" => N39,
        /// Hospital Costs
        "40" => N40,
        /// Other Medical
        "41" => N41,
        /// Vocational Rehabilitation Evaluation
        "42" => N42,
        /// Vocational Rehabilitation Education
        "43" => N43,
        /// Other Vocational Rehabilitation
        "44" => N44,
        /// Total Temporary Catastrophic
        "45" => N45,
        /// Unknown Payment Type
        "46" => N46,
        /// Vocational Rehabilitation Maintenance
        "57" => N57,
        /// Claim Value Recovery
        "58" => N58,
        /// Deductible Recovery
        "60" => N60,
        /// Additional Living Expense
        "61" => N61,
        /// Alternative Dispute Resolution Services
        "62" => N62,
        /// Anesthesiologist
        "63" => N63,
        /// Annuity
        "64" => N64,
        /// Attorney Expenses
        "65" => N65,
        /// Attorney Fees
        "66" => N66,
        /// Auto Liability Subrogation
        "67" => N67,
        /// Not Eligible
        "69" => N69,
        /// Catastrophic Benefits
        "71" => N71,
        /// Central Index Bureau Filing Fees
        "72" => N72,
        /// Chiropractor
        "73" => N73,
        /// Indemnity Reserves
        "74" => N74,
        /// Medical Reserves
        "75" => N75,
        /// Expense Reserves
        "76" => N76,
        /// Vocational Rehabilitation Reserves
        "77" => N77,
        /// Indemnity Payments
        "78" => N78,
        /// Medical Payments
        "79" => N79,
        /// Expense Payments
        "80" => N80,
        /// Vocational Rehabilitation Payments
        "81" => N81,
        /// Claim Payment
        "82" => N82,
        /// Claim Reserve
        "83" => N83,
        /// Bodily Injury Payment
        "84" => N84,
        /// Bodily Injury Reserve
        "85" => N85,
        /// Property Damage Payment
        "86" => N86,
        /// Property Damage Reserve
        "87" => N87,
        /// Comprehensive Payment
        "88" => N88,
        /// Comprehensive Reserve
        "89" => N89,
        /// Collision Payment
        "90" => N90,
        /// Collision Reserve
        "91" => N91,
        /// Salvage
        "92" => N92,
        /// Expert Witness Fees
        "93" => N93,
        /// Claimant Legal Expense
        "94" => N94,
        /// Claimant Medical Evaluation
        "95" => N95,
        /// Commercial Photographers
        "96" => N96,
        /// Mandated Medical Exam
        "97" => N97,
        /// Death Benefits
        "98" => N98,
        /// Defense Attorney Fees and Expenses
        "99" => N99,
        /// Funeral Benefits
        "A1" => A1,
        /// Hearing Attendance or Representation Fees
        "A2" => A2,
        /// Hospital - Inpatient Charges
        "A3" => A3,
        /// Hospital - Outpatient Charges
        "A4" => A4,
        /// Hospital Bill Audit
        "A5" => A5,
        /// Impairment Income Benefits
        "A6" => A6,
        /// Independent Adjustor Expense
        "A7" => A7,
        /// Independent Medical Exam
        "A8" => A8,
        /// Legal Expenses - Miscellaneous
        "A9" => A9,
        /// Chiropractic Expenses
        "AA" => Aa,
        /// Dental Expenses
        "AB" => Ab,
        /// Physical Therapy Costs
        "AC" => Ac,
        /// Pharmaceutical Costs
        "AD" => Ad,
        /// Durable Medical Costs
        "AE" => Ae,
        /// Medical Travel Costs
        "AF" => Af,
        /// Employee Medical-Legal Costs
        "AG" => Ag,
        /// Employer-Claim Administrator Medical-Legal Costs
        "AH" => Ah,
        /// Agreed upon or Directed Medical-Legal Costs
        "AI" => Ai,
        /// Unallocated Funds
        "AJ" => Aj,
        /// Future Credit Recovery
        "AK" => Ak,
        /// Liability Settlement
        "B1" => B1,
        /// Life Pension
        "B2" => B2,
        /// Lump Sum Remarriage Payment
        "B3" => B3,
        /// Lump Sum Settlement Amount
        "B4" => B4,
        /// Major Case of Individual Case Reporting
        "B5" => B5,
        /// Medical Equipment
        "B6" => B6,
        /// Peer Review Board Expenses
        "B7" => B7,
        /// Medical Management Services
        "B8" => B8,
        /// Medical Records or Reports
        "B9" => B9,
        /// Medical Rehabilitation
        "C1" => C1,
        /// Medical Test (Trial)
        "C2" => C2,
        /// No-fault benefit or expense (Non-Workers Compensation)
        "C3" => C3,
        /// Nursing Care
        "C4" => C4,
        /// Extended Care Facility
        "C5" => C5,
        /// State Fund - Other
        "C6" => C6,
        /// Penalties Paid on Medical Benefits
        "C7" => C7,
        /// Penalties Paid on Indemnity Benefits
        "C8" => C8,
        /// Pension Indemnity Benefit
        "C9" => C9,
        /// Personal Property or Contents
        "D2" => D2,
        /// Pharmacy
        "D3" => D3,
        /// Photographs other than Commercial
        "D4" => D4,
        /// Physiotherapy
        "D5" => D5,
        /// PPO (Preferred Provider Organization) Fees or Expenses
        "D6" => D6,
        /// Product Liability Subrogation
        "D7" => D7,
        /// Property Adjustment
        "D8" => D8,
        /// Provider Bill Audit Expense
        "D9" => D9,
        /// Radiology
        "E1" => E1,
        /// Time and Expense Fees
        "E2" => E2,
        /// Scheduled Award
        "E3" => E3,
        /// Social Security
        "E4" => E4,
        /// State Second Injury Fund
        "E5" => E5,
        /// State Supplemental Fund
        "E6" => E6,
        /// Supplemental Permanent Total Benefits
        "E7" => E7,
        /// Employee Interest
        "E9" => E9,
        /// Testing Lab
        "F1" => F1,
        /// Third Party Contribution
        "F2" => F2,
        /// Transportation - Other
        "F3" => F3,
        /// Transportation - Medical Treatment
        "F4" => F4,
        /// Treating or Panel Physician
        "F5" => F5,
        /// Utilization Review - Hospital
        "F6" => F6,
        /// Utilization Review - Physician
        "F7" => F7,
        /// Unallocated Indemnity
        "F8" => F8,
        /// Unallocated Medical
        "F9" => F9,
        /// Witness Fees and Expenses - Other than Expert Witness
        "G2" => G2,
        /// Vocational Rehabilitation Training
        "G3" => G3,
        /// Appraisal Fees
        "G4" => G4,
        /// Autopsy Fees
        "G5" => G5,
        /// Surveillance Special Investigation Fees and Expenses
        "G6" => G6,
        /// Temporary Income Benefits
        "G7" => G7,
        /// Federal Second Injury Fund
        "G9" => G9,
        /// Federal Supplemental Fund
        "H2" => H2,
        /// Flat Rate
        "H3" => H3,
        /// Employer Paid Temporary Total Disability
        "H4" => H4,
        /// Employer Paid Temporary Partial Disability
        "H5" => H5,
        /// Court Reporter Fees
        "H6" => H6,
        /// Private Investigator Fees
        "H7" => H7,
    }
);

crate::code_enum!(
    /// **1131** Level of Individual, Test, or Course Code
    ///
    /// - Data element: 1131
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Level of Individual, Test, or Course Code. Code values verified against the Stedi X12 reference.
    E1131 {
        /// Administrative
        "0A" => N0A,
        /// Kindergarten
        "0K" => N0K,
        /// First grade
        "01" => N01,
        /// Second grade
        "02" => N02,
        /// Third grade
        "03" => N03,
        /// Fourth grade
        "04" => N04,
        /// Fifth grade
        "05" => N05,
        /// Sixth grade
        "06" => N06,
        /// Seventh grade
        "07" => N07,
        /// Eighth grade
        "08" => N08,
        /// Ninth grade
        "09" => N09,
        /// Tenth grade
        "10" => N10,
        /// Eleventh grade
        "11" => N11,
        /// Twelfth grade
        "12" => N12,
        /// Managerial
        "14" => N14,
        /// Other Professional
        "15" => N15,
        /// Clerical
        "16" => N16,
        /// Technical
        "17" => N17,
        /// Skilled Crafts
        "18" => N18,
        /// Service Maintenance
        "19" => N19,
        /// Non-Degree or Temporary Undergraduate in Postsecondary school
        "20" => N20,
        /// Postsecondary First Year Student
        "21" => N21,
        /// Postsecondary Sophomore
        "22" => N22,
        /// Postsecondary Junior
        "23" => N23,
        /// Postsecondary Senior
        "24" => N24,
        /// Postsecondary Post-Baccalaureate Student
        "25" => N25,
        /// Postsecondary Non-Degree Graduate Student
        "26" => N26,
        /// Postsecondary Professional Student
        "27" => N27,
        /// Postsecondary Master's Degree Student
        "28" => N28,
        /// Postsecondary Doctoral Student
        "29" => N29,
        /// Postdoctoral Student
        "30" => N30,
        /// Postsecondary Bachelor Preliminary Year
        "31" => N31,
        /// Postsecondary Fifth Year Student
        "32" => N32,
        /// Postsecondary Masters Qualifying Year
        "33" => N33,
        /// First-year Graduate
        "34" => N34,
        /// Second-year Graduate
        "35" => N35,
        /// Third-year Graduate
        "36" => N36,
        /// Beyond Third-year Graduate
        "37" => N37,
        /// First-year, Attended College Before
        "38" => N38,
        /// First-year Professional
        "39" => N39,
        /// Second-year Professional
        "40" => N40,
        /// Third-year Professional
        "41" => N41,
        /// Beyond Third-year Professional
        "42" => N42,
        /// High School
        "AA" => Aa,
        /// Middle School
        "AB" => Ab,
        /// Junior High School
        "AC" => Ac,
        /// Adult
        "AD" => Ad,
        /// Associate Degree
        "AS" => As,
        /// Baccalaureate (Bachelor's) Degree
        "BD" => Bd,
        /// Medical Test (Trial)
        "C2" => C2,
        /// Clinical Medicine
        "CL" => Cl,
        /// Doctoral Degree
        "DD" => Dd,
        /// Elementary School
        "EL" => El,
        /// Employment
        "EM" => Em,
        /// Executive
        "EX" => Ex,
        /// Fall Term First Professional
        "FL" => Fl,
        /// First Professional
        "FP" => Fp,
        /// Fall Term Graduate
        "FT" => Ft,
        /// Full-time First-time Degree-seeking Undergraduate
        "FU" => Fu,
        /// First-time Degree-seeking Undergraduate
        "FV" => Fv,
        /// Graduate
        "GR" => Gr,
        /// High School Graduate or Equivalent
        "HG" => Hg,
        /// Attended high school, but did not graduate.
        "HS" => Hs,
        /// Infant (0 to age 2)
        "IF" => If,
        /// Intramural
        "IN" => In,
        /// Local Education Agency (LEA)
        "LA" => La,
        /// Master's Degree
        "MD" => Md,
        /// Mixed Grades
        "MG" => Mg,
        /// Middle or Junior High School
        "MS" => Ms,
        /// None
        "NO" => No,
        /// Pre-Kindergarten Level 0
        "P0" => P0,
        /// Pre-Kindergarten Level 1
        "P1" => P1,
        /// Pre-Kindergarten Level 2
        "P2" => P2,
        /// Pre-Kindergarten Level 3
        "P3" => P3,
        /// Pre-Kindergarten Level 4
        "P4" => P4,
        /// Pre-Kindergarten Level 5
        "P5" => P5,
        /// Postsecondary Certificate or Diploma
        "PC" => Pc,
        /// Professional Degree or Certification
        "PD" => Pd,
        /// Professional
        "PF" => Pf,
        /// Pre-Kindergarten
        "PK" => Pk,
        /// Pre-clinical Medicine
        "PM" => Pm,
        /// Postsecondary
        "PO" => Po,
        /// Some Postsecondary (e.g., college)
        "PS" => Ps,
        /// School
        "SA" => Sa,
        /// Instructional
        "SP" => Sp,
        /// Secondary School
        "SS" => Ss,
        /// Support Services
        "ST" => St,
        /// Twelve Month First Professional
        "TF" => Tf,
        /// Twelve Month Graduate
        "TW" => Tw,
        /// Undergraduate
        "UG" => Ug,
        /// Ungraded
        "UN" => Un,
        /// Varsity
        "VR" => Vr,
        /// Vocational School
        "VS" => Vs,
    }
);

crate::code_enum!(
    /// **1136** Code Category
    ///
    /// - Data element: 1136
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code Category. Code values verified against the Stedi X12 reference.
    E1136 {
        /// Employee Mobility
        "00" => N00,
        /// Pre-existing Conditions
        "01" => N01,
        /// Diagnosis
        "02" => N02,
        /// Condition
        "03" => N03,
        /// Occurrence
        "04" => N04,
        /// Occurrence Span
        "05" => N05,
        /// Value
        "06" => N06,
        /// Ambulance Certification
        "07" => N07,
        /// Chiropractic Certification
        "08" => N08,
        /// Durable Medical Equipment Certification
        "09" => N09,
        /// Enteral or Parenteral Therapy Certification
        "10" => N10,
        /// Oxygen Therapy Certification
        "11" => N11,
        /// Admitting Diagnosis
        "12" => N12,
        /// Principal Diagnosis
        "13" => N13,
        /// Pre-Existing Physical Condition
        "14" => N14,
        /// Pre-Existing Mental Condition
        "15" => N15,
        /// Routine Foot Care Class Finding
        "16" => N16,
        /// Systemic Condition for Routine Foot Care
        "17" => N17,
        /// Co-op Advertising
        "18" => N18,
        /// Commercial Advertising
        "19" => N19,
        /// Specimen Kit Type Code
        "20" => N20,
        /// Laboratory Test Condition Code
        "21" => N21,
        /// Automobile Loss
        "22" => N22,
        /// Laboratory Results Identification Code
        "23" => N23,
        /// Line of Business Code
        "24" => N24,
        /// United States Department of Vital Statistics E-Code
        "25" => N25,
        /// Employment Status Information
        "26" => N26,
        /// Income
        "27" => N27,
        /// Loan Information
        "28" => N28,
        /// Injury or Illness
        "29" => N29,
        /// Benefit Adjustment
        "30" => N30,
        /// Claimant
        "31" => N31,
        /// Contractholder Branch
        "32" => N32,
        /// Contractholder
        "33" => N33,
        /// Secondary Claim Administrator
        "34" => N34,
        /// Primary Claim Administrator
        "35" => N35,
        /// Reporting Agency
        "36" => N36,
        /// Process
        "37" => N37,
        /// Hazardous Material
        "38" => N38,
        /// Activity
        "39" => N39,
        /// Accident
        "40" => N40,
        /// Initial Treatment
        "41" => N41,
        /// Cause of Injury
        "42" => N42,
        /// Part of Body
        "43" => N43,
        /// Nature of Injury
        "44" => N44,
        /// Source of Injury
        "45" => N45,
        /// Job
        "46" => N46,
        /// Loss Prevention
        "47" => N47,
        /// Managed Care
        "48" => N48,
        /// Risk Management
        "49" => N49,
        /// Claim Handling
        "50" => N50,
        /// Event or Exposure
        "51" => N51,
        /// Equipment or Materials or Chemicals
        "52" => N52,
        /// Coverage
        "53" => N53,
        /// Overbite
        "54" => N54,
        /// Overjet
        "55" => N55,
        /// Profile
        "56" => N56,
        /// Crossbite
        "57" => N57,
        /// Arch Asymmetry
        "58" => N58,
        /// Dentition Midline
        "59" => N59,
        /// Crowding
        "60" => N60,
        /// Molars
        "61" => N61,
        /// Cuspids
        "62" => N62,
        /// Interviewee
        "63" => N63,
        /// Verification of Deposit
        "64" => N64,
        /// Verification of Mortgage
        "65" => N65,
        /// Verification of Income or Employment or Both
        "66" => N66,
        /// Verification of Rent
        "67" => N67,
        /// Verification of Loan or Installment Debt or Both
        "68" => N68,
        /// Anti-fungal Therapy
        "69" => N69,
        /// Hospice
        "70" => N70,
        /// Primary Diagnosis
        "71" => N71,
        /// Secondary Diagnosis
        "72" => N72,
        /// Tertiary Diagnosis
        "73" => N73,
        /// Procedure Code
        "74" => N74,
        /// Functional Limitations
        "75" => N75,
        /// Activities Permitted
        "76" => N76,
        /// Mental Status
        "77" => N77,
        /// Manner Property Title Held
        "78" => N78,
        /// Property Improvements
        "79" => N79,
        /// Complete Appraisal
        "80" => N80,
        /// Limited Appraisal
        "81" => N81,
        /// Restricted Appraisal Report Limiting Conditions
        "82" => N82,
        /// Route of Administration
        "83" => N83,
        /// Borrower Information
        "84" => N84,
        /// Contract Information
        "85" => N85,
        /// Fannie Mae (Federal National Mortgage Association)
        "86" => N86,
        /// Freddie Mac (Federal Home Loan Mortgage Corporation)
        "87" => N87,
        /// Deductible
        "88" => N88,
        /// Advertising Copy
        "89" => N89,
        /// Private Remarks
        "90" => N90,
        /// Compensation Notes
        "91" => N91,
        /// Open House Notes
        "92" => N92,
        /// Tour Notes
        "93" => N93,
        /// Terms of Sale
        "94" => N94,
        /// Restrictions
        "95" => N95,
        /// Disclosures
        "96" => N96,
        /// Exceptions
        "97" => N97,
        /// Inclusions
        "98" => N98,
        /// Lease Type
        "99" => N99,
        /// Contracting District Type
        "A0" => A0,
        /// Mortgage Record Change
        "A1" => A1,
        /// Mortgage Insurance Termination
        "A2" => A2,
        /// Mortgage Insurance Cancellation
        "A3" => A3,
        /// Mortgage Servicing Transfer
        "A4" => A4,
        /// Appraisal
        "A5" => A5,
        /// State License Disciplinary Action
        "A6" => A6,
        /// Source of Data
        "A7" => A7,
        /// Endorsement
        "A8" => A8,
        /// Notification
        "A9" => A9,
        /// All
        "AA" => Aa,
        /// Agent's Questions
        "AB" => Ab,
        /// Agent's Share
        "AC" => Ac,
        /// Benefits
        "AD" => Ad,
        /// Contact or Reference Information
        "AE" => Ae,
        /// Cost Basis
        "AF" => Af,
        /// Driving Infractions
        "AG" => Ag,
        /// Excess Dividend Use
        "AH" => Ah,
        /// Home Health Aide
        "AI" => Ai,
        /// Existing Coverage Information
        "AJ" => Aj,
        /// Hospitalization
        "AK" => Ak,
        /// Activity Limitations
        "AL" => Al,
        /// Juvenile Information
        "AM" => Am,
        /// Occupation Information
        "AN" => An,
        /// Personal Finance and Business Information
        "AO" => Ao,
        /// Appearance
        "AP" => Ap,
        /// Rating Information
        "AQ" => Aq,
        /// Arrest
        "AR" => Ar,
        /// Replaced Amount
        "AS" => As,
        /// Authority
        "AT" => At,
        /// Automated Underwriting Information
        "AU" => Au,
        /// Aviation
        "AV" => Av,
        /// Surgery
        "AW" => Aw,
        /// Travel Information
        "AX" => Ax,
        /// Age Remark
        "AY" => Ay,
        /// Property Remark
        "AZ" => Az,
        /// Audit Data
        "B1" => B1,
        /// Declaration Sheet Indicator
        "B2" => B2,
        /// Servicing Data
        "B3" => B3,
        /// Single Family
        "B4" => B4,
        /// Multifamily
        "B5" => B5,
        /// Payment Handling
        "B6" => B6,
        /// Ginnie Mae 1
        "B7" => B7,
        /// Ginnie Mae 2
        "B8" => B8,
        /// Ginnie Mae 2 Custom
        "B9" => B9,
        /// Bankruptcy
        "BA" => Ba,
        /// Business Beneficiary
        "BB" => Bb,
        /// Building Condition
        "BC" => Bc,
        /// Buydown
        "BD" => Bd,
        /// Beneficiary
        "BE" => Be,
        /// Tax Agency Parcel Identifier
        "BF" => Bf,
        /// Historical Performance
        "BG" => Bg,
        /// Product Rules
        "BH" => Bh,
        /// Commercial Property
        "BI" => Bi,
        /// Unimproved Land
        "BJ" => Bj,
        /// Banking
        "BK" => Bk,
        /// New Contract
        "BL" => Bl,
        /// Original Contract
        "BM" => Bm,
        /// Access
        "BN" => Bn,
        /// Bond
        "BO" => Bo,
        /// Bankruptcy Petition
        "BP" => Bp,
        /// Agent Sales Trend
        "BQ" => Bq,
        /// Broker's Price Opinion
        "BR" => Br,
        /// Bankruptcy Statement of Financial Affairs
        "BS" => Bs,
        /// Billings Trend
        "BT" => Bt,
        /// Assets
        "BU" => Bu,
        /// Cash Flow
        "BV" => Bv,
        /// Competition
        "BW" => Bw,
        /// Credit Line
        "BX" => Bx,
        /// Creditors Arrangement
        "BY" => By,
        /// Creditors Meeting
        "BZ" => Bz,
        /// Depreciation Conditions
        "C1" => C1,
        /// Adverse Environment Conditions
        "C2" => C2,
        /// Miscellaneous Adverse Conditions
        "C3" => C3,
        /// Site Conditions
        "C4" => C4,
        /// Subject Property Conditions
        "C5" => C5,
        /// Board of Directors
        "C6" => C6,
        /// Reserve
        "C7" => C7,
        /// Payment
        "C8" => C8,
        /// Comorbidity
        "C9" => C9,
        /// Citizenship
        "CA" => Ca,
        /// Continuing Education
        "CB" => Cb,
        /// Compensation Calculation
        "CC" => Cc,
        /// Cause of Death
        "CD" => Cd,
        /// Condominium
        "CE" => Ce,
        /// Cooperative
        "CF" => Cf,
        /// Conviction
        "CG" => Cg,
        /// Direct Sales Trend
        "CH" => Ch,
        /// Export Trend
        "CI" => Ci,
        /// Financial Embarrassment
        "CJ" => Cj,
        /// Indebtedness
        "CK" => Ck,
        /// Cancellation
        "CL" => Cl,
        /// Claim Amounts
        "CM" => Cm,
        /// Comparison
        "CN" => Cn,
        /// County
        "CO" => Co,
        /// Complications
        "CP" => Cp,
        /// Initial Capital
        "CQ" => Cq,
        /// Current Ratio
        "CR" => Cr,
        /// Common Stock
        "CS" => Cs,
        /// Commission Trend
        "CT" => Ct,
        /// Stockholders
        "CU" => Cu,
        /// Damage
        "CV" => Cv,
        /// Working Capital
        "CW" => Cw,
        /// Compensation Allocation
        "CX" => Cx,
        /// Dividend Use
        "CY" => Cy,
        /// Excess Premium Use
        "CZ" => Cz,
        /// Unpaid Invoices
        "D1" => D1,
        /// Withdrawals
        "D2" => D2,
        /// Imports
        "D3" => D3,
        /// Placed for Collection
        "D4" => D4,
        /// Drug Adjudication Information
        "DA" => Da,
        /// Liquidation Proceedings
        "DB" => Db,
        /// Location
        "DC" => Dc,
        /// Discharge Diagnosis
        "DD" => Dd,
        /// Departmental
        "DE" => De,
        /// Profit Margin
        "DF" => Df,
        /// Proposal
        "DG" => Dg,
        /// Receivership
        "DH" => Dh,
        /// Driver Identification Information
        "DI" => Di,
        /// Provider Characteristics and Resources
        "DJ" => Dj,
        /// Secondary Source of Injury
        "DK" => Dk,
        /// Petitiions
        "DL" => Dl,
        /// Registered Charges
        "DM" => Dm,
        /// Criminal Proceedings
        "DN" => Dn,
        /// Historical Criminal Proceedings
        "DO" => Do,
        /// Directions to Property
        "DP" => Dp,
        /// Driving
        "DR" => Dr,
        /// Driver Record Information
        "DV" => Dv,
        /// Spectacle Lenses
        "E1" => E1,
        /// Contact Lenses
        "E2" => E2,
        /// Spectacle Frames
        "E3" => E3,
        /// Employment
        "E4" => E4,
        /// Examiner's Comments
        "E5" => E5,
        /// Intercompany Relations
        "EB" => Eb,
        /// Judgments
        "EC" => Ec,
        /// Liens
        "ED" => Ed,
        /// Operating Surplus Trend
        "EE" => Ee,
        /// Participating Interest
        "EF" => Ef,
        /// Protested Bills
        "EG" => Eg,
        /// Subcontracting Details
        "EH" => Eh,
        /// Suits
        "EI" => Ei,
        /// Uniform Commercial Code (UCC) Filings
        "EJ" => Ej,
        /// Detrimental Legal Filings
        "EK" => Ek,
        /// Customer Details
        "EL" => El,
        /// Supplier Detail
        "EM" => Em,
        /// Employee Relocation
        "ER" => Er,
        /// Education or Training
        "ET" => Et,
        /// Financial
        "FA" => Fa,
        /// Family Coverage
        "FC" => Fc,
        /// Family History
        "FH" => Fh,
        /// Financing
        "FI" => Fi,
        /// Flood Determination
        "FL" => Fl,
        /// Franchise Tax Payments
        "FP" => Fp,
        /// Financial Remarks
        "FR" => Fr,
        /// Foreign Travel
        "FT" => Ft,
        /// Demonstrations
        "GD" => Gd,
        /// Shelf Format
        "GS" => Gs,
        /// Guarantees
        "GU" => Gu,
        /// Fixed
        "HA" => Ha,
        /// Adjustable
        "HB" => Hb,
        /// Rate Adjustment
        "HC" => Hc,
        /// Payment Adjustment
        "HD" => Hd,
        /// Life of Loan
        "HE" => He,
        /// Periodic Interest Rate
        "HF" => Hf,
        /// Principal and Interest
        "HG" => Hg,
        /// Health or Medical
        "HH" => Hh,
        /// Late Charge
        "HI" => Hi,
        /// Default Note Holder's Cost
        "HJ" => Hj,
        /// Prepayment
        "HK" => Hk,
        /// Limited Payment
        "HL" => Hl,
        /// Rate Lookback
        "HM" => Hm,
        /// Payment Lookback
        "HN" => Hn,
        /// Index
        "HO" => Ho,
        /// Mortgage Margin
        "HP" => Hp,
        /// Single Family 2-4 Units
        "HQ" => Hq,
        /// Amortization
        "HR" => Hr,
        /// Rate Conversion
        "HS" => Hs,
        /// Interest Only
        "HT" => Ht,
        /// Premium Audit Key Question
        "HU" => Hu,
        /// History
        "HY" => Hy,
        /// Hazardous Sports
        "HZ" => Hz,
        /// Issued Capital
        "IC" => Ic,
        /// Identification
        "ID" => Id,
        /// Insurance History or Other Coverage
        "IH" => Ih,
        /// Impairment
        "IM" => Im,
        /// Insurance
        "IN" => In,
        /// License Revocation
        "LA" => La,
        /// Location Status
        "LC" => Lc,
        /// Level Remarks
        "LE" => Le,
        /// Liability Status
        "LI" => Li,
        /// Local Language Description
        "LL" => Ll,
        /// Listing Remarks
        "LR" => Lr,
        /// Life Style
        "LS" => Ls,
        /// Legal Type
        "LT" => Lt,
        /// Loss Trend
        "LZ" => Lz,
        /// Marital Status
        "MA" => Ma,
        /// Miscellaneous
        "MI" => Mi,
        /// Multiple Listing Service
        "ML" => Ml,
        /// Management
        "MN" => Mn,
        /// Modification
        "MO" => Mo,
        /// Medication or Prescription
        "MP" => Mp,
        /// Medical Social Worker
        "MS" => Ms,
        /// Military Status
        "MT" => Mt,
        /// Nominal Capital
        "NC" => Nc,
        /// New Licensed Staff
        "NL" => Nl,
        /// Nature of Suit
        "NS" => Ns,
        /// Not Work Related
        "NW" => Nw,
        /// Owner Pays Notes
        "OA" => Oa,
        /// Occupation Class
        "OC" => Oc,
        /// Outside Financing
        "OF" => Of,
        /// Other Investor
        "OI" => Oi,
        /// Operations Trend
        "ON" => On,
        /// Operations
        "OP" => Op,
        /// Occupational Therapy
        "OT" => Ot,
        /// Operations Outlook
        "OU" => Ou,
        /// Performance
        "PA" => Pa,
        /// Profitability
        "PB" => Pb,
        /// Paid in Capital
        "PC" => Pc,
        /// Public Records
        "PD" => Pd,
        /// Penalty
        "PE" => Pe,
        /// Profit Trend
        "PF" => Pf,
        /// Possession Notes
        "PG" => Pg,
        /// Photo Instructions
        "PI" => Pi,
        /// Patient Subjective Complaints
        "PJ" => Pj,
        /// Parking Notes
        "PN" => Pn,
        /// Profit Outlook
        "PO" => Po,
        /// Property
        "PR" => Pr,
        /// Preferred Stock
        "PS" => Ps,
        /// Physical Therapy
        "PT" => Pt,
        /// Physician Examination Results
        "PX" => Px,
        /// Reason for Weight Loss
        "R1" => R1,
        /// Association of American Railroads Special Proper Shipping Name Flag
        "R2" => R2,
        /// Association of American Railroads Intermodal Indicator
        "R3" => R3,
        /// Association of American Railroads U.S. to Canada Flag
        "R4" => R4,
        /// Residential Status
        "R5" => R5,
        /// Revocation
        "RC" => Rc,
        /// Recovery
        "RE" => Re,
        /// Real Estate Property Information
        "RI" => Ri,
        /// Radio License Application
        "RL" => Rl,
        /// Remedy
        "RM" => Rm,
        /// Related Entities
        "RN" => Rn,
        /// Retirement Plan Type
        "RP" => Rp,
        /// Reinstatement
        "RR" => Rr,
        /// Reason Last Seen
        "RS" => Rs,
        /// Registration Type
        "RT" => Rt,
        /// Results
        "RU" => Ru,
        /// Revenue Trend
        "RV" => Rv,
        /// Investment Trend
        "S1" => S1,
        /// Royalty Trend
        "S2" => S2,
        /// Purchases Trend
        "S3" => S3,
        /// Labor Infraction
        "S4" => S4,
        /// Debentures
        "S5" => S5,
        /// Source Fund
        "SA" => Sa,
        /// Starting Details
        "SD" => Sd,
        /// Summary and Evaluation
        "SE" => Se,
        /// Showing Instructions
        "SI" => Si,
        /// Suits, Judgments & Liens
        "SJ" => Sj,
        /// Supplement Note or Line
        "SL" => Sl,
        /// Skilled Nursing
        "SN" => Sn,
        /// Statement Preparation
        "SP" => Sp,
        /// Sales Trend
        "SR" => Sr,
        /// Suspension
        "SS" => Ss,
        /// Speech Therapy
        "ST" => St,
        /// Substance Use
        "SU" => Su,
        /// Reported Statement of Witness
        "SW" => Sw,
        /// Size
        "SZ" => Sz,
        /// Tobacco
        "TB" => Tb,
        /// Tests
        "TE" => Te,
        /// Target Fund
        "TF" => Tf,
        /// Therapy
        "TH" => Th,
        /// Action
        "TI" => Ti,
        /// Terms
        "TM" => Tm,
        /// Trend
        "TN" => Tn,
        /// Tenant Pays Notes
        "TP" => Tp,
        /// Treatment
        "TR" => Tr,
        /// Two to Four Units
        "TW" => Tw,
        /// Tax Service
        "TX" => Tx,
        /// Production Capacity
        "UA" => Ua,
        /// Actual Production
        "UB" => Ub,
        /// Branch Trend
        "UC" => Uc,
        /// Retail Locations
        "UD" => Ud,
        /// Net Profit
        "UE" => Ue,
        /// Ordinary Profit
        "UF" => Uf,
        /// Declared Profit to Local Tax Office
        "UG" => Ug,
        /// Market Trend
        "UH" => Uh,
        /// Pre Tax Profit
        "UI" => Ui,
        /// Net Worth
        "UJ" => Uj,
        /// Debt to Equity
        "UK" => Uk,
        /// Equity Return
        "UL" => Ul,
        /// Stability
        "UM" => Um,
        /// Efficiency
        "UN" => Un,
        /// Outlook
        "UO" => Uo,
        /// Update
        "UP" => Up,
        /// Corporate Registration
        "UQ" => Uq,
        /// Voter Registration Application
        "VA" => Va,
        /// Voter Registration Application Disposition
        "VD" => Vd,
        /// Violation
        "VO" => Vo,
        /// Warning
        "WA" => Wa,
        /// Prognosis
        "WB" => Wb,
        /// Treatment Plan
        "WD" => Wd,
        /// Work Restrictions
        "WE" => We,
        /// Witness Statement
        "WF" => Wf,
        /// Conditions Affecting Total Employees and Hours
        "WG" => Wg,
        /// Injury Work Related
        "WH" => Wh,
        /// Illness Work Related
        "WI" => Wi,
        /// Controvert Reason
        "WK" => Wk,
        /// Supervisor's Comments
        "WL" => Wl,
        /// Willful Misconduct
        "WM" => Wm,
        /// Supervisor's Exception
        "WN" => Wn,
        /// Claim Related Work Assignment Changes
        "WO" => Wo,
        /// 30 Day Delay Reason
        "WP" => Wp,
        /// Employee Comment
        "WQ" => Wq,
        /// Employee Comment Not Provided Reason
        "WR" => Wr,
        /// Medical Records Not Attached Reason
        "WS" => Ws,
        /// Work Exposures and Duration
        "WT" => Wt,
        /// Letter of Credit Overdrawn
        "X1" => X1,
        /// Cargo Receipt Not Signed
        "X2" => X2,
        /// Customs Statement Missing from Invoice
        "X3" => X3,
        /// Purchase Order Not on Letter of Credit (Except Masters)
        "X4" => X4,
        /// Reduced Draft
        "X5" => X5,
        /// Time Drafts
        "X6" => X6,
        /// Demand for Payment
        "X7" => X7,
        /// Early Presentation of Documents
        "X8" => X8,
        /// Physician - Patient Report Inconsistency
        "YR" => Yr,
        /// Physician Test Results
        "YT" => Yt,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1154** Work Intensity Code
    ///
    /// - Data element: 1154
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Work Intensity Code. Code values verified against the Stedi X12 reference.
    E1154 {
        /// Heavy
        "H" => H,
        /// Light
        "L" => L,
        /// Medium
        "M" => M,
        /// Regular
        "R" => R,
    }
);

crate::code_enum!(
    /// **1161** Product Option Code
    ///
    /// - Data element: 1161
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Product Option Code. Code values verified against the Stedi X12 reference.
    E1161 {
        /// Pretax
        "1" => N1,
        /// Post-tax
        "2" => N2,
        /// Qualified
        "3" => N3,
        /// Non-qualified
        "4" => N4,
        /// 401K
        "5" => N5,
        /// Individual Retirement Account
        "6" => N6,
        /// Keogh
        "7" => N7,
        /// Simplified Employee Pension
        "8" => N8,
        /// Single Premium
        "9" => N9,
        /// Flexible Premium
        "10" => N10,
        /// Variable Premium
        "11" => N11,
        /// Fixed Premium
        "12" => N12,
        /// Registered under the Income Tax Act of Canada
        "13" => N13,
        /// Non-Registered under the Income Tax Act of Canada
        "14" => N14,
        /// Registered Spousal Case
        "15" => N15,
        /// Exclusive
        "28" => N28,
        /// Shopped
        "29" => N29,
        /// Lead Reinsurer
        "30" => N30,
        /// Facultative Excess
        "31" => N31,
        /// First to Die
        "A" => A,
        /// Last to Die
        "B" => B,
        /// Bank Account
        "BA" => Ba,
        /// Child Rider
        "C" => C,
        /// Discontinue One-Bill Submission
        "D" => D,
        /// Government Allocation
        "GA" => Ga,
        /// Benefit Continuation
        "N" => N,
        /// One-Bill Submission Not Chosen
        "NC" => Nc,
        /// One-Bill Submission
        "O" => O,
        /// Payroll Deduction
        "PD" => Pd,
        /// Salary Continuation
        "S" => S,
    }
);

crate::code_enum!(
    /// **1196** Breakdown Structure Detail Code
    ///
    /// - Data element: 1196
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Breakdown Structure Detail Code. Code values verified against the Stedi X12 reference.
    E1196 {
        /// Labor
        "01" => N01,
        /// Material
        "02" => N02,
        /// Other Direct Costs (ODC)
        "03" => N03,
        /// Subcontract
        "04" => N04,
        /// Labor and Material
        "05" => N05,
        /// Labor Overhead
        "06" => N06,
        /// Material Overhead
        "07" => N07,
        /// Other Overhead
        "08" => N08,
        /// Total Overhead
        "09" => N09,
        /// Total General and Administrative
        "10" => N10,
        /// Engineering Labor
        "11" => N11,
        /// Engineering Overhead
        "12" => N12,
        /// Manufacturing Labor
        "13" => N13,
        /// Manufacturing Overhead
        "14" => N14,
        /// Interdivisional Work Authorization
        "15" => N15,
        /// Intercomponent Work Authorization
        "16" => N16,
        /// Change Document Number
        "17" => N17,
        /// Funds Appropriation
        "18" => N18,
        /// Nonrecurring Labor
        "19" => N19,
        /// Nonrecurring Material
        "20" => N20,
        /// Nonrecurring Other Direct Costs
        "21" => N21,
        /// Nonrecurring Subcontract
        "22" => N22,
        /// Nonrecurring Labor and Material
        "23" => N23,
        /// Nonrecurring Engineering Labor
        "24" => N24,
        /// Nonrecurring Manufacturing Labor
        "25" => N25,
        /// Recurring
        "26" => N26,
        /// Nonrecurring
        "27" => N27,
        /// Total
        "28" => N28,
        /// Nonrecurring and Total
        "29" => N29,
        /// Recurring and Total
        "30" => N30,
        /// Direct Cost or Hours
        "31" => N31,
        /// Plant-wide Overhead
        "32" => N32,
        /// General & Administrative
        "33" => N33,
        /// Cost of Money
        "34" => N34,
        /// Profit or Fee
        "35" => N35,
        /// Total Price
        "36" => N36,
        /// Total Cost Less General & Administrative
        "37" => N37,
        /// Total Cost Plus General & Administrative
        "38" => N38,
        /// Other Costs
        "39" => N39,
        /// Standard
        "40" => N40,
        /// Variance
        "41" => N41,
        /// Purchased Equipment
        "42" => N42,
        /// Materials and Purchased Items
        "43" => N43,
        /// Quality Control Labor
        "44" => N44,
        /// Other Government Effort
        "45" => N45,
        /// Commercial Effort
        "46" => N46,
        /// Total Direct Cost Base
        "47" => N47,
        /// Indirect Labor
        "48" => N48,
        /// Employee Benefits
        "49" => N49,
        /// Payroll Taxes
        "50" => N50,
        /// Employment
        "51" => N51,
        /// Communications and Travel
        "52" => N52,
        /// Production Related
        "53" => N53,
        /// Facilities - Building and Land
        "54" => N54,
        /// Facilities - Furniture and Equipment
        "55" => N55,
        /// Administration
        "56" => N56,
        /// Future Business
        "57" => N57,
        /// Credits
        "58" => N58,
        /// Government Furnished Equipment
        "59" => N59,
        /// Government Furnished Material
        "60" => N60,
        /// Government Furnished Property
        "61" => N61,
        /// Engineering
        "62" => N62,
        /// Tooling
        "63" => N63,
        /// Tooling Design
        "64" => N64,
        /// Tooling Fabrication
        "65" => N65,
        /// Quality Control
        "66" => N66,
        /// Manufacturing
        "67" => N67,
        /// Total Salaries and Wages
        "70" => N70,
        /// Total Salaries and Wages and Employee Benefits
        "71" => N71,
        /// Total Purchased Equipment
        "72" => N72,
        /// Travel - Domestic
        "73" => N73,
        /// Travel - Foreign
        "74" => N74,
        /// Participant Stipends
        "75" => N75,
        /// Participant Travel
        "77" => N77,
        /// Participant Other
        "78" => N78,
        /// Participant Total
        "79" => N79,
        /// Publication Costs
        "80" => N80,
        /// Consultant Services
        "81" => N81,
        /// Computer Automated Data Processing (ADP) Services
        "82" => N82,
        /// Funding From Non-Federal Sources
        "83" => N83,
        /// Total Direct Costs
        "84" => N84,
        /// Human Subject Costs
        "85" => N85,
        /// Animal Costs
        "86" => N86,
        /// Alternations and Renovations
        "87" => N87,
        /// Cost Sharing
        "88" => N88,
        /// Project/Task
        "90" => N90,
        /// Advanced Procurement Indicator
        "91" => N91,
        /// Department Indicator
        "A1" => A1,
        /// Transfer from Department
        "A2" => A2,
        /// Fiscal Year Indicator
        "A3" => A3,
        /// Basic Symbol Number
        "A4" => A4,
        /// Sub-class
        "A5" => A5,
        /// Sub-Account Symbol
        "A6" => A6,
        /// Air Force Transportation Account Code (ATAC)
        "AT" => At,
        /// Budget Activity Number
        "B1" => B1,
        /// Budget Sub-activity Number
        "B2" => B2,
        /// Budget Line Item Identification
        "B3" => B3,
        /// Project/Task/Budget Sub-Line
        "B4" => B4,
        /// Advance Procurement Year
        "B5" => B5,
        /// Fund Code
        "B6" => B6,
        /// Billings
        "BL" => Bl,
        /// Base Year Costs
        "BY" => By,
        /// Program Element
        "C1" => C1,
        /// Budgetary Restrictions
        "C2" => C2,
        /// Funding Type
        "C3" => C3,
        /// Commitments
        "CM" => Cm,
        /// Current Year Costs
        "CY" => Cy,
        /// Defense Agency Level Organization
        "D1" => D1,
        /// Major Command Level Organization
        "D2" => D2,
        /// Field Level Organization
        "D3" => D3,
        /// Work Center
        "D4" => D4,
        /// Allotment Recipient
        "D5" => D5,
        /// Sub-allotment Recipient
        "D6" => D6,
        /// Work Center Recipient
        "D7" => D7,
        /// Major Reimbursement Source Code
        "E1" => E1,
        /// Detail Reimbursement Source Code
        "E2" => E2,
        /// Customer Indicator
        "E3" => E3,
        /// Equipment Maintenance
        "EH" => Eh,
        /// Escalation
        "ES" => Es,
        /// Expenditures
        "EX" => Ex,
        /// Object Class
        "F1" => F1,
        /// Object Sub-class
        "F2" => F2,
        /// Government or Public Sector Identifier
        "F3" => F3,
        /// Foreign Currency Code
        "F4" => F4,
        /// Country Code
        "F5" => F5,
        /// Participant Fees
        "FE" => Fe,
        /// Fixed Price
        "FP" => Fp,
        /// Forward Priced
        "FR" => Fr,
        /// Program or Planning Code
        "G1" => G1,
        /// Special Interest Code or Special Program Cost Code
        "G2" => G2,
        /// Cost Code
        "H1" => H1,
        /// Labor Type Code
        "H2" => H2,
        /// Cost Allocation Code
        "H3" => H3,
        /// Classification Code
        "H4" => H4,
        /// Abbreviated Department of Defense (DoD) Budget and Accounting Classification Code (BACC)
        "I1" => I1,
        /// Insurance
        "IN" => In,
        /// Document or Record Reference Number
        "J1" => J1,
        /// Standard Document Reference Number (SDRN) Line Item Number (SLIN)
        "J2" => J2,
        /// Standard Document Reference Number (SDRN) Sub Line Item Number (SSLIN)
        "J3" => J3,
        /// Accounting Classification Reference Code
        "K6" => K6,
        /// Accounting Installation Number
        "L1" => L1,
        /// Labor - Military
        "LM" => Lm,
        /// Local Installation Data
        "M1" => M1,
        /// Movement Designator Code (MDC)
        "MD" => Md,
        /// Management Reserve
        "MR" => Mr,
        /// Transaction Type
        "N1" => N1,
        /// Non-production
        "NP" => Np,
        /// Disbursing Station Number
        "P1" => P1,
        /// International Balance of Payments (IBOP) Code
        "P2" => P2,
        /// Disbursing Office Voucher Number
        "P3" => P3,
        /// Rental or Lease of Equipment or Facilities
        "RL" => Rl,
        /// Service Contracts
        "SC" => Sc,
        /// Transportation Account Code (TAC)
        "TA" => Ta,
        /// Termination Costs
        "TC" => Tc,
        /// Technical Data
        "TD" => Td,
        /// Training
        "TR" => Tr,
        /// Participant Tuition
        "TU" => Tu,
        /// Undistributed Budget
        "UB" => Ub,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1197** Financial Transaction Status Code
    ///
    /// - Data element: 1197
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Financial Transaction Status Code. Code values verified against the Stedi X12 reference.
    E1197 {
        /// Adjustment
        "AD" => Ad,
        /// Authorized
        "AV" => Av,
        /// Back Value, Transactions had been Posted to Incorrect Account
        "BA" => Ba,
        /// Back Value, Miscellaneous
        "BM" => Bm,
        /// Back Value, Only Partial Amount Credited/Debited to Account
        "BP" => Bp,
        /// Back Value, Transaction Not Received by Bank
        "BV" => Bv,
        /// Canceled or Voided
        "C" => C,
        /// Miscellaneous Debit
        "D" => D,
        /// Miscellaneous Credit
        "M" => M,
        /// Nonauthorized
        "NA" => Na,
        /// Outstanding
        "O" => O,
        /// Paid
        "P" => P,
        /// Reconciled
        "R" => R,
        /// Return, Insufficient Data
        "RD" => Rd,
        /// Re-deposit
        "RE" => Re,
        /// Return, Insufficient Funds
        "RI" => Ri,
        /// Return
        "RN" => Rn,
        /// Payment Stopped
        "S" => S,
        /// Truncated
        "TR" => Tr,
    }
);
