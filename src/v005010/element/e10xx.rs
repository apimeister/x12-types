//! X12 data elements 1000-1099.

crate::code_enum!(
    /// **1005** Hierarchical Structure Code
    ///
    /// - Data element: 1005
    /// - Type: Identifier (ID)
    /// - Length: min 4, max 4
    ///
    /// Code indicating the hierarchical application structure of a transaction set
    /// that uses the HL segment (BHT-01). Common codes named below.
    E1005 {
        /// Shipment, Order, Packaging, Item
        "0001" => N0001,
        /// Shipment, Order, Item
        "0004" => N0004,
        /// Information Source, Receiver, Provider, Subscriber, Dependent
        "0010" => N0010,
        /// Provider of Service, Subscriber, Dependent
        "0016" => N0016,
        /// Subscriber, Dependent
        "0017" => N0017,
        /// Information Source, Receiver, Provider, Group, Site of Service
        "0024" => N0024,
        /// Group, Member
        "0059" => N0059,
        /// Information Source, Information Receiver, Company/Corporation, Operating Unit
        "0079" => N0079,
        /// Information Source, Employer, Patient
        "0080" => N0080,
        /// Information Source, Receiver, Provider of Service, Patient
        "0085" => N0085,
    }
);

crate::code_enum!(
    /// **1065** Entity Type Qualifier
    ///
    /// - Data element: 1065
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code qualifying the type of entity (NM1-02).
    E1065 {
        /// Person
        "1" => N1,
        /// Non-Person Entity
        "2" => N2,
    }
);

crate::num_element!(
    /// **1024** Number of Tank Compartments
    ///
    /// - Data element: 1024
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 2
    ///
    /// Number of Tank Compartments.
    E1024
);

crate::num_element!(
    /// **1041** Sequence Value
    ///
    /// - Data element: 1041
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Sequence Value.
    E1041
);

crate::num_element!(
    /// **1043** Diameter
    ///
    /// - Data element: 1043
    /// - Type: Numeric (R)
    /// - Length: min 1, max 2
    ///
    /// Diameter.
    E1043
);

crate::num_element!(
    /// **1095** Year
    ///
    /// - Data element: 1095
    /// - Type: Numeric (N0)
    /// - Length: min 4, max 4
    ///
    /// Year.
    E1095
);

crate::code_enum!(
    /// **1073** Yes/No Condition or Response Code
    ///
    /// - Data element: 1073
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Yes/No Condition or Response Code. Code values verified against the Stedi X12 reference.
    E1073 {
        /// No
        "N" => N,
        /// Unknown
        "U" => U,
        /// Not Applicable
        "W" => W,
        /// Yes
        "Y" => Y,
    }
);

crate::code_enum!(
    /// **1004** Percent Qualifier
    ///
    /// - Data element: 1004
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Percent Qualifier. Code values verified against the Stedi X12 reference.
    E1004 {
        /// Lease Rate Factor
        "1" => N1,
        /// Guaranteed Interest Rate
        "01" => N01,
        /// Early Termination Rate
        "2" => N2,
        /// Locked Interest Rate
        "02" => N02,
        /// Renewal Rate
        "3" => N3,
        /// Creditors to Sales Costs
        "03" => N03,
        /// Renewal Rate Cap
        "4" => N4,
        /// Interest Payable to Sales
        "04" => N04,
        /// One Family
        "5" => N5,
        /// Target Fee or Profit
        "05" => N05,
        /// 2-4 Family
        "6" => N6,
        /// Current Schedule Variance
        "06" => N06,
        /// Multifamily
        "7" => N7,
        /// Current Cost Variance
        "07" => N07,
        /// Commercial
        "8" => N8,
        /// Cumulative Schedule Variance
        "08" => N08,
        /// Other
        "9" => N9,
        /// Cumulative Cost Variance
        "09" => N09,
        /// Complete
        "10" => N10,
        /// Remaining
        "11" => N11,
        /// Vacant
        "12" => N12,
        /// Owner Occupancy
        "13" => N13,
        /// Availability Factor
        "14" => N14,
        /// Efficiency Factor
        "15" => N15,
        /// Scrap or Rework Yield
        "16" => N16,
        /// Physical (Percent) Completed
        "17" => N17,
        /// Percent of Value
        "18" => N18,
        /// Premium Rate
        "19" => N19,
        /// Loan-to-Value Balance Remaining
        "20" => N20,
        /// Annual Demand Quantity Weighting Factor
        "21" => N21,
        /// Annual Demand Value Weighting Factor
        "22" => N22,
        /// Essentiality Weighting Factor
        "23" => N23,
        /// Procurement Lead-time Weighting Factor
        "24" => N24,
        /// Months To Procurement Weighting Factor
        "25" => N25,
        /// Annual Demand Frequency Weighting Factor
        "26" => N26,
        /// Tenant Occupancy
        "27" => N27,
        /// Occupancy Rate
        "28" => N28,
        /// Working Capital Need to Sales
        "29" => N29,
        /// Active Contracts Delinquent-Contractor Caused
        "30" => N30,
        /// Active Contracts Delinquent-All Causes
        "31" => N31,
        /// Active Line Items Delinquent-Contractor Caused
        "32" => N32,
        /// Active Line Items Delinquent-All Causes
        "33" => N33,
        /// Contracts Completed Delinquent-Contractor Caused
        "34" => N34,
        /// Contracts Completed Delinquent-All Causes
        "35" => N35,
        /// Line Items Completed Delinquent-Contractor Caused
        "36" => N36,
        /// Line Items Completed Delinquent-All Causes
        "37" => N37,
        /// Delinquent Active Line Items in the Aging Population
        "38" => N38,
        /// Liquid Assets to Sales
        "39" => N39,
        /// Participation
        "40" => N40,
        /// Servicing Fee
        "41" => N41,
        /// Percentage of Original Loan Amount
        "42" => N42,
        /// Percentage of Principal Balance
        "43" => N43,
        /// Percentage of Principal and Interest Payment
        "44" => N44,
        /// Percentage of Total Mortgage Payment
        "45" => N45,
        /// Guarantee Fee Per Contract
        "46" => N46,
        /// Guarantee Fee After Alternate Payment Method
        "47" => N47,
        /// Guarantee Fee After Buyup or Buydown
        "48" => N48,
        /// Buyup or Buydown Rate per Basis Point
        "49" => N49,
        /// Security Margin
        "50" => N50,
        /// Insurance Coverage
        "51" => N51,
        /// Guarantee Amount
        "52" => N52,
        /// Participation Retained or Owned
        "53" => N53,
        /// Inventory to Cost of Goods Sold
        "54" => N54,
        /// Net Profit Margin
        "55" => N55,
        /// Financial Expenses to Sales
        "56" => N56,
        /// Return on Value Added
        "57" => N57,
        /// Employee Costs to Value Added
        "58" => N58,
        /// Basement Finished
        "59" => N59,
        /// Late Charge Rate
        "60" => N60,
        /// Total Assets to Total Liability
        "62" => N62,
        /// Indebtedness
        "65" => N65,
        /// Liquid Ratio
        "66" => N66,
        /// Net Worth to Total Assets
        "67" => N67,
        /// Subcontracted
        "68" => N68,
        /// Percent of Points Paid by Borrower
        "69" => N69,
        /// Percent of Points Paid by Seller
        "70" => N70,
        /// Percent of Points Paid by Other
        "71" => N71,
        /// Cooperative
        "72" => N72,
        /// Markup
        "73" => N73,
        /// Sales per Employee
        "76" => N76,
        /// Sales to Net Working Capital
        "77" => N77,
        /// Tax Percentage Factor
        "78" => N78,
        /// Total Liability to Net Worth
        "79" => N79,
        /// Percentage of Time
        "80" => N80,
        /// Predominant Occupancy Vacant
        "81" => N81,
        /// Asset to Sales
        "82" => N82,
        /// Capital per Employee
        "83" => N83,
        /// Cash Sales
        "84" => N84,
        /// Collection Period (Days)
        "85" => N85,
        /// Costs per Employee
        "86" => N86,
        /// Accounts Payable (Creditors) to Sales
        "87" => N87,
        /// Current Liabilities to Net Worth
        "88" => N88,
        /// Current Liabilities to Inventory (Stock)
        "89" => N89,
        /// Current Ratio
        "90" => N90,
        /// Fixed Assets to Net Worth
        "91" => N91,
        /// Inventory (Stock) Turnover
        "92" => N92,
        /// Profit per Employee
        "93" => N93,
        /// Quick Ratio (Acid Test)
        "94" => N94,
        /// Retainage Required for Project
        "95" => N95,
        /// Return on Assets
        "96" => N96,
        /// Return on Sales (Profit Margin)
        "97" => N97,
        /// Return on Capital
        "98" => N98,
        /// Shareholders' Return (Return on Net Worth)
        "99" => N99,
        /// Non-recoverable Depreciation
        "A" => A,
        /// Contracts Obtained by Negotiation
        "AB" => Ab,
        /// Cost Plus Basis
        "AC" => Ac,
        /// Lump Sum Payments
        "AD" => Ad,
        /// Purchases on Letter of Credit
        "AE" => Ae,
        /// Purchases on Floor Plan
        "AF" => Af,
        /// Change in Sales
        "AG" => Ag,
        /// Change in Profit
        "AH" => Ah,
        /// Damage
        "AI" => Ai,
        /// Interest on Liabilities
        "AJ" => Aj,
        /// Risk Margin
        "AK" => Ak,
        /// Liability Ratio
        "AL" => Al,
        /// Interest Cover
        "AM" => Am,
        /// Annual Limit
        "AN" => An,
        /// Allocation
        "AP" => Ap,
        /// Ordinance Percentage
        "AQ" => Aq,
        /// Accounts Receivable Turnover
        "AR" => Ar,
        /// Annual Interest
        "AS" => As,
        /// Apartments
        "AT" => At,
        /// Minimum Purchase Liability
        "AU" => Au,
        /// Assessment Ratio
        "AV" => Av,
        /// Annual Yield
        "AY" => Ay,
        /// Students Enrolled in Postsecondary Programs
        "AZ" => Az,
        /// Recoverable Depreciation
        "B" => B,
        /// All Shares of Stock Owned
        "BA" => Ba,
        /// Class of Stock Owned
        "BB" => Bb,
        /// Voting Shares Issued
        "BC" => Bc,
        /// Property in County
        "BD" => Bd,
        /// Property in State
        "BE" => Be,
        /// Apportionment
        "BF" => Bf,
        /// Bid Guarantee
        "BG" => Bg,
        /// Ownership
        "BH" => Bh,
        /// Fixed Assets to Total Assets
        "BI" => Bi,
        /// Inventory to Total Assets
        "BJ" => Bj,
        /// Accounts Receivable to Total Assets
        "BK" => Bk,
        /// Industry Probability of Distress
        "BL" => Bl,
        /// Inventory Financing
        "BM" => Bm,
        /// Total Base Period Wages
        "BP" => Bp,
        /// Business Portion
        "BS" => Bs,
        /// Betterment Percentage
        "BT" => Bt,
        /// Built-up
        "BU" => Bu,
        /// Depreciation
        "C" => C,
        /// Contribution
        "CA" => Ca,
        /// Commission
        "CB" => Cb,
        /// Change
        "CH" => Ch,
        /// Charge
        "CJ" => Cj,
        /// Condominium
        "CN" => Cn,
        /// Corporate
        "CO" => Co,
        /// Contract to Lease
        "CP" => Cp,
        /// Contractor Share Ratio
        "CR" => Cr,
        /// Customer Share Ratio
        "CS" => Cs,
        /// Capacity Used
        "CU" => Cu,
        /// Current Bad Debt Provision
        "CV" => Cv,
        /// Overhead on Recoverable Depreciation
        "D" => D,
        /// Decrement Factor
        "DF" => Df,
        /// Per Day Limit
        "DY" => Dy,
        /// Profit on Recoverable Depreciation
        "E" => E,
        /// Estimate Accuracy
        "EA" => Ea,
        /// Escalation Factor
        "EF" => Ef,
        /// Actual Direct Federal Support
        "EG" => Eg,
        /// Actual Direct Non-Federal Support
        "EH" => Eh,
        /// Estimated Direct Federal Support
        "EI" => Ei,
        /// Estimated Direct Non-Federal Support
        "EJ" => Ej,
        /// Federal
        "EK" => Ek,
        /// Federal Allocation
        "EL" => El,
        /// Fixed Federal
        "EM" => Em,
        /// Minimum
        "EN" => En,
        /// Non-Federal
        "EO" => Eo,
        /// Expense Percentage
        "EP" => Ep,
        /// Non-Federal Allocation
        "EQ" => Eq,
        /// Prime Interest Rate
        "ER" => Er,
        /// Inflation
        "ES" => Es,
        /// Probability of Distress
        "ET" => Et,
        /// Overhead
        "F" => F,
        /// Fault Isolation Time
        "FT" => Ft,
        /// Forecasted Vacancy and Collection Loss
        "FV" => Fv,
        /// Net Sales to Fixed Assets Ratio
        "G" => G,
        /// Gross Adjustment
        "GA" => Ga,
        /// Total Liabilities to Total Assets Ratio
        "H" => H,
        /// Intersell
        "IA" => Ia,
        /// Industrial
        "IN" => In,
        /// Discount
        "J" => J,
        /// Non-current Assets to Net Worth
        "K" => K,
        /// Variation in Quantity Over
        "KA" => Ka,
        /// Variation in Quantity Under
        "KB" => Kb,
        /// Non-current Assets to Total Assets
        "L" => L,
        /// Limit
        "LM" => Lm,
        /// Miscellaneous Ownership Percentage
        "M" => M,
        /// Maximum Allowable Withholding from Disposable Income
        "MA" => Ma,
        /// Monthly Limit
        "MN" => Mn,
        /// Market Share
        "MS" => Ms,
        /// Maximum Owned by Small Shareholders
        "MX" => Mx,
        /// Net Profit Payment Ownership Percentage
        "N" => N,
        /// Net Adjustment
        "NA" => Na,
        /// New Homes
        "NH" => Nh,
        /// Overriding Royalty Payment Ownership Percentage
        "O" => O,
        /// Overall Capitalization Rate
        "OC" => Oc,
        /// Offtake
        "OF" => Of,
        /// Outside Hours
        "OH" => Oh,
        /// Option Percentage
        "OP" => Op,
        /// Production Payment Ownership Percentage
        "P" => P,
        /// Lease Production
        "PA" => Pa,
        /// Previous Bad Debt Provision
        "PB" => Pb,
        /// Primary Coverage
        "PC" => Pc,
        /// Paid in Capital
        "PD" => Pd,
        /// Per Person Deductible
        "PF" => Pf,
        /// Per Occurrence Deductible
        "PH" => Ph,
        /// Per Person Monthly Limit
        "PM" => Pm,
        /// Per Occurrence Monthly Limit
        "PN" => Pn,
        /// Per Person Limit
        "PP" => Pp,
        /// Per Occurrence Limit
        "PR" => Pr,
        /// Previous Participation
        "PT" => Pt,
        /// Percent of Value of Real Estate Taxes
        "PV" => Pv,
        /// Percent of Funding
        "PW" => Pw,
        /// Percent Relative to Industry Average
        "Q" => Q,
        /// Percent of Day Employed
        "Q1" => Q1,
        /// Percent of Time Employed
        "Q2" => Q2,
        /// Percent of Time Employed in This District
        "Q3" => Q3,
        /// Percent of Time Spent in This Assignment
        "Q4" => Q4,
        /// Percent of Time Spent as Administrator
        "Q5" => Q5,
        /// Royalty Ownership Percentage
        "R" => R,
        /// Above Share
        "RA" => Ra,
        /// Below Share
        "RB" => Rb,
        /// Contractor Above Share
        "RC" => Rc,
        /// Contractor Below Share
        "RD" => Rd,
        /// Percent Removed
        "RE" => Re,
        /// Fixed Fee Rate
        "RF" => Rf,
        /// Progress Payment Rate
        "RG" => Rg,
        /// Incentive Fee Rate
        "RI" => Ri,
        /// Progress Payment Liquidation Rate
        "RL" => Rl,
        /// Real Estate Owned and Foreclosures
        "RO" => Ro,
        /// Royalty
        "RP" => Rp,
        /// Estimated
        "RQ" => Rq,
        /// Alternate Progress Payment Liquidation Rate
        "RR" => Rr,
        /// Resale
        "RS" => Rs,
        /// Special Overriding Royalty Ownership Percentage
        "S" => S,
        /// Set Aside
        "SA" => Sa,
        /// Subcontracted Costs
        "SC" => Sc,
        /// Moisture Content
        "SD" => Sd,
        /// Protein Content
        "SE" => Se,
        /// Percent Dockage
        "SF" => Sf,
        /// Test Weight
        "SG" => Sg,
        /// Percent from Stormwater
        "ST" => St,
        /// Tax Ownership Percentage
        "T" => T,
        /// Change in Ordinary Income
        "TA" => Ta,
        /// Change in Taxable Income
        "TB" => Tb,
        /// Common Stock Reported
        "TC" => Tc,
        /// Earnings per Share
        "TD" => Td,
        /// Equity to Deposit
        "TE" => Te,
        /// Equity to Loan
        "TF" => Tf,
        /// Invested Capital
        "TG" => Tg,
        /// Loan to Deposit
        "TH" => Th,
        /// Operates at Capacity
        "TI" => Ti,
        /// Ordinary Profit to Sales
        "TJ" => Tj,
        /// Payment Period
        "TK" => Tk,
        /// Preferred Stock Reported
        "TL" => Tl,
        /// Previous Percent of Value
        "TM" => Tm,
        /// Return on Equity of Minority Interest
        "TN" => Tn,
        /// Tract
        "TP" => Tp,
        /// Current Liabilities to Total Liabilities
        "TQ" => Tq,
        /// Gross Profit Margin
        "TR" => Tr,
        /// Current Assets to Total Liabilities
        "TS" => Ts,
        /// Tax Rate
        "TX" => Tx,
        /// Sales to Current Assets
        "U" => U,
        /// Working Capital to Sales
        "V" => V,
        /// Working Ownership Percentage
        "W" => W,
        /// Working Interest
        "WI" => Wi,
        /// Per Week Limit
        "WK" => Wk,
        /// Purchase Ownership Percentage
        "X" => X,
        /// Interest
        "X1" => X1,
        /// Percent of Day
        "X2" => X2,
        /// Percent of Week
        "X3" => X3,
        /// Percent of Year
        "X4" => X4,
        /// Asset Turnover
        "XT" => Xt,
        /// Percent Relative to National Average
        "Y" => Y,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1023** Hazard Zone Code
    ///
    /// - Data element: 1023
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Hazard Zone Code. Code values verified against the Stedi X12 reference.
    E1023 {
        /// Hazard Zone A
        "A" => A,
        /// Hazard Zone B
        "B" => B,
        /// Hazard Zone C
        "C" => C,
        /// Hazard Zone D
        "D" => D,
    }
);

crate::code_enum!(
    /// **1025** Loading or Discharge Location Code
    ///
    /// - Data element: 1025
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Loading or Discharge Location Code. Code values verified against the Stedi X12 reference.
    E1025 {
        /// Bottom
        "B" => B,
        /// Center
        "C" => C,
        /// Rear
        "R" => R,
    }
);

crate::code_enum!(
    /// **1026** Vessel Material Code
    ///
    /// - Data element: 1026
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Vessel Material Code. Code values verified against the Stedi X12 reference.
    E1026 {
        /// Other Material
        "999" => N999,
        /// Aluminum
        "ALU" => Alu,
        /// Carbon Steel
        "CST" => Cst,
        /// Fiberglass
        "FBG" => Fbg,
        /// Nickel
        "NIK" => Nik,
        /// Stainless Steel
        "SST" => Sst,
        /// Titanium
        "TTN" => Ttn,
    }
);

crate::code_enum!(
    /// **1029** Claim Status Code
    ///
    /// - Data element: 1029
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Claim Status Code. Code values verified against the Stedi X12 reference.
    E1029 {
        /// Processed as Primary
        "1" => N1,
        /// Processed as Secondary
        "2" => N2,
        /// Processed as Tertiary
        "3" => N3,
        /// Denied
        "4" => N4,
        /// Pended
        "5" => N5,
        /// Approved as amended
        "6" => N6,
        /// Approved as submitted
        "7" => N7,
        /// Cancelled due to inactivity
        "8" => N8,
        /// Pending - under investigation
        "9" => N9,
        /// Received, but not in process
        "10" => N10,
        /// Rejected, duplicate claim
        "11" => N11,
        /// Rejected, please resubmit with corrections
        "12" => N12,
        /// Suspended
        "13" => N13,
        /// Suspended - incomplete claim
        "14" => N14,
        /// Suspended - investigation with field
        "15" => N15,
        /// Suspended - return with material
        "16" => N16,
        /// Suspended - review pending
        "17" => N17,
        /// Suspended Product Registration
        "18" => N18,
        /// Processed as Primary, Forwarded to Additional Payer(s)
        "19" => N19,
        /// Processed as Secondary, Forwarded to Additional Payer(s)
        "20" => N20,
        /// Processed as Tertiary, Forwarded to Additional Payer(s)
        "21" => N21,
        /// Reversal of Previous Payment
        "22" => N22,
        /// Not Our Claim, Forwarded to Additional Payer(s)
        "23" => N23,
        /// Transferred to Proper Carrier
        "24" => N24,
        /// Predetermination Pricing Only - No Payment
        "25" => N25,
        /// Documentation Claim - No Payment Associated
        "26" => N26,
        /// Reviewed
        "27" => N27,
        /// Repriced
        "28" => N28,
        /// Audited
        "29" => N29,
        /// Processed as Conditional
        "30" => N30,
        /// Additional
        "AD" => Ad,
        /// Appealed
        "AP" => Ap,
        /// Weekly Certification
        "CC" => Cc,
        /// Closed
        "CL" => Cl,
        /// Open
        "CP" => Cp,
        /// Initial
        "I" => I,
        /// Reaudited
        "RA" => Ra,
        /// Reissue
        "RB" => Rb,
        /// Reopened and Closed
        "RC" => Rc,
        /// Redetermination
        "RD" => Rd,
        /// Reopened
        "RO" => Ro,
    }
);

crate::code_enum!(
    /// **1030** Gasket Type Code
    ///
    /// - Data element: 1030
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Gasket Type Code. Code values verified against the Stedi X12 reference.
    E1030 {
        /// Other Gasket Type
        "999" => N999,
        /// Asbestos
        "ASB" => Asb,
        /// Buna
        "BUN" => Bun,
        /// Butyl Rubber
        "BUT" => But,
        /// Ethylene Propylene (EPDM)
        "EPD" => Epd,
        /// FDA White-Food Grade
        "FDA" => Fda,
        /// Hypolon
        "HYP" => Hyp,
        /// Kalrez
        "KAL" => Kal,
        /// Leather
        "LEA" => Lea,
        /// Natural Rubber
        "NAT" => Nat,
        /// Neoprene
        "NEO" => Neo,
        /// Santoprene
        "SAN" => San,
        /// Teflon
        "TEF" => Tef,
        /// Viton
        "VIT" => Vit,
    }
);

crate::code_enum!(
    /// **1031** Trailer Lining Type Code
    ///
    /// - Data element: 1031
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Trailer Lining Type Code. Code values verified against the Stedi X12 reference.
    E1031 {
        /// Other Trailer Lining Type
        "999" => N999,
        /// Butyl Rubber
        "BUT" => But,
        /// Ceramic
        "CER" => Cer,
        /// Chlorinated Polyethylene Elastomer
        "CHL" => Chl,
        /// Derekane
        "DER" => Der,
        /// Halor
        "HAL" => Hal,
        /// Herosite
        "HER" => Her,
        /// Kynar
        "KYN" => Kyn,
        /// Natural Rubber
        "NAT" => Nat,
        /// Nickel Cladding
        "NIC" => Nic,
        /// Polypropylene
        "POL" => Pol,
        /// Teflon
        "TEF" => Tef,
        /// Viton
        "VIT" => Vit,
    }
);

crate::code_enum!(
    /// **1032** Claim Filing Indicator Code
    ///
    /// - Data element: 1032
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Claim Filing Indicator Code. Code values verified against the Stedi X12 reference.
    E1032 {
        /// Property Conveyance
        "01" => N01,
        /// Mortgage Assignment
        "02" => N02,
        /// Automatic Mortgage Assignment
        "03" => N03,
        /// Mortgage Coinsurance
        "04" => N04,
        /// Supplemental Claim
        "05" => N05,
        /// Property Nonconveyance (Claim without Conveyance of Title)
        "06" => N06,
        /// Property Preforeclosure Sale
        "07" => N07,
        /// Initial Claim
        "08" => N08,
        /// Self-pay
        "09" => N09,
        /// Central Certification
        "10" => N10,
        /// Other Non-Federal Programs
        "11" => N11,
        /// Preferred Provider Organization (PPO)
        "12" => N12,
        /// Point of Service (POS)
        "13" => N13,
        /// Exclusive Provider Organization (EPO)
        "14" => N14,
        /// Indemnity Insurance
        "15" => N15,
        /// Health Maintenance Organization (HMO) Medicare Risk
        "16" => N16,
        /// Dental Maintenance Organization
        "17" => N17,
        /// Deed-in-Lieu Property Sold
        "18" => N18,
        /// Deed-in-Lieu Property Not Sold
        "19" => N19,
        /// Foreclosure Complete Property Sold
        "20" => N20,
        /// Foreclosure Complete Property Not Sold
        "21" => N21,
        /// Liability Insurance
        "22" => N22,
        /// Special Forbearance
        "31" => N31,
        /// Loan Modifications
        "32" => N32,
        /// Partial Claim
        "33" => N33,
        /// Automobile Medical
        "AM" => Am,
        /// Blue Cross/Blue Shield
        "BL" => Bl,
        /// Champus
        "CH" => Ch,
        /// Commercial Insurance Co.
        "CI" => Ci,
        /// Contractual
        "CN" => Cn,
        /// Disability
        "DS" => Ds,
        /// Federal Employees Program
        "FI" => Fi,
        /// Health Maintenance Organization
        "HM" => Hm,
        /// Liability
        "LI" => Li,
        /// Liability Medical
        "LM" => Lm,
        /// Medicare Part A
        "MA" => Ma,
        /// Medicare Part B
        "MB" => Mb,
        /// Medicaid
        "MC" => Mc,
        /// Managed Care Non-HMO
        "MH" => Mh,
        /// Other Federal Program
        "OF" => Of,
        /// Self-administered Group
        "SA" => Sa,
        /// Title V
        "TV" => Tv,
        /// Veterans Affairs Plan
        "VA" => Va,
        /// Workers' Compensation First Report of Injury
        "WB" => Wb,
        /// Workers' Compensation Health Claim
        "WC" => Wc,
        /// Workers' Compensation Subsequent Report of Injury
        "WD" => Wd,
        /// Workers' Compensation Combined First and Subsequent Report
        "WE" => We,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1042** Load or Device Code
    ///
    /// - Data element: 1042
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Load or Device Code. Code values verified against the Stedi X12 reference.
    E1042 {
        /// Air Compressor
        "AC" => Ac,
        /// Blower
        "BL" => Bl,
        /// Blower, Stainless Steel
        "BS" => Bs,
        /// Pump, Stainless Steel
        "PS" => Ps,
        /// Pump
        "PU" => Pu,
    }
);

crate::code_enum!(
    /// **1044** Hose Type Code
    ///
    /// - Data element: 1044
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Hose Type Code. Code values verified against the Stedi X12 reference.
    E1044 {
        /// Anhydrous Ammonia
        "AAM" => Aam,
        /// Acid
        "ACD" => Acd,
        /// Cement Discharge
        "CEM" => Cem,
        /// Cross Link Polyethylene (Chemical-Solvent)
        "CRO" => Cro,
        /// FDA Tube (Food Grade)
        "FDA" => Fda,
        /// Galvanized Steel (Hot Asphalt and Tar)
        "GAL" => Gal,
        /// Hot Air Blower
        "HOT" => Hot,
        /// Liquid Propane Gas
        "LIQ" => Liq,
        /// Nitrile Tube (Petroleum)
        "NIR" => Nir,
        /// Nitrile Tube (Hot Asphalt and Tar)
        "NIT" => Nit,
        /// Pure Gum Tube (Food Grade, Dry Bulk)
        "PUR" => Pur,
        /// Stainless Steel
        "SST" => Sst,
        /// Teflon (Fluorocarbon)
        "TEF" => Tef,
        /// Viton (Fluoroelastomer)
        "VIT" => Vit,
    }
);

crate::code_enum!(
    /// **1045** Inlet or Outlet Material Type Code
    ///
    /// - Data element: 1045
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Inlet or Outlet Material Type Code. Code values verified against the Stedi X12 reference.
    E1045 {
        /// Aluminum
        "AL" => Al,
        /// Brass
        "BR" => Br,
        /// Iron
        "IR" => Ir,
        /// Steel, Nonstainless
        "NS" => Ns,
        /// Stainless Steel
        "SS" => Ss,
    }
);

crate::code_enum!(
    /// **1046** Inlet or Outlet Fitting Type Code
    ///
    /// - Data element: 1046
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Inlet or Outlet Fitting Type Code. Code values verified against the Stedi X12 reference.
    E1046 {
        /// Four Bolt Flange
        "AC" => Ac,
        /// Dry Disconnect, Female
        "DF" => Df,
        /// Dry Disconnect, Male
        "DM" => Dm,
        /// Quick Fit (Camlock), Female
        "QF" => Qf,
        /// Quick Fit (Camlock), Male
        "QM" => Qm,
        /// Threaded, Female
        "TH" => Th,
        /// Threaded, Male
        "TM" => Tm,
    }
);

crate::code_enum!(
    /// **1047** Miscellaneous Equipment Code
    ///
    /// - Data element: 1047
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Miscellaneous Equipment Code. Code values verified against the Stedi X12 reference.
    E1047 {
        /// Aerator (Dry Bulk)
        "AD" => Ad,
        /// Air Filter
        "AF" => Af,
        /// Air Dryer
        "AI" => Ai,
        /// Cyclone
        "CY" => Cy,
        /// Dry Bulk Filter
        "DB" => Db,
        /// Dry Brake Valve
        "DR" => Dr,
        /// Meter
        "ME" => Me,
        /// Nozzle, Drumming
        "ND" => Nd,
        /// Nozzle, Boxing
        "NO" => No,
        /// Pressure and or Vacuum Valve
        "PR" => Pr,
        /// Self Contained Breathing Apparatus (SCBA)
        "SC" => Sc,
        /// Service Truck
        "SE" => Se,
        /// Vibrator (Dry Bulk)
        "VI" => Vi,
    }
);

crate::code_enum!(
    /// **1053** Market Exchange Identifier
    ///
    /// - Data element: 1053
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Market Exchange Identifier. Code values verified against the Stedi X12 reference.
    E1053 {
        /// London (England) Exchange - First Closing
        "LX1" => Lx1,
        /// London (England) Exchange - Second Closing
        "LX2" => Lx2,
    }
);

crate::code_enum!(
    /// **1054** Commodity Identification
    ///
    /// - Data element: 1054
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Commodity Identification. Code values verified against the Stedi X12 reference.
    E1054 {
        /// Silver
        "AG" => Ag,
        /// Gold
        "AU" => Au,
        /// Platinum
        "PT" => Pt,
    }
);

crate::code_enum!(
    /// **1068** Gender Code
    ///
    /// - Data element: 1068
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Gender Code. Code values verified against the Stedi X12 reference.
    E1068 {
        /// Not Provided
        "A" => A,
        /// Not Applicable
        "B" => B,
        /// Female
        "F" => F,
        /// Male
        "M" => M,
        /// Non-sexed
        "N" => N,
        /// Unknown
        "U" => U,
        /// Unsexable
        "X" => X,
    }
);

crate::code_enum!(
    /// **1069** Individual Relationship Code
    ///
    /// - Data element: 1069
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Individual Relationship Code. Code values verified against the Stedi X12 reference.
    E1069 {
        /// Spouse
        "01" => N01,
        /// Son or Daughter
        "02" => N02,
        /// Father or Mother
        "03" => N03,
        /// Grandfather or Grandmother
        "04" => N04,
        /// Grandson or Granddaughter
        "05" => N05,
        /// Uncle or Aunt
        "06" => N06,
        /// Nephew or Niece
        "07" => N07,
        /// Cousin
        "08" => N08,
        /// Adopted Child
        "09" => N09,
        /// Foster Child
        "10" => N10,
        /// Son-in-law or Daughter-in-law
        "11" => N11,
        /// Brother-in-law or Sister-in-law
        "12" => N12,
        /// Mother-in-law or Father-in-law
        "13" => N13,
        /// Brother or Sister
        "14" => N14,
        /// Ward
        "15" => N15,
        /// Stepparent
        "16" => N16,
        /// Stepson or Stepdaughter
        "17" => N17,
        /// Self
        "18" => N18,
        /// Child
        "19" => N19,
        /// Employee
        "20" => N20,
        /// Unknown
        "21" => N21,
        /// Handicapped Dependent
        "22" => N22,
        /// Sponsored Dependent
        "23" => N23,
        /// Dependent of a Minor Dependent
        "24" => N24,
        /// Ex-spouse
        "25" => N25,
        /// Guardian
        "26" => N26,
        /// Student
        "27" => N27,
        /// Friend
        "28" => N28,
        /// Significant Other
        "29" => N29,
        /// Both Parents
        "30" => N30,
        /// Court Appointed Guardian
        "31" => N31,
        /// Mother
        "32" => N32,
        /// Father
        "33" => N33,
        /// Other Adult
        "34" => N34,
        /// Emancipated Minor
        "36" => N36,
        /// Agency Representative
        "37" => N37,
        /// Collateral Dependent
        "38" => N38,
        /// Organ Donor
        "39" => N39,
        /// Cadaver Donor
        "40" => N40,
        /// Injured Plaintiff
        "41" => N41,
        /// Child Where Insured Has No Financial Responsibility
        "43" => N43,
        /// Widow
        "45" => N45,
        /// Widower
        "46" => N46,
        /// State Fund
        "47" => N47,
        /// Stepfather
        "48" => N48,
        /// Stepmother
        "49" => N49,
        /// Foster Parent
        "50" => N50,
        /// Emergency Contact
        "51" => N51,
        /// Employer
        "52" => N52,
        /// Life Partner
        "53" => N53,
        /// Adopted Daughter
        "55" => N55,
        /// Adopted Son
        "56" => N56,
        /// Adoptive Father
        "57" => N57,
        /// Adoptive Mother
        "58" => N58,
        /// Adoptive Parents
        "59" => N59,
        /// Annuitant
        "60" => N60,
        /// Aunt
        "61" => N61,
        /// Brother
        "62" => N62,
        /// Brother-in-Law
        "63" => N63,
        /// Business
        "64" => N64,
        /// Business Associate
        "65" => N65,
        /// Business Insurance Trust
        "66" => N66,
        /// Business Partner
        "67" => N67,
        /// Charity
        "68" => N68,
        /// Children of Marriage
        "70" => N70,
        /// Company
        "71" => N71,
        /// Corporation
        "72" => N72,
        /// Creditor
        "73" => N73,
        /// Daughter
        "74" => N74,
        /// Daughter-in-Law
        "75" => N75,
        /// Dependent
        "76" => N76,
        /// Estate
        "78" => N78,
        /// Ex-wife
        "79" => N79,
        /// Family Member
        "80" => N80,
        /// Father-in-Law
        "81" => N81,
        /// Fiancé (Male)
        "82" => N82,
        /// Financée (Female)
        "83" => N83,
        /// Fiduciary
        "84" => N84,
        /// Foster Daughter
        "86" => N86,
        /// Foster Father
        "87" => N87,
        /// Foster Mother
        "88" => N88,
        /// Foster Son
        "90" => N90,
        /// God Daughter
        "91" => N91,
        /// God Father
        "92" => N92,
        /// God Parents
        "93" => N93,
        /// God Son
        "94" => N94,
        /// Grandchildren
        "95" => N95,
        /// Granddaughter
        "96" => N96,
        /// Grandfather
        "97" => N97,
        /// Grandmother
        "98" => N98,
        /// Grandparents
        "99" => N99,
        /// Grandson
        "A1" => A1,
        /// Great Aunt
        "A2" => A2,
        /// Ex-husband
        "A3" => A3,
        /// Half Brother
        "A4" => A4,
        /// Half Sister
        "A5" => A5,
        /// Husband
        "A6" => A6,
        /// Institution
        "A7" => A7,
        /// Mortgage Holder
        "A8" => A8,
        /// Mother-in-Law
        "A9" => A9,
        /// Nephew
        "B1" => B1,
        /// Niece
        "B2" => B2,
        /// Parents-in-Law
        "B3" => B3,
        /// Partnership
        "B4" => B4,
        /// Partner
        "B5" => B5,
        /// Personal Insurance Trust
        "B6" => B6,
        /// Sister
        "B7" => B7,
        /// Sister-in-Law
        "B8" => B8,
        /// Sole Proprietorship
        "B9" => B9,
        /// Son
        "C1" => C1,
        /// Son-in-Law
        "C2" => C2,
        /// Step Brother
        "C3" => C3,
        /// Step Children
        "C4" => C4,
        /// Step Daughter
        "C5" => C5,
        /// Step Sister
        "C8" => C8,
        /// Step Son
        "C9" => C9,
        /// Trust
        "D1" => D1,
        /// Trustee
        "D2" => D2,
        /// Uncle
        "D3" => D3,
        /// Wife
        "D4" => D4,
        /// Teacher
        "D5" => D5,
        /// School Counselor
        "D6" => D6,
        /// School Principal
        "D7" => D7,
        /// Other School Administrator
        "D8" => D8,
        /// Coach
        "D9" => D9,
        /// Activity Sponsor
        "E1" => E1,
        /// Supervisor
        "E2" => E2,
        /// Co-worker
        "E3" => E3,
        /// Minister or Priest
        "E4" => E4,
        /// Ecclesiastical or Religious Leader
        "E5" => E5,
        /// God Mother
        "E6" => E6,
        /// Probation Officer
        "E7" => E7,
        /// Accountant
        "E8" => E8,
        /// Advisor
        "E9" => E9,
        /// Alma Mater
        "F1" => F1,
        /// Applicant
        "F2" => F2,
        /// Banker
        "F3" => F3,
        /// Clergyman
        "F6" => F6,
        /// Client
        "F7" => F7,
        /// Club or Organization Officer
        "F8" => F8,
        /// Doctor
        "F9" => F9,
        /// Educator/Teacher/Instructor
        "G2" => G2,
        /// Betrothed
        "G3" => G3,
        /// Insured
        "G4" => G4,
        /// Lawyer
        "G5" => G5,
        /// Medical Care Provider
        "G6" => G6,
        /// Neighbor
        "G7" => G7,
        /// Other Relationship
        "G8" => G8,
        /// Other Relative
        "G9" => G9,
        /// Owner
        "H1" => H1,
        /// Payor
        "H4" => H4,
        /// None
        "N1" => N1,
        /// Non-applicable Individual Relationship Category
        "OT" => Ot,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **1081** Loan Purpose Code
    ///
    /// - Data element: 1081
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Loan Purpose Code. Code values verified against the Stedi X12 reference.
    E1081 {
        /// Purchase Existing Home Previously Occupied
        "01" => N01,
        /// Purchase Existing Home Not Previously Occupied
        "02" => N02,
        /// Finance Improvement to Existing Property
        "03" => N03,
        /// Construct Home
        "04" => N04,
        /// Refinance
        "05" => N05,
        /// Purchase New Condominium Unit
        "06" => N06,
        /// Purchase Existing Condominium Unit
        "07" => N07,
        /// Finance Co-Operative Purchase
        "08" => N08,
        /// Manufactured Home
        "09" => N09,
        /// Manufactured Home and Lot
        "10" => N10,
        /// Manufactured Home and To Buy Lot
        "11" => N11,
        /// Manufactured Home and Lot Loan Refinanced
        "12" => N12,
        /// Construct New Home and Convert to Permanent
        "13" => N13,
        /// Purchase Unimproved Land
        "14" => N14,
        /// Other Loan Purpose
        "15" => N15,
        /// Purchase, Purpose Unidentified
        "16" => N16,
        /// Single Family
        "17" => N17,
        /// Multifamily Construction
        "18" => N18,
        /// Multifamily Project
        "19" => N19,
        /// Single Family Serial Note
        "20" => N20,
        /// All Terrain Vehicle
        "21" => N21,
        /// Dirt Bike
        "22" => N22,
        /// Farm Equipment
        "23" => N23,
        /// Jet Ski
        "24" => N24,
        /// Truck
        "25" => N25,
        /// Motorcycle
        "26" => N26,
        /// Snowmobile
        "27" => N27,
        /// Home Equity
        "28" => N28,
        /// Purchase Money Mortgage
        "30" => N30,
        /// Additional Financing
        "31" => N31,
        /// Auto Lease
        "AL" => Al,
        /// Airplane
        "AR" => Ar,
        /// Automobile
        "AU" => Au,
        /// Boat
        "BO" => Bo,
        /// Charge Cards
        "CC" => Cc,
        /// Charged Off
        "CO" => Co,
        /// Camper or Trailer
        "CT" => Ct,
        /// First Mortgage Real Estate
        "FM" => Fm,
        /// Mobile Home
        "MH" => Mh,
        /// Personal
        "PE" => Pe,
        /// Real Estate
        "RE" => Re,
        /// Recreation Vehicle
        "RV" => Rv,
        /// Student Loan
        "SL" => Sl,
        /// Second Mortgage Real Estate
        "SM" => Sm,
        /// Sharedraft Overdraft
        "SO" => So,
    }
);

crate::code_enum!(
    /// **1085** Loan Payment Type Code
    ///
    /// - Data element: 1085
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Loan Payment Type Code. Code values verified against the Stedi X12 reference.
    E1085 {
        /// Adjustable Payment Based on Index
        "01" => N01,
        /// Fixed Payment with Balloon Option
        "02" => N02,
        /// Pledged
        "03" => N03,
        /// Growing Equity Mortgage (GEM)
        "04" => N04,
        /// Fixed Payment
        "05" => N05,
        /// Graduated Payment Mortgage (GPM)
        "06" => N06,
        /// Interest Only with Balloon
        "07" => N07,
        /// Graduated Payment Adjustable Rate Mortgage
        "08" => N08,
        /// Extended Term
        "09" => N09,
        /// Wraparound Mortgage
        "10" => N10,
        /// Collateral Pledge Graduated Payment Mortgage
        "11" => N11,
        /// Fixed Payment With Buydown
        "12" => N12,
        /// Other Loan Payment Type
        "13" => N13,
        /// Step Rate
        "14" => N14,
        /// Tiered
        "15" => N15,
        /// Renegotiated Rate
        "16" => N16,
        /// Reverse Annuity
        "17" => N17,
        /// Reverse Installment Buydown
        "18" => N18,
        /// Shared Appreciation
        "19" => N19,
        /// Second Mortgage
        "20" => N20,
        /// Interest Only
        "21" => N21,
        /// Non-level
        "22" => N22,
        /// Biweekly
        "23" => N23,
        /// Five-year Balloon
        "24" => N24,
        /// Seven-year Balloon
        "25" => N25,
        /// Regular Reducing
        "26" => N26,
        /// Skip
        "27" => N27,
        /// Balloon
        "28" => N28,
        /// Single
        "29" => N29,
        /// Fixed Principal Payment
        "30" => N30,
        /// Start Up Mortgage
        "32" => N32,
    }
);

crate::code_enum!(
    /// **1086** Loan Rate Type Code
    ///
    /// - Data element: 1086
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Loan Rate Type Code. Code values verified against the Stedi X12 reference.
    E1086 {
        /// Fixed
        "1" => N1,
        /// Adjustable or Variable
        "2" => N2,
        /// Student Loan Split
        "3" => N3,
        /// Student Loan Variable
        "4" => N4,
        /// Simple
        "5" => N5,
        /// Compound
        "6" => N6,
        /// Discount
        "7" => N7,
    }
);

crate::num_element!(
    /// **1021** Sampling Sequence Value
    ///
    /// - Data element: 1021
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Sampling Sequence Value.
    E1021
);
