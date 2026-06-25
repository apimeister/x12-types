//! X12 data elements 0700-0799.

crate::code_enum!(
    /// **735** Hierarchical Level Code
    ///
    /// - Data element: 735
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code defining the characteristic of a level in a hierarchical structure
    /// (HL-03). Common codes named below; any other round-trips as `Unknown`.
    E735 {
        /// Shipment
        "S" => S,
        /// Order
        "O" => O,
        /// Item
        "I" => I,
        /// Pack
        "P" => P,
        /// Transportation Equipment
        "E" => E,
        /// Component
        "F" => F,
        /// Bill of Materials
        "H" => H,
        /// Provider of Service
        "19" => N19,
        /// Information Source
        "20" => N20,
        /// Information Receiver
        "21" => N21,
        /// Subscriber
        "22" => N22,
        /// Dependent
        "23" => N23,
        /// Patient
        "PT" => Pt,
        /// Employer
        "EM" => Em,
    }
);

crate::code_enum!(
    /// **736** Hierarchical Child Code
    ///
    /// - Data element: 736
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code indicating if there are hierarchical child data segments subordinate to
    /// the level being described (HL-04).
    E736 {
        /// No Subordinate HL Segment in This Hierarchical Structure
        "0" => N0,
        /// Additional Subordinate HL Data Segment in This Hierarchical Structure
        "1" => N1,
    }
);

crate::code_enum!(
    /// **750** Product/Process Characteristic Code
    ///
    /// - Data element: 750
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code identifying the general class of a product or process characteristic
    /// (PID-02). Common codes named below.
    E750 {
        /// Limiting Operation
        "01" => N01,
        /// General Product Form
        "02" => N02,
        /// Preservative
        "03" => N03,
        /// Parameter Specification
        "04" => N04,
        /// Category Classification
        "05" => N05,
        /// Product Type Identifier
        "08" => N08,
        /// Major Grade
        "10" => N10,
        /// Color
        "35" => N35,
        /// Grade Level
        "38" => N38,
        /// Chemistry
        "68" => N68,
        /// Mechanical
        "71" => N71,
    }
);

crate::code_enum!(
    /// **737** Measurement Reference ID Code
    ///
    /// - Data element: 737
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the broad category to which a measurement applies.
    ///
    /// The variants below are a working subset; the published list has 196 codes.
    /// Add the remainder here as `/// description` + `"CODE" => Variant,` lines.
    E737 {
        /// Average Balance
        "AB" => Ab,
        /// Maximum Balance
        "AC" => Ac,
        /// Capacity
        "AM" => Am,
        /// Work
        "AN" => An,
        /// Average Reading
        "AV" => Av,
        /// Relative Humidity
        "AW" => Aw,
        /// Billed Dimensions
        "BD" => Bd,
        /// Concentration
        "CO" => Co,
        /// Counts
        "CT" => Ct,
        /// Finished Dimensions
        "FD" => Fd,
        /// Load Planning Dimensions
        "LD" => Ld,
        /// Accuracy
        "MA" => Ma,
        /// Ordered Dimensions
        "OD" => Od,
        /// Physical Dimensions
        "PD" => Pd,
        /// Package Dimensions
        "PK" => Pk,
        /// Position
        "PO" => Po,
        /// Shipped Dimensions
        "SD" => Sd,
        /// Surface
        "SU" => Su,
        /// Temperature
        "TE" => Te,
        /// Time
        "TI" => Ti,
        /// Total Dimensions
        "TO" => To,
        /// Test Results
        "TR" => Tr,
        /// Single Test Limits
        "TS" => Ts,
        /// Weights
        "WT" => Wt,
    }
);

crate::code_enum!(
    /// **738** Measurement Qualifier
    ///
    /// - Data element: 738
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Code identifying a specific product or process characteristic to which a
    /// measurement applies.
    ///
    /// The variants below are a working subset; the published list has ~600 codes.
    /// Add the remainder here as `/// description` + `"CODE" => Variant,` lines.
    E738 {
        /// Consolidated Weight
        "A" => A,
        /// Ambient Temperature
        "AD" => Ad,
        /// Billed Weight
        "B" => B,
        /// Brightness
        "BR" => Br,
        /// Caliper
        "CA" => Ca,
        /// Diameter
        "DI" => Di,
        /// Density
        "DN" => Dn,
        /// Elongation
        "EA" => Ea,
        /// Gross Weight
        "G" => G,
        /// Height
        "HT" => Ht,
        /// Length
        "LN" => Ln,
        /// Minimum
        "MI" => Mi,
        /// Maximum
        "MX" => Mx,
        /// Actual Net Weight
        "N" => N,
        /// Outside Diameter
        "OD" => Od,
        /// Tare Weight
        "T" => T,
        /// Tenacity
        "TE" => Te,
        /// Thickness
        "TH" => Th,
        /// Volume
        "VOL" => Vol,
        /// Width
        "WD" => Wd,
        /// Weight
        "WT" => Wt,
    }
);

crate::num_element!(
    /// **739** Measurement Value
    ///
    /// - Data element: 739
    /// - Type: Decimal number (R)
    /// - Length: min 1, max 20
    ///
    /// The value of the measurement. `as_f64()`/`as_i64()` give typed views; the
    /// raw text is preserved so precision and formatting round-trip exactly.
    E739
);

crate::num_element!(
    /// **740** Range Minimum
    ///
    /// - Data element: 740
    /// - Type: Decimal number (R)
    /// - Length: min 1, max 20
    ///
    /// The minimum value of a measurement range.
    E740
);

crate::num_element!(
    /// **741** Range Maximum
    ///
    /// - Data element: 741
    /// - Type: Decimal number (R)
    /// - Length: min 1, max 20
    ///
    /// The maximum value of a measurement range.
    E741
);

crate::num_element!(
    /// **713** Installment Group Indicator
    ///
    /// - Data element: 713
    /// - Type: Numeric (N0)
    /// - Length: min 2, max 2
    ///
    /// Installment Group Indicator.
    E713
);

crate::num_element!(
    /// **719** Segment Position in Transaction Set
    ///
    /// - Data element: 719
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Segment Position in Transaction Set.
    E719
);

crate::num_element!(
    /// **723** Data Element Reference Number
    ///
    /// - Data element: 723
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 4
    ///
    /// Data Element Reference Number.
    E723
);

crate::num_element!(
    /// **761** Equipment Number Check Digit
    ///
    /// - Data element: 761
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 1
    ///
    /// Equipment Number Check Digit.
    E761
);

crate::num_element!(
    /// **768** Quantity Must Purchase
    ///
    /// - Data element: 768
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 9
    ///
    /// Quantity Must Purchase.
    E768
);

crate::num_element!(
    /// **773** Quantity Free
    ///
    /// - Data element: 773
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 9
    ///
    /// Quantity Free.
    E773
);

crate::num_element!(
    /// **782** Monetary Amount
    ///
    /// - Data element: 782
    /// - Type: Numeric (R)
    /// - Length: min 1, max 18
    ///
    /// Monetary Amount.
    E782
);

crate::code_enum!(
    /// **701** Information Type
    ///
    /// - Data element: 701
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Information Type. Code values verified against the Stedi X12 reference.
    E701 {
        /// Shipment Method of Payment
        "01" => N01,
        /// Transportation Responsibility Location
        "02" => N02,
        /// Title Passage Location
        "03" => N03,
        /// Sales Contact
        "04" => N04,
        /// Special Services that will be Required
        "05" => N05,
        /// Special Charges that will be Required
        "06" => N06,
        /// Allowance or Charge Method of Handling
        "07" => N07,
        /// Allowance or Charge Rate per Unit, Basis of Calculation or Total Amount
        "08" => N08,
        /// Type of Payment Terms
        "09" => N09,
        /// Beginning of Terms Period
        "10" => N10,
        /// Complete Discount Detail (Terms, Base/Value, Limits)
        "11" => N11,
        /// Hazardous Material
        "12" => N12,
        /// Non-Hazardous Material
        "13" => N13,
        /// As Indicated by the Cited Reference
        "14" => N14,
        /// Affiliated Companies
        "15" => N15,
        /// All Information
        "16" => N16,
        /// Background
        "17" => N17,
        /// Bank Name and Address
        "18" => N18,
        /// Bankruptcy
        "19" => N19,
        /// Bankruptcy Plan
        "20" => N20,
        /// Bankruptcy Schedule
        "21" => N21,
        /// Branches
        "22" => N22,
        /// Buying Details
        "23" => N23,
        /// Companies Dealt With
        "24" => N24,
        /// Completed Projects
        "25" => N25,
        /// Corporate Registration
        "26" => N26,
        /// Credit Score
        "27" => N27,
        /// Criminal Proceedings
        "28" => N28,
        /// Employees
        "29" => N29,
        /// Equipment
        "30" => N30,
        /// Financial Information
        "31" => N31,
        /// Legal Claim
        "32" => N32,
        /// Legal Details and Capital
        "33" => N33,
        /// Lien
        "34" => N34,
        /// Line of Business Codes
        "35" => N35,
        /// Line of Credit Details
        "36" => N36,
        /// Management Comments
        "37" => N37,
        /// Management Partial Figures
        "38" => N38,
        /// Other Public Record
        "39" => N39,
        /// Outside Comments
        "40" => N40,
        /// Ownership
        "41" => N41,
        /// Payment Breakdown
        "42" => N42,
        /// Payment Breakdown by Industry
        "43" => N43,
        /// Payment Score
        "44" => N44,
        /// Placed for Collection Details
        "45" => N45,
        /// Pledging
        "46" => N46,
        /// Preferential Claims
        "47" => N47,
        /// Protested Bills
        "48" => N48,
        /// Financial Update
        "49" => N49,
        /// Financing Statement (Uniform Commercial Code)
        "50" => N50,
        /// Full Financial Statement
        "51" => N51,
        /// General Information
        "52" => N52,
        /// Identification Numbers
        "53" => N53,
        /// Indirect Affiliate
        "54" => N54,
        /// Individual Experience
        "55" => N55,
        /// Judgment
        "56" => N56,
        /// Law Suit
        "57" => N57,
        /// Rating
        "58" => N58,
        /// Record Item
        "59" => N59,
        /// Reporting Agency Analysis
        "60" => N60,
        /// Secured Charges
        "61" => N61,
        /// Selling Details
        "62" => N62,
        /// Special Events
        "63" => N63,
        /// Subsidiaries
        "64" => N64,
        /// Summary Information
        "65" => N65,
        /// Work in Progress
        "66" => N66,
        /// Operation Details
        "67" => N67,
        /// Interview
        "68" => N68,
        /// Vendors
        "69" => N69,
        /// Slowness Comments
        "70" => N70,
        /// Subordination Agreement
        "71" => N71,
        /// Guarantees
        "72" => N72,
        /// Signing Authority
        "73" => N73,
        /// Ratios
        "74" => N74,
        /// Industry Averages
        "75" => N75,
        /// Petitions
        "76" => N76,
        /// Claims on Other Companies
        "77" => N77,
        /// Specific Details
        "78" => N78,
        /// Acquisitions
        "79" => N79,
        /// Financial Embarrassment Details
        "80" => N80,
        /// Offshore Company Details
        "81" => N81,
        /// Receivership
        "82" => N82,
        /// Ship Details
        "83" => N83,
        /// No Other Information
        "84" => N84,
        /// Ranking Details
        "85" => N85,
        /// Transportation Equipment
        "86" => N86,
        /// Delinquency Details
        "87" => N87,
        /// Failure Projection Details
        "88" => N88,
        /// Board of Directors' Details
        "89" => N89,
        /// Property Details
        "91" => N91,
        /// Management Council Details
        "92" => N92,
    }
);

crate::code_enum!(
    /// **717** Transaction Set Acknowledgment Code
    ///
    /// - Data element: 717
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Transaction Set Acknowledgment Code. Code values verified against the Stedi X12 reference.
    E717 {
        /// Accepted
        "A" => A,
        /// Accepted But Errors Were Noted
        "E" => E,
        /// Rejected, Message Authentication Code (MAC) Failed
        "M" => M,
        /// Rejected
        "R" => R,
        /// Rejected, Assurance Failed Validity Tests
        "W" => W,
        /// Rejected, Content After Decryption Could Not Be Analyzed
        "X" => X,
    }
);

crate::code_enum!(
    /// **718** Transaction Set Syntax Error Code
    ///
    /// - Data element: 718
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Transaction Set Syntax Error Code. Code values verified against the Stedi X12 reference.
    E718 {
        /// Transaction Set Not Supported
        "1" => N1,
        /// Transaction Set Trailer Missing
        "2" => N2,
        /// Transaction Set Control Number in Header and Trailer Do Not Match
        "3" => N3,
        /// Number of Included Segments Does Not Match Actual Count
        "4" => N4,
        /// One or More Segments in Error
        "5" => N5,
        /// Missing or Invalid Transaction Set Identifier
        "6" => N6,
        /// Missing or Invalid Transaction Set Control Number
        "7" => N7,
        /// Authentication Key Name Unknown
        "8" => N8,
        /// Encryption Key Name Unknown
        "9" => N9,
        /// Requested Service (Authentication or Encrypted) Not Available
        "10" => N10,
        /// Unknown Security Recipient
        "11" => N11,
        /// Incorrect Message Length (Encryption Only)
        "12" => N12,
        /// Message Authentication Code Failed
        "13" => N13,
        /// Unknown Security Originator
        "15" => N15,
        /// Syntax Error in Decrypted Text
        "16" => N16,
        /// Security Not Supported
        "17" => N17,
        /// Transaction Set not in Functional Group
        "18" => N18,
        /// Invalid Transaction Set Implementation Convention Reference
        "19" => N19,
        /// Transaction Set Control Number Not Unique within the Functional Group
        "23" => N23,
        /// S3E Security End Segment Missing for S3S Security Start Segment
        "24" => N24,
        /// S3S Security Start Segment Missing for S3E Security End Segment
        "25" => N25,
        /// S4E Security End Segment Missing for S4S Security Start Segment
        "26" => N26,
        /// S4S Security Start Segment Missing for S4E Security End Segment
        "27" => N27,
    }
);

crate::code_enum!(
    /// **720** Segment Syntax Error Code
    ///
    /// - Data element: 720
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Segment Syntax Error Code. Code values verified against the Stedi X12 reference.
    E720 {
        /// Unrecognized segment ID
        "1" => N1,
        /// Unexpected segment
        "2" => N2,
        /// Mandatory segment missing
        "3" => N3,
        /// Loop Occurs Over Maximum Times
        "4" => N4,
        /// Segment Exceeds Maximum Use
        "5" => N5,
        /// Segment Not in Defined Transaction Set
        "6" => N6,
        /// Segment Not in Proper Sequence
        "7" => N7,
        /// Segment Has Data Element Errors
        "8" => N8,
    }
);

crate::code_enum!(
    /// **743** Returnable Container Freight Payment Responsibility Code
    ///
    /// - Data element: 743
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Returnable Container Freight Payment Responsibility Code. Code values verified against the Stedi X12 reference.
    E743 {
        /// Paid By Customer
        "C" => C,
        /// Free
        "F" => F,
        /// Paid By Supplier
        "S" => S,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **744** Print Option Code
    ///
    /// - Data element: 744
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Print Option Code. Code values verified against the Stedi X12 reference.
    E744 {
        /// Container Information
        "CN" => Cn,
        /// Disclosure
        "DI" => Di,
        /// Container Size/Type
        "EN" => En,
        /// Total Weights/Volume
        "MV" => Mv,
        /// Package Count and Type
        "PN" => Pn,
        /// Rates and Charges
        "RN" => Rn,
        /// Seal Numbers
        "SN" => Sn,
        /// Tariff
        "TN" => Tn,
        /// Destination Clause On First Page
        "UF" => Uf,
        /// Volume
        "VN" => Vn,
        /// Weights
        "WN" => Wn,
        /// Weight/Measure Both English and Metric
        "XY" => Xy,
    }
);

crate::code_enum!(
    /// **748** Movement Authority Code
    ///
    /// - Data element: 748
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Movement Authority Code. Code values verified against the Stedi X12 reference.
    E748 {
        /// Released From Bad Order
        "A" => A,
        /// Bad Order Setback
        "B" => B,
        /// Car Not Weighed
        "C" => C,
        /// Empty Received as a Load
        "D" => D,
        /// Destination Switch Move
        "DS" => Ds,
        /// Embargoed Shipment
        "E" => E,
        /// Refused by Customer - Unneeded
        "G" => G,
        /// Haulage Movement
        "HM" => Hm,
        /// Interchanged in Error
        "I" => I,
        /// Not in Compliance With Special Car Order 90 or Special Car Order 100 Empty
        "L" => L,
        /// Empty - No handling required by Car Service Rules
        "M" => M,
        /// No Record Rights
        "N" => N,
        /// No Clearance
        "NC" => Nc,
        /// Not on Universal Machine Language Equipment Register
        "NU" => Nu,
        /// Overweight Shipment
        "O" => O,
        /// Old Age (Over-age)
        "OA" => Oa,
        /// Origin Switch Move
        "OS" => Os,
        /// Intermediate Switch Carrier
        "P" => P,
        /// Refused by Customer - Unfit
        "R" => R,
        /// RBOX or RGON Exception
        "RB" => Rb,
        /// Car Applied to Shipper Car Order - Place for Loading
        "S" => S,
        /// Transfer or Adjustment of Lading Car Service Rate 10, Sections 1 & 2
        "T" => T,
        /// No Waybill
        "W" => W,
        /// Association of American Railroads Restriction
        "X" => X,
    }
);

crate::code_enum!(
    /// **749** Supplementary Information Qualifier
    ///
    /// - Data element: 749
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Supplementary Information Qualifier. Code values verified against the Stedi X12 reference.
    E749 {
        /// Bill of Lading Body Clause
        "BLC" => Blc,
        /// Certified Inspection Report
        "CIR" => Cir,
        /// Certificate of Analysis
        "COA" => Coa,
        /// Commercial Invoice
        "COM" => Com,
        /// Consular Invoice
        "CON" => Con,
        /// Certified Test Report
        "CTP" => Ctp,
        /// Customs Invoice
        "CUS" => Cus,
        /// Domestic Routing Instructions
        "DOM" => Dom,
        /// Export Instructions
        "EXP" => Exp,
        /// Fumigation Certificate
        "FUM" => Fum,
        /// Insurance Certificate
        "INS" => Ins,
        /// Include Printing Instructions
        "IPI" => Ipi,
        /// Letter of Credit
        "LOC" => Loc,
        /// Material Safety Data Sheet
        "MSD" => Msd,
        /// Onboard Validation
        "OBV" => Obv,
        /// Onward Routing Instructions
        "ONR" => Onr,
        /// Certificate of Origin
        "ORI" => Ori,
        /// Packing List
        "PAK" => Pak,
        /// Quality Report
        "QAL" => Qal,
        /// Sanitary Certificate
        "SAN" => San,
        /// Shipper's Export Declaration
        "SED" => Sed,
        /// Suppress Printing Instructions
        "SPI" => Spi,
        /// Supporting Detail
        "SUP" => Sup,
        /// Sea Waybill
        "SWB" => Swb,
    }
);

crate::code_enum!(
    /// **755** Report Type Code
    ///
    /// - Data element: 755
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Report Type Code. Code values verified against the Stedi X12 reference.
    E755 {
        /// Product Transfer
        "01" => N01,
        /// Resale
        "02" => N02,
        /// Report Justifying Treatment Beyond Utilization Guidelines
        "03" => N03,
        /// Drugs Administered
        "04" => N04,
        /// Treatment Diagnosis
        "05" => N05,
        /// Initial Assessment
        "06" => N06,
        /// Functional Goals
        "07" => N07,
        /// Plan of Treatment
        "08" => N08,
        /// Progress Report
        "09" => N09,
        /// Continued Treatment
        "10" => N10,
        /// Chemical Analysis
        "11" => N11,
        /// Mechanical Properties
        "12" => N12,
        /// Certified Test Report
        "13" => N13,
        /// Core Loss Test Report
        "14" => N14,
        /// Justification for Admission
        "15" => N15,
        /// Western Region 01 (Internal Revenue Service Summary of line 31 on 1040)
        "16" => N16,
        /// Return From Tax Payer Report (Internal Revenue Service Summary)
        "17" => N17,
        /// Note Receivable
        "18" => N18,
        /// Credit Report
        "19" => N19,
        /// Process Plan
        "20" => N20,
        /// Recovery Plan
        "21" => N21,
        /// Functional Plan
        "22" => N22,
        /// Contractual Plan
        "23" => N23,
        /// Non-Contractual Plan
        "24" => N24,
        /// Purchase Plan
        "25" => N25,
        /// Production Plan
        "26" => N26,
        /// Contract Delivery Schedule
        "27" => N27,
        /// Master Delivery Schedule
        "28" => N28,
        /// Assembly Plan
        "29" => N29,
        /// Lead Time Chart
        "30" => N30,
        /// Master Schedule
        "31" => N31,
        /// Master Production Schedule
        "32" => N32,
        /// Manufacturing Flow Diagram
        "33" => N33,
        /// Lot Release Plan
        "34" => N34,
        /// Subcontractor Plan
        "35" => N35,
        /// Development Plan
        "36" => N36,
        /// Lease Agreement
        "37" => N37,
        /// Court Judgment
        "38" => N38,
        /// Lottery Winning Letter
        "39" => N39,
        /// Curb Side Inspection
        "40" => N40,
        /// Statistical Model
        "41" => N41,
        /// Interior and Exterior Inspection
        "42" => N42,
        /// Non-Inspection Report
        "43" => N43,
        /// Review Appraisal
        "44" => N44,
        /// Investor Guidelines
        "45" => N45,
        /// Sole Discretion Inspection
        "46" => N46,
        /// Brokers Price Opinion
        "47" => N47,
        /// Social Security Benefit Letter
        "48" => N48,
        /// Divorce Decree
        "49" => N49,
        /// Contract
        "50" => N50,
        /// Gift Letter
        "51" => N51,
        /// Will
        "52" => N52,
        /// Trust Agreement
        "53" => N53,
        /// Award Letter
        "54" => N54,
        /// Rental Agreement
        "55" => N55,
        /// Preliminary Real Estate Settlement Statement
        "56" => N56,
        /// Income Statement (Internal Revenue Service Form 1099)
        "57" => N57,
        /// Utility Bill
        "58" => N58,
        /// Benefit Letter
        "59" => N59,
        /// Canceled Check
        "60" => N60,
        /// Individual Tax Return (Internal Revenue Service Form 1040)
        "61" => N61,
        /// Asset Account Statement
        "62" => N62,
        /// Statement of Profit and Loss
        "63" => N63,
        /// Partner Share of Income, Credit, Deductions, (Internal Revenue Service Form K1)
        "64" => N64,
        /// Partnership Tax Return (Internal Revenue Service Form 1120)
        "65" => N65,
        /// Pay Stub
        "66" => N66,
        /// Wage and Tax Statement (Internal Revenue Service Form W2)
        "67" => N67,
        /// Year End Statement
        "68" => N68,
        /// Bank Statement
        "69" => N69,
        /// Verification of Loan/Installment Debt Form
        "70" => N70,
        /// Verification of Deposit Form
        "71" => N71,
        /// Verification of Mortgage/Rent Form
        "72" => N72,
        /// Verification of Employment Form
        "73" => N73,
        /// Corporation Tax Return (Internal Revenue Service Form 1065)
        "74" => N74,
        /// Title Certificates
        "75" => N75,
        /// Real Estate Settlement Statement (Housing and Urban Development Form - 1 "HUD1")
        "76" => N76,
        /// Support Data for Verification
        "77" => N77,
        /// Visa/Export License
        "78" => N78,
        /// Multi-Country Textile Declaration
        "79" => N79,
        /// Single Country Textile Declaration
        "80" => N80,
        /// Negative Textile Declaration
        "81" => N81,
        /// Endangered Species Convention on International Trade in Endangered Species (CITES)
        "82" => N82,
        /// Trademark Release
        "83" => N83,
        /// Water Resistance Statement
        "84" => N84,
        /// Certificate of Ceramicware Ceramics Commodities Inspection Bureau (CCIB)
        "85" => N85,
        /// Wearing Apparel Detail Sheet (WADS)
        "86" => N86,
        /// Interim Footwear Invoice
        "87" => N87,
        /// Impact Resistance Statement
        "88" => N88,
        /// Toxic Substance Compliance Statement
        "89" => N89,
        /// Foreign Shippers Declaration
        "90" => N90,
        /// Veterinarian Certificate
        "91" => N91,
        /// Child Labor Certificate
        "92" => N92,
        /// Prison Labor Certificate
        "93" => N93,
        /// Purchase Order Copy
        "94" => N94,
        /// Product Analysis
        "95" => N95,
        /// American Automotive Labeling Act Certificate
        "96" => N96,
        /// Broker Market Analysis
        "97" => N97,
        /// Air Emissions Statements
        "A1" => A1,
        /// Anti-Dumping Gasoline Program Report
        "A2" => A2,
        /// Allergies/Sensitivities Document
        "A3" => A3,
        /// Autopsy Report
        "A4" => A4,
        /// Agent Inventory Report
        "AA" => Aa,
        /// Assembly Drawing
        "AB" => Ab,
        /// Assay Certificate
        "AC" => Ac,
        /// Agent/Distributor Inventory Report
        "AD" => Ad,
        /// Attachment
        "AE" => Ae,
        /// Aid Form
        "AF" => Af,
        /// Actual
        "AG" => Ag,
        /// Easement Report
        "AH" => Ah,
        /// Conditions, Covenant, and Restrictions Report
        "AI" => Ai,
        /// Impound Account Escrow Analysis Report
        "AJ" => Aj,
        /// Closing Escrow Analysis Report
        "AK" => Ak,
        /// Reserved Escrow Analysis Report
        "AL" => Al,
        /// Ambulance Certification
        "AM" => Am,
        /// Title Policy
        "AN" => An,
        /// Average Outgoing Quality Report
        "AO" => Ao,
        /// Advanced Problem Notification
        "AP" => Ap,
        /// Housing and Urban Development (HUD) 1 Report
        "AQ" => Aq,
        /// Asset Reclassification Extension Request
        "AR" => Ar,
        /// Admission Summary
        "AS" => As,
        /// Purchase Order Attachment
        "AT" => At,
        /// Automobile Claim Report
        "AU" => Au,
        /// Averaging Areas Report
        "AV" => Av,
        /// Air Waybill
        "AW" => Aw,
        /// Asset Reclassification Extension Response
        "AX" => Ax,
        /// Tax Certificate
        "AY" => Ay,
        /// Home Owner Authorization
        "AZ" => Az,
        /// Batch Report
        "B1" => B1,
        /// Prescription
        "B2" => B2,
        /// Physician Order
        "B3" => B3,
        /// Referral Form
        "B4" => B4,
        /// Budget
        "BA" => Ba,
        /// Buy or Sell Exchange Contract Status Statement
        "BB" => Bb,
        /// Bill of Lading Copy
        "BC" => Bc,
        /// Benzene Content Averaging Report
        "BE" => Be,
        /// Bailment Warehouse Withdrawal Request
        "BF" => Bf,
        /// Bill of Lading
        "BL" => Bl,
        /// Bill of Material
        "BM" => Bm,
        /// Beneficiary Certificate
        "BN" => Bn,
        /// Bill of Lading Original
        "BO" => Bo,
        /// Benchmark Testing Results
        "BR" => Br,
        /// Baseline
        "BS" => Bs,
        /// Blanket Test Results
        "BT" => Bt,
        /// Bill of Sale
        "BW" => Bw,
        /// Biennial Report
        "BY" => By,
        /// Cost Data Summary
        "C1" => C1,
        /// Functional Cost and Hour
        "C2" => C2,
        /// Progress Curve
        "C3" => C3,
        /// Plant-Wide Data
        "C4" => C4,
        /// Certified Cost and Price Data
        "C5" => C5,
        /// Wage Determination
        "C6" => C6,
        /// Credit Transfer Summary Report
        "C7" => C7,
        /// Chemical/Radiological Report
        "C8" => C8,
        /// Certification/Authorization Document
        "C9" => C9,
        /// Certificate of Analysis
        "CA" => Ca,
        /// Chiropractic Justification
        "CB" => Cb,
        /// C.A.A. Certificate of Conformance (British CAA)
        "CC" => Cc,
        /// Customer/Distributor Inventory Report
        "CD" => Cd,
        /// Constable Report
        "CE" => Ce,
        /// Capability
        "CF" => Cf,
        /// Certificate of Origin
        "CG" => Cg,
        /// Certificate of Weight
        "CH" => Ch,
        /// Certificate of Inspection Report
        "CI" => Ci,
        /// Complications Document
        "CJ" => Cj,
        /// Consent Form(s)
        "CK" => Ck,
        /// Cable
        "CL" => Cl,
        /// Customer/Manufacturer Inventory Report
        "CM" => Cm,
        /// Customer's Report of Nonconformance
        "CN" => Cn,
        /// Consignment Order
        "CO" => Co,
        /// Certificate of Compliance (Material Certification)
        "CP" => Cp,
        /// County Record
        "CQ" => Cq,
        /// Letter of Credit
        "CR" => Cr,
        /// Consigned Inventory Sales Report
        "CS" => Cs,
        /// Certification
        "CT" => Ct,
        /// Customer Notification Letter
        "CU" => Cu,
        /// Change of Hospice Benefit
        "CV" => Cv,
        /// Corrective Work Order
        "CW" => Cw,
        /// Cost/Schedule Status Report (C/SSR)
        "CX" => Cx,
        /// Contract Funds Status Report (CFSR)
        "CY" => Cy,
        /// Campus Police Report
        "CZ" => Cz,
        /// Drug Profile Document
        "D2" => D2,
        /// Dental Models
        "DA" => Da,
        /// Durable Medical Equipment Prescription
        "DB" => Db,
        /// Distributor/Customer Inventory Report
        "DC" => Dc,
        /// Distributor Inventory Report
        "DD" => Dd,
        /// Certificate of Quality
        "DE" => De,
        /// DA59 Special Customs Invoice for South Africa
        "DF" => Df,
        /// Diagnostic Report
        "DG" => Dg,
        /// Nitrogen Certificate
        "DH" => Dh,
        /// Directory
        "DI" => Di,
        /// Discharge Monitoring Report
        "DJ" => Dj,
        /// Drawback Affidavit
        "DK" => Dk,
        /// Draft and Transmittal Letter
        "DL" => Dl,
        /// Distributor/Manufacturer Inventory Report
        "DM" => Dm,
        /// Deviation/Nonconformance Test Results and Request for Action
        "DN" => Dn,
        /// Delinquency
        "DQ" => Dq,
        /// Datalog Report
        "DR" => Dr,
        /// Discharge Summary
        "DS" => Ds,
        /// Department of Transportation
        "DT" => Dt,
        /// Commercial
        "DU" => Du,
        /// Condominium
        "DV" => Dv,
        /// Drawing(s)
        "DW" => Dw,
        /// Exporter's Certificate and Agreement
        "E1" => E1,
        /// Electrical Average Outgoing Quality Report
        "EA" => Ea,
        /// Explanation of Benefits (Coordination of Benefits or Medicare Secondary Payor)
        "EB" => Eb,
        /// Engineering Change Order
        "EC" => Ec,
        /// Environmental Exposure Document
        "ED" => Ed,
        /// Election of Hospice Benefit
        "EH" => Eh,
        /// Eligibility
        "EL" => El,
        /// Experimental Material Purchase Order
        "EP" => Ep,
        /// Engineering Change Request
        "ER" => Er,
        /// Source Selection Plan
        "ES" => Es,
        /// Shippers Export Declaration
        "EX" => Ex,
        /// Barrel for Barrel Exchange Contract Status Statement
        "EY" => Ey,
        /// Cost Performance Report (CPR) Format 1
        "F1" => F1,
        /// Cost Performance Report (CPR) Format 2
        "F2" => F2,
        /// Cost Performance Report (CPR) Format 3
        "F3" => F3,
        /// Cost Performance Report (CPR) Format 4
        "F4" => F4,
        /// Cost Performance Report (CPR) Format 5
        "F5" => F5,
        /// Transportation Carrier Inspection Report
        "F6" => F6,
        /// Government Inspection Report
        "F7" => F7,
        /// Inspection Waiver (Written)
        "F8" => F8,
        /// Inspection Waiver (Oral)
        "F9" => F9,
        /// Federal Bureau of Investigation
        "FB" => Fb,
        /// Fumigation Certificate
        "FC" => Fc,
        /// Federal Specification Compliance
        "FD" => Fd,
        /// Federal Emergency Management Agency
        "FE" => Fe,
        /// Limitation of Heavy Elements
        "FH" => Fh,
        /// Fire Report
        "FI" => Fi,
        /// Family Medical History Document
        "FM" => Fm,
        /// Post-Operative Radiology Films
        "FO" => Fo,
        /// Pre-Operative Radiology Films
        "FR" => Fr,
        /// Certificate of Free Sale
        "FS" => Fs,
        /// State Form
        "G1" => G1,
        /// Clearance Letter
        "G2" => G2,
        /// Background Release
        "G3" => G3,
        /// Exam Results
        "G4" => G4,
        /// Prelicense Certificate
        "G5" => G5,
        /// National Association of Securities Dealers Certification
        "G6" => G6,
        /// License Copy
        "G7" => G7,
        /// Gas Processor's Report
        "GP" => Gp,
        /// Gas Transporter's Report
        "GT" => Gt,
        /// Health Certificate
        "HC" => Hc,
        /// Hazardous Material Incident
        "HI" => Hi,
        /// History and Physical
        "HP" => Hp,
        /// Health Clinic Records
        "HR" => Hr,
        /// Hazardous Waste Manifest
        "HW" => Hw,
        /// Consular Invoice
        "I2" => I2,
        /// Customs Invoice
        "I3" => I3,
        /// Forwarder's Invoice
        "I4" => I4,
        /// Immunization Record
        "I5" => I5,
        /// Carrier's Invoice
        "I6" => I6,
        /// Insurance Attachment
        "IA" => Ia,
        /// Insurance Certificate
        "IC" => Ic,
        /// Import License
        "IM" => Im,
        /// Inspection Request
        "IN" => In,
        /// Inventory Parameter Report
        "IP" => Ip,
        /// State School Immunization Records
        "IR" => Ir,
        /// Index System
        "IS" => Is,
        /// Certified Inspection and Test Results
        "IT" => It,
        /// Inspection Result
        "IU" => Iu,
        /// Invoice
        "IV" => Iv,
        /// Certificate of Good Standing
        "JA" => Ja,
        /// Tax Status Clearance
        "JB" => Jb,
        /// Consent to Use Name
        "JC" => Jc,
        /// Certificate of Registration
        "JD" => Jd,
        /// Certificate of Existence
        "JE" => Je,
        /// Certificate of Status
        "JF" => Jf,
        /// Certificate of Name Change
        "JG" => Jg,
        /// Certificate of Merger
        "JH" => Jh,
        /// Certificate of Significant Change
        "JI" => Ji,
        /// Balance Sheet
        "JK" => Jk,
        /// Application of Name Reservation
        "JL" => Jl,
        /// Schedule of Capital
        "JM" => Jm,
        /// Foreign Tax Return
        "JN" => Jn,
        /// Permit Application
        "JO" => Jo,
        /// Admission Tax Return
        "JP" => Jp,
        /// Addendum to Articles
        "JQ" => Jq,
        /// Articles and Amendments
        "JR" => Jr,
        /// Appointment of Commissioner as Registered Agent
        "JS" => Js,
        /// Certificate of Disclosure
        "JT" => Jt,
        /// Notice of Registered Office
        "JV" => Jv,
        /// Notice of Directors
        "JW" => Jw,
        /// Organization and First Biennial Report
        "JX" => Jx,
        /// Agreement of Statutory Agent
        "JY" => Jy,
        /// Consent to Act
        "JZ" => Jz,
        /// Contract Data Requirements List (CDRL)
        "KA" => Ka,
        /// Kosher Certificate
        "KC" => Kc,
        /// Engineering Drawing List
        "KD" => Kd,
        /// Purchased Engineering Data List
        "KE" => Ke,
        /// Support Documents
        "KF" => Kf,
        /// Purchased Documents
        "KG" => Kg,
        /// Proposal Support Data
        "KH" => Kh,
        /// Purchased Drawings
        "KI" => Ki,
        /// Change Proposal Data
        "KJ" => Kj,
        /// Report of Assignment or Modification of Key Events
        "KY" => Ky,
        /// Request for Assignment or Modification of Key Events
        "KZ" => Kz,
        /// Laboratory Results
        "LA" => La,
        /// Legalized Bill of Lading
        "LB" => Lb,
        /// Location Inventory Report
        "LC" => Lc,
        /// Laboratory Quality Review Variation, Deviation
        "LD" => Ld,
        /// Latest Revised Estimate
        "LE" => Le,
        /// Legalized Certificate of Origin
        "LG" => Lg,
        /// Legalized Invoice
        "LI" => Li,
        /// Laboratory Quality Review Order, Waiver
        "LO" => Lo,
        /// Labor Plan
        "LP" => Lp,
        /// Laboratory Quality Review Order, Deviation
        "LR" => Lr,
        /// Lease Settlement Statement
        "LS" => Ls,
        /// License Application Attachment
        "LT" => Lt,
        /// Laboratory Quality Review Variation, Waiver
        "LW" => Lw,
        /// Medical Record Attachment
        "M1" => M1,
        /// Manufacturer/Agent Inventory Report
        "MA" => Ma,
        /// Manufacturer/Distributor Inventory Report
        "MB" => Mb,
        /// Manufacturer/Customer Inventory Report
        "MC" => Mc,
        /// Material Data Sheets
        "MD" => Md,
        /// Major Deviation Request
        "ME" => Me,
        /// Manufacturing Specification
        "MF" => Mf,
        /// Migrant Student Records Transfer System (MSRTS) Record
        "MG" => Mg,
        /// Report of Full Maintenance Period Detail
        "MH" => Mh,
        /// Mortgage Insurance Certification
        "MI" => Mi,
        /// Request for Maintenance Period Status
        "MJ" => Mj,
        /// Report of Maintenance Period Status
        "MK" => Mk,
        /// Request for Full Maintenance Period Detail
        "ML" => Ml,
        /// Manufacturer Inventory Report
        "MM" => Mm,
        /// Minor Deviation Request
        "MN" => Mn,
        /// Manufacturer's Statement of Origin
        "MO" => Mo,
        /// Request for Establishment, Modification, or Cancellation of Maintenance Period
        "MP" => Mp,
        /// Report of Establishment, Modification, or Cancellation of Maintenance Period
        "MQ" => Mq,
        /// Material Inspection and Receiving Report
        "MR" => Mr,
        /// Material Safety Data Sheet
        "MS" => Ms,
        /// Models
        "MT" => Mt,
        /// Metered Volumes
        "MV" => Mv,
        /// Motor Vehicle Report
        "MZ" => Mz,
        /// National Insurance Crime Bureau Assignment
        "NA" => Na,
        /// Certificate of Quantity
        "NC" => Nc,
        /// Commercial Invoice
        "ND" => Nd,
        /// National Insurance Crime Bureau
        "NI" => Ni,
        /// National Insurance Crime Bureau Total Loss
        "NL" => Nl,
        /// Monthly Contractor Financial Management Report
        "NM" => Nm,
        /// Nursing Notes
        "NN" => Nn,
        /// National Insurance Crime Bureau Other than Theft
        "NO" => No,
        /// Quarterly Contractor Financial Management Report
        "NQ" => Nq,
        /// NOx Emissions Averaging Report
        "NR" => Nr,
        /// National Insurance Crime Bureau Total Theft
        "NT" => Nt,
        /// Operative Note
        "OB" => Ob,
        /// Oxygen Content Averaging Report
        "OC" => Oc,
        /// Orders and Treatments Document
        "OD" => Od,
        /// Objective Physical Examination (including vital signs) Document
        "OE" => Oe,
        /// Ocean Bill of Lading
        "OL" => Ol,
        /// Outside Production Operation Sheet
        "OP" => Op,
        /// Oil Storer's Report
        "OR" => Or,
        /// Organization Breakdown Structure
        "OS" => Os,
        /// Oil Transporter's Report
        "OT" => Ot,
        /// Oxygen Therapy Certification
        "OX" => Ox,
        /// Support Data for Claim
        "OZ" => Oz,
        /// Packing List
        "P1" => P1,
        /// Protest
        "P2" => P2,
        /// Receipt
        "P3" => P3,
        /// Pathology Report
        "P4" => P4,
        /// Patient Medical History Document
        "P5" => P5,
        /// Periodontal Charts
        "P6" => P6,
        /// Periodontal Reports
        "P7" => P7,
        /// Property Claim Report
        "P8" => P8,
        /// Part Drawing
        "PA" => Pa,
        /// Product Catalog
        "PB" => Pb,
        /// Process Change Notice
        "PC" => Pc,
        /// Proof of Delivery
        "PD" => Pd,
        /// Parenteral or Enteral Certification
        "PE" => Pe,
        /// Product Specification
        "PF" => Pf,
        /// Packaging Specification
        "PG" => Pg,
        /// Production History - Property Level
        "PH" => Ph,
        /// Product Availability Inquiry
        "PI" => Pi,
        /// Purchasing Specification
        "PJ" => Pj,
        /// Storage Information Inquiry
        "PK" => Pk,
        /// Property Insurance Loss Register
        "PL" => Pl,
        /// Proof of Insurance
        "PM" => Pm,
        /// Physical Therapy Notes
        "PN" => Pn,
        /// Prosthetics or Orthotic Certification
        "PO" => Po,
        /// Proposal
        "PP" => Pp,
        /// Paramedical Results
        "PQ" => Pq,
        /// Purchase Report
        "PR" => Pr,
        /// Pipeline/Shipper Inventory Report
        "PS" => Ps,
        /// Inter-Plant Inventory Report
        "PT" => Pt,
        /// Police Report
        "PV" => Pv,
        /// Production History - Well Level
        "PW" => Pw,
        /// Production, Injection and Disposition Report
        "PX" => Px,
        /// Physician's Report
        "PY" => Py,
        /// Physical Therapy Certification
        "PZ" => Pz,
        /// Cause and Corrective Action Report
        "QC" => Qc,
        /// Quality Review Order, Purchasing
        "QD" => Qd,
        /// Quality Detail
        "QE" => Qe,
        /// Quality Review Order, Manufacturing
        "QM" => Qm,
        /// Quality Report
        "QR" => Qr,
        /// Quality Review Order Supplement
        "QS" => Qs,
        /// Quality Summary
        "QT" => Qt,
        /// Reformulated Gasoline/Anti-Dumping Company Registration
        "R1" => R1,
        /// Reformulated Gasoline/Anti-Dumping Facility Registration
        "R2" => R2,
        /// Technical Information Package
        "R3" => R3,
        /// Purchased Technical Information Package
        "R4" => R4,
        /// Technical Information
        "R5" => R5,
        /// Miscellaneous Information
        "R6" => R6,
        /// Compliance Review
        "R7" => R7,
        /// Accident
        "R9" => R9,
        /// Revision Announcement
        "RA" => Ra,
        /// Radiology Films
        "RB" => Rb,
        /// Request for Cause and Corrective Action Report
        "RC" => Rc,
        /// Payment Bond
        "RD" => Rd,
        /// Performance Bond
        "RE" => Re,
        /// Reliability Fail Rate Report
        "RF" => Rf,
        /// Residential
        "RG" => Rg,
        /// Bid Bond
        "RH" => Rh,
        /// Request for Manufacturing Engineer Appraisal
        "RM" => Rm,
        /// Supplier's Report of Nonconformance
        "RN" => Rn,
        /// Regular Order
        "RO" => Ro,
        /// Radiology Reports
        "RR" => Rr,
        /// Report of Tests and Analysis Report
        "RT" => Rt,
        /// Reid Vapor Pressure (RVP) Averaging Report
        "RV" => Rv,
        /// Renewable Oxygen Content Averaging Report
        "RX" => Rx,
        /// Supply and Shipment Status Report
        "S1" => S1,
        /// Supply Status Report
        "S2" => S2,
        /// Exception Supply Status Report
        "S3" => S3,
        /// Exception Supply and Shipment Status Report
        "S4" => S4,
        /// Product Quality Deficiency Report Category I
        "S5" => S5,
        /// Product Quality Deficiency Report Category II
        "S6" => S6,
        /// "Walsh-Healey Act" Manufacturer or Regular Dealer
        "S7" => S7,
        /// Report of Findings
        "S8" => S8,
        /// Representation
        "S9" => S9,
        /// State Police Report
        "SA" => Sa,
        /// Sample Approval and Rejection List
        "SB" => Sb,
        /// Sanitary Certificate
        "SC" => Sc,
        /// Support Data for a Request for Quote
        "SD" => Sd,
        /// Security Police Report
        "SE" => Se,
        /// Contract Security Classification Specification
        "SF" => Sf,
        /// Symptoms Document
        "SG" => Sg,
        /// Sheriff Report
        "SH" => Sh,
        /// Seller Inventory Report
        "SI" => Si,
        /// Statement of Work
        "SJ" => Sj,
        /// Sample Bale List
        "SL" => Sl,
        /// Shipping Manifests
        "SM" => Sm,
        /// Shipping Notice
        "SN" => Sn,
        /// Secretary Certificate
        "SO" => So,
        /// Specification
        "SP" => Sp,
        /// Statistical Quality Documents
        "SQ" => Sq,
        /// Statistical Report
        "SR" => Sr,
        /// Seller Sales Report
        "SS" => Ss,
        /// Student Educational Record (Transcript)
        "ST" => St,
        /// Supplier's Certificate
        "SU" => Su,
        /// Survey
        "SV" => Sv,
        /// Sea Waybill
        "SW" => Sw,
        /// Steamship Due Bill
        "SX" => Sx,
        /// Train Sheet
        "SY" => Sy,
        /// Title Bill
        "T1" => T1,
        /// Preliminary Title Work
        "T2" => T2,
        /// Loan Documents
        "T3" => T3,
        /// Tax Information
        "T4" => T4,
        /// Toxics Emissions Performance Averaging Report
        "T5" => T5,
        /// Toxics Release Inventory
        "T6" => T6,
        /// Therapy Notes
        "T7" => T7,
        /// Asset Support Inquiry
        "TA" => Ta,
        /// Asset Support Advice
        "TB" => Tb,
        /// Physical Inventory Request
        "TC" => Tc,
        /// Asset Reclassification Response
        "TD" => Td,
        /// Asset Reclassification Request
        "TE" => Te,
        /// Transaction History Request
        "TF" => Tf,
        /// Two to Four Family
        "TG" => Tg,
        /// Total Theft Claim Report
        "TH" => Th,
        /// Asset Status Inquiry
        "TI" => Ti,
        /// Asset Status Advice
        "TJ" => Tj,
        /// Logistics Transfer Inquiry
        "TK" => Tk,
        /// Logistics Transfer Advice
        "TL" => Tl,
        /// Stock Sale Report
        "TM" => Tm,
        /// Delayed Sale Report
        "TN" => Tn,
        /// Demand Report
        "TO" => To,
        /// Treatments Certificate
        "TP" => Tp,
        /// Storage Information Advice
        "TQ" => Tq,
        /// Transmittal Letter
        "TR" => Tr,
        /// Sulfur, Olefins, and T90 Averaging Report
        "TS" => Ts,
        /// Title Transfer
        "TT" => Tt,
        /// Tax-exempt Certificate
        "TX" => Tx,
        /// Survey Report
        "U1" => U1,
        /// Union Agreement
        "UA" => Ua,
        /// Certificate of Designation of Registered Agent
        "UB" => Ub,
        /// List of Officers and Directors
        "UD" => Ud,
        /// Resolution and Consent Form
        "UE" => Ue,
        /// Domestic Business Corporation Initial Report
        "UF" => Uf,
        /// Registered Agent Application
        "UG" => Ug,
        /// Articles of Incorporation
        "UH" => Uh,
        /// Certificate of Compliance
        "UI" => Ui,
        /// Certificate of Authorization
        "UJ" => Uj,
        /// Charter
        "UK" => Uk,
        /// Other Type of Report
        "UL" => Ul,
        /// Affidavit of Acceptance
        "UM" => Um,
        /// Resolution Adopting Fictitious Name
        "UN" => Un,
        /// Trade Name Application
        "UO" => Uo,
        /// Declaration of Solicitor
        "UP" => Up,
        /// Memorandum of Association
        "UQ" => Uq,
        /// Notice of Registered Agent
        "UR" => Ur,
        /// "BUY AMERICA" Certification of Compliance
        "US" => Us,
        /// Dissolution of Existing Registration
        "UU" => Uu,
        /// Appointment of Statutory Agent
        "UV" => Uv,
        /// Regulatory Approval for Professional Association
        "UX" => Ux,
        /// Initial Annual Report
        "UY" => Uy,
        /// Certificate of Fact
        "UZ" => Uz,
        /// Voter Registration Application
        "V1" => V1,
        /// Voter Registration Application Disposition
        "V2" => V2,
        /// Voter Information Record
        "V3" => V3,
        /// Change of Name and/or Address
        "V4" => V4,
        /// Death Notification
        "V5" => V5,
        /// Felony Conviction Notification
        "V6" => V6,
        /// Incompetency Notification
        "V7" => V7,
        /// Variance Analysis
        "VA" => Va,
        /// Volatile Organic Compounds (VOC) Emissions Averaging Report
        "VC" => Vc,
        /// Data Request for Vendor's Specifications or Drawings.
        "VD" => Vd,
        /// Visual/Mechanical Average Outgoing Quality Report
        "VM" => Vm,
        /// Safe Drinking Water Bacteriological Report
        "W1" => W1,
        /// Safe Drinking Water Report
        "W2" => W2,
        /// Fictitious Name Statement
        "WA" => Wa,
        /// Work Breakdown Structure
        "WB" => Wb,
        /// Request for Assignment or Deletion of Work Candidate
        "WC" => Wc,
        /// Report of Assignment or Deletion of Work Candidate
        "WD" => Wd,
        /// Business Conducted Prior to Qualification Form
        "WE" => We,
        /// By-Laws
        "WF" => Wf,
        /// Appointment of Agent for Service and Consent to Act
        "WG" => Wg,
        /// Certificate of Name Clearance
        "WH" => Wh,
        /// Well Information
        "WI" => Wi,
        /// Work Progress
        "WP" => Wp,
        /// Well Test Information
        "WT" => Wt,
        /// Complete Appraisal
        "X1" => X1,
        /// Limited Appraisal
        "X2" => X2,
        /// Self-contained Report
        "X3" => X3,
        /// Summary Report
        "X4" => X4,
        /// Restricted Report
        "X5" => X5,
        /// Equipment Test Results
        "XE" => Xe,
        /// Photographs
        "XP" => Xp,
        /// Appraisal
        "Y1" => Y1,
        /// Broker Price Opinion
        "Y2" => Y2,
        /// Real Estate Property Information
        "Y3" => Y3,
        /// Flood Determination Report
        "ZA" => Za,
        /// Conventional Ammunition Suspension Report
        "ZB" => Zb,
        /// Self Monitoring Report
        "ZC" => Zc,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **756** Report Transmission Code
    ///
    /// - Data element: 756
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Report Transmission Code. Code values verified against the Stedi X12 reference.
    E756 {
        /// Summary Electronic Invoice
        "1" => N1,
        /// Full Electronic Invoice
        "2" => N2,
        /// Printed Invoice Sent by Mail
        "3" => N3,
        /// Summary Electronic Invoice and Printed Invoice Sent by Mail
        "4" => N4,
        /// Full Electronic Invoice and Printed Invoice Sent by Mail
        "5" => N5,
        /// No paper Invoice
        "6" => N6,
        /// Summary Electronic Invoice and No paper Invoice
        "7" => N7,
        /// Full Electronic Invoice and No Paper Invoice
        "8" => N8,
        /// Electronic Mail
        "9" => N9,
        /// Available on Request at Provider Site
        "AA" => Aa,
        /// Previously Submitted to Payer
        "AB" => Ab,
        /// American College of Radiology/National Electronic Manufacturers Association (ACR/NEMA DICOM) Format
        "AC" => Ac,
        /// Certification Included in this Claim
        "AD" => Ad,
        /// Electronically After Shipping
        "AE" => Ae,
        /// Narrative Segment Included in this Claim
        "AF" => Af,
        /// No Documentation is Required
        "AG" => Ag,
        /// By Mail After Shipping
        "AM" => Am,
        /// American Society for Testing and Materials Format (ASTM E1238)
        "AS" => As,
        /// American Society for Testing and Materials Format (ASTM E1384)
        "AT" => At,
        /// By Data Pattern
        "AU" => Au,
        /// By Mail and Electronically
        "BE" => Be,
        /// By Mail
        "BM" => Bm,
        /// Best Way (Sender's Option)
        "BW" => Bw,
        /// Courier Diskette
        "CD" => Cd,
        /// Courier
        "CF" => Cf,
        /// Courier Paper
        "CP" => Cp,
        /// Courier Tape
        "CT" => Ct,
        /// Data
        "DA" => Da,
        /// Electronically Only
        "EL" => El,
        /// E-Mail
        "EM" => Em,
        /// File Transfer
        "FT" => Ft,
        /// By Fax
        "FX" => Fx,
        /// On General Services Administration (GSA) Form 10050
        "GS" => Gs,
        /// Health Industry Level 7 Interface Standards (HL/7) Format
        "HL" => Hl,
        /// Electronic Image
        "IA" => Ia,
        /// Electronically with Invoice
        "IE" => Ie,
        /// By Mail with Invoice
        "IM" => Im,
        /// Binary Image
        "MB" => Mb,
        /// Mail Diskette
        "MD" => Md,
        /// Magnetic Media
        "MN" => Mn,
        /// Mail Paper
        "MP" => Mp,
        /// Mail Tape
        "MT" => Mt,
        /// Not Specified
        "NS" => Ns,
        /// On-Line
        "OL" => Ol,
        /// Printed Original Required
        "PO" => Po,
        /// Electronically Before Shipping
        "SE" => Se,
        /// By Mail Before Shipping
        "SM" => Sm,
        /// With Ship Notice
        "SN" => Sn,
        /// Society for Worldwide Interbank Financial Telecommunication (SWIFT)
        "SW" => Sw,
        /// Telex
        "TA" => Ta,
        /// Separately, Electronically at Time of Shipping
        "TE" => Te,
        /// Separately, by Mail at Time of Shipping
        "TM" => Tm,
        /// Text
        "TX" => Tx,
        /// Voice
        "VO" => Vo,
        /// With Shipment (With Package)
        "WS" => Ws,
    }
);

crate::code_enum!(
    /// **759** Reportable Quantity Code
    ///
    /// - Data element: 759
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Reportable Quantity Code. Code values verified against the Stedi X12 reference.
    E759 {
        /// Reportable Quantity
        "RQ" => Rq,
    }
);

crate::code_enum!(
    /// **771** Market Area Code Qualifier
    ///
    /// - Data element: 771
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Market Area Code Qualifier. Code values verified against the Stedi X12 reference.
    E771 {
        /// Pricing Market
        "001" => N001,
        /// Store Number
        "002" => N002,
        /// See Free-form Text
        "003" => N003,
        /// Promotion Market
        "004" => N004,
        /// Pricing and Promotion Market-Manufacturer
        "005" => N005,
        /// All Stores
        "006" => N006,
        /// Co-op Advertising Market
        "007" => N007,
        /// Pricing and Promotion Market-Customer
        "008" => N008,
        /// State or Province Abbreviation Code
        "009" => N009,
        /// Zip Code
        "010" => N010,
        /// Zip Code Beginning Value
        "011" => N011,
        /// Zip Code Ending Value
        "012" => N012,
        /// D-U-N-S+4, D-U-N-S Number With Four Character Suffix
        "013" => N013,
        /// Store Group
        "111" => N111,
        /// Global Location Number (GLN)
        "GLN" => Gln,
    }
);

crate::code_enum!(
    /// **786** Security Level Code
    ///
    /// - Data element: 786
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Security Level Code. Code values verified against the Stedi X12 reference.
    E786 {
        /// Company Non-Classified
        "00" => N00,
        /// Company Internal Use Only
        "01" => N01,
        /// Company Confidential
        "02" => N02,
        /// Company Confidential, Restricted (Need to Know)
        "03" => N03,
        /// Company Registered (Signature Required)
        "04" => N04,
        /// Personal
        "05" => N05,
        /// Supplier Proprietary
        "06" => N06,
        /// Company Defined (Trading Partner Level)
        "09" => N09,
        /// Competition Sensitive
        "11" => N11,
        /// Court Restricted
        "20" => N20,
        /// Juvenile Record Restricted
        "21" => N21,
        /// Government Non-Classified
        "90" => N90,
        /// Government Confidential
        "92" => N92,
        /// Government Secret
        "93" => N93,
        /// Government Top Secret
        "94" => N94,
        /// Government Defined (Trading Partner Level)
        "99" => N99,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::num_element!(
    /// **725** Data Element Reference Number
    ///
    /// - Data element: 725
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 4
    ///
    /// Data Element Reference Number.
    E725
);
