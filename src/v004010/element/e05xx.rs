//! X12 data elements 0500-0599.

crate::num_element!(
    /// **554** Assigned Number
    ///
    /// - Data element: 554
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Assigned Number.
    E554
);

crate::num_element!(
    /// **567** Equipment Length
    ///
    /// - Data element: 567
    /// - Type: Numeric (N0)
    /// - Length: min 4, max 5
    ///
    /// Equipment Length.
    E567
);

crate::code_enum!(
    /// **533** Water Movement Code
    ///
    /// - Data element: 533
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Water Movement Code. Code values verified against the Stedi X12 reference.
    E533 {
        /// Prior and Subsequent Water Movement
        "B" => B,
        /// No Water Movement
        "N" => N,
        /// Prior Water Movement Only
        "P" => P,
        /// Subsequent Water Movement Only
        "S" => S,
    }
);

crate::code_enum!(
    /// **534** Inland Transportation Code
    ///
    /// - Data element: 534
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Inland Transportation Code. Code values verified against the Stedi X12 reference.
    E534 {
        /// Carrier's Overland Common Point
        "CO" => Co,
        /// Interchange
        "IC" => Ic,
        /// Inland Point Interchange (Micro Land-Bridge)
        "IP" => Ip,
        /// Mini Land-Bridge
        "ML" => Ml,
        /// Rail Overland Common Point
        "RO" => Ro,
        /// Store - Door
        "SD" => Sd,
        /// Substituted Service
        "SU" => Su,
    }
);

crate::code_enum!(
    /// **560** Special Services Code
    ///
    /// - Data element: 560
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 10
    ///
    /// Special Services Code. Code values verified against the Stedi X12 reference.
    E560 {
        /// Alterations
        "A0010" => A0010,
        /// Anneal/Heat (Steel or Glass Treatment)
        "A0020" => A0020,
        /// Art Work
        "A0030" => A0030,
        /// Tank Wash Required
        "AA" => Aa,
        /// Acid (Battery)
        "AC" => Ac,
        /// Accept at Destination
        "AD" => Ad,
        /// Affidavit
        "AF" => Af,
        /// Aircraft On Ground (AOG)
        "AG" => Ag,
        /// Administration
        "AM" => Am,
        /// Accept at Origin
        "AO" => Ao,
        /// Black Lung Tax
        "B0020" => B0020,
        /// Burning
        "B0040" => B0040,
        /// Buyer Hand Carry
        "BH" => Bh,
        /// Bill and Hold
        "BI" => Bi,
        /// Bop Sheet
        "BOP" => Bop,
        /// Shipper Load and Count
        "C1" => C1,
        /// Carrier Load and Count
        "C2" => C2,
        /// Capping
        "C0012" => C0012,
        /// Coating (Dip, Rustproof, EDP)
        "C0032" => C0032,
        /// Certificate of Conformance
        "C0036" => C0036,
        /// Certificate of Origin
        "C0038" => C0038,
        /// Cataloging Services
        "CA" => Ca,
        /// Carrier Unload
        "CC" => Cc,
        /// COD Request
        "CD" => Cd,
        /// Cut and Parallel
        "CH" => Ch,
        /// Cut
        "CI" => Ci,
        /// Continuous Movement
        "CM" => Cm,
        /// Combination Performance and Non-performance
        "CN" => Cn,
        /// Cooperative Unloading
        "CO" => Co,
        /// Cigarette Stamping
        "CS" => Cs,
        /// Canadian Customers Self Assessment (CSA)
        "CSA" => Csa,
        /// Count and Recount
        "CT" => Ct,
        /// Consignee Unload
        "CU" => Cu,
        /// Carry-in Service
        "CY" => Cy,
        /// Cable Pressurization
        "CZ" => Cz,
        /// One - Day Service
        "D1" => D1,
        /// Two - Day Service
        "D2" => D2,
        /// Determined Freight
        "D0020" => D0020,
        /// Layout/Design
        "D0024" => D0024,
        /// Driver Assisted Unloading
        "D0031" => D0031,
        /// Driver Assisted Loading
        "D0032" => D0032,
        /// Delivery Acknowledgment
        "DA" => Da,
        /// De-Installation
        "DE" => De,
        /// Drop and Hook Receiving
        "DH" => Dh,
        /// Die Service Charge
        "DI" => Di,
        /// Delivery
        "DL" => Dl,
        /// Designated Supplier Inspection
        "DS" => Ds,
        /// Declared Value
        "DV" => Dv,
        /// Exclusive use
        "E0030" => E0030,
        /// Engraving
        "EG" => Eg,
        /// Emergency Service
        "EM" => Em,
        /// Enclosure
        "EN" => En,
        /// Equipment Manufacturer Restoration Audit
        "ER" => Er,
        /// Exclusive Use Of Equipment
        "EU" => Eu,
        /// Expedited Service
        "EX" => Ex,
        /// Full Service
        "F1" => F1,
        /// Free Goods
        "FG" => Fg,
        /// Floor Stock Protection
        "FS" => Fs,
        /// Grinding
        "G0010" => G0010,
        /// Government Inspection
        "G0052" => G0052,
        /// Grouped Items
        "GI" => Gi,
        /// Gas Pressure
        "GP" => Gp,
        /// Guaranteed Inspection Technical Service
        "GU" => Gu,
        /// Temperature Protection
        "H1" => H1,
        /// Handling Service
        "HC" => Hc,
        /// Hointins and Hauling
        "HH" => Hh,
        /// Hauling and Hoisting
        "HS" => Hs,
        /// Installation
        "I0012" => I0012,
        /// Installation & Warranty
        "I0013" => I0013,
        /// Inspection
        "I0021" => I0021,
        /// Identification
        "I0022" => I0022,
        /// Inside Cable Connectors
        "IC" => Ic,
        /// Inside Delivery
        "ID" => Id,
        /// Invoice with Goods
        "IG" => Ig,
        /// Insurance Provided by Lessee
        "IK" => Ik,
        /// Insurance Provided by Lessor
        "IL" => Il,
        /// Inspect at Destination
        "IM" => Im,
        /// Insurance
        "IN" => In,
        /// Inspect at Origin
        "IO" => Io,
        /// In-stock Merchandise
        "IP" => Ip,
        /// Interim Use Permitted at Special Rate
        "IQ" => Iq,
        /// Installation and Training
        "IR" => Ir,
        /// Invoice Services
        "IS" => Is,
        /// In Transit Price Protection
        "IT" => It,
        /// Koshering
        "KO" => Ko,
        /// Shipper Load, Carrier Count
        "L1" => L1,
        /// Labeling
        "LA" => La,
        /// Loading Service
        "LL" => Ll,
        /// Lease Shortfall Consideration
        "LS" => Ls,
        /// Memo Returnable Container
        "M0010" => M0010,
        /// Mounting
        "M0042" => M0042,
        /// Mail Invoice
        "MI" => Mi,
        /// Mail Invoice to Each Location
        "ML" => Ml,
        /// Annual Maintenance
        "MNTAN" => Mntan,
        /// Monthly Maintenance
        "MNTMN" => Mntmn,
        /// One-Time Maintenance
        "MNTON" => Mnton,
        /// Non-returnable Containers
        "N0020" => N0020,
        /// Non-Returnable
        "N0021" => N0021,
        /// Notarized Affidavit
        "N0032" => N0032,
        /// Notify Consignee Before Delivery
        "NC" => Nc,
        /// Outside Cable Connectors
        "OA" => Oa,
        /// On-site Service
        "ON" => On,
        /// Oversized Package
        "OP" => Op,
        /// Painting (Primer or Finish)
        "P0012" => P0012,
        /// Phosphatizing (Steel Treatment)
        "P0014" => P0014,
        /// Pickle and Oil
        "P0016" => P0016,
        /// Plating
        "P0018" => P0018,
        /// Preparation
        "P0022" => P0022,
        /// Pack Invoice with Shipment
        "PA" => Pa,
        /// Previous Billing
        "PB" => Pb,
        /// Packaging Service
        "PC" => Pc,
        /// Pulling Eyes
        "PE" => Pe,
        /// Proof & Composition
        "PF" => Pf,
        /// Palletizing
        "PL" => Pl,
        /// Pilot Inspection
        "PLI" => Pli,
        /// Perpetual Movement
        "PM" => Pm,
        /// Purchase Option
        "PO" => Po,
        /// Progress Payment Requirement
        "PP" => Pp,
        /// Pickup Service Furnished
        "PSF" => Psf,
        /// Repair
        "R0072" => R0072,
        /// Returnable Container
        "R0076" => R0076,
        /// Returnable
        "R0077" => R0077,
        /// Rework
        "R0110" => R0110,
        /// Residential Delivery
        "RD" => Rd,
        /// Recall Service
        "RE" => Re,
        /// Roundtrip Movement
        "RM" => Rm,
        /// Return Parts to Customer
        "RP" => Rp,
        /// Shipper Load, Consignee Unload
        "S1" => S1,
        /// Slip Sheet, Truck
        "S2" => S2,
        /// Seller Hand Carry
        "S3" => S3,
        /// Shearing
        "S0014" => S0014,
        /// Shotblasting
        "S0016" => S0016,
        /// Sleeving
        "S0022" => S0022,
        /// Ship to Stock Quality Audit
        "S0024" => S0024,
        /// Special Packaging
        "S0050" => S0050,
        /// Stamping
        "S0052" => S0052,
        /// Source Inspection
        "S0054" => S0054,
        /// Strapping
        "S0056" => S0056,
        /// Supplemental Items
        "S0080" => S0080,
        /// Shrinkage Allowance
        "SD" => Sd,
        /// Same - Day Service
        "SG" => Sg,
        /// Special Handling Service
        "SH" => Sh,
        /// Subject To Tax On Resale
        "SJ" => Sj,
        /// Slip Sheet
        "SLP" => Slp,
        /// Specification Review
        "SPI" => Spi,
        /// Slip Sheet, Rail
        "SR" => Sr,
        /// Tax Liability - One Time
        "SS" => Ss,
        /// Tax Liability - Amortized
        "ST" => St,
        /// Set-up
        "SU" => Su,
        /// Service Center
        "SV" => Sv,
        /// Swell
        "SW" => Sw,
        /// Onetime Tooling
        "T1" => T1,
        /// Tools for Printing
        "T0070" => T0070,
        /// Ticketing Service
        "TC" => Tc,
        /// Tendered as Truckload
        "TE" => Te,
        /// Technology Exchange
        "TH" => Th,
        /// Consecutive Movement
        "TM" => Tm,
        /// Tooling
        "TO" => To,
        /// Unitized
        "UN" => Un,
        /// USDA Inspected, Stamping Certification
        "US" => Us,
        /// Drop Yard
        "V1" => V1,
        /// Drop Dock
        "V2" => V2,
        /// Warranties
        "W0010" => W0010,
        /// Will Call
        "WC" => Wc,
        /// Warehousing
        "WH" => Wh,
        /// Combine All Same Day Shipment
        "X0010" => X0010,
        /// Expanded Service
        "XP" => Xp,
        /// Third Party Pallets
        "XX" => Xx,
        /// Split Pickup
        "YY" => Yy,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **562** Rate or Value Type Code
    ///
    /// - Data element: 562
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Rate or Value Type Code. Code values verified against the Stedi X12 reference.
    E562 {
        /// Base Coverage
        "1" => N1,
        /// Coverage for Rider
        "2" => N2,
        /// Coverage in Units
        "3" => N3,
        /// Retention Per Life
        "4" => N4,
        /// Retention Per Policy
        "5" => N5,
        /// Total Reinsurance Amount
        "6" => N6,
        /// Face Amount
        "7" => N7,
        /// Coverage Amount
        "8" => N8,
        /// Specified Amount
        "9" => N9,
        /// Daily Basis Rate
        "A" => A,
        /// Average Basic Rate
        "AB" => Ab,
        /// Actual
        "AC" => Ac,
        /// Advance
        "AD" => Ad,
        /// Average Effective Rate
        "AE" => Ae,
        /// Age Reduced Rate
        "AR" => Ar,
        /// Bailout
        "B" => B,
        /// Blended
        "BL" => Bl,
        /// Bonus
        "BO" => Bo,
        /// Chargeback
        "CB" => Cb,
        /// Conversion Rate
        "CR" => Cr,
        /// Declared
        "D" => D,
        /// Direct Rate
        "DR" => Dr,
        /// Deferred Trail Rate
        "DT" => Dt,
        /// Earned
        "E" => E,
        /// Estimated
        "ES" => Es,
        /// Fixed
        "F" => F,
        /// Guaranteed
        "G" => G,
        /// Indexed
        "I" => I,
        /// Initial
        "IN" => In,
        /// Original
        "O" => O,
        /// Overhead or Indirect Rate
        "OI" => Oi,
        /// Other
        "OT" => Ot,
        /// Paid
        "P" => P,
        /// Participation
        "PA" => Pa,
        /// Portfolio Rate
        "PO" => Po,
        /// Renewal First Year Rate
        "RF" => Rf,
        /// Renewal Rate
        "RR" => Rr,
        /// Renewal Subsequent Year Rate
        "RS" => Rs,
        /// Special
        "SA" => Sa,
        /// Split
        "SP" => Sp,
        /// Standard Rate
        "SR" => Sr,
        /// Tiered
        "T" => T,
        /// Trail Rate
        "TR" => Tr,
        /// Unit or Lot Average
        "UA" => Ua,
        /// Unit Total
        "UT" => Ut,
        /// Variable
        "VA" => Va,
        /// Withheld
        "W" => W,
    }
);

crate::code_enum!(
    /// **563** Sales Requirement Code
    ///
    /// - Data element: 563
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Sales Requirement Code. Code values verified against the Stedi X12 reference.
    E563 {
        /// Section (8a) Set-Aside
        "8A" => N8A,
        /// Add Quantity to Make Minimum Weight Requirement
        "AA" => Aa,
        /// Notify Prior to Shipment if Quantity Specified is not Available
        "AB" => Ab,
        /// Set-aside for American Indian-owned Business
        "AI" => Ai,
        /// Restricted to Approved Sources
        "AS" => As,
        /// Back Order Only If New Item
        "B" => B,
        /// Restricted to Historically Black College or University or Minority Institution
        "BC" => Bc,
        /// Ship Partial - Balance Back Order
        "BK" => Bk,
        /// Consignment
        "C" => C,
        /// Bid Guarantee
        "D" => D,
        /// Equal Product Allowed
        "E" => E,
        /// Restricted to Educational Institutions
        "EI" => Ei,
        /// Factory Ship
        "F" => F,
        /// Ship Full Truck Only
        "FT" => Ft,
        /// Guaranteed Sale
        "GS" => Gs,
        /// Historically Underutilized Business (HUB) Zone
        "HZ" => Hz,
        /// Restricted to Industrial Preparedness Program Participants
        "IP" => Ip,
        /// Substitute Item Allowed
        "IS" => Is,
        /// Small Purchase Set Aside for Small Businesses
        "K" => K,
        /// Labor Surplus Area Set-Aside
        "LS" => Ls,
        /// Multi-year Award
        "MY" => My,
        /// No Back Order
        "N" => N,
        /// No Substitutes
        "NS" => Ns,
        /// Back Order If Items Are Out of Stock or Not Yet Published
        "O" => O,
        /// Large Purchase, Set-Aside for Small Business
        "P" => P,
        /// Ship As Soon As Possible
        "P2" => P2,
        /// May Preship
        "P3" => P3,
        /// Do Not Preship
        "P4" => P4,
        /// Small, Disadvantaged Business Set-Aside
        "Q" => Q,
        /// On Qualified Bidders List
        "QB" => Qb,
        /// Exclude Import Quota in First Cost
        "QE" => Qe,
        /// Include Import Quota in First Cost
        "QI" => Qi,
        /// On Qualified Manufacturers List
        "QM" => Qm,
        /// Restricted to Qualified Products List (QPL) Products
        "QP" => Qp,
        /// Combined Small, Disadvantaged Business and Labor Surplus Area Set-Aside
        "R" => R,
        /// Partial Labor Surplus Area Set Aside
        "S" => S,
        /// Ship Complete
        "SC" => Sc,
        /// Small Remaining Balance Cancellation Allowed
        "SE" => Se,
        /// Ship Partial, Carload Lots Only
        "SF" => Sf,
        /// Small Remaining Balance Cancellation Not Allowed
        "SG" => Sg,
        /// Ship In-Place
        "SI" => Si,
        /// Ship Partial, Balance Cancel
        "SP" => Sp,
        /// Ship Partial, Item Qty Proportional To Total Order
        "SQ" => Sq,
        /// Ship Per Schedule
        "SS" => Ss,
        /// Ship Partial, Truckload Lots Only
        "ST" => St,
        /// Ship Partial, Balance Substitute
        "SU" => Su,
        /// Ship Per Release
        "SV" => Sv,
        /// Ship Per Release or Buyer Authorization
        "SW" => Sw,
        /// Small Business with Small, Disadvantaged Business Consideration Set-Aside
        "T" => T,
        /// Restricted to U.S. and Canadian Sources
        "UC" => Uc,
        /// Other Unlisted Sales Condition
        "UL" => Ul,
        /// Unrestricted Procurement
        "UP" => Up,
        /// Warehouse Ship
        "W" => W,
        /// Warranty
        "WY" => Wy,
        /// Back Order if Out of Stock
        "Y" => Y,
        /// Restricted to Young Investigator Program
        "YI" => Yi,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **564** Do-Not-Exceed Action Code
    ///
    /// - Data element: 564
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Do-Not-Exceed Action Code. Code values verified against the Stedi X12 reference.
    E564 {
        /// Cancel Balance of Order/Item that Exceeds Value Specified in Data Element 565
        "0" => N0,
        /// Cancel Entire Order/Item
        "1" => N1,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **566** Product/Service Substitution Code
    ///
    /// - Data element: 566
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Product/Service Substitution Code. Code values verified against the Stedi X12 reference.
    E566 {
        /// No Quantity or Unit of Measure Change Allowed
        "A" => A,
        /// No Substitution Allowed
        "B0" => B0,
        /// Supply any Binding if Edition Ordered Not Available
        "B1" => B1,
        /// Supply Paper Binding if Edition Ordered Not Available
        "B2" => B2,
        /// Supply Cloth Binding if Edition Ordered Not Available
        "B3" => B3,
        /// Supply Library Binding if Edition Ordered Not Available
        "B4" => B4,
        /// Equivalent Item Substitution
        "X" => X,
        /// Alternate Item Substitution Allowed
        "Y" => Y,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **568** Electronic Form Note Reference Code
    ///
    /// - Data element: 568
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Electronic Form Note Reference Code. Code values verified against the Stedi X12 reference.
    E568 {
        /// Abstract
        "ABS" => Abs,
        /// Data Maintenance Request
        "DMR" => Dmr,
        /// Industry-required comment (Data Element 480 in segment E01 indicates which industry this comment references.)
        "IND" => Ind,
        /// Comment which is not part of the standard
        "NON" => Non,
        /// Purpose
        "PUR" => Pur,
        /// Scope
        "SCP" => Scp,
        /// Comment which is part of the standard
        "STD" => Std,
    }
);

crate::code_enum!(
    /// **569** Account Number Qualifier
    ///
    /// - Data element: 569
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Account Number Qualifier. Code values verified against the Stedi X12 reference.
    E569 {
        /// New Business Account
        "1" => N1,
        /// Time Deposit
        "01" => N01,
        /// Group Number
        "2" => N2,
        /// Policy Number
        "3" => N3,
        /// Checking Account
        "03" => N03,
        /// Money Market Fund
        "4" => N4,
        /// Retirement Account Vested Interest at Market Value
        "5" => N5,
        /// Stock
        "05" => N05,
        /// Retirement Account Vested Interest Net Cash Value
        "6" => N6,
        /// Bond
        "06" => N06,
        /// Individual Retirement Account (IRA) Vested Interest at Market Value
        "7" => N7,
        /// Life Insurance Face Value
        "07" => N07,
        /// Individual Retirement Account (IRA) Vested Interest Net Cash Value
        "8" => N8,
        /// Retirement Account - Vested Interest
        "08" => N08,
        /// Corporate Controlled Retirement Account Vested Interest
        "9" => N9,
        /// Business Account
        "10" => N10,
        /// Trust Fund Account
        "11" => N11,
        /// Stocks and Bonds Account
        "12" => N12,
        /// Life Insurance Account - Net Cash Value
        "13" => N13,
        /// Employee Retirement Account (401K)
        "14" => N14,
        /// Agency
        "AG" => Ag,
        /// Agency Location Code (ALC)
        "ALC" => Alc,
        /// Annuity
        "ANN" => Ann,
        /// Application
        "AP" => Ap,
        /// Brokerage Account
        "BA" => Ba,
        /// Charge Back Account for Returns
        "CB" => Cb,
        /// Credit Card
        "CC" => Cc,
        /// Controlled Disbursement Master Account
        "CDM" => Cdm,
        /// Controlled Disbursement Sub Account
        "CDS" => Cds,
        /// Cash Management
        "CM" => Cm,
        /// Credit Union
        "CRU" => Cru,
        /// Demand Deposit
        "DA" => Da,
        /// Debit Card
        "DC" => Dc,
        /// Existing Carrier
        "EC" => Ec,
        /// Enriched Plan Contract Number
        "EPC" => Epc,
        /// Financial Institution General Ledger Account
        "GL" => Gl,
        /// Loan Account
        "LN" => Ln,
        /// Long-term Disability Policy Number
        "LTD" => Ltd,
        /// Mutual Fund
        "MUT" => Mut,
        /// Primary Bank
        "PRI" => Pri,
        /// Return Items on Demand Deposit Account
        "RD" => Rd,
        /// Relationship Summary Account
        "REL" => Rel,
        /// Return Items on Savings Account
        "RS" => Rs,
        /// Savings
        "SG" => Sg,
        /// Safekeeping (Custody) Account
        "SKA" => Ska,
        /// Mutually Defined
        "Z" => Z,
        /// Zero Balance Master Account
        "ZB1" => Zb1,
        /// Zero Balance Affiliate Account
        "ZB2" => Zb2,
    }
);

crate::code_enum!(
    /// **580** Amendment Type Code
    ///
    /// - Data element: 580
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Amendment Type Code. Code values verified against the Stedi X12 reference.
    E580 {
        /// Add a Bill of Lading
        "A" => A,
        /// Add In-Bond Movement
        "B" => B,
        /// Add New Customs Broker
        "C" => C,
        /// Delete a Bill of Lading
        "D" => D,
        /// Delete Second Notify Party
        "N" => N,
        /// Replace a New Manifest Quantity, Either an Increase or Decrease in the Original Quantity
        "R" => R,
        /// Add Second Notify Party
        "S" => S,
    }
);

crate::code_enum!(
    /// **595** Compartment ID Code
    ///
    /// - Data element: 595
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Compartment ID Code. Code values verified against the Stedi X12 reference.
    E595 {
        /// Brake End
        "1" => N1,
        /// 2nd from Brake End
        "2" => N2,
        /// 3rd from Brake End
        "3" => N3,
        /// 4th from Brake End
        "4" => N4,
        /// 5th from Brake End
        "5" => N5,
        /// 6th from Brake End
        "6" => N6,
    }
);

crate::num_element!(
    /// **565** Do-Not-Exceed Amount
    ///
    /// - Data element: 565
    /// - Type: Numeric (N2)
    /// - Length: min 2, max 9
    ///
    /// Do-Not-Exceed Amount.
    E565
);
