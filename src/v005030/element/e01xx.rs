//! X12 data elements 0100-0199.

crate::code_enum!(
    /// **143** Transaction Set Identifier Code
    ///
    /// - Data element: 143
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Transaction Set Identifier Code. Code values verified against the Stedi X12 reference.
    E143 {
        /// Insurance Plan Description
        "100" => N100,
        /// Name and Address Lists
        "101" => N101,
        /// Associated Data
        "102" => N102,
        /// Abandoned Property Filings
        "103" => N103,
        /// Air Shipment Information
        "104" => N104,
        /// Business Entity Filings
        "105" => N105,
        /// Motor Carrier Rate Proposal
        "106" => N106,
        /// Request for Motor Carrier Rate Proposal
        "107" => N107,
        /// Response to a Motor Carrier Rate Proposal
        "108" => N108,
        /// Vessel Content Details
        "109" => N109,
        /// Air Freight Details and Invoice
        "110" => N110,
        /// Individual Insurance Policy and Client Information
        "111" => N111,
        /// Property Damage Report
        "112" => N112,
        /// Election Campaign and Lobbyist Reporting
        "113" => N113,
        /// Vehicle Shipping Order
        "120" => N120,
        /// Vehicle Service
        "121" => N121,
        /// Vehicle Damage
        "124" => N124,
        /// Multilevel Railcar Load Details
        "125" => N125,
        /// Vehicle Application Advice
        "126" => N126,
        /// Vehicle Baying Order
        "127" => N127,
        /// Dealer Information
        "128" => N128,
        /// Vehicle Carrier Rate Update
        "129" => N129,
        /// Student Educational Record (Transcript)
        "130" => N130,
        /// Student Educational Record (Transcript) Acknowledgment
        "131" => N131,
        /// Human Resource Information
        "132" => N132,
        /// Educational Institution Record
        "133" => N133,
        /// Student Aid Origination Record
        "135" => N135,
        /// Educational Testing and Prospect Request and Report
        "138" => N138,
        /// Student Loan Guarantee Result
        "139" => N139,
        /// Product Registration
        "140" => N140,
        /// Product Service Claim Response
        "141" => N141,
        /// Product Service Claim
        "142" => N142,
        /// Product Service Notification
        "143" => N143,
        /// Student Loan Transfer and Status Verification
        "144" => N144,
        /// Request for Student Educational Record (Transcript)
        "146" => N146,
        /// Response to Request for Student Educational Record (Transcript)
        "147" => N147,
        /// Report of Injury, Illness or Incident
        "148" => N148,
        /// Notice of Tax Adjustment or Assessment
        "149" => N149,
        /// Tax Rate Notification
        "150" => N150,
        /// Electronic Filing of Tax Return Data Acknowledgment
        "151" => N151,
        /// Statistical Government Information
        "152" => N152,
        /// Unemployment Insurance Tax Claim or Charge Information
        "153" => N153,
        /// Secured Interest Filing
        "154" => N154,
        /// Business Credit Report
        "155" => N155,
        /// Notice of Power of Attorney
        "157" => N157,
        /// Tax Jurisdiction Sourcing
        "158" => N158,
        /// Motion Picture Booking Confirmation
        "159" => N159,
        /// Transportation Automatic Equipment Identification
        "160" => N160,
        /// Train Sheet
        "161" => N161,
        /// Transportation Appointment Schedule Information
        "163" => N163,
        /// Revenue Receipts Statement
        "170" => N170,
        /// Court and Law Enforcement Notice
        "175" => N175,
        /// Court Submission
        "176" => N176,
        /// Environmental Compliance Reporting
        "179" => N179,
        /// Return Merchandise Authorization and Notification
        "180" => N180,
        /// Royalty Regulatory Report
        "185" => N185,
        /// Insurance Underwriting Requirements Reporting
        "186" => N186,
        /// Premium Audit Request and Return
        "187" => N187,
        /// Educational Course Inventory
        "188" => N188,
        /// Application for Admission to Educational Institutions
        "189" => N189,
        /// Student Enrollment Verification
        "190" => N190,
        /// Student Loan Pre-Claims and Claims
        "191" => N191,
        /// Grant or Assistance Application
        "194" => N194,
        /// Federal Communications Commission (FCC) License Application
        "195" => N195,
        /// Contractor Cost Data Reporting
        "196" => N196,
        /// Real Estate Title Evidence
        "197" => N197,
        /// Loan Verification Information
        "198" => N198,
        /// Real Estate Settlement Information
        "199" => N199,
        /// Mortgage Credit Report
        "200" => N200,
        /// Residential Loan Application
        "201" => N201,
        /// Secondary Mortgage Market Loan Delivery
        "202" => N202,
        /// Secondary Mortgage Market Investor Report
        "203" => N203,
        /// Motor Carrier Load Tender
        "204" => N204,
        /// Mortgage Note
        "205" => N205,
        /// Real Estate Inspection
        "206" => N206,
        /// Motor Carrier Freight Details and Invoice
        "210" => N210,
        /// Motor Carrier Bill of Lading
        "211" => N211,
        /// Motor Carrier Delivery Trailer Manifest
        "212" => N212,
        /// Motor Carrier Shipment Status Inquiry
        "213" => N213,
        /// Transportation Carrier Shipment Status Message
        "214" => N214,
        /// Motor Carrier Pickup Manifest
        "215" => N215,
        /// Motor Carrier Shipment Pickup Notification
        "216" => N216,
        /// Motor Carrier Loading and Route Guide
        "217" => N217,
        /// Logistics Service Request
        "219" => N219,
        /// Logistics Service Response
        "220" => N220,
        /// Cartage Work Assignment
        "222" => N222,
        /// Consolidators Freight Bill and Invoice
        "223" => N223,
        /// Motor Carrier Summary Freight Bill Manifest
        "224" => N224,
        /// Response to a Cartage Work Assignment
        "225" => N225,
        /// Trailer Usage Report
        "227" => N227,
        /// Equipment Inspection Report
        "228" => N228,
        /// Motor Carrier Package Status
        "240" => N240,
        /// Data Status Tracking
        "242" => N242,
        /// Product Source Information
        "244" => N244,
        /// Real Estate Tax Service Response
        "245" => N245,
        /// Account Assignment/Inquiry and Service/Status
        "248" => N248,
        /// Animal Toxicological Data
        "249" => N249,
        /// Purchase Order Shipment Management Document
        "250" => N250,
        /// Pricing Support
        "251" => N251,
        /// Insurance Producer Administration
        "252" => N252,
        /// Underwriting Information Services
        "255" => N255,
        /// Periodic Compensation
        "256" => N256,
        /// Residential Mortgage Insurance Explanation of Benefits
        "259" => N259,
        /// Application for Mortgage Insurance Benefits
        "260" => N260,
        /// Real Estate Information Request
        "261" => N261,
        /// Real Estate Information Report
        "262" => N262,
        /// Residential Mortgage Insurance Application Response
        "263" => N263,
        /// Mortgage Loan Default Status
        "264" => N264,
        /// Real Estate Title Insurance Services Order
        "265" => N265,
        /// Mortgage or Property Record Change Notification
        "266" => N266,
        /// Individual Life, Annuity and Disability Application
        "267" => N267,
        /// Annuity Activity
        "268" => N268,
        /// Health Care Benefit Coordination Verification
        "269" => N269,
        /// Eligibility, Coverage or Benefit Inquiry
        "270" => N270,
        /// Eligibility, Coverage or Benefit Information
        "271" => N271,
        /// Property and Casualty Loss Notification
        "272" => N272,
        /// Insurance/Annuity Application Status
        "273" => N273,
        /// Healthcare Provider Information
        "274" => N274,
        /// Patient Information
        "275" => N275,
        /// Health Care Claim Status Request
        "276" => N276,
        /// Health Care Information Status Notification
        "277" => N277,
        /// Health Care Services Review Information
        "278" => N278,
        /// Voter Registration Information
        "280" => N280,
        /// Tax or Fee Exemption Certification
        "283" => N283,
        /// Commercial Vehicle Safety Reports
        "284" => N284,
        /// Commercial Vehicle Safety and Credentials Information Exchange
        "285" => N285,
        /// Commercial Vehicle Credentials
        "286" => N286,
        /// Wage Determination
        "288" => N288,
        /// Cooperative Advertising Agreements
        "290" => N290,
        /// Reservation (Booking Request) (Ocean)
        "300" => N300,
        /// Confirmation (Ocean)
        "301" => N301,
        /// Booking Cancellation (Ocean)
        "303" => N303,
        /// Shipping Instructions
        "304" => N304,
        /// Customs Manifest
        "309" => N309,
        /// Freight Receipt and Invoice (Ocean)
        "310" => N310,
        /// Canada Customs Information
        "311" => N311,
        /// Arrival Notice (Ocean)
        "312" => N312,
        /// Shipment Status Inquiry (Ocean)
        "313" => N313,
        /// Status Details (Ocean)
        "315" => N315,
        /// Delivery/Pickup Order
        "317" => N317,
        /// Terminal Information
        "319" => N319,
        /// Terminal Operations and Intermodal Ramp Activity
        "322" => N322,
        /// Vessel Schedule and Itinerary (Ocean)
        "323" => N323,
        /// Vessel Stow Plan (Ocean)
        "324" => N324,
        /// Consolidation of Goods In Container
        "325" => N325,
        /// Consignment Summary List
        "326" => N326,
        /// Customs Status Information
        "350" => N350,
        /// U.S. Customs Carrier General Order Status
        "352" => N352,
        /// Customs Events Advisory Details
        "353" => N353,
        /// U.S. Customs Automated Manifest Archive Status
        "354" => N354,
        /// U.S. Customs Acceptance/Rejection
        "355" => N355,
        /// U.S. Customs Permit to Transfer Request
        "356" => N356,
        /// U.S. Customs In-Bond Information
        "357" => N357,
        /// Customs Consist Information
        "358" => N358,
        /// Carrier Interchange Agreement (Ocean)
        "361" => N361,
        /// Cargo Insurance Advice of Shipment
        "362" => N362,
        /// Rail Carrier Shipment Information
        "404" => N404,
        /// Rail Carrier Freight Details and Invoice
        "410" => N410,
        /// Trailer or Container Repair Billing
        "412" => N412,
        /// Rail Carhire Settlements
        "414" => N414,
        /// Rail Carrier Waybill Interchange
        "417" => N417,
        /// Rail Advance Interchange Consist
        "418" => N418,
        /// Advance Car Disposition
        "419" => N419,
        /// Car Handling Information
        "420" => N420,
        /// Estimated Time of Arrival and Car Scheduling
        "421" => N421,
        /// Equipment Order
        "422" => N422,
        /// Rail Industrial Switch List
        "423" => N423,
        /// Rail Carrier Services Settlement
        "424" => N424,
        /// Rail Waybill Request
        "425" => N425,
        /// Rail Revenue Waybill
        "426" => N426,
        /// Railroad Retirement Activity
        "429" => N429,
        /// Railroad Station Master File
        "431" => N431,
        /// Rail Deprescription
        "432" => N432,
        /// Railroad Reciprocal Switch File
        "433" => N433,
        /// Railroad Mark Register Update Activity
        "434" => N434,
        /// Standard Transportation Commodity Code Master
        "435" => N435,
        /// Locomotive Information
        "436" => N436,
        /// Railroad Junctions and Interchanges Activity
        "437" => N437,
        /// Shipment Weights
        "440" => N440,
        /// Railroad Event Report
        "451" => N451,
        /// Railroad Problem Log Inquiry or Advice
        "452" => N452,
        /// Railroad Service Commitment Advice
        "453" => N453,
        /// Railroad Parameter Trace Registration
        "455" => N455,
        /// Railroad Equipment Inquiry or Advice
        "456" => N456,
        /// Railroad Price Distribution Request or Response
        "460" => N460,
        /// Rail Rate Reply
        "463" => N463,
        /// Rate Request
        "466" => N466,
        /// Rate Docket Journal Log
        "468" => N468,
        /// Railroad Clearance
        "470" => N470,
        /// Rail Route File Maintenance
        "475" => N475,
        /// Ratemaking Action
        "485" => N485,
        /// Rate Docket Expiration
        "486" => N486,
        /// Rate Group Definition
        "490" => N490,
        /// Miscellaneous Rates
        "492" => N492,
        /// Rail Scale Rates
        "494" => N494,
        /// Medical Event Reporting
        "500" => N500,
        /// Vendor Performance Review
        "501" => N501,
        /// Pricing History
        "503" => N503,
        /// Clauses and Provisions
        "504" => N504,
        /// Requisition
        "511" => N511,
        /// Material Obligation Validation
        "517" => N517,
        /// Income or Asset Offset
        "521" => N521,
        /// Material Due-In and Receipt
        "527" => N527,
        /// Logistics Reassignment
        "536" => N536,
        /// Notice of Employment Status
        "540" => N540,
        /// Contract Abstract
        "561" => N561,
        /// Contract Completion Status
        "567" => N567,
        /// Contract Payment Management Report
        "568" => N568,
        /// U.S. Customs Export Shipment Information
        "601" => N601,
        /// Transportation Services Tender
        "602" => N602,
        /// Excavation Communication
        "620" => N620,
        /// Well Information
        "625" => N625,
        /// Maintenance Service Order
        "650" => N650,
        /// Intermodal Group Loading Plan
        "715" => N715,
        /// Request for Routing Instructions
        "753" => N753,
        /// Routing Instructions
        "754" => N754,
        /// Contract Pricing Proposal
        "805" => N805,
        /// Project Schedule Reporting
        "806" => N806,
        /// Invoice
        "810" => N810,
        /// Consolidated Service Invoice/Statement
        "811" => N811,
        /// Credit/Debit Adjustment
        "812" => N812,
        /// Electronic Filing of Tax Return Data
        "813" => N813,
        /// General Request, Response or Confirmation
        "814" => N814,
        /// Cryptographic Service Message
        "815" => N815,
        /// Organizational Relationships
        "816" => N816,
        /// Commission Sales Report
        "818" => N818,
        /// Joint Interest Billing and Operating Expense Statement
        "819" => N819,
        /// Payment Order/Remittance Advice
        "820" => N820,
        /// Financial Information Reporting
        "821" => N821,
        /// Account Analysis
        "822" => N822,
        /// Lockbox
        "823" => N823,
        /// Application Advice
        "824" => N824,
        /// Tax Information Exchange
        "826" => N826,
        /// Financial Return Notice
        "827" => N827,
        /// Debit Authorization
        "828" => N828,
        /// Payment Cancellation Request
        "829" => N829,
        /// Planning Schedule with Release Capability
        "830" => N830,
        /// Application Control Totals
        "831" => N831,
        /// Price/Sales Catalog
        "832" => N832,
        /// Mortgage Credit Report Order
        "833" => N833,
        /// Benefit Enrollment and Maintenance
        "834" => N834,
        /// Health Care Claim Payment/Advice
        "835" => N835,
        /// Procurement Notices
        "836" => N836,
        /// Health Care Claim
        "837" => N837,
        /// Trading Partner Profile
        "838" => N838,
        /// Project Cost Reporting
        "839" => N839,
        /// Request for Quotation
        "840" => N840,
        /// Specifications/Technical Information
        "841" => N841,
        /// Nonconformance Report
        "842" => N842,
        /// Response to Request for Quotation
        "843" => N843,
        /// Product Transfer Account Adjustment
        "844" => N844,
        /// Price Authorization Acknowledgment/Status
        "845" => N845,
        /// Inventory Inquiry/Advice
        "846" => N846,
        /// Material Claim
        "847" => N847,
        /// Material Safety Data Sheet
        "848" => N848,
        /// Response to Product Transfer Account Adjustment
        "849" => N849,
        /// Purchase Order
        "850" => N850,
        /// Asset Schedule
        "851" => N851,
        /// Product Activity Data
        "852" => N852,
        /// Routing and Carrier Instruction
        "853" => N853,
        /// Shipment Delivery Discrepancy Information
        "854" => N854,
        /// Purchase Order Acknowledgment
        "855" => N855,
        /// Ship Notice/Manifest
        "856" => N856,
        /// Shipment and Billing Notice
        "857" => N857,
        /// Shipment Information
        "858" => N858,
        /// Freight Invoice
        "859" => N859,
        /// Purchase Order Change Request - Buyer Initiated
        "860" => N860,
        /// Receiving Advice/Acceptance Certificate
        "861" => N861,
        /// Shipping Schedule
        "862" => N862,
        /// Report of Test Results
        "863" => N863,
        /// Text Message
        "864" => N864,
        /// Purchase Order Change Acknowledgment/Request - Seller Initiated
        "865" => N865,
        /// Production Sequence
        "866" => N866,
        /// Product Transfer and Resale Report
        "867" => N867,
        /// Electronic Form Structure
        "868" => N868,
        /// Order Status Inquiry
        "869" => N869,
        /// Order Status Report
        "870" => N870,
        /// Component Parts Content
        "871" => N871,
        /// Residential Mortgage Insurance Application
        "872" => N872,
        /// Commodity Movement Services
        "873" => N873,
        /// Commodity Movement Services Response
        "874" => N874,
        /// Grocery Products Purchase Order
        "875" => N875,
        /// Grocery Products Purchase Order Change
        "876" => N876,
        /// Manufacturer Coupon Family Code Structure
        "877" => N877,
        /// Product Authorization/De-authorization
        "878" => N878,
        /// Price Information
        "879" => N879,
        /// Grocery Products Invoice
        "880" => N880,
        /// Manufacturer Coupon Redemption Detail
        "881" => N881,
        /// Direct Store Delivery Summary Information
        "882" => N882,
        /// Market Development Fund Allocation
        "883" => N883,
        /// Market Development Fund Settlement
        "884" => N884,
        /// Retail Account Characteristics
        "885" => N885,
        /// Customer Call Reporting
        "886" => N886,
        /// Coupon Notification
        "887" => N887,
        /// Item Maintenance
        "888" => N888,
        /// Promotion Announcement
        "889" => N889,
        /// Deduction Research Report
        "891" => N891,
        /// Item Information Request
        "893" => N893,
        /// Delivery/Return Base Record
        "894" => N894,
        /// Delivery/Return Acknowledgment or Adjustment
        "895" => N895,
        /// Product Dimension Maintenance
        "896" => N896,
        /// Loss or Damage Claim - General Commodities
        "920" => N920,
        /// Loss or Damage Claim - Motor Vehicle
        "924" => N924,
        /// Claim Tracer
        "925" => N925,
        /// Claim Status Report and Tracer Reply
        "926" => N926,
        /// Automotive Inspection Detail
        "928" => N928,
        /// Warehouse Shipping Order
        "940" => N940,
        /// Warehouse Stock Transfer Shipment Advice
        "943" => N943,
        /// Warehouse Stock Transfer Receipt Advice
        "944" => N944,
        /// Warehouse Shipping Advice
        "945" => N945,
        /// Warehouse Inventory Adjustment Advice
        "947" => N947,
        /// Functional Group Totals
        "980" => N980,
        /// Response to a Load Tender
        "990" => N990,
        /// Secured Receipt or Acknowledgment
        "993" => N993,
        /// File Transfer
        "996" => N996,
        /// Functional Acknowledgment
        "997" => N997,
        /// Set Cancellation
        "998" => N998,
        /// Implementation Acknowledgment
        "999" => N999,
    }
);
