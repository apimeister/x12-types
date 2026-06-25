//! X12 data elements 0600-0699.

crate::code_enum!(
    /// **668** Line Item Status Code
    ///
    /// - Data element: 668
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the action taken by the seller on a line item requested by
    /// the buyer.
    E668 {
        /// Item Accepted and Shipped
        "AC" => Ac,
        /// Item Accepted and Released for Shipment
        "AR" => Ar,
        /// Item Accepted - Partial Shipment, Balance Backordered
        "BP" => Bp,
        /// Item Accepted - Date Rescheduled
        "DR" => Dr,
        /// Item Accepted
        "IA" => Ia,
        /// Item Backordered
        "IB" => Ib,
        /// Item Accepted - Changes Made
        "IC" => Ic,
        /// Item Deleted
        "ID" => Id,
        /// Item Accepted, Price Pending
        "IE" => Ie,
        /// Item on Hold
        "IH" => Ih,
        /// Item Accepted - Price Changed
        "IP" => Ip,
        /// Item Accepted - Quantity Changed
        "IQ" => Iq,
        /// Item Rejected
        "IR" => Ir,
    }
);

crate::code_enum!(
    /// **640** Transaction Type Code
    ///
    /// - Data element: 640
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the type of transaction. Common codes named below.
    E640 {
        /// Location Address Message
        "01" => N01,
        /// Report Message
        "03" => N03,
        /// Electronic Mail Message
        "04" => N04,
        /// Normal
        "33" => N33,
        /// Emergency Request
        "34" => N34,
        /// Customer Shipment
        "42" => N42,
        /// Audit
        "49" => N49,
        /// Information Only, No Response Required
        "54" => N54,
        /// New Service Order
        "55" => N55,
        /// Sale
        "98" => N98,
        /// Order
        "OR" => Or,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **673** Quantity Qualifier
    ///
    /// - Data element: 673
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the type of quantity. Common codes named below; any other
    /// round-trips as `Unknown`.
    E673 {
        /// Discrete Quantity
        "01" => N01,
        /// Cumulative Quantity
        "02" => N02,
        /// Quantity Available to Promise
        "37" => N37,
        /// Total Receipts
        "38" => N38,
        /// Shipped Quantity
        "39" => N39,
        /// Quantity Ordered
        "41" => N41,
        /// Cumulative Quantity Required
        "63" => N63,
        /// Acceptance Quantity
        "72" => N72,
    }
);

crate::num_element!(
    /// **610** Amount
    ///
    /// - Data element: 610
    /// - Type: Numeric (N2)
    /// - Length: min 1, max 15
    ///
    /// Amount.
    E610
);

crate::num_element!(
    /// **646** Quantity Shipped to Date
    ///
    /// - Data element: 646
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Quantity Shipped to Date.
    E646
);

crate::num_element!(
    /// **649** Multiplier
    ///
    /// - Data element: 649
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Multiplier.
    E649
);

crate::num_element!(
    /// **677** Item Depth
    ///
    /// - Data element: 677
    /// - Type: Numeric (R)
    /// - Length: min 1, max 6
    ///
    /// Item Depth.
    E677
);

crate::code_enum!(
    /// **623** Time Code
    ///
    /// - Data element: 623
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Time Code. Code values verified against the Stedi X12 reference.
    E623 {
        /// Equivalent to ISO P01
        "01" => N01,
        /// Equivalent to ISO P02
        "02" => N02,
        /// Equivalent to ISO P03
        "03" => N03,
        /// Equivalent to ISO P04
        "04" => N04,
        /// Equivalent to ISO P05
        "05" => N05,
        /// Equivalent to ISO P06
        "06" => N06,
        /// Equivalent to ISO P07
        "07" => N07,
        /// Equivalent to ISO P08
        "08" => N08,
        /// Equivalent to ISO P09
        "09" => N09,
        /// Equivalent to ISO P10
        "10" => N10,
        /// Equivalent to ISO P11
        "11" => N11,
        /// Equivalent to ISO P12
        "12" => N12,
        /// Equivalent to ISO M12
        "13" => N13,
        /// Equivalent to ISO M11
        "14" => N14,
        /// Equivalent to ISO M10
        "15" => N15,
        /// Equivalent to ISO M09
        "16" => N16,
        /// Equivalent to ISO M08
        "17" => N17,
        /// Equivalent to ISO M07
        "18" => N18,
        /// Equivalent to ISO M06
        "19" => N19,
        /// Equivalent to ISO M05
        "20" => N20,
        /// Equivalent to ISO M04
        "21" => N21,
        /// Equivalent to ISO M03
        "22" => N22,
        /// Equivalent to ISO M02
        "23" => N23,
        /// Equivalent to ISO M01
        "24" => N24,
        /// Alaska Daylight Time
        "AD" => Ad,
        /// Alaska Standard Time
        "AS" => As,
        /// Alaska Time
        "AT" => At,
        /// Central Daylight Time
        "CD" => Cd,
        /// Central Standard Time
        "CS" => Cs,
        /// Central Time
        "CT" => Ct,
        /// Eastern Daylight Time
        "ED" => Ed,
        /// Eastern Standard Time
        "ES" => Es,
        /// Eastern Time
        "ET" => Et,
        /// Greenwich Mean Time
        "GM" => Gm,
        /// Hawaii-Aleutian Daylight Time
        "HD" => Hd,
        /// Hawaii-Aleutian Standard Time
        "HS" => Hs,
        /// Hawaii-Aleutian Time
        "HT" => Ht,
        /// Local Time
        "LT" => Lt,
        /// Mountain Daylight Time
        "MD" => Md,
        /// Mountain Standard Time
        "MS" => Ms,
        /// Mountain Time
        "MT" => Mt,
        /// Newfoundland Daylight Time
        "ND" => Nd,
        /// Newfoundland Standard Time
        "NS" => Ns,
        /// Newfoundland Time
        "NT" => Nt,
        /// Pacific Daylight Time
        "PD" => Pd,
        /// Pacific Standard Time
        "PS" => Ps,
        /// Pacific Time
        "PT" => Pt,
        /// Atlantic Daylight Time
        "TD" => Td,
        /// Atlantic Standard Time
        "TS" => Ts,
        /// Atlantic Time
        "TT" => Tt,
        /// Universal Time Coordinate
        "UT" => Ut,
    }
);

crate::code_enum!(
    /// **626** Excess Transportation Reason Code
    ///
    /// - Data element: 626
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Excess Transportation Reason Code. Code values verified against the Stedi X12 reference.
    E626 {
        /// Schedule Increase, Forecast Change or Special Car Order
        "A" => A,
        /// Engineering Change or Late Release
        "B" => B,
        /// Specification (Schedule) Error/Overbuilding
        "C" => C,
        /// Shipment Tracing Delay
        "D" => D,
        /// Plant Inventory Loss
        "E" => E,
        /// Building Ahead of Schedule
        "F" => F,
        /// Vendor Behind Schedule
        "G" => G,
        /// Failed to Include In Last Shipment or Unauthorized Premium
        "H" => H,
        /// Carrier Loss Claim
        "I" => I,
        /// Transportation Failure
        "J" => J,
        /// Insufficient Weight For Carload
        "K" => K,
        /// Reject or Discrepancy
        "L" => L,
        /// Transportation Delay
        "M" => M,
        /// Lack of Railcar or Railroad Equipment
        "N" => N,
        /// Releasing Error
        "P" => P,
        /// Record Error or Late Reported Discrepancy Report
        "R" => R,
        /// Common or Peculiar Part Schedule Increase
        "T" => T,
        /// Alternate Supplier Shipping for Responsible Supplier
        "U" => U,
        /// Direct Schedule or Locally Controlled
        "V" => V,
        /// Purchasing Waiver Approval
        "W" => W,
        /// Authorization Code to be Determined
        "X" => X,
        /// Pilot Material, Samples or Pre-Production
        "Y" => Y,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **627** Excess Transportation Responsibility Code
    ///
    /// - Data element: 627
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Excess Transportation Responsibility Code. Code values verified against the Stedi X12 reference.
    E627 {
        /// Customer Plant (Receiving Location)
        "A" => A,
        /// Material Release Issuer
        "B" => B,
        /// Supplier Authority
        "S" => S,
        /// Responsibility to be Determined
        "X" => X,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **639** Basis of Unit Price Code
    ///
    /// - Data element: 639
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Basis of Unit Price Code. Code values verified against the Stedi X12 reference.
    E639 {
        /// Bill
        "AA" => Aa,
        /// Pay
        "AB" => Ab,
        /// Advise Price
        "AP" => Ap,
        /// Average Wholesale Price
        "AW" => Aw,
        /// Before Discount
        "BD" => Bd,
        /// Broker
        "BR" => Br,
        /// Biweekly Price per Unit
        "BW" => Bw,
        /// Catalog
        "CA" => Ca,
        /// Current Price (Subject to Change)
        "CP" => Cp,
        /// Carnet
        "CR" => Cr,
        /// Contract
        "CT" => Ct,
        /// Distributor
        "DI" => Di,
        /// Daily Price per Unit
        "DP" => Dp,
        /// Dealer
        "DR" => Dr,
        /// Discount
        "DS" => Ds,
        /// Estimated Credit
        "EC" => Ec,
        /// Shift Differential
        "EH" => Eh,
        /// Estimated
        "ES" => Es,
        /// Fabrication Cost
        "FB" => Fb,
        /// Formula
        "FO" => Fo,
        /// Fixed Price
        "FX" => Fx,
        /// Per 100 Feet
        "HF" => Hf,
        /// Price per Hundred
        "HP" => Hp,
        /// Price Per 100,000
        "HT" => Ht,
        /// Price with Government Furnished Property
        "KA" => Ka,
        /// Escalated Price
        "KP" => Kp,
        /// In Stock
        "KR" => Kr,
        /// Catalog Price per Hundred
        "LC" => Lc,
        /// Catalog Price per Dozen
        "LD" => Ld,
        /// Catalog Price per Each
        "LE" => Le,
        /// Catalog Price per Thousand
        "LM" => Lm,
        /// Previous Catalog Price
        "LR" => Lr,
        /// Midterm Endorsement Price per Unit
        "ME" => Me,
        /// Price per Milliliter
        "ML" => Ml,
        /// No Charge
        "NC" => Nc,
        /// Not to Exceed
        "NE" => Ne,
        /// No Quote
        "NQ" => Nq,
        /// Not Separately Priced
        "NS" => Ns,
        /// Net
        "NT" => Nt,
        /// Price per Troy Ounce
        "PA" => Pa,
        /// Annual Price Per Unit
        "PB" => Pb,
        /// Price per Dozen
        "PD" => Pd,
        /// Price per Each
        "PE" => Pe,
        /// Price Per Foot
        "PF" => Pf,
        /// Price per Gram
        "PG" => Pg,
        /// Price per Kilogram
        "PK" => Pk,
        /// Price per Liter
        "PL" => Pl,
        /// Monthly Price Per Unit
        "PM" => Pm,
        /// Price per Ten
        "PN" => Pn,
        /// Price per Ounce
        "PO" => Po,
        /// Price per Pound
        "PP" => Pp,
        /// Posted
        "PQ" => Pq,
        /// Promotion
        "PR" => Pr,
        /// Price Per Thousand Square Foot
        "PS" => Ps,
        /// Price per Ton
        "PT" => Pt,
        /// Quarterly Price per Unit
        "PU" => Pu,
        /// Provisional Price
        "PV" => Pv,
        /// Price per Yard
        "PY" => Py,
        /// Quoted Price per Each
        "QE" => Qe,
        /// Quoted Price per Hundred
        "QH" => Qh,
        /// Previous Quoted Price
        "QR" => Qr,
        /// Quoted Price per Thousand
        "QS" => Qs,
        /// Quoted
        "QT" => Qt,
        /// Retail Price per Hundred
        "RC" => Rc,
        /// Retail Price per Dozen
        "RD" => Rd,
        /// Retail Price per Each
        "RE" => Re,
        /// Retail Price per Thousand
        "RM" => Rm,
        /// Resale Price
        "RS" => Rs,
        /// Retail
        "RT" => Rt,
        /// Semi Annual Price per Unit
        "SA" => Sa,
        /// Submitted Contract
        "SC" => Sc,
        /// Semi Monthly Price per Unit
        "SM" => Sm,
        /// Suggested Retail
        "SR" => Sr,
        /// Standard
        "ST" => St,
        /// Submitted Wholesale
        "SW" => Sw,
        /// To be negotiated.
        "TB" => Tb,
        /// Contract Price per Hundred
        "TC" => Tc,
        /// Contract Price per Dozen
        "TD" => Td,
        /// Contract Price per Each
        "TE" => Te,
        /// Per 1000 Feet
        "TF" => Tf,
        /// Contract Price per Thousand
        "TM" => Tm,
        /// Price per Thousand
        "TP" => Tp,
        /// Price Per 10,000
        "TT" => Tt,
        /// Price per Unit of Measure
        "UM" => Um,
        /// Verbal Quote
        "VQ" => Vq,
        /// Wholesale Price per Hundred
        "WC" => Wc,
        /// Wholesale Price per Dozen
        "WD" => Wd,
        /// Wholesale Price per Each
        "WE" => We,
        /// Wholesale
        "WH" => Wh,
        /// Weekly Price per Unit
        "WI" => Wi,
        /// Wholesale Price per Thousand
        "WM" => Wm,
    }
);

crate::code_enum!(
    /// **641** Status Reason Code
    ///
    /// - Data element: 641
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Status Reason Code. Code values verified against the Stedi X12 reference.
    E641 {
        /// Death of Principal Mortgagor
        "001" => N001,
        /// Illness of Principal Mortgagor
        "002" => N002,
        /// Illness of Mortgagor's Family Member
        "003" => N003,
        /// Death of Mortgagor's Family Member
        "004" => N004,
        /// Marital Difficulties
        "005" => N005,
        /// Curtailment of Income
        "006" => N006,
        /// Excessive Obligations - Same Income, Including Habitual Nonpayment of Debts
        "007" => N007,
        /// Abandonment of Property
        "008" => N008,
        /// Distant Employment Transfer
        "009" => N009,
        /// Neighborhood Problem
        "010" => N010,
        /// Property Problem
        "011" => N011,
        /// Inability to Sell Property
        "012" => N012,
        /// Inability to Rent Property
        "013" => N013,
        /// Military Service
        "014" => N014,
        /// Default Detail
        "015" => N015,
        /// Unemployment
        "016" => N016,
        /// Business Failure
        "017" => N017,
        /// Bankruptcy
        "018" => N018,
        /// Casualty Loss
        "019" => N019,
        /// Moved - Vacated
        "020" => N020,
        /// Dissatisfied with Property
        "021" => N021,
        /// Energy-Environment Cost
        "022" => N022,
        /// Servicing Problems
        "023" => N023,
        /// Auto Repairs
        "024" => N024,
        /// Extended Reasons
        "025" => N025,
        /// Payment Adjustment
        "026" => N026,
        /// Payment Dispute
        "027" => N027,
        /// Due on Transfer
        "028" => N028,
        /// Transfer of Ownership Pending
        "029" => N029,
        /// Fraud
        "030" => N030,
        /// Unable to Contact Borrower
        "031" => N031,
        /// Air traffic control delay
        "032" => N032,
        /// Delivery Commitment Waived by the Customer
        "033" => N033,
        /// Borrower Action
        "035" => N035,
        /// Automatic Conversion
        "036" => N036,
        /// Lender Approval Required
        "037" => N037,
        /// Lender Approval Not Required
        "038" => N038,
        /// Owner-occupied Loan Outstanding
        "039" => N039,
        /// Loan Reached Maturity
        "040" => N040,
        /// Triggered by Interest Increase
        "041" => N041,
        /// Specified Time Period Completed
        "042" => N042,
        /// Transfer Without Written Notice
        "043" => N043,
        /// Triggered by Payment Increase
        "044" => N044,
        /// Facility Added
        "045" => N045,
        /// Facility Changed
        "046" => N046,
        /// Location Added
        "047" => N047,
        /// Location Changed
        "048" => N048,
        /// Merchandise Damaged or Destroyed
        "049" => N049,
        /// Internal Systems Problems
        "050" => N050,
        /// Vendor-Supplied Carrier Delay
        "051" => N051,
        /// Import Container Delay
        "052" => N052,
        /// Notice of Term Enrollment
        "053" => N053,
        /// Term Grade Report
        "054" => N054,
        /// Change of Venue Granted
        "055" => N055,
        /// Removed to Federal Court
        "056" => N056,
        /// Foreclosure Completed
        "057" => N057,
        /// Servicing Rights Transferred to Non-member
        "058" => N058,
        /// Investor is Pulling Servicing as a Result of Default
        "059" => N059,
        /// Servicer Pulling Servicing from Sub-Servicer as a Result of Default
        "060" => N060,
        /// Consolidation, Extension, Modification (CEM)
        "061" => N061,
        /// Renewal and Extension
        "062" => N062,
        /// Reinstatement - Loan Not Reassigned to Mortgage Electronic Registration System (MERS)
        "063" => N063,
        /// Member Resignation or Expulsion
        "064" => N064,
        /// State Action
        "068" => N068,
        /// Company Action
        "069" => N069,
        /// Voluntary Surrender
        "070" => N070,
        /// Producer Requested
        "071" => N071,
        /// Awaiting State Confirmation
        "072" => N072,
        /// Continuing Education Non-compliance
        "073" => N073,
        /// Non-renewal
        "074" => N074,
        /// For Cause
        "075" => N075,
        /// Lack of Production
        "076" => N076,
        /// Retired
        "077" => N077,
        /// Disability
        "078" => N078,
        /// Requested in Error
        "079" => N079,
        /// Continuing Medical Education (CME)
        "080" => N080,
        /// Faculty Appointment
        "081" => N081,
        /// Health Care Fellowship Appointment
        "082" => N082,
        /// Health Care Internship Appointment
        "083" => N083,
        /// Health Care Residency Appointment
        "084" => N084,
        /// Ownership Change Due to Flow Sale of Servicing Rights
        "085" => N085,
        /// Ownership Change Due to Bulk Sale of Servicing Rights
        "086" => N086,
        /// College Preparatory Diploma
        "087" => N087,
        /// Accurate
        "088" => N088,
        /// Calculated
        "089" => N089,
        /// Not Selected
        "090" => N090,
        /// Reprocessed
        "091" => N091,
        /// Selected
        "092" => N092,
        /// Not Verified
        "093" => N093,
        /// Within Tolerance
        "094" => N094,
        /// Without Documentation
        "095" => N095,
        /// Not Available
        "096" => N096,
        /// Part-time
        "097" => N097,
        /// Missed Delivery
        "A01" => A01,
        /// Release Signature or Release Number
        "A02" => A02,
        /// Incorrect Address
        "A03" => A03,
        /// Indirect Delivery
        "A04" => A04,
        /// Unable To Locate
        "A05" => A05,
        /// Address Corrected - Delivery Attempt
        "A06" => A06,
        /// Refused by Consignee
        "A07" => A07,
        /// Not In On Delivery Completed
        "A08" => A08,
        /// Damaged - Delivery Completed
        "A09" => A09,
        /// Damaged - Delivery Not Completed
        "A10" => A10,
        /// Business Closed
        "A11" => A11,
        /// Package Sorted To Wrong Route
        "A12" => A12,
        /// Other
        "A13" => A13,
        /// Returned to Shipper
        "A14" => A14,
        /// Business On Strike
        "A15" => A15,
        /// Payment Received
        "A16" => A16,
        /// Customer Requested Future Delivery
        "A17" => A17,
        /// Missort
        "A18" => A18,
        /// Restricted Articles Incompatible
        "A19" => A19,
        /// Restricted Articles Unacceptable
        "A20" => A20,
        /// Bulk Plane
        "A21" => A21,
        /// Package Missed Inbound Plane At Origin Station
        "A22" => A22,
        /// Customer Dropped Off Package After Aircraft Depart
        "A23" => A23,
        /// Accident
        "A24" => A24,
        /// Package Received At Destination Station Without Airbill
        "A25" => A25,
        /// Consignee Related
        "A26" => A26,
        /// Driver Related
        "A27" => A27,
        /// Package Missorted During Aircraft Unload
        "A28" => A28,
        /// Hold Changed To Delivery Package
        "A29" => A29,
        /// Mechanical Breakdown
        "A30" => A30,
        /// Arrived In Station After Courier Dispatch
        "A31" => A31,
        /// Aircraft Arrived Late In Hub
        "A32" => A32,
        /// Other Carrier-Related
        "A33" => A33,
        /// Package Shipped From Overgoods
        "A34" => A34,
        /// Holding In Overgoods
        "A36" => A36,
        /// Damaged Rewrapped In Hub
        "A37" => A37,
        /// Detached Airbill
        "A38" => A38,
        /// Previous Stop
        "A39" => A39,
        /// Shipper Related
        "A40" => A40,
        /// Standard Air Package
        "A41" => A41,
        /// Holiday - Closed
        "A42" => A42,
        /// Weather or Natural Disaster Related
        "A43" => A43,
        /// Delivery Not Completed
        "A45" => A45,
        /// Recipient Unavailable -- Delivery Delayed
        "A46" => A46,
        /// ODA/Cartage Agent
        "A49" => A49,
        /// Improper International Paperwork
        "A50" => A50,
        /// Carrier Keying Error
        "A51" => A51,
        /// No Requested Arrival Date
        "A52" => A52,
        /// Shipper Changed Scheduled Shipment Date
        "A53" => A53,
        /// Hold Due to Customs/Documentation
        "A55" => A55,
        /// Unable to Contact Recipient For Broker Information
        "A58" => A58,
        /// Hold At Location
        "A59" => A59,
        /// International Manifest
        "A61" => A61,
        /// Puerto Rican Tax Authorities Holding Package
        "A63" => A63,
        /// Non-FEC Broker Advised
        "A64" => A64,
        /// Customs Release
        "A65" => A65,
        /// Package Delivered Before Commitment
        "A73" => A73,
        /// Package Delivered After Commitment
        "A74" => A74,
        /// Invalid Account Format
        "A75" => A75,
        /// Account Not Found
        "A76" => A76,
        /// Name Specified Does Not Match Account
        "A77" => A77,
        /// Item or Service Already Established, Cannot Add
        "A78" => A78,
        /// Item or Service Not Established, Cannot Modify
        "A79" => A79,
        /// Item or Service Not Available
        "A80" => A80,
        /// Item or Service Not Available on Requested Date
        "A81" => A81,
        /// Address Specified Does Not Match Account
        "A82" => A82,
        /// Unauthorized or Invalid Action
        "A83" => A83,
        /// Civil Event Related Delay
        "A84" => A84,
        /// Customer-requested Early Delivery
        "A85" => A85,
        /// Exceeds Service Limitations
        "A91" => A91,
        /// Past Cutoff Time
        "A95" => A95,
        /// Insufficient Pickup Time
        "A96" => A96,
        /// Missed Pickup
        "A98" => A98,
        /// Alternate Carrier Delivered
        "A99" => A99,
        /// Abnormal
        "ABN" => Abn,
        /// Actual Contractor-Caused Delay
        "ACC" => Acc,
        /// Anticipated Contractor-Caused Delay
        "ACD" => Acd,
        /// Assignment Form Required
        "AFR" => Afr,
        /// Actual Government-Caused Delay
        "AGC" => Agc,
        /// Anticipated Government-Caused Delay
        "AGD" => Agd,
        /// Agent Not Appointed
        "ANA" => Ana,
        /// Agent Not Licensed
        "ANL" => Anl,
        /// Application Incomplete
        "API" => Api,
        /// Reconsigned
        "B01" => B01,
        /// Appointment or Pre-Arranged Delivery Date
        "B02" => B02,
        /// Trap for Customer
        "B03" => B03,
        /// Held for Payment
        "B04" => B04,
        /// Held for Consignee
        "B05" => B05,
        /// Consignee Closed (Inventory, Vacation, Etc.)
        "B06" => B06,
        /// Dock Pickup
        "B07" => B07,
        /// Improper Unloading Facility or Equipment
        "B08" => B08,
        /// Receiving Time Restricted
        "B09" => B09,
        /// Order Notify
        "B10" => B10,
        /// Held for Protective Service
        "B11" => B11,
        /// Connecting Line or Cartage Pickup
        "B12" => B12,
        /// Held per Shipper
        "B13" => B13,
        /// Missing Documents
        "B14" => B14,
        /// Border Clearance
        "B15" => B15,
        /// Road Conditions
        "B16" => B16,
        /// Did not complete secondary school
        "B17" => B17,
        /// Standard high school diploma
        "B18" => B18,
        /// Advanced or honors diploma
        "B19" => B19,
        /// Vocational/Technical Preparatory Diploma
        "B20" => B20,
        /// Special education diploma
        "B21" => B21,
        /// Certificate of completion or attendance
        "B22" => B22,
        /// Special certificate of completion
        "B23" => B23,
        /// General Education Development Diploma (GED)
        "B24" => B24,
        /// Other high school equivalency diploma
        "B25" => B25,
        /// International diploma or certificate (such as International Baccalaureate)
        "B26" => B26,
        /// Student is eligible to continue or return or both
        "B27" => B27,
        /// Student is on suspension or dismissal
        "B28" => B28,
        /// Student is expelled (from PreK - grade 12)
        "B29" => B29,
        /// Currently enrolled but courses in progress not included
        "B30" => B30,
        /// Not currently enrolled
        "B31" => B31,
        /// Previous enrollment. Used for entry or exit or both at school other than the sending school
        "B32" => B32,
        /// Unreported - Information is not available in record
        "B33" => B33,
        /// Currently enrolled and courses in progress are included
        "B34" => B34,
        /// Highest Honors
        "B35" => B35,
        /// Second Highest Honors
        "B36" => B36,
        /// Third Highest Honors
        "B37" => B37,
        /// Dropped
        "B38" => B38,
        /// Academic Probation
        "B39" => B39,
        /// Suspended
        "B40" => B40,
        /// Requested record will not be sent; Cannot identify student
        "B41" => B41,
        /// Requested record will not be sent electronically; Paper copy will be sent
        "B42" => B42,
        /// Requested record will not be sent; Have student contact us
        "B43" => B43,
        /// Part of requested record being sent; Remainder to be sent by hard copy
        "B44" => B44,
        /// Requested record will not be sent; No record of student
        "B45" => B45,
        /// Requested record will not be sent; Degree or Diploma not yet awarded
        "B46" => B46,
        /// Requested record will not be sent; Institutional policy requires student release. Have student contact us
        "B47" => B47,
        /// Record being sent at request of student
        "B48" => B48,
        /// Record being sent to replace one previously sent
        "B49" => B49,
        /// Requested record being sent
        "B50" => B50,
        /// Student on Suspension or Dismissal; Eligible to Apply for Re-entry
        "B51" => B51,
        /// According to established regulations or statutes, the student is considered to be a "dropout"
        "B52" => B52,
        /// Student Qualifies for Special Services
        "B53" => B53,
        /// Passed Proficiency Test
        "B54" => B54,
        /// Passed Screening
        "B55" => B55,
        /// Better Features
        "B56" => B56,
        /// Rating Changed
        "B57" => B57,
        /// Better Performance
        "BPR" => Bpr,
        /// Bottom Well Cannot Handle Two or More Units
        "BW2" => Bw2,
        /// Bottom Well Cannot Handle Unit Assigned because of Length
        "BWL" => Bwl,
        /// Bottom Well Cannot Handle Unit Assigned because of Type
        "BWT" => Bwt,
        /// Bottom Well Cannot Handle Unit Assigned because of Width
        "BWW" => Bww,
        /// Waiting for Customer Pickup
        "C01" => C01,
        /// Credit Hold
        "C02" => C02,
        /// Suspended at Customer Request
        "C03" => C03,
        /// Customer Vacation
        "C04" => C04,
        /// Customer Strike
        "C05" => C05,
        /// Waiting Shipping Instructions
        "C06" => C06,
        /// Waiting for Customer Specified Carrier
        "C07" => C07,
        /// Collect on Delivery Required
        "C08" => C08,
        /// Cash Not Available from Consignee
        "C09" => C09,
        /// Customs (Import/Export)
        "C10" => C10,
        /// No Requested Arrival Date Provided To Carrier By Shipper
        "C11" => C11,
        /// No Requested Arrival Time Provided To Carrier By Shipper
        "C12" => C12,
        /// Loan Paid in Full
        "C13" => C13,
        /// Loan Refinanced, Insured by Insurer Receiving Report
        "C14" => C14,
        /// Loan Refinanced, Insured by Other Insurance Carrier
        "C15" => C15,
        /// Loan Refinanced, No Insurance Required
        "C16" => C16,
        /// Coverage No Longer Required
        "C17" => C17,
        /// No Outstanding Commitments
        "C18" => C18,
        /// Court Probation
        "C19" => C19,
        /// Complete
        "C20" => C20,
        /// Changed Broker Dealer
        "CBD" => Cbd,
        /// Clearance to Destination Exceeded
        "CDE" => Cde,
        /// Changed Agent
        "CHA" => Cha,
        /// Conflict of Interest Exists
        "CIE" => Cie,
        /// Check in Mail
        "CIM" => Cim,
        /// Collateral Assignment
        "CLA" => Cla,
        /// Contract or Lost Policy Statement Required
        "CLP" => Clp,
        /// Clear - No motor vehicle violations
        "CLR" => Clr,
        /// Cosmetic
        "COS" => Cos,
        /// Carrier Dispatch Error
        "D01" => D01,
        /// Driver Not Available
        "D02" => D02,
        /// Student has attended a nonpublic school or home education program in- or out-of-state this year, but is entering a public school in this state for the first time this school year
        "D03" => D03,
        /// Student was received from another attendance reporting unit in the same school
        "D04" => D04,
        /// Student was received from a school in the same district
        "D05" => D05,
        /// Student was received from another public school outside the district either in- or out-of-state
        "D06" => D06,
        /// Student was received from a nonpublic school either in or out of the district or has returned after having been enrolled in a home education program; The student must have been enrolled previously in a public school this year
        "D07" => D07,
        /// Student unexpectedly reentered the same school after withdrawing or being discharged
        "D08" => D08,
        /// Student was expected to attend a school but did not enter as expected for unknown reasons
        "D09" => D09,
        /// Student was promoted, retained, or transferred to another attendance-reporting unit in the same school
        "D10" => D10,
        /// Student was promoted, retained, or transferred to another school in the same district
        "D11" => D11,
        /// Student withdrew to attend another public school in the same district
        "D12" => D12,
        /// Student withdrew to attend another public school in- or out-of-state
        "D13" => D13,
        /// Student Over Compulsory Attendance Age Left School Voluntarily with No Intention of Returning
        "D14" => D14,
        /// Student Graduated from School with a Standard Diploma
        "D15" => D15,
        /// Student Graduated from School with a Special Diploma
        "D16" => D16,
        /// Student Left School with a Certificate of Completion
        "D17" => D17,
        /// Student Left School with a Special Certificate of Completion
        "D18" => D18,
        /// Student Left School with a State General Education Development (GED) High School Diploma
        "D19" => D19,
        /// Student Withdrew to Attend a Non-Public School or Home Education Program In- or Out-of-State.
        "D20" => D20,
        /// Student withdrew from school due to hardship
        "D21" => D21,
        /// Student has not entered any school in this or any other state this school year
        "D22" => D22,
        /// Previously attended out-of-state public school but is entering a public school in this state for the first time this school year
        "D23" => D23,
        /// Returned to Regular Education Program
        "D24" => D24,
        /// Reclassified Fully English Proficient
        "D25" => D25,
        /// Retained in Current Grade
        "D26" => D26,
        /// Placed in Next Grade After Expected Grade
        "D27" => D27,
        /// Placed in Transitional Program (K-1)
        "D28" => D28,
        /// Status Pending Completion of Summer School (K-12)
        "D29" => D29,
        /// Declined Services
        "D30" => D30,
        /// Administratively Placed in a Higher Grade
        "D31" => D31,
        /// Academically Placed in a Higher Grade
        "D32" => D32,
        /// Promotion Status not Applicable
        "D33" => D33,
        /// Promoted
        "D34" => D34,
        /// Delayed
        "D50" => D50,
        /// Currently Applying
        "D51" => D51,
        /// Previously Applied
        "D52" => D52,
        /// Graduate from a College
        "D53" => D53,
        /// Transfer from a University Program
        "D54" => D54,
        /// Graduate from a University Program
        "D55" => D55,
        /// Exchange Student
        "D56" => D56,
        /// Returning Student Admitted to a New Program
        "D57" => D57,
        /// Returning Student Admitted to the Same Program
        "D58" => D58,
        /// Returning or Continuing Student Changing to Unclassified or General or Unspecified Studies
        "D59" => D59,
        /// Continuing Student Changing to a New Program
        "D60" => D60,
        /// Special Permission
        "D61" => D61,
        /// Graduate from a Technical Institute
        "D62" => D62,
        /// Transfer from a College
        "D63" => D63,
        /// Deferred Maintenance
        "DEF" => Def,
        /// Deceased
        "EB1" => Eb1,
        /// Did not Attend this Semester
        "EB2" => Eb2,
        /// Withdrawn
        "EB3" => Eb3,
        /// Graduated
        "EB4" => Eb4,
        /// Never Attended
        "EB5" => Eb5,
        /// Full-Time Enrollment
        "EB6" => Eb6,
        /// Half-Time Enrollment
        "EB7" => Eb7,
        /// Less Than Half-Time Enrollment
        "EB8" => Eb8,
        /// Approved Leave of Absence
        "EB9" => Eb9,
        /// No Record Found
        "EBA" => Eba,
        /// Three-quarter Time
        "EBO" => Ebo,
        /// Export Release Not Required
        "ENR" => Enr,
        /// Export Release Required
        "ERR" => Err,
        /// Excessive Dimension Cannot be Accepted
        "EXD" => Exd,
        /// Enrolled at Extension
        "EXT" => Ext,
        /// Non-express Clearance Delay
        "F73" => F73,
        /// International Non-carrier Delay
        "F74" => F74,
        /// Flatcar Shortage
        "FCS" => Fcs,
        /// Free Lunch Eligible
        "FLE" => Fle,
        /// Failed to Release Billing
        "FRB" => Frb,
        /// Freeze Damage
        "FZD" => Fzd,
        /// Hit - At Least One Motor Vehicle Violation
        "HIT" => Hit,
        /// Hazardous Material Placement
        "HZM" => Hzm,
        /// Irrevocable Beneficiary
        "IBF" => Ibf,
        /// Individual Education Program
        "IEP" => Iep,
        /// Incarceration
        "INC" => Inc,
        /// Incomplete - Final
        "INF" => Inf,
        /// Incomplete - In Progress
        "INP" => Inp,
        /// Investment Selections
        "INS" => Ins,
        /// Investment Objectives Changed
        "IOC" => Ioc,
        /// Internal Revenue Service Lien
        "IRS" => Irs,
        /// Failed Material Returned for Repair
        "IV1" => Iv1,
        /// Material Shipped Between Intermediate Points
        "IV2" => Iv2,
        /// Joint Ownership
        "JOW" => Jow,
        /// Juvenile Policy
        "JVP" => Jvp,
        /// Limited English Proficient
        "LEP" => Lep,
        /// Liquidated
        "LIQ" => Liq,
        /// Load Shifted
        "LSH" => Lsh,
        /// Multiple Assignments
        "MAS" => Mas,
        /// Migrant Education Program
        "MEP" => Mep,
        /// Mental Incompetency
        "MIN" => Min,
        /// Middle Position Cannot Handle Two or More Units
        "MP2" => Mp2,
        /// Middle Position Cannot Handle Unit Assigned because of Length
        "MPL" => Mpl,
        /// Middle Position Cannot Handle Unit Assigned because of Type
        "MPT" => Mpt,
        /// Middle Position Cannot Handle Unit Assigned because of Width
        "MPW" => Mpw,
        /// Multiple Conditions
        "MTC" => Mtc,
        /// New Generation Product
        "NGP" => Ngp,
        /// No New Money (IRA > 70 1/2)
        "NNM" => Nnm,
        /// Normal
        "NOR" => Nor,
        /// No Withdrawals
        "NWD" => Nwd,
        /// Processing Delay
        "P01" => P01,
        /// Waiting Inspection
        "P02" => P02,
        /// Production Falldown
        "P03" => P03,
        /// Held for Full Carrier Load
        "P04" => P04,
        /// Waiting Test Results
        "P05" => P05,
        /// Producer Strike
        "P06" => P06,
        /// Producer Vacation
        "P07" => P07,
        /// Pending Agent Appointment
        "PAA" => Paa,
        /// Pending Agent License
        "PAL" => Pal,
        /// Power of Attorney
        "POA" => Poa,
        /// Rejected - Insufficient or Incorrect Information
        "REJ" => Rej,
        /// Railroad Failed to Meet Schedule
        "RFM" => Rfm,
        /// Replacement Form Required
        "RFR" => Rfr,
        /// Reduced Price Lunch Eligible
        "RLE" => Rle,
        /// Reason Unknown
        "RUN" => Run,
        /// Delivery Shortage
        "S01" => S01,
        /// Surrender Charges
        "SCH" => Sch,
        /// Surrender Form Required
        "SFR" => Sfr,
        /// Signature Required
        "SGR" => Sgr,
        /// Shipment Overweight
        "SOW" => Sow,
        /// Storm
        "STM" => Stm,
        /// Tractor With Sleeper Car Not Available
        "T01" => T01,
        /// Tractor, Conventional, Not Available
        "T02" => T02,
        /// Trailer Not Available
        "T03" => T03,
        /// Trailer Not Usable Due to Prior Product
        "T04" => T04,
        /// Trailer Class Not Available
        "T05" => T05,
        /// Trailer Volume Not Available
        "T06" => T06,
        /// Insufficient Delivery Time
        "T07" => T07,
        /// Train Derailment
        "TDR" => Tdr,
        /// Theft
        "THT" => Tht,
        /// Temporary Income Loss
        "TIL" => Til,
        /// Top Position Cannot Handle Two or More Units
        "TP2" => Tp2,
        /// Top Position Cannot Handle Unit Assigned because of Length
        "TPL" => Tpl,
        /// Top Position Cannot Handle Unit Assigned because of Type
        "TPT" => Tpt,
        /// Top Position Cannot Handle Unit Assigned because of Width
        "TPW" => Tpw,
        /// Unknown Cause of Delay
        "UCD" => Ucd,
        /// Undetermined
        "UND" => Und,
        /// Vandalism
        "VAN" => Van,
        /// Out of Stock
        "W01" => W01,
        /// Equipment Cut
        "W02" => W02,
        /// Booking Location Request
        "W03" => W03,
        /// On Hold
        "W04" => W04,
        /// Order Discrepancy
        "W05" => W05,
        /// Receiving Location Request
        "W06" => W06,
        /// Inventory Discrepancy
        "W07" => W07,
        /// Material Shortage
        "W08" => W08,
        /// Substitution
        "W09" => W09,
        /// Diverted Item
        "W10" => W10,
        /// Loading Error
        "W11" => W11,
        /// Inbound Carrier Failure
        "W12" => W12,
        /// Product Allocation Exceeded
        "W13" => W13,
        /// Improperly Sized Order
        "W14" => W14,
        /// Wrong Equipment
        "W15" => W15,
        /// Insufficient Equipment Space
        "W16" => W16,
        /// Waiting Application Delivery
        "WAD" => Wad,
        /// Weight Limit of Car Exceeded
        "WLC" => Wlc,
        /// Weight Limit of Truck Exceeded
        "WLT" => Wlt,
        /// Weight Limit of Well Exceeded
        "WLW" => Wlw,
        /// Waiting for Proof
        "WTP" => Wtp,
        /// Alternative Career Exploration
        "X10" => X10,
        /// Educator in Another District
        "XX1" => Xx1,
        /// Educator in Another State
        "XX2" => Xx2,
        /// Educator Outside U.S.
        "XX3" => Xx3,
        /// Other Educational Occupation
        "XX4" => Xx4,
        /// Not Offered Reemployment
        "XX5" => Xx5,
        /// Long Term Substitute
        "XX6" => Xx6,
        /// Intra-District Transfer from Licensed Position to Nonlicensed Position
        "XX7" => Xx7,
        /// No Assignment
        "XX8" => Xx8,
        /// Staff Reduction
        "XX9" => Xx9,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::code_enum!(
    /// **645** Related Company Indication Code
    ///
    /// - Data element: 645
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Related Company Indication Code. Code values verified against the Stedi X12 reference.
    E645 {
        /// Transaction Between Unrelated Companies
        "N" => N,
        /// Transaction Between Related Companies
        "Y" => Y,
    }
);

crate::code_enum!(
    /// **648** Price Multiplier Qualifier
    ///
    /// - Data element: 648
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Price Multiplier Qualifier. Code values verified against the Stedi X12 reference.
    E648 {
        /// Cost Markup Multiplier - Original Cost
        "CSD" => Csd,
        /// Cost Markup Multiplier - Retail Cost
        "CSR" => Csr,
        /// Discount Multiplier
        "DIS" => Dis,
        /// Cost Markup Multiplier - Wholesale Cost
        "ILP" => Ilp,
        /// Percent Solution Multiplier
        "PSP" => Psp,
        /// Selling Multiplier
        "SEL" => Sel,
    }
);

crate::code_enum!(
    /// **662** Relationship Code
    ///
    /// - Data element: 662
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Relationship Code. Code values verified against the Stedi X12 reference.
    E662 {
        /// Add
        "A" => A,
        /// Delete
        "D" => D,
        /// Included
        "I" => I,
        /// Information Only
        "O" => O,
        /// Substituted
        "S" => S,
    }
);

crate::code_enum!(
    /// **665** Residue Indicator Code
    ///
    /// - Data element: 665
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Residue Indicator Code. Code values verified against the Stedi X12 reference.
    E665 {
        /// Residue Last Contained Description (Small Means of Containment)
        "G" => G,
        /// Residue Last Contained Description (Packages)
        "P" => P,
        /// Residue Last Contained Description (Rail Car)
        "R" => R,
    }
);

crate::code_enum!(
    /// **670** Change or Response Type Code
    ///
    /// - Data element: 670
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Change or Response Type Code. Code values verified against the Stedi X12 reference.
    E670 {
        /// Packing Configuration
        "AA" => Aa,
        /// Add Additional Item(s)
        "AI" => Ai,
        /// Changes To Line Items
        "CA" => Ca,
        /// Change of Date Terms
        "CB" => Cb,
        /// Changes To Terms
        "CC" => Cc,
        /// Changes To Item Level Allowance/Charges
        "CE" => Ce,
        /// Cancel Previously Transmitted Purchase Order
        "CF" => Cf,
        /// Changes To Total Level Allowance/Charges
        "CG" => Cg,
        /// Change To Original Confirmation Of Original Announcement
        "CH" => Ch,
        /// Change To Confirmation Of Revised Announcement
        "CI" => Ci,
        /// Change of Dates
        "CT" => Ct,
        /// Delete Item(s)
        "DI" => Di,
        /// (Multiple) For Unit Price Quantity Reschedule Change
        "MU" => Mu,
        /// Concurrent Item (No Change)
        "NC" => Nc,
        /// Original Confirmation Of Revised Announcement
        "OA" => Oa,
        /// Original Confirmation Of Original Announcement
        "OC" => Oc,
        /// Price Change
        "PC" => Pc,
        /// Unit Price/Quantity Change
        "PQ" => Pq,
        /// Unit Price/Reschedule Change
        "PR" => Pr,
        /// Quantity Decrease
        "QD" => Qd,
        /// Quantity Increase
        "QI" => Qi,
        /// Replace Mode of Shipment
        "RA" => Ra,
        /// Replace All Dates
        "RB" => Rb,
        /// Reject Item Change
        "RC" => Rc,
        /// Replacement Item
        "RE" => Re,
        /// Replacement Item with Modifications
        "RM" => Rm,
        /// Reschedule/Quantity Change
        "RQ" => Rq,
        /// Reschedule
        "RS" => Rs,
        /// Replace All Values
        "RZ" => Rz,
        /// Transfer Item
        "TI" => Ti,
    }
);

crate::code_enum!(
    /// **672** Resource Authorization Code
    ///
    /// - Data element: 672
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Resource Authorization Code. Code values verified against the Stedi X12 reference.
    E672 {
        /// Finished (Labor, Material, and Overhead/Burden)
        "FI" => Fi,
        /// Quantity Based Pricing Groups
        "GS" => Gs,
        /// Labor
        "LB" => Lb,
        /// Labor and Material
        "LM" => Lm,
        /// Material
        "MT" => Mt,
        /// Cumulative Quantity Required Prior to First Schedule Period
        "PQ" => Pq,
        /// Previously Released Requirements
        "PR" => Pr,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **678** Ship/Delivery or Calendar Pattern Code
    ///
    /// - Data element: 678
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Ship/Delivery or Calendar Pattern Code. Code values verified against the Stedi X12 reference.
    E678 {
        /// 1st Week of the Month
        "1" => N1,
        /// 2nd Week of the Month
        "2" => N2,
        /// 3rd Week of the Month
        "3" => N3,
        /// 4th Week of the Month
        "4" => N4,
        /// 5th Week of the Month
        "5" => N5,
        /// 1st & 3rd Weeks of the Month
        "6" => N6,
        /// 2nd & 4th Weeks of the Month
        "7" => N7,
        /// 1st Working Day of Period
        "8" => N8,
        /// Last Working Day of Period
        "9" => N9,
        /// Monday through Friday
        "A" => A,
        /// Monday through Saturday
        "B" => B,
        /// Monday through Sunday
        "C" => C,
        /// Monday
        "D" => D,
        /// Tuesday
        "E" => E,
        /// Wednesday
        "F" => F,
        /// Thursday
        "G" => G,
        /// Friday
        "H" => H,
        /// Saturday
        "J" => J,
        /// Sunday
        "K" => K,
        /// Monday through Thursday
        "L" => L,
        /// Immediately
        "M" => M,
        /// As Directed
        "N" => N,
        /// Daily Mon. through Fri.
        "O" => O,
        /// 1/2 Mon. & 1/2 Thurs.
        "P" => P,
        /// 1/2 Tues. & 1/2 Thurs.
        "Q" => Q,
        /// 1/2 Wed. & 1/2 Fri.
        "R" => R,
        /// Once Anytime Mon. through Fri.
        "S" => S,
        /// Sunday, Monday, Thursday, Friday, Saturday
        "SA" => Sa,
        /// Tuesday through Saturday
        "SB" => Sb,
        /// Sunday, Wednesday, Thursday, Friday, Saturday
        "SC" => Sc,
        /// Monday, Wednesday, Thursday, Friday, Saturday
        "SD" => Sd,
        /// Tuesday through Friday
        "SG" => Sg,
        /// Monday, Tuesday and Thursday
        "SL" => Sl,
        /// Monday, Tuesday and Friday
        "SP" => Sp,
        /// Wednesday and Thursday
        "SX" => Sx,
        /// Monday, Wednesday and Thursday
        "SY" => Sy,
        /// Tuesday, Thursday and Friday
        "SZ" => Sz,
        /// 1/2 Tue. & 1/2 Fri.
        "T" => T,
        /// 1/2 Mon. & 1/2 Wed.
        "U" => U,
        /// 1/3 Mon., 1/3 Wed., 1/3 Fri.
        "V" => V,
        /// Whenever Necessary
        "W" => W,
        /// Weekend
        "WE" => We,
        /// 1/2 By Wed., Bal. By Fri.
        "X" => X,
        /// None (Also Used to Cancel or Override a Previous Pattern)
        "Y" => Y,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **682** Part Release Status Code
    ///
    /// - Data element: 682
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Part Release Status Code. Code values verified against the Stedi X12 reference.
    E682 {
        /// Fabrication Authorization Quantity is Final Model Year Requirement; The Part is a Non carryover Item for the Subsequent Model Year from the Supplier
        "1" => N1,
        /// Material Authorization Quantity is Final Model Year Requirement; The Part is a Non carryover Item for the Subsequent Model Year from the Supplier
        "2" => N2,
        /// Part will be Canceled at the Material Quantity
        "3" => N3,
        /// Part is a Carryover Item from Subsequent Model Year from the Supplier
        "4" => N4,
        /// Part is a New Item for the Subsequent Model Year
        "5" => N5,
        /// Part is a New Item Due to an Interim Engineering Release
        "6" => N6,
        /// Final Release for this Cancelled Part
        "7" => N7,
        /// Planning Quantity is Final Model Year Requirement The Part is a Non-Carryover Item for the Subsequent Model Year From the Supplier
        "8" => N8,
        /// Final Forecast for this Cancelled Part
        "9" => N9,
        /// All Time Buy. Unless Specifically Stated, This Code is Not to be Interpreted as Authorization for Tool Scrap
        "A" => A,
        /// Balance-Out Part; Eng. Change Pending
        "B" => B,
        /// Terminal Buy
        "D" => D,
        /// Systems Automated Buy
        "E" => E,
        /// Corporation Campaign Buy
        "F" => F,
        /// Source Surplus
        "G" => G,
        /// Reschedule
        "H" => H,
        /// Merchandising Act Promotion Buy
        "I" => I,
        /// Minimum or Lot Buy
        "J" => J,
        /// Die Tie
        "K" => K,
        /// Landau Specification
        "L" => L,
        /// Mill Run Purchase
        "M" => M,
        /// Solicitation
        "N" => N,
        /// Pilot/Pre-production
        "P" => P,
        /// Special Large Quantity Buy
        "Q" => Q,
        /// Replacement Orders
        "R" => R,
        /// Initial Orders
        "S" => S,
        /// Trim Order
        "T" => T,
        /// Confirming Phone Order
        "U" => U,
        /// Cover Overshipment - Do Not Reship
        "W" => W,
        /// Economic Order Quantity
        "X" => X,
        /// Production Year End Surplus
        "Y" => Y,
    }
);

crate::code_enum!(
    /// **687** Class of Trade Code
    ///
    /// - Data element: 687
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Class of Trade Code. Code values verified against the Stedi X12 reference.
    E687 {
        /// Branches
        "AA" => Aa,
        /// Members
        "AB" => Ab,
        /// Commercial Enterprises
        "AC" => Ac,
        /// Special Trades
        "AD" => Ad,
        /// Financial Institutions
        "AE" => Ae,
        /// Fast Food
        "AF" => Af,
        /// Agent
        "AG" => Ag,
        /// Non-Profit
        "AH" => Ah,
        /// Importer
        "AI" => Ai,
        /// Schools
        "AJ" => Aj,
        /// Gaming
        "AL" => Al,
        /// Prisons
        "AM" => Am,
        /// Theme Park
        "AN" => An,
        /// Multi-Purpose
        "AO" => Ao,
        /// Purchaser
        "AP" => Ap,
        /// Restaurant
        "AQ" => Aq,
        /// Vending
        "AR" => Ar,
        /// Seller
        "AS" => As,
        /// Buying Group
        "BG" => Bg,
        /// Broker
        "BR" => Br,
        /// Combined
        "CB" => Cb,
        /// Consolidator (Master Distributor)
        "CN" => Cn,
        /// Contractor
        "CO" => Co,
        /// Chain Store Retail
        "CR" => Cr,
        /// Confection
        "CX" => Cx,
        /// Convenience Store
        "CY" => Cy,
        /// Dealer
        "DE" => De,
        /// Doctors' Offices
        "DF" => Df,
        /// Distributor
        "DI" => Di,
        /// Drug Store
        "DR" => Dr,
        /// Exporter
        "EX" => Ex,
        /// Food Service
        "FS" => Fs,
        /// Grocery Accounts
        "GA" => Ga,
        /// General Merchandise
        "GM" => Gm,
        /// General Retail
        "GR" => Gr,
        /// Government
        "GV" => Gv,
        /// Hospitals
        "HS" => Hs,
        /// Industrial
        "ID" => Id,
        /// Institutional
        "IN" => In,
        /// Independent Retail
        "IR" => Ir,
        /// Jobber
        "JB" => Jb,
        /// Long-term Care
        "LC" => Lc,
        /// Managed Care
        "MC" => Mc,
        /// Manufacturer
        "MF" => Mf,
        /// Military
        "ML" => Ml,
        /// OEM
        "OE" => Oe,
        /// Off Premise
        "OF" => Of,
        /// On Premise
        "ON" => On,
        /// Professional
        "PF" => Pf,
        /// Outpatient Pharmacy
        "PH" => Ph,
        /// Pet Supply Store
        "PT" => Pt,
        /// Private Label
        "PY" => Py,
        /// Resale
        "RS" => Rs,
        /// Services
        "SA" => Sa,
        /// Support
        "SB" => Sb,
        /// Securities
        "SE" => Se,
        /// Stationer
        "ST" => St,
        /// General Trade
        "TR" => Tr,
        /// Warehousing
        "WA" => Wa,
        /// Warehouse Clubs
        "WC" => Wc,
        /// Wholesaler
        "WH" => Wh,
        /// User
        "WS" => Ws,
    }
);

crate::code_enum!(
    /// **699** Commodity/Geographic Logical Connector Code
    ///
    /// - Data element: 699
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Commodity/Geographic Logical Connector Code. Code values verified against the Stedi X12 reference.
    E699 {
        /// If one or more segments of a given segment definition contain this element indicating a '-' value, rate application is denied upon satisfaction of any one of these segments, so indicated; In the event that rate restrictions on two segments of similar definition are satisfied, one segment containing this element indicating a '+' value and the other segment indicating a '-', rate application is denied. In other words, a minus takes precedence over a plus.
        "M" => M,
        /// If one or more segments of a given segment definition contain this element indicating a '+' value, rate application requires satisfaction of the restriction stated on at least one of those segments, so indicated; In the event that rate restrictions on two segments of similar definition are satisfied, one segment containing this element indicating a '+' value and the other segment indicating a '-', rate application is denied. In other words, a minus takes precedence over a plus.
        "P" => P,
    }
);
