//! X12 data elements 0500-0599.

crate::code_enum!(
    /// **571** Tare Qualifier Code
    ///
    /// - Data element: 571
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code identifying the type of tare weight.
    E571 {
        /// Actual
        "A" => A,
        /// Marked
        "M" => M,
    }
);

crate::code_enum!(
    /// **591** Payment Method Code
    ///
    /// - Data element: 591
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Code identifying the method for the movement of payment instructions (BPR-04).
    E591 {
        /// Automated Clearing House (ACH)
        "ACH" => Ach,
        /// Check
        "CHK" => Chk,
        /// Direct Deposit
        "DDP" => Ddp,
        /// Financial Institution Option
        "BOP" => Bop,
        /// Clearing House Interbank Payments System (CHIPS) Funds/Wire Transfer
        "CWT" => Cwt,
        /// Federal Reserve Funds/Wire Transfer - Nonrepetitive
        "FWT" => Fwt,
        /// Society for Worldwide Interbank Financial Telecommunications (S.W.I.F.T.)
        "SWT" => Swt,
        /// International Electronic Funds Transfer
        "IWT" => Iwt,
        /// Non-Payment Data
        "NON" => Non,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::code_enum!(
    /// **559** Agency Qualifier Code
    ///
    /// - Data element: 559
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the agency assigning the code values. Common codes named
    /// below; any other round-trips as `Unknown`.
    E559 {
        /// American Medical Association
        "AM" => Am,
        /// American Petroleum Institute
        "AP" => Ap,
        /// Department of Defense
        "DD" => Dd,
        /// Centers for Medicare and Medicaid Services
        "HC" => Hc,
        /// International Standards Organization
        "IS" => Is,
        /// Dun & Bradstreet
        "93" => N93,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **522** Amount Qualifier Code
    ///
    /// - Data element: 522
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Code to qualify amount. Common codes named below; any other round-trips as
    /// `Unknown`.
    E522 {
        /// Line Item Total
        "1" => N1,
        /// Batch Total
        "2" => N2,
        /// Deposit Total
        "3" => N3,
        /// Total Invoice Amount
        "5" => N5,
        /// Amount Subject to Total Monetary Discount
        "6" => N6,
        /// Discount Amount Due
        "7" => N7,
        /// Total Monetary Discount Amount
        "8" => N8,
        /// Total Operational Statement Amount
        "9" => N9,
        /// Deductible Amount
        "D2" => D2,
        /// Interest
        "I" => I,
        /// Premium Amount
        "P3" => P3,
        /// Tax
        "T" => T,
        /// Net
        "N" => N,
        /// Coverage Amount
        "AU" => Au,
        /// Balance Due
        "BD" => Bd,
    }
);

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
    /// **501** Customs Documentation Handling Code
    ///
    /// - Data element: 501
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Customs Documentation Handling Code. Code values verified against the Stedi X12 reference.
    E501 {
        /// Proforma Entered/B13 Not Required
        "10" => N10,
        /// Proforma and B13 Entered
        "14" => N14,
        /// Proforma Entered and B13 with Car
        "15" => N15,
        /// Proforma Entered and B13 by Summary Reporting
        "16" => N16,
        /// Proforma Entered with B13 with Broker Port of Exit
        "17" => N17,
        /// Proforma with Car/B13 Not Required
        "20" => N20,
        /// Proforma with Car and B13 Entered
        "24" => N24,
        /// Proforma and B13 with Car
        "25" => N25,
        /// Proforma with Car and B13 by Summary Reporting
        "26" => N26,
        /// Proforma with Car and B13 with Broker Port of Exit
        "27" => N27,
        /// Proforma with Broker Port of Exit/B13 Not Required
        "30" => N30,
        /// Proforma with Broker Port of Exit and B13 Entered
        "34" => N34,
        /// Proforma with Broker Port of Exit and B13 with Car
        "35" => N35,
        /// Proforma with Broker Port of Exit B13 by Summary Reporting
        "36" => N36,
        /// Proforma and B13 with Broker Port of Exit
        "37" => N37,
        /// Customs Cleared
        "40" => N40,
        /// Customs A 4 +
        "90" => N90,
    }
);

crate::code_enum!(
    /// **506** (DFI) ID Number Qualifier
    ///
    /// - Data element: 506
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// (DFI) ID Number Qualifier. Code values verified against the Stedi X12 reference.
    E506 {
        /// ABA Transit Routing Number Including Check Digits (9 digits)
        "01" => N01,
        /// Swift Identification (8 or 11 characters)
        "02" => N02,
        /// CHIPS (3 or 4 digits)
        "03" => N03,
        /// Canadian Bank Branch and Institution Number
        "04" => N04,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **521** Product Transfer Type Code
    ///
    /// - Data element: 521
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Product Transfer Type Code. Code values verified against the Stedi X12 reference.
    E521 {
        /// Demand Information Only
        "BB" => Bb,
        /// Issue - Other Agency
        "BC" => Bc,
        /// Issue - Other Department
        "BD" => Bd,
        /// Grant Aid
        "BE" => Be,
        /// Foreign Military Sale
        "BF" => Bf,
        /// Test and Evaluation
        "BG" => Bg,
        /// Furnished Goods and Services
        "BH" => Bh,
        /// Reutilization and Marketing
        "BI" => Bi,
        /// Relocation
        "BJ" => Bj,
        /// Modification
        "BK" => Bk,
        /// Repair or Nondestructive Testing
        "BL" => Bl,
        /// Loan
        "BM" => Bm,
        /// Return
        "BN" => Bn,
        /// Designated Items
        "BO" => Bo,
        /// Broken Price
        "BP" => Bp,
        /// Other
        "BQ" => Bq,
        /// Exchange
        "BR" => Br,
        /// Bailment Stock Withdrawal
        "BS" => Bs,
        /// Book Entry
        "BT" => Bt,
        /// Transfer via Depository Trust Company (DTC)
        "BU" => Bu,
        /// Physical Delivery
        "BV" => Bv,
        /// Transfer for Charitable Contribution
        "CC" => Cc,
        /// Replacement Due to Complaint
        "CO" => Co,
        /// Contract Sale
        "CS" => Cs,
        /// Oil Deliveries
        "DL" => Dl,
        /// Drop Ship Sale
        "DS" => Ds,
        /// Flowing Gas Information
        "FG" => Fg,
        /// Gas Disposition
        "GD" => Gd,
        /// Gas Plant Liquids Summary
        "GP" => Gp,
        /// Gas Receipts
        "GR" => Gr,
        /// Inventory adjustment
        "IA" => Ia,
        /// Interbranch
        "IB" => Ib,
        /// Offshore Movement/Sale
        "OF" => Of,
        /// Onshore Movement/Sale
        "ON" => On,
        /// Oil Transfer and Storage
        "OT" => Ot,
        /// Price adjustment
        "PA" => Pa,
        /// Price Book
        "PB" => Pb,
        /// Property Level Movement/Sale
        "PL" => Pl,
        /// Physical Meter Information
        "PM" => Pm,
        /// Production Origin
        "PO" => Po,
        /// Pool Level Movement and/or Sale
        "PP" => Pp,
        /// Reapplication of Order
        "RA" => Ra,
        /// Return of Broken Price
        "RB" => Rb,
        /// Received From Vendor
        "RC" => Rc,
        /// Return of List Price
        "RP" => Rp,
        /// Oil Receipts
        "RT" => Rt,
        /// Return to Usable Inventory
        "RU" => Ru,
        /// Return to Vendor
        "RV" => Rv,
        /// Stocking adjustment
        "SA" => Sa,
        /// Scrap
        "SC" => Sc,
        /// Ship and Debit Sale
        "SD" => Sd,
        /// Ship and Debit
        "SH" => Sh,
        /// Sample
        "SM" => Sm,
        /// Stock Sale
        "SS" => Ss,
        /// Summary
        "SU" => Su,
        /// Transfer for Disposal
        "TD" => Td,
        /// Well Level Movement/Sale
        "WL" => Wl,
    }
);

crate::code_enum!(
    /// **546** Status Code
    ///
    /// - Data element: 546
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Status Code. Code values verified against the Stedi X12 reference.
    E546 {
        /// Accepted
        "01" => N01,
        /// Approved
        "02" => N02,
        /// Back Bill
        "03" => N03,
        /// Conditional Issue
        "04" => N04,
        /// Converted
        "05" => N05,
        /// Deceased
        "06" => N06,
        /// Deleted
        "07" => N07,
        /// Delivery Pending
        "08" => N08,
        /// Double Jeopardy
        "09" => N09,
        /// Extended Term
        "10" => N10,
        /// Automatic
        "11" => N11,
        /// Follow Up
        "12" => N12,
        /// Facultative
        "13" => N13,
        /// Group Expired
        "14" => N14,
        /// Incomplete
        "15" => N15,
        /// In Force
        "16" => N16,
        /// Facultative Placed
        "17" => N17,
        /// Issued
        "18" => N18,
        /// New Business
        "19" => N19,
        /// Lapsed
        "20" => N20,
        /// Mailed
        "21" => N21,
        /// Policy Change
        "22" => N22,
        /// Not Taken
        "23" => N23,
        /// Offer Final
        "24" => N24,
        /// Offer - First Subject To
        "25" => N25,
        /// Paid
        "26" => N26,
        /// Paid Up
        "27" => N27,
        /// Payor Death
        "28" => N28,
        /// Payor Disabled
        "29" => N29,
        /// Pending
        "30" => N30,
        /// Placed
        "31" => N31,
        /// Postpone
        "32" => N32,
        /// Premium Paying
        "33" => N33,
        /// Quoted
        "34" => N34,
        /// Reduced Paid up
        "35" => N35,
        /// Reinstatement
        "36" => N36,
        /// Reapproved
        "37" => N37,
        /// Receipted
        "38" => N38,
        /// Reissue
        "39" => N39,
        /// Repended
        "40" => N40,
        /// Satisfied
        "41" => N41,
        /// Trial Application
        "42" => N42,
        /// Limited
        "43" => N43,
        /// Terminated
        "44" => N44,
        /// Waived
        "45" => N45,
        /// Contingent
        "46" => N46,
        /// Sold
        "47" => N47,
        /// Rented
        "48" => N48,
        /// On Probation
        "49" => N49,
        /// Under Contract
        "50" => N50,
        /// Deposit
        "51" => N51,
        /// Continue to Show
        "52" => N52,
        /// Temporarily off the Market
        "53" => N53,
        /// Leased
        "54" => N54,
        /// Off-Market
        "55" => N55,
        /// Forwarded
        "56" => N56,
        /// Not Required
        "57" => N57,
        /// Ordered
        "58" => N58,
        /// Outstanding
        "59" => N59,
        /// Pending 1035 Exchange
        "60" => N60,
        /// Required
        "61" => N61,
        /// Do Not Mail
        "62" => N62,
        /// Do Not Rent or Exchange
        "63" => N63,
        /// Filed
        "64" => N64,
        /// Facultative Submission
        "65" => N65,
        /// Additional Information
        "66" => N66,
        /// Status Request
        "67" => N67,
        /// Submission Change
        "68" => N68,
        /// Rating Reduction Requested
        "69" => N69,
        /// Facultative Opinion
        "70" => N70,
        /// Reserve Facilities
        "71" => N71,
        /// Offer Accepted
        "72" => N72,
        /// Placed Other Carrier
        "73" => N73,
        /// Capacity Unavailable
        "74" => N74,
        /// Retained
        "75" => N75,
        /// Death
        "76" => N76,
        /// Disabled
        "77" => N77,
        /// Divorced
        "78" => N78,
        /// Free Look Canceled
        "79" => N79,
        /// Processed
        "80" => N80,
        /// Rejected
        "81" => N81,
        /// Reversed
        "82" => N82,
        /// Non-Transferable
        "83" => N83,
        /// Sent for Registration Transfer to be Delivered at a Later Date
        "84" => N84,
        /// In Transfer at Time of Request
        "85" => N85,
        /// Temporary
        "86" => N86,
        /// Legal Description Provided
        "87" => N87,
        /// Legal Description Incomplete
        "88" => N88,
        /// Legal Description Provided Via Paper
        "89" => N89,
        /// Inactive Taxing Authority
        "90" => N90,
        /// Inactive Taxing Authority - No longer collects separate city tax
        "91" => N91,
        /// Inactive Taxing Authority - No longer collects separate school tax
        "92" => N92,
        /// Inactive Taxing Authority - No longer collects separate special district tax
        "93" => N93,
        /// Inactive Taxing Authority - No longer collects separate combined city and school tax
        "94" => N94,
        /// Inactive Taxing Authority - No longer collects separate county tax
        "95" => N95,
        /// Foreclosure
        "96" => N96,
        /// Under Appeal
        "98" => N98,
        /// Other Status
        "99" => N99,
        /// Annuitized
        "AA" => Aa,
        /// Arrived
        "AB" => Ab,
        /// Active
        "AC" => Ac,
        /// Applicant Denied
        "AD" => Ad,
        /// Multiple Delinquency
        "AE" => Ae,
        /// Legal Description Not Provided
        "AF" => Af,
        /// Partial Payment
        "AG" => Ag,
        /// Deeded Property
        "AH" => Ah,
        /// Money In-house
        "AI" => Ai,
        /// Returned Check
        "AJ" => Aj,
        /// Bankruptcy
        "AK" => Ak,
        /// Current Tax Open
        "AL" => Al,
        /// Current Tax Paid
        "AM" => Am,
        /// Tax Delinquent
        "AN" => An,
        /// Tax Exception or Protest
        "AO" => Ao,
        /// Tax Paid
        "AP" => Ap,
        /// Assuming Company Review
        "AR" => Ar,
        /// Contemplated
        "C1" => C1,
        /// Contained in Record
        "C2" => C2,
        /// Current
        "C3" => C3,
        /// Ceding Company Review - With Copy to Assuming Company
        "CC" => Cc,
        /// Canceled
        "CD" => Cd,
        /// Ceding Company Review - No Copy to Assuming Company
        "CE" => Ce,
        /// Canceled Free Look Cash
        "CF" => Cf,
        /// Changeable
        "CH" => Ch,
        /// Canceled by Internal Revenue Service
        "CI" => Ci,
        /// Closed
        "CL" => Cl,
        /// Complete
        "CO" => Co,
        /// Closed Record
        "CR" => Cr,
        /// Contained in Transaction
        "CT" => Ct,
        /// Declined
        "DC" => Dc,
        /// Declined - No Response from Claimant to Correspondence
        "DN" => Dn,
        /// Declined/Paid Salvage Only
        "DS" => Ds,
        /// Duplicate
        "DU" => Du,
        /// Equipment Failed
        "EF" => Ef,
        /// Estimated
        "ES" => Es,
        /// Exchange or Transfer Withdrawn
        "EW" => Ew,
        /// Expired
        "EX" => Ex,
        /// Follow Up 10 or More
        "F0" => F0,
        /// Follow Up 1
        "F1" => F1,
        /// Follow Up 2
        "F2" => F2,
        /// Follow Up 3
        "F3" => F3,
        /// Follow Up 4
        "F4" => F4,
        /// Follow Up 5
        "F5" => F5,
        /// Follow Up 6
        "F6" => F6,
        /// Follow Up 7
        "F7" => F7,
        /// Follow Up 8
        "F8" => F8,
        /// Follow Up 9
        "F9" => F9,
        /// Free Look
        "FL" => Fl,
        /// Filed Pending Further Correspondence
        "FP" => Fp,
        /// Future
        "FT" => Ft,
        /// Faxed
        "FX" => Fx,
        /// Inactive
        "IA" => Ia,
        /// Letter of Acceptance Sent
        "LA" => La,
        /// Matured
        "MT" => Mt,
        /// Not in Good Order
        "NA" => Na,
        /// New Premium Only
        "NP" => Np,
        /// No Record Claim Receipt
        "NR" => Nr,
        /// No Longer Under Treatment
        "NT" => Nt,
        /// On File
        "OF" => Of,
        /// On Hand - Under Investigation
        "OH" => Oh,
        /// Other Parties Obligations Have Been Excluded
        "OP" => Op,
        /// Pending Death
        "PD" => Pd,
        /// Paid in Full
        "PF" => Pf,
        /// Partial Payment Received
        "PM" => Pm,
        /// Paid in Part/Denied in Part
        "PP" => Pp,
        /// Paperwork Received
        "PR" => Pr,
        /// Paperwork Pending
        "PW" => Pw,
        /// Rated
        "RA" => Ra,
        /// Rescinded
        "RC" => Rc,
        /// Received
        "RE" => Re,
        /// Request
        "RQ" => Rq,
        /// Restricted
        "RS" => Rs,
        /// Revoked
        "RV" => Rv,
        /// Suspense
        "SP" => Sp,
        /// Signature Required
        "SR" => Sr,
        /// Suspended
        "SS" => Ss,
        /// Surrendered
        "SU" => Su,
        /// Under Treatment
        "UT" => Ut,
        /// Withdrawn
        "WD" => Wd,
        /// Waiting Initial Payment
        "WI" => Wi,
        /// Waiting Period
        "WP" => Wp,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **558** Reservation Action Code
    ///
    /// - Data element: 558
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Reservation Action Code. Code values verified against the Stedi X12 reference.
    E558 {
        /// Reservation Accepted
        "A" => A,
        /// Conditional Acceptance
        "B" => B,
        /// Counter Proposal Made
        "C" => C,
        /// Reservation Cancelled
        "D" => D,
        /// Reservation Declined
        "E" => E,
        /// New
        "N" => N,
        /// Delete
        "R" => R,
        /// Split Booking
        "S" => S,
        /// Change
        "U" => U,
        /// Change in Vessel
        "V" => V,
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
    /// **570** Scale Type Code
    ///
    /// - Data element: 570
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Scale Type Code. Code values verified against the Stedi X12 reference.
    E570 {
        /// Private
        "P" => P,
        /// Rail
        "R" => R,
    }
);

crate::code_enum!(
    /// **584** Employment Status Code
    ///
    /// - Data element: 584
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Employment Status Code. Code values verified against the Stedi X12 reference.
    E584 {
        /// Substitute
        "00" => N00,
        /// Leave of Absence with Pay
        "AA" => Aa,
        /// Leave of Absence without Pay
        "AB" => Ab,
        /// Active
        "AC" => Ac,
        /// Apprenticeship Full-time
        "AD" => Ad,
        /// Active Reserve
        "AE" => Ae,
        /// Flexible Work Plan
        "AF" => Af,
        /// Alerted
        "AG" => Ag,
        /// Assigned
        "AH" => Ah,
        /// Affiliated with Outside Organization
        "AI" => Ai,
        /// Adjunct
        "AJ" => Aj,
        /// Active Military - Overseas
        "AO" => Ao,
        /// Apprenticeship Part-time
        "AP" => Ap,
        /// Apprenticeship
        "AQ" => Aq,
        /// Academy Student
        "AS" => As,
        /// Presidential Appointee
        "AT" => At,
        /// Active Military - USA
        "AU" => Au,
        /// Non-applicable Employment Status Category
        "CA" => Ca,
        /// Contractor
        "CC" => Cc,
        /// Consolidated Omnibus Budget Reconciliation Act (COBRA)
        "CO" => Co,
        /// Continued
        "CT" => Ct,
        /// Discharged or Terminated for Cause
        "DC" => Dc,
        /// Dishonorably Discharged
        "DD" => Dd,
        /// Deceased
        "DI" => Di,
        /// Disqualified: Medical or Physical Condition
        "DQ" => Dq,
        /// Disqualified: Other
        "DR" => Dr,
        /// Disabled
        "DS" => Ds,
        /// Employed by Outside Organization
        "EO" => Eo,
        /// Furloughed: Job Abolished, Force Reduction
        "FA" => Fa,
        /// Furloughed: Bumped or Displaced
        "FB" => Fb,
        /// Furloughed: Facility Closed
        "FC" => Fc,
        /// Furloughed: Other
        "FO" => Fo,
        /// Full-time
        "FT" => Ft,
        /// Honorably Discharged
        "HD" => Hd,
        /// Inactive
        "IA" => Ia,
        /// Inactive Reserves
        "IR" => Ir,
        /// Leave of Absence
        "L1" => L1,
        /// Administrative Leave of Absence
        "L2" => L2,
        /// Annual Leave of Absence
        "L3" => L3,
        /// Leave of Absence due to Bereavement
        "L4" => L4,
        /// Jury Duty
        "L5" => L5,
        /// Suspension
        "L6" => L6,
        /// Sabbatical Leave of Absence
        "L7" => L7,
        /// Leave of Absence: Personal
        "LA" => La,
        /// Leave of Absence: Education
        "LE" => Le,
        /// Leave of Absence: Family Medical Leave Act (FMLA)
        "LF" => Lf,
        /// Leave of Absence: Maternity
        "LM" => Lm,
        /// Leave of Absence for Non-Military Government Request Other Than Jury Duty
        "LO" => Lo,
        /// Leave of Absence: Sickness
        "LS" => Ls,
        /// Leave of Absence: Union
        "LU" => Lu,
        /// Leave of Absence: Without Permission, Unauthorized
        "LW" => Lw,
        /// Leave of Absence: Military
        "LX" => Lx,
        /// Not Employed
        "NE" => Ne,
        /// On Strike
        "OS" => Os,
        /// Other
        "OT" => Ot,
        /// Promoted
        "PA" => Pa,
        /// Part-time Contractual
        "PC" => Pc,
        /// Plan to Enlist
        "PE" => Pe,
        /// Permanent
        "PM" => Pm,
        /// Part-time Noncontractual
        "PN" => Pn,
        /// Probationary
        "PR" => Pr,
        /// Part-time
        "PT" => Pt,
        /// Previous
        "PV" => Pv,
        /// Piece Worker
        "PW" => Pw,
        /// Resigned: Retired
        "RA" => Ra,
        /// Relocated
        "RB" => Rb,
        /// Reassigned
        "RC" => Rc,
        /// Resigned: Moved
        "RD" => Rd,
        /// Recommissioned
        "RE" => Re,
        /// Resigned: Injury
        "RI" => Ri,
        /// Retired Military - Overseas
        "RM" => Rm,
        /// Resigned: Personal Reasons
        "RP" => Rp,
        /// Retired Without Recall
        "RR" => Rr,
        /// Retired
        "RT" => Rt,
        /// Retired Military - USA
        "RU" => Ru,
        /// Dual Retired Status
        "RW" => Rw,
        /// Resigned: Accepted Separation Allowance
        "SA" => Sa,
        /// Separated
        "SB" => Sb,
        /// Self-Employed
        "SE" => Se,
        /// Seasonal
        "SL" => Sl,
        /// Suspended
        "SU" => Su,
        /// Terminated
        "TE" => Te,
        /// Temporary Full-Time
        "TF" => Tf,
        /// Temporary
        "TM" => Tm,
        /// Tenured
        "TN" => Tn,
        /// Temporary Part-Time
        "TP" => Tp,
        /// Transferred
        "TR" => Tr,
        /// Unknown
        "UK" => Uk,
        /// Volunteer
        "VO" => Vo,
        /// Extra Duties Not Requiring Certification
        "XD" => Xd,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **592** Lading Description Qualifier
    ///
    /// - Data element: 592
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Lading Description Qualifier. Code values verified against the Stedi X12 reference.
    E592 {
        /// Bill of Lading Description (Destination Country's Language)
        "B" => B,
        /// Chemical Description
        "C" => C,
        /// Department of Transportation Description
        "D" => D,
        /// Export License Description
        "E" => E,
        /// Commercial Invoice Description
        "I" => I,
        /// Letter of Credit Description
        "L" => L,
        /// Product Description
        "P" => P,
    }
);

crate::code_enum!(
    /// **594** Frequency Code
    ///
    /// - Data element: 594
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Frequency Code. Code values verified against the Stedi X12 reference.
    E594 {
        /// Annualized; 12-month equivalent
        "0" => N0,
        /// Weekly
        "1" => N1,
        /// Biweekly
        "2" => N2,
        /// Semimonthly
        "3" => N3,
        /// Monthly
        "4" => N4,
        /// Other
        "5" => N5,
        /// Daily
        "6" => N6,
        /// Annual
        "7" => N7,
        /// Two Calendar Months
        "8" => N8,
        /// Lump-Sum Separation Allowance
        "9" => N9,
        /// Quarter-to-Date
        "A" => A,
        /// Year-to-Date
        "B" => B,
        /// Single
        "C" => C,
        /// Policy Period
        "D" => D,
        /// Claim Period
        "E" => E,
        /// Unit Report Identifier
        "F" => F,
        /// Month-to-Date
        "G" => G,
        /// Hourly
        "H" => H,
        /// Current Period
        "J" => J,
        /// Quarterly
        "Q" => Q,
        /// Semiannual
        "S" => S,
        /// Unknown
        "U" => U,
        /// Mutually Defined
        "Z" => Z,
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
