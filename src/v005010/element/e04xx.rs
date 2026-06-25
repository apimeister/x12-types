//! X12 data elements 0400-0499.

crate::code_enum!(
    /// **432** Date Qualifier
    ///
    /// - Data element: 432
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying type of date (used in G62 and other grocery date segments).
    E432 {
        /// Cancel After This Date
        "01" => N01,
        /// Invoice Date
        "03" => N03,
        /// Purchase Order Date
        "04" => N04,
        /// Requested Ship Date/Pickup Date
        "10" => N10,
        /// Shipped on This Date
        "11" => N11,
        /// Terms Net Due Date
        "13" => N13,
        /// Estimated Delivery Date
        "17" => N17,
        /// Expiration Date
        "36" => N36,
        /// Requested Delivery Date
        "68" => N68,
        /// Scheduled Delivery Date
        "70" => N70,
        /// Date Issued
        "85" => N85,
    }
);

crate::code_enum!(
    /// **478** Credit/Debit Flag Code
    ///
    /// - Data element: 478
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code indicating whether amount is a credit or debit.
    E478 {
        /// Credit
        "C" => C,
        /// Debit
        "D" => D,
    }
);

crate::num_element!(
    /// **406** Quantity of Pallets Shipped
    ///
    /// - Data element: 406
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Quantity of Pallets Shipped.
    E406
);

crate::num_element!(
    /// **408** Temperature
    ///
    /// - Data element: 408
    /// - Type: Numeric (R)
    /// - Length: min 1, max 4
    ///
    /// Temperature.
    E408
);

crate::num_element!(
    /// **409** Quantity of Pallets Received
    ///
    /// - Data element: 409
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Quantity of Pallets Received.
    E409
);

crate::num_element!(
    /// **410** Quantity of Pallets Returned
    ///
    /// - Data element: 410
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Quantity of Pallets Returned.
    E410
);

crate::num_element!(
    /// **411** Quantity Contested
    ///
    /// - Data element: 411
    /// - Type: Numeric (R)
    /// - Length: min 1, max 7
    ///
    /// Quantity Contested.
    E411
);

crate::num_element!(
    /// **413** Quantity Received
    ///
    /// - Data element: 413
    /// - Type: Numeric (R)
    /// - Length: min 1, max 7
    ///
    /// Quantity Received.
    E413
);

crate::num_element!(
    /// **416** Pallet Block and Tiers
    ///
    /// - Data element: 416
    /// - Type: Numeric (N0)
    /// - Length: min 6, max 6
    ///
    /// Pallet Block and Tiers.
    E416
);

crate::num_element!(
    /// **418** Item List Cost - New
    ///
    /// - Data element: 418
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Item List Cost - New.
    E418
);

crate::num_element!(
    /// **419** Item List Cost - Old
    ///
    /// - Data element: 419
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Item List Cost - Old.
    E419
);

crate::num_element!(
    /// **420** Price New, Suggested Retail
    ///
    /// - Data element: 420
    /// - Type: Numeric (N2)
    /// - Length: min 2, max 7
    ///
    /// Price New, Suggested Retail.
    E420
);

crate::num_element!(
    /// **421** Price Old, Suggested Retail
    ///
    /// - Data element: 421
    /// - Type: Numeric (N2)
    /// - Length: min 2, max 7
    ///
    /// Price Old, Suggested Retail.
    E421
);

crate::num_element!(
    /// **427** Unit Price Difference
    ///
    /// - Data element: 427
    /// - Type: Numeric (R)
    /// - Length: min 1, max 15
    ///
    /// Unit Price Difference.
    E427
);

crate::date_element!(
    /// **446** Terms Net Due Date
    ///
    /// - Data element: 446
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Terms Net Due Date.
    E446
);

crate::num_element!(
    /// **466** Total Stop-offs
    ///
    /// - Data element: 466
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 2
    ///
    /// Total Stop-offs.
    E466
);

crate::num_element!(
    /// **467** Priority
    ///
    /// - Data element: 467
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 1
    ///
    /// Priority.
    E467
);

crate::num_element!(
    /// **468** Port Call File Number
    ///
    /// - Data element: 468
    /// - Type: Numeric (N0)
    /// - Length: min 4, max 4
    ///
    /// Port Call File Number.
    E468
);

crate::num_element!(
    /// **470** Priority Code
    ///
    /// - Data element: 470
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 1
    ///
    /// Priority Code.
    E470
);

crate::num_element!(
    /// **477** Credit/Debit Quantity
    ///
    /// - Data element: 477
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Credit/Debit Quantity.
    E477
);

crate::num_element!(
    /// **488** Percent, Integer Format
    ///
    /// - Data element: 488
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Percent, Integer Format.
    E488
);

crate::code_enum!(
    /// **407** Seal Status Code
    ///
    /// - Data element: 407
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Seal Status Code. Code values verified against the Stedi X12 reference.
    E407 {
        /// Intact
        "01" => N01,
        /// Broken
        "02" => N02,
        /// Missing
        "03" => N03,
        /// Replaced
        "04" => N04,
    }
);

crate::code_enum!(
    /// **412** Receiving Condition Code
    ///
    /// - Data element: 412
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Receiving Condition Code. Code values verified against the Stedi X12 reference.
    E412 {
        /// Damaged Product or Container
        "01" => N01,
        /// Quantity Short
        "02" => N02,
        /// Quantity Over
        "03" => N03,
        /// Quality Problem
        "04" => N04,
        /// Incorrect Product
        "05" => N05,
        /// Non-standard Container
        "06" => N06,
        /// Good Condition
        "07" => N07,
        /// Rejected
        "08" => N08,
        /// Hold
        "09" => N09,
        /// Material Scrapped
        "10" => N10,
        /// Adjust Supplier Shipped Cumulative Quantity
        "11" => N11,
        /// Quantity Over - Returned to Supplier
        "12" => N12,
        /// Quantity Received, But Cannot Process Because No Matching Ship Notice
        "13" => N13,
        /// Quantity Received and Processed with No Matching Ship Notice/Manifest
        "14" => N14,
        /// Not Received - Ship Notice Required
        "16" => N16,
    }
);

crate::code_enum!(
    /// **422** Promotion Condition Code
    ///
    /// - Data element: 422
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Promotion Condition Code. Code values verified against the Stedi X12 reference.
    E422 {
        /// Product Allocation
        "01" => N01,
        /// Number of Buy Restriction
        "02" => N02,
        /// Promotion Period Restriction
        "03" => N03,
        /// Special Promotional Discount Terms
        "04" => N04,
        /// Number of Shipment Restrictions
        "05" => N05,
        /// Minimum Order Quantity Required
        "06" => N06,
        /// Minimum Order Quantity Required - Any Combination of Products
        "07" => N07,
        /// Maximum Retail Price
        "08" => N08,
        /// Ad, Size Not Specified
        "10" => N10,
        /// Ad, 1 Col Inch
        "11" => N11,
        /// Ad, 2 Col Inch
        "12" => N12,
        /// Ad, 3 Col Inch
        "13" => N13,
        /// Ad, 4 Col Inch
        "14" => N14,
        /// Ad, 5 Col Inch
        "15" => N15,
        /// Ad, 6 Col Inch or Greater
        "16" => N16,
        /// Ad, Full Page
        "17" => N17,
        /// Ad, Half Page
        "18" => N18,
        /// Ad, Quarter Page
        "19" => N19,
        /// Ad, Coupon
        "20" => N20,
        /// Ad, Color
        "21" => N21,
        /// Ad, Product Illustration
        "22" => N22,
        /// Ad, National Line Rate
        "23" => N23,
        /// Ad, Roto
        "25" => N25,
        /// Ad, Free Standing
        "26" => N26,
        /// Ad, Window Signs
        "28" => N28,
        /// Ad, Price Reduction
        "32" => N32,
        /// Ad, Circulars
        "33" => N33,
        /// Ad, Television
        "34" => N34,
        /// Ad, Radio
        "35" => N35,
        /// Ad, Handbill
        "36" => N36,
        /// Ad, Newspaper Insert
        "37" => N37,
        /// Ad, See Free-form Message
        "39" => N39,
        /// Display
        "40" => N40,
        /// Display, End Aisle
        "41" => N41,
        /// Display, Aisle Stack
        "42" => N42,
        /// Display, Cases per Store
        "44" => N44,
        /// Display, Number of Days
        "45" => N45,
        /// Display, Shelf Extender
        "46" => N46,
        /// Display, Number of Stores
        "49" => N49,
        /// Display, Price Reduction
        "51" => N51,
        /// Display, See Free-form Message
        "59" => N59,
        /// Reduced Price
        "60" => N60,
        /// Reduced Price, Number of Days
        "61" => N61,
        /// Reduced Price With Shelf Sign
        "62" => N62,
        /// Reduced Price, In Price/Order Guide
        "63" => N63,
        /// Reduced Price, In Store Coupons
        "64" => N64,
        /// Reduced Price, See Free-form Message
        "69" => N69,
        /// Retail Distribution (One Case per Store)
        "70" => N70,
        /// Retail Distribution (X Cases per Store)
        "71" => N71,
        /// Retail Distribution (X Number of Stores)
        "72" => N72,
        /// Committed Purchases
        "73" => N73,
        /// Growth Over Last Year
        "74" => N74,
        /// Growth Over Last Month
        "75" => N75,
        /// Resale
        "76" => N76,
        /// Growth Over Last Quarter
        "77" => N77,
        /// Retail Distribution
        "79" => N79,
        /// Accrual Limit
        "81" => N81,
        /// Flat Limit
        "82" => N82,
        /// Number of Required Promotion Performances
        "83" => N83,
        /// Product Scanning Report Required
        "84" => N84,
        /// Advertisement or Display or Reduced Price
        "88" => N88,
        /// See Free-form Text
        "99" => N99,
        /// Ad, Size A
        "A1" => A1,
        /// Ad, Size B
        "A2" => A2,
        /// Ad, Size C
        "A3" => A3,
        /// In-Store Electronics
        "A4" => A4,
        /// Ad, Double Page
        "A5" => A5,
        /// Every Day Low Pricing
        "B1" => B1,
        /// Demonstration Required
        "B2" => B2,
        /// Ad, Predominant With Reduced Price
        "B3" => B3,
        /// Value Pack Program
        "C1" => C1,
        /// Educational Program
        "C2" => C2,
        /// Freestanding Shipper Display
        "C3" => C3,
        /// Refrigerated Table
        "C4" => C4,
        /// Warehouse Withdrawal Report Required
        "WR" => Wr,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **423** Promotion Status Code
    ///
    /// - Data element: 423
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Promotion Status Code. Code values verified against the Stedi X12 reference.
    E423 {
        /// New
        "01" => N01,
        /// Change
        "02" => N02,
        /// Cancel
        "03" => N03,
        /// Replace
        "04" => N04,
        /// Confirmation
        "05" => N05,
        /// Replace All Dates
        "06" => N06,
        /// Accept
        "07" => N07,
    }
);

crate::code_enum!(
    /// **426** Adjustment Reason Code
    ///
    /// - Data element: 426
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Adjustment Reason Code. Code values verified against the Stedi X12 reference.
    E426 {
        /// Automated Credit Card Transaction
        "00" => N00,
        /// Pricing Error
        "01" => N01,
        /// Allowance/Charge Error
        "02" => N02,
        /// Extension Error
        "03" => N03,
        /// Item Not Accepted - Damaged
        "04" => N04,
        /// Item Not Accepted - Quality
        "05" => N05,
        /// Quantity Contested
        "06" => N06,
        /// Incorrect Product
        "07" => N07,
        /// Substitute Product
        "08" => N08,
        /// Terms of Sale Error
        "09" => N09,
        /// Pallet/Container Charge Error
        "10" => N10,
        /// Returns - Damage
        "11" => N11,
        /// Returns - Quality
        "12" => N12,
        /// Returns - Dating
        "13" => N13,
        /// Returns - Promotion
        "14" => N14,
        /// Returns - Recall
        "15" => N15,
        /// Non-Invoice Related Allowance/Charge
        "16" => N16,
        /// Required Data Missing
        "17" => N17,
        /// Not Company Bill
        "18" => N18,
        /// Duplicate Billing
        "19" => N19,
        /// Balance Due Declined
        "20" => N20,
        /// Shipment Method of Payment Incorrect
        "21" => N21,
        /// Weight Error
        "22" => N22,
        /// Special Charge Not Authorized
        "23" => N23,
        /// Incorrect Discount
        "24" => N24,
        /// Item Not Accepted
        "25" => N25,
        /// Invoice Cancelled
        "26" => N26,
        /// Product Transfers Subject to Charge Back
        "27" => N27,
        /// Rebated Shipments
        "28" => N28,
        /// Fee Incorrect
        "29" => N29,
        /// Delivery Charge Incorrect
        "30" => N30,
        /// Pickup Charge Incorrect
        "31" => N31,
        /// Oversize Premium Invalid
        "32" => N32,
        /// Currency Exchange Incorrect
        "33" => N33,
        /// Declared Value Incorrect
        "34" => N34,
        /// Commodity Code Incorrect
        "35" => N35,
        /// Scale Number Incorrect
        "36" => N36,
        /// Dimensions Incorrect
        "37" => N37,
        /// Service Incorrect
        "38" => N38,
        /// Shipper/Consignee Ref. Number Missing
        "39" => N39,
        /// Address Incorrect
        "40" => N40,
        /// Item Not Accepted-Delay
        "41" => N41,
        /// Item Not Accepted - Loss
        "42" => N42,
        /// Missing Discount
        "43" => N43,
        /// Required Documents Missing
        "44" => N44,
        /// Stale Bill Over 180 Days Old
        "45" => N45,
        /// Transportation Charge Incorrect
        "46" => N46,
        /// Advanced Charge Incorrect
        "47" => N47,
        /// Service Charge
        "48" => N48,
        /// Processing Charge
        "49" => N49,
        /// Late Charge
        "50" => N50,
        /// Interest Penalty Charge
        "51" => N51,
        /// Credit for Overpayment
        "52" => N52,
        /// Remittance for Previous Underpayment
        "53" => N53,
        /// Freight Deducted
        "54" => N54,
        /// Tax Deducted
        "55" => N55,
        /// Advertising Allowance Taken
        "56" => N56,
        /// Volume Discount Taken
        "57" => N57,
        /// Invoice billing received after final billing
        "58" => N58,
        /// Item not received
        "59" => N59,
        /// No open item on file
        "60" => N60,
        /// No open order on file
        "61" => N61,
        /// Material/Item Description Error
        "62" => N62,
        /// Customer Paid Invoice Which Was Previously Disputed
        "63" => N63,
        /// Sale of Property
        "64" => N64,
        /// Claim Paid on Appraisal
        "65" => N65,
        /// Disability Insurance or Income
        "66" => N66,
        /// Death Benefit Reduction
        "68" => N68,
        /// Employer Provided Pension
        "69" => N69,
        /// Advanced Ship Notice Not Received
        "70" => N70,
        /// Advertising Allowance
        "71" => N71,
        /// Authorized Return
        "72" => N72,
        /// Bill of Lading Not Received
        "73" => N73,
        /// Cancel or Adjust Prior Credit/Debit Adjustment
        "74" => N74,
        /// Total Order Not Received
        "75" => N75,
        /// Cash Discount
        "76" => N76,
        /// Competitive Allowance
        "77" => N77,
        /// Competitive Price
        "78" => N78,
        /// Cooperative Advertising
        "79" => N79,
        /// Overpayment
        "80" => N80,
        /// Credit as Agreed
        "81" => N81,
        /// Defective Allowance
        "82" => N82,
        /// Evaluated Receipt Settlement (ERS) Delivery Charge
        "83" => N83,
        /// Deviation from Order Date
        "84" => N84,
        /// Distribution Discount/Allowance
        "85" => N85,
        /// Duplicate Payment
        "86" => N86,
        /// Duplicate Shipment
        "87" => N87,
        /// Duty Charge Variance
        "88" => N88,
        /// Early Buy Allowance
        "89" => N89,
        /// Early Payment Allowance
        "90" => N90,
        /// Engraving Charge
        "91" => N91,
        /// Merchandise Not Ordered
        "92" => N92,
        /// Field Destroy
        "93" => N93,
        /// Fixtures Charge
        "94" => N94,
        /// Floor Stock Protection
        "95" => N95,
        /// Goods to Follow
        "96" => N96,
        /// Handling Charge
        "97" => N97,
        /// Labor Charges
        "98" => N98,
        /// Late Shipment of Goods
        "99" => N99,
        /// Layout/Design Charge
        "A1" => A1,
        /// Misshipped (Wrong Product Received)
        "A2" => A2,
        /// New Store Allowance
        "A3" => A3,
        /// Nonreceipt of Goods
        "A4" => A4,
        /// Overage
        "A5" => A5,
        /// Packing Violations
        "A6" => A6,
        /// Payment on Account
        "A7" => A7,
        /// Promotional Allowance
        "A8" => A8,
        /// Proof of Delivery Not Received
        "A9" => A9,
        /// Prepaid Benefit or Advances
        "AA" => Aa,
        /// Partial Wage Continuation
        "AB" => Ab,
        /// Apportionment or Contribution
        "AC" => Ac,
        /// Non-Automated Credit Card Transaction
        "AD" => Ad,
        /// Unemployment Compensation
        "AE" => Ae,
        /// Guarantee Fee
        "AF" => Af,
        /// Guaranteed Amount
        "AG" => Ag,
        /// Origination Fee
        "AH" => Ah,
        /// Released to Borrower
        "AL" => Al,
        /// Applied to Borrower's Account
        "AM" => Am,
        /// Auto No Fault
        "AN" => An,
        /// Returned to Lender
        "AO" => Ao,
        /// Acceleration of Benefits
        "AP" => Ap,
        /// Returns - Overstock
        "AQ" => Aq,
        /// Acceleration of Reimbursement of Attorney Fees
        "AR" => Ar,
        /// Returns - Discontinued
        "AS" => As,
        /// Account Location Closed
        "AT" => At,
        /// Accessory Payment
        "AU" => Au,
        /// Advertising Contribution
        "AV" => Av,
        /// Student Card Payment
        "AW" => Aw,
        /// Person No Longer Employed
        "AX" => Ax,
        /// Employee on Leave
        "AY" => Ay,
        /// Employee on Strike
        "AZ" => Az,
        /// Proof of Shipment Not Received
        "B1" => B1,
        /// Rebate
        "B2" => B2,
        /// Recovery Allowance
        "B3" => B3,
        /// Refurbishing Charge
        "B4" => B4,
        /// Repair of Goods
        "B5" => B5,
        /// Repay Discount
        "B6" => B6,
        /// Restocking Charge
        "B7" => B7,
        /// Routing Violation
        "B8" => B8,
        /// Samples Not Received
        "B9" => B9,
        /// Canadian Goods and Services Tax
        "BA" => Ba,
        /// Quebec Goods and Services Tax
        "BB" => Bb,
        /// Canadian Harmonized Goods and Services Tax
        "BC" => Bc,
        /// Bad Debt Adjustment
        "BD" => Bd,
        /// Fixture Allowance
        "BE" => Be,
        /// Return Allowance
        "BF" => Bf,
        /// Bag Charge
        "BG" => Bg,
        /// Opportunity Buy
        "BH" => Bh,
        /// Hanger Charge
        "BI" => Bi,
        /// Insurance Charge
        "BJ" => Bj,
        /// Postage Charge
        "BK" => Bk,
        /// Net Check Returned
        "BL" => Bl,
        /// Net Collection Expense (Factor)
        "BM" => Bm,
        /// Bonus
        "BN" => Bn,
        /// Change to Box Office Gross
        "BO" => Bo,
        /// Net Chargeback of Client Risk (Factor)
        "BP" => Bp,
        /// Benefit Rate
        "BR" => Br,
        /// Paid During Period
        "BS" => Bs,
        /// Settlement of Account
        "C1" => C1,
        /// Special Allowance
        "C2" => C2,
        /// Special Finish
        "C3" => C3,
        /// Stock Balance
        "C4" => C4,
        /// Temporary Allowance
        "C5" => C5,
        /// Testing Charge
        "C6" => C6,
        /// Third Party Allowance
        "C7" => C7,
        /// Ticketing Error
        "C8" => C8,
        /// Ticketing Service
        "C9" => C9,
        /// Cancelled Promotion
        "CA" => Ca,
        /// Collected Balance Adjustment Incorrect
        "CB" => Cb,
        /// Employer's Legal Expenses Paid to Date
        "CE" => Ce,
        /// Valuation As Of
        "CK" => Ck,
        /// Covered by Credit Memo
        "CM" => Cm,
        /// Adjusted Compensation Rate
        "CO" => Co,
        /// Employer Paid Benefits due
        "CP" => Cp,
        /// Change
        "CQ" => Cq,
        /// Capitation Interest
        "CR" => Cr,
        /// Adjustment
        "CS" => Cs,
        /// Capitation Payment
        "CT" => Ct,
        /// Charge for Unrequested Service
        "CU" => Cu,
        /// Capital Passthru
        "CV" => Cv,
        /// Certified Registered Nurse Anesthetist Passthru
        "CW" => Cw,
        /// Transfer Between Accounts
        "D1" => D1,
        /// Transportation Direct Billing
        "D2" => D2,
        /// Unauthorized Deduction
        "D3" => D3,
        /// Unauthorized Product
        "D4" => D4,
        /// Volume Discount
        "D5" => D5,
        /// Recovery of Standard Allowances
        "D6" => D6,
        /// Cost Associated with Reworked Material
        "D7" => D7,
        /// Count and Recount Allowance
        "D8" => D8,
        /// Store Stock Price Protection
        "D9" => D9,
        /// Daylight Overdraft Charge Reversed
        "DA" => Da,
        /// Pension or Retirement Plan Disability Benefits
        "DB" => Db,
        /// Canadian Pension Plan
        "DC" => Dc,
        /// Railroad Disability
        "DD" => Dd,
        /// Deposited Item Price Incorrect
        "DE" => De,
        /// Family Social Security
        "DF" => Df,
        /// Deduction Film Rental
        "DG" => Dg,
        /// District Advance
        "DH" => Dh,
        /// Social Security Supplemental Income
        "DI" => Di,
        /// Jones Act (Merchant Seaman Injured on the Job)
        "DJ" => Dj,
        /// Damaged Film
        "DK" => Dk,
        /// Deposited Item Volume Error
        "DL" => Dl,
        /// Direct Medical Education Passthru
        "DM" => Dm,
        /// Other Group Insurance
        "DO" => Do,
        /// Pension or Retirement Benefits
        "DP" => Dp,
        /// Social Security Retirements
        "DR" => Dr,
        /// Social Security Disability
        "DS" => Ds,
        /// Railroad Retirement
        "DT" => Dt,
        /// Discontinued Product
        "DU" => Du,
        /// Civil Servants Plan
        "DV" => Dv,
        /// Subrogation
        "DW" => Dw,
        /// Royalty Deduction Type
        "DX" => Dx,
        /// Distribution Issue
        "DY" => Dy,
        /// Recoupment
        "E1" => E1,
        /// Covered By Debit Memo
        "E2" => E2,
        /// Withholding
        "E3" => E3,
        /// Warehouse Stock Price Protection
        "E4" => E4,
        /// Invoice Price Protection
        "E5" => E5,
        /// Goods and Services Tax 0% Rate, International Documentation Will Follow
        "E6" => E6,
        /// Goods and Services Tax Decreased Due to Billing Error
        "E7" => E7,
        /// Goods and Services Tax Increased Due to Billing Error
        "E8" => E8,
        /// Order Cancelled
        "E9" => E9,
        /// Encoding Error
        "EE" => Ee,
        /// Expanded Promotion
        "EP" => Ep,
        /// Employer Reimbursement
        "ER" => Er,
        /// Expenses
        "EX" => Ex,
        /// Defective
        "F1" => F1,
        /// Social Security
        "F3" => F3,
        /// No-Fault
        "F4" => F4,
        /// Other Long-term Disability Offset
        "F5" => F5,
        /// Indirect Offset Excess
        "F6" => F6,
        /// Black Lung Disease
        "F7" => F7,
        /// Rehabilitation
        "F8" => F8,
        /// Educational Benefit
        "F9" => F9,
        /// Anticipation Taken
        "FA" => Fa,
        /// Forwarding Balance
        "FB" => Fb,
        /// Fund Allocation
        "FC" => Fc,
        /// Late Interest Paid
        "FI" => Fi,
        /// Film Rental
        "FR" => Fr,
        /// Change to Film Rental Terms
        "FT" => Ft,
        /// Forward Buy
        "FU" => Fu,
        /// Profit Sharing
        "G1" => G1,
        /// Workmen's Compensation
        "G2" => G2,
        /// Pension
        "G3" => G3,
        /// Other Disability Plan
        "G4" => G4,
        /// Other Income
        "G5" => G5,
        /// Salary Continuance
        "G6" => G6,
        /// Short-term Disability
        "G7" => G7,
        /// Permanent and Total Disability
        "G8" => G8,
        /// State Disability Plan
        "G9" => G9,
        /// Free Goods
        "GA" => Ga,
        /// Coupon Related
        "GB" => Gb,
        /// Market Development Fund Deduction
        "GC" => Gc,
        /// Samples
        "GD" => Gd,
        /// Slotting Charge
        "GE" => Ge,
        /// Invalid
        "GF" => Gf,
        /// Unsalable Merchandise
        "GG" => Gg,
        /// Billback Allowance Deduction
        "GH" => Gh,
        /// Unsalable Payments Deduction
        "GJ" => Gj,
        /// Split Commission Deduction - Basis Amount
        "GK" => Gk,
        /// Unresolved Customer Deduction
        "GL" => Gl,
        /// Split Commissions Deduction - Commission Amount
        "GM" => Gm,
        /// General Advance
        "GN" => Gn,
        /// Graduate Medical Education Passthru
        "GO" => Go,
        /// Guarantee
        "GR" => Gr,
        /// Information Forthcoming
        "H1" => H1,
        /// Payment Previously Sent
        "H2" => H2,
        /// Loan Paid in Full
        "H3" => H3,
        /// Bill Insured
        "H4" => H4,
        /// Loan Service Released
        "H5" => H5,
        /// Partial Payment Remitted
        "H6" => H6,
        /// Payment Forthcoming
        "H7" => H7,
        /// Bill Mortgagee
        "H8" => H8,
        /// Coverage Summary Needed
        "H9" => H9,
        /// Change to House Allowance
        "HA" => Ha,
        /// Medical Payment
        "HB" => Hb,
        /// Indemnity Payment
        "HC" => Hc,
        /// Expense Payment
        "HD" => Hd,
        /// Comprehensive Payment
        "HE" => He,
        /// Collision Payment
        "HF" => Hf,
        /// Bodily Injury Payment
        "HG" => Hg,
        /// Property Damage Payment
        "HI" => Hi,
        /// Hemophilia Clotting Factor Supplement
        "HM" => Hm,
        /// Invoice Amount Does Not Match Account Analysis Statement
        "IA" => Ia,
        /// Interest Charge-Rate Reduced to New York Prime Rate
        "IC" => Ic,
        /// Interest Charge - Rate Reduced to Earnings Credit Rate
        "IE" => Ie,
        /// Insufficient Funds
        "IF" => If,
        /// Interest Charge-Rate Reduced to London Inter-bank Offer Rate (LIBOR)
        "IL" => Il,
        /// Inventory Policy
        "IN" => In,
        /// Incentive Premium Payment
        "IP" => Ip,
        /// Internal Revenue Service Withholding
        "IR" => Ir,
        /// Interim Settlement
        "IS" => Is,
        /// Nonreimbursable
        "J1" => J1,
        /// Transportation Discrepancy Report (TDR) Submitted
        "J2" => J2,
        /// Promised Adjustment Not Received
        "J3" => J3,
        /// Deficiency Report Authorized Adjustment
        "J4" => J4,
        /// Duplicate Summary Billing (Same Bill Number)
        "J5" => J5,
        /// Duplicate Summary Billing (Different Bill Number)
        "J6" => J6,
        /// Duplicate Summary Billing (Second Billing)
        "J7" => J7,
        /// Unable to Process
        "J8" => J8,
        /// Noninterfund Bill
        "J9" => J9,
        /// Journal Entry
        "JE" => Je,
        /// Other Coverage Placed
        "K1" => K1,
        /// Invalid Policy Number
        "K2" => K2,
        /// Renewal Not Received
        "K3" => K3,
        /// Audit
        "L1" => L1,
        /// Discount
        "L2" => L2,
        /// Penalty
        "L3" => L3,
        /// Administrative Fees
        "L4" => L4,
        /// Interest Due
        "L5" => L5,
        /// Interest Owed
        "L6" => L6,
        /// Miscellaneous Deductions
        "L7" => L7,
        /// Miscellaneous Credits
        "L8" => L8,
        /// Military Distribution Adjustment
        "L9" => L9,
        /// Loans against Future Compensations
        "LA" => La,
        /// Levy
        "LE" => Le,
        /// Lawyer or Claimant Attorney Fees
        "LF" => Lf,
        /// Ledger Overdraft Charge
        "LO" => Lo,
        /// Late Payment
        "LP" => Lp,
        /// Lump Sum
        "LS" => Ls,
        /// Advertising - Unidentified
        "M1" => M1,
        /// Commissions Deductions
        "M2" => M2,
        /// Gift Certificates
        "M3" => M3,
        /// Salary Deduction
        "M4" => M4,
        /// Marketing Allowance
        "MA" => Ma,
        /// Pickup Allowance
        "MB" => Mb,
        /// Miscellaneous Costs
        "MC" => Mc,
        /// Incorrect Purchase Order Number on Bill of Lading
        "MD" => Md,
        /// Purchase Order Number Not on Bill of Lading
        "ME" => Me,
        /// Multiple Purchase Order Numbers on Invoice
        "MF" => Mf,
        /// Purchase Order Number Incorrect on Invoice
        "MG" => Mg,
        /// Purchase Order Number Not on Invoice
        "MH" => Mh,
        /// Transfer or Debit Balance
        "MI" => Mi,
        /// Truckload Allowance
        "MJ" => Mj,
        /// Warehouse Allowance
        "MK" => Mk,
        /// Maximum Allowable Levy Exceeded
        "ML" => Ml,
        /// Incorrect Product ID on Cartons
        "MM" => Mm,
        /// Incorrect Purchase Order Number on Carton
        "MN" => Mn,
        /// No Product ID on Cartons
        "MO" => Mo,
        /// No Purchase Order Number on Carton
        "MP" => Mp,
        /// Storage Charges
        "MQ" => Mq,
        /// Manufacturer to Retail Bill-Back Allowance
        "MR" => Mr,
        /// Miscellaneous Event
        "MV" => Mv,
        /// Second Injury Fund
        "N1" => N1,
        /// Future Credit Amount
        "N2" => N2,
        /// Vocational Rehabilitation
        "N3" => N3,
        /// Uninsured Employer
        "N4" => N4,
        /// Silicosis, Dust Diseases, and Logging Industry Fund
        "N5" => N5,
        /// Vocationally Handicapped Fund
        "N6" => N6,
        /// Non-designated Fund
        "N7" => N7,
        /// Cash Receipt
        "NA" => Na,
        /// Non-Billable
        "NB" => Nb,
        /// Negative Charge
        "NC" => Nc,
        /// Royalty
        "ND" => Nd,
        /// New Location
        "NL" => Nl,
        /// New Promotion
        "NO" => No,
        /// New Product
        "NP" => Np,
        /// Negative Repayment
        "NR" => Nr,
        /// Organ Acquisition Passthru
        "OA" => Oa,
        /// Offset for Affiliated Providers
        "OB" => Ob,
        /// Overstock Condition
        "OC" => Oc,
        /// Court Ordered Lien
        "OL" => Ol,
        /// Order Policy
        "OP" => Op,
        /// Other Trading-Partner Specific Reason
        "OT" => Ot,
        /// Price Change
        "P1" => P1,
        /// Product Changeover
        "P2" => P2,
        /// Production Issue
        "P3" => P3,
        /// Picture Advance
        "PA" => Pa,
        /// Materials
        "PB" => Pb,
        /// Amendment of Limits
        "PC" => Pc,
        /// Paid to Date
        "PD" => Pd,
        /// Employment Benefits
        "PE" => Pe,
        /// Payment First
        "PF" => Pf,
        /// Carrier Filed Limitation
        "PG" => Pg,
        /// Commissions
        "PH" => Ph,
        /// Periodic Interim Payment
        "PI" => Pi,
        /// Intercompany Sale
        "PJ" => Pj,
        /// Lodging
        "PK" => Pk,
        /// Payment Final
        "PL" => Pl,
        /// Previous Month's Earnings Credit Carried Forward
        "PM" => Pm,
        /// Meals
        "PN" => Pn,
        /// Federal Deposit Insurance Corporation (FDIC) Premium Overcharge
        "PO" => Po,
        /// Quebec Pension Plan
        "PP" => Pp,
        /// Overtime
        "PQ" => Pq,
        /// Prior Film Rental Payment
        "PR" => Pr,
        /// Product Exclusion
        "PS" => Ps,
        /// Payment
        "PT" => Pt,
        /// Severance
        "PU" => Pu,
        /// Special
        "PV" => Pv,
        /// State Limitation
        "PW" => Pw,
        /// Store Certificates
        "PX" => Px,
        /// Tips
        "PY" => Py,
        /// Equipment Rental
        "PZ" => Pz,
        /// Returned Deposited Item Price Incorrect
        "R1" => R1,
        /// Returned Deposited Item Volume Error
        "R2" => R2,
        /// Returned Deposited Item Notification Volume Error
        "R3" => R3,
        /// Reduced Promotion
        "R4" => R4,
        /// Revised Plan (Re-plan)
        "R5" => R5,
        /// Revised Promotion
        "R6" => R6,
        /// Retro-activity Adjustment
        "RA" => Ra,
        /// Agreed Freight Allowance
        "RB" => Rb,
        /// Authorized Air Shipment
        "RC" => Rc,
        /// Receiving Discrepancy
        "RD" => Rd,
        /// Return on Equity
        "RE" => Re,
        /// Commission Discrepancy
        "RF" => Rf,
        /// Difference On Returns
        "RG" => Rg,
        /// Early Shipment of Goods
        "RH" => Rh,
        /// Excessive Packing Materials
        "RI" => Ri,
        /// Failure to Consolidate
        "RJ" => Rj,
        /// Freight on Returns Inbound Only
        "RK" => Rk,
        /// Freight on Returns Outbound Only
        "RL" => Rl,
        /// Returned Material
        "RM" => Rm,
        /// Handling Charge for Item Not Received
        "RN" => Rn,
        /// Handling Charge for Non Receipt of Goods
        "RO" => Ro,
        /// Handling Charge for Late Advance Ship Notice
        "RP" => Rp,
        /// Handling Charge for Samples Not Received
        "RQ" => Rq,
        /// Handling Charge for Unreadable Advance Ship Notice
        "RR" => Rr,
        /// Incorrect Packing Assortment
        "RS" => Rs,
        /// In-store Decoration Allowance
        "RT" => Rt,
        /// Interest
        "RU" => Ru,
        /// Label Placement
        "RV" => Rv,
        /// Margin Contribution
        "RW" => Rw,
        /// Mark Down Allowance
        "RX" => Rx,
        /// Multiple Shipment Penalty
        "RY" => Ry,
        /// Opening Order Allowance
        "RZ" => Rz,
        /// Paper Invoice Missing
        "SA" => Sa,
        /// Service Cancelled
        "SC" => Sc,
        /// Screening Deduction
        "SD" => Sd,
        /// Paper Packing Slip Missing
        "SE" => Se,
        /// Shipping and Freight Charge
        "SF" => Sf,
        /// Pickup Charge or Advanced Charges
        "SG" => Sg,
        /// Presentation of Merchandise Not As Specified
        "SH" => Sh,
        /// Purchaser Supplied Raw Material
        "SI" => Si,
        /// Returns Freight Discrepancy
        "SJ" => Sj,
        /// Salary Discrepancy
        "SK" => Sk,
        /// Student Loan Repayment
        "SL" => Sl,
        /// Shipped Past Purchase Order Cancellation Date
        "SM" => Sm,
        /// Should Be Shipped Collect, But Was Shipped Prepaid
        "SN" => Sn,
        /// Should be Shipped Prepaid, But Was Shipped Collect
        "SO" => So,
        /// Sales Promotion
        "SP" => Sp,
        /// Store Contest Allowance
        "SR" => Sr,
        /// Trade Discount
        "SS" => Ss,
        /// Unauthorized Air Shipment-Chargeback is the Difference Between Air and Ground
        "ST" => St,
        /// Unauthorized Air Shipment-Partial Freight Chargeback
        "SU" => Su,
        /// Unauthorized Air Shipment-Full Freight Chargeback
        "SV" => Sv,
        /// Unauthorized or Incorrect Carrier
        "SW" => Sw,
        /// Volume Break Chargeback
        "SX" => Sx,
        /// Weight Break Chargeback
        "SY" => Sy,
        /// Theatre Advance
        "TA" => Ta,
        /// Total Deductions
        "TD" => Td,
        /// Transportation Issue
        "TI" => Ti,
        /// Third Party Liability
        "TL" => Tl,
        /// Uncollected Balance Charge Reversed
        "UB" => Ub,
        /// Unable to Locate Account
        "UL" => Ul,
        /// Value Date Incorrect for Credit
        "VC" => Vc,
        /// Value Date Incorrect for Debit
        "VD" => Vd,
        /// Void
        "VO" => Vo,
        /// Safety Violations
        "W6" => W6,
        /// Intoxication
        "W8" => W8,
        /// Noncooperation: Rehabilitation, Training, Education, Medical
        "W9" => W9,
        /// Actual Reduced Earnings
        "WA" => Wa,
        /// Regular Wage Minus Disability Benefit
        "WB" => Wb,
        /// Subrogation Recovery
        "WC" => Wc,
        /// Weather-related Event
        "WE" => We,
        /// Payment Partial
        "WH" => Wh,
        /// Overpayment Recovery
        "WO" => Wo,
        /// Deemed Reduced Earnings
        "WR" => Wr,
        /// Special Fund Recovery
        "WS" => Ws,
        /// Deductibles Recovery
        "WT" => Wt,
        /// Unspecified Recovery
        "WU" => Wu,
        /// Overpayment Credit
        "WW" => Ww,
        /// Cost of Living
        "WX" => Wx,
        /// Continuous Disability Period
        "WY" => Wy,
        /// Illegally Employed Minor
        "WZ" => Wz,
        /// Age 65 Reduction
        "XA" => Xa,
        /// Appeal
        "XB" => Xb,
        /// Advance Reimbursement
        "XC" => Xc,
        /// Individual Payment
        "XD" => Xd,
        /// Employer Credit
        "XE" => Xe,
        /// Outlier
        "XF" => Xf,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **433** F.O.B. Point Code
    ///
    /// - Data element: 433
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// F.O.B. Point Code. Code values verified against the Stedi X12 reference.
    E433 {
        /// City
        "01" => N01,
        /// Destination
        "02" => N02,
        /// Plant or Producing Location
        "03" => N03,
        /// Point of Origin
        "04" => N04,
    }
);

crate::code_enum!(
    /// **441** Tax Exempt Code
    ///
    /// - Data element: 441
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Tax Exempt Code. Code values verified against the Stedi X12 reference.
    E441 {
        /// Exempt (For Export)
        "0" => N0,
        /// Yes (Tax Exempt)
        "1" => N1,
        /// No (Not Tax Exempt)
        "2" => N2,
        /// Exempt (For Resale)
        "3" => N3,
        /// Not Exempt/For Resale
        "4" => N4,
        /// Exempt (Not For Resale)
        "5" => N5,
        /// Not Exempt, Not For Resale
        "6" => N6,
        /// Direct Pay ID
        "7" => N7,
        /// Exempt (Sale to U.S. Government)
        "8" => N8,
        /// Exempt (Per State Law)
        "9" => N9,
        /// Labor Taxable, Material Exempt
        "A" => A,
        /// Material Taxable, Labor Exempt
        "B" => B,
        /// Not Taxable
        "C" => C,
        /// Disabled
        "D" => D,
        /// Exempt Toll Service
        "E" => E,
        /// Exempt (Goods and Services Tax)
        "F" => F,
        /// Exempt (Provincial Sales Tax)
        "G" => G,
        /// Homestead
        "H" => H,
        /// Agriculture
        "I" => I,
        /// Working Farm
        "J" => J,
        /// Open Space
        "K" => K,
        /// Exempt Local Service
        "L" => L,
        /// Disabled Veteran
        "M" => M,
        /// Non-Homestead
        "N" => N,
        /// Over 65
        "O" => O,
        /// Exempt from School Property Tax
        "P" => P,
        /// Exempt from Local Property Tax
        "Q" => Q,
        /// Recurring Exempt
        "R" => R,
        /// Exempt from County Property Tax
        "S" => S,
        /// Totally Exempt
        "T" => T,
        /// Usage Exempt
        "U" => U,
        /// Exempt from State Property Tax
        "V" => V,
        /// Other Property Tax Exemption
        "W" => W,
        /// Exempt - Letter on file
        "X" => X,
    }
);

crate::code_enum!(
    /// **444** Purchase Order Instruction Code
    ///
    /// - Data element: 444
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Purchase Order Instruction Code. Code values verified against the Stedi X12 reference.
    E444 {
        /// Add Quantity to Meet Minimum Weight Requirement
        "AQ" => Aq,
        /// Back Order if Not Available
        "BO" => Bo,
        /// Back Order Not Allowed
        "BX" => Bx,
        /// Notify Prior to Shipment if Quantity Specified is Not Available
        "NS" => Ns,
        /// Substitution Allowed
        "SA" => Sa,
        /// Substitution Not Allowed
        "SX" => Sx,
    }
);

crate::code_enum!(
    /// **445** Terms Exception Code
    ///
    /// - Data element: 445
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Terms Exception Code. Code values verified against the Stedi X12 reference.
    E445 {
        /// Deferred
        "DF" => Df,
        /// Damage Guarantee
        "DG" => Dg,
        /// Discount Not Applicable
        "DN" => Dn,
        /// Extended Billing
        "EB" => Eb,
        /// Guaranteed Sale
        "GS" => Gs,
        /// Initial Purchase
        "IP" => Ip,
        /// New Item Introduction
        "NI" => Ni,
        /// Pre-Stocking
        "PS" => Ps,
        /// Special Purchase
        "SP" => Sp,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **456** Temperature Probe Location Code
    ///
    /// - Data element: 456
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Temperature Probe Location Code. Code values verified against the Stedi X12 reference.
    E456 {
        /// General
        "01" => N01,
        /// Head
        "02" => N02,
        /// Middle
        "03" => N03,
        /// Tail
        "04" => N04,
    }
);

crate::code_enum!(
    /// **465** Container Terms Code Qualifier
    ///
    /// - Data element: 465
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Container Terms Code Qualifier. Code values verified against the Stedi X12 reference.
    E465 {
        /// Military Standard Transportation and Movement Procedures (MILSTAMP)
        "I" => I,
    }
);

crate::code_enum!(
    /// **486** Product Transfer Movement Type Code
    ///
    /// - Data element: 486
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Product Transfer Movement Type Code. Code values verified against the Stedi X12 reference.
    E486 {
        /// Adjustment In
        "AI" => Ai,
        /// Adjustment Out
        "AO" => Ao,
        /// Customer to Distributor
        "CD" => Cd,
        /// Distributor to Customer
        "DC" => Dc,
        /// Distributor to Manufacturer
        "DM" => Dm,
        /// Manufacturer to Distributor
        "MD" => Md,
        /// Transfer In
        "TI" => Ti,
        /// Transfer Out
        "TO" => To,
    }
);
