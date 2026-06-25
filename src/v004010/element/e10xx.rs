//! X12 data elements 1000-1099.

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
    /// **1043** Diameter
    ///
    /// - Data element: 1043
    /// - Type: Numeric (R)
    /// - Length: min 1, max 2
    ///
    /// Diameter.
    E1043
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
    /// **1066** Citizenship Status Code
    ///
    /// - Data element: 1066
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Citizenship Status Code. Code values verified against the Stedi X12 reference.
    E1066 {
        /// U.S. Citizen
        "1" => N1,
        /// Non-Resident Alien
        "2" => N2,
        /// Resident Alien
        "3" => N3,
        /// Illegal Alien
        "4" => N4,
        /// Alien
        "5" => N5,
        /// U.S. Citizen - Non-Resident
        "6" => N6,
        /// U.S. Citizen - Resident
        "7" => N7,
        /// Citizen
        "8" => N8,
        /// Non-citizen with Student Authorization
        "9" => N9,
        /// Non-permanent Resident Alien
        "A" => A,
        /// Permanent Visa
        "B" => B,
        /// Temporary Visa
        "C" => C,
        /// Work Permit
        "D" => D,
        /// Nordic Citizen
        "E" => E,
        /// Non-Nordic Citizen
        "F" => F,
        /// Naturalized Citizen
        "G" => G,
        /// Eligible Non-citizen
        "H" => H,
        /// Ineligible Non-citizen
        "I" => I,
    }
);

crate::code_enum!(
    /// **1067** Marital Status Code
    ///
    /// - Data element: 1067
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Marital Status Code. Code values verified against the Stedi X12 reference.
    E1067 {
        /// Common Law
        "A" => A,
        /// Registered Domestic Partner
        "B" => B,
        /// Not Applicable
        "C" => C,
        /// Divorced
        "D" => D,
        /// Single
        "I" => I,
        /// Unknown
        "K" => K,
        /// Married
        "M" => M,
        /// Unreported
        "R" => R,
        /// Separated
        "S" => S,
        /// Unmarried (Single or Divorced or Widowed)
        "U" => U,
        /// Widowed
        "W" => W,
        /// Legally Separated
        "X" => X,
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
