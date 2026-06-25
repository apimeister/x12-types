//! X12 data elements 0400-0499.

crate::code_enum!(
    /// **455** Responsible Agency Code
    ///
    /// - Data element: 455
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Responsible Agency Code. Code values verified against the Stedi X12 reference.
    E455 {
        /// Transportation Data Coordinating Committee (TDCC)
        "T" => T,
        /// Accredited Standards Committee X12
        "X" => X,
    }
);

crate::code_enum!(
    /// **479** Functional Identifier Code
    ///
    /// - Data element: 479
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Functional Identifier Code. Code values verified against the Stedi X12 reference.
    E479 {
        /// Account Analysis (822)
        "AA" => Aa,
        /// Logistics Service Request (219)
        "AB" => Ab,
        /// Associated Data (102)
        "AC" => Ac,
        /// Individual Life, Annuity and Disability Application (267)
        "AD" => Ad,
        /// Premium Audit Request and Return (187)
        "AE" => Ae,
        /// Application for Admission to Educational Institutions (189)
        "AF" => Af,
        /// Application Advice (824)
        "AG" => Ag,
        /// Logistics Service Response (220)
        "AH" => Ah,
        /// Automotive Inspection Detail (928)
        "AI" => Ai,
        /// Student Educational Record (Transcript) Acknowledgment (131)
        "AK" => Ak,
        /// Set Cancellation (998)
        "AL" => Al,
        /// Item Information Request (893)
        "AM" => Am,
        /// Return Merchandise Authorization and Notification (180)
        "AN" => An,
        /// Income or Asset Offset (521)
        "AO" => Ao,
        /// Abandoned Property Filings (103)
        "AP" => Ap,
        /// U.S. Customs Manifest (309)
        "AQ" => Aq,
        /// Warehouse Stock Transfer Shipment Advice (943)
        "AR" => Ar,
        /// Transportation Appointment Schedule Information (163)
        "AS" => As,
        /// Animal Toxicological Data (249)
        "AT" => At,
        /// U.S. Customs Status Information (350)
        "AU" => Au,
        /// U.S. Customs Carrier General Order Status (352)
        "AV" => Av,
        /// Warehouse Inventory Adjustment Advice (947)
        "AW" => Aw,
        /// U.S. Customs Events Advisory Details (353)
        "AX" => Ax,
        /// U.S. Customs Automated Manifest Archive Status (354)
        "AY" => Ay,
        /// U.S. Customs Acceptance/Rejection (355)
        "AZ" => Az,
        /// U.S. Customs Permit to Transfer Request (356)
        "BA" => Ba,
        /// U.S. Customs In-Bond Information (357)
        "BB" => Bb,
        /// Business Credit Report (155)
        "BC" => Bc,
        /// U.S. Customs Consist Information (358)
        "BD" => Bd,
        /// Benefit Enrollment and Maintenance (834)
        "BE" => Be,
        /// Business Entity Filings (105)
        "BF" => Bf,
        /// Motor Carrier Bill of Lading (211)
        "BL" => Bl,
        /// Shipment and Billing Notice (857)
        "BS" => Bs,
        /// Purchase Order Change Acknowledgment/Request - Seller Initiated (865)
        "CA" => Ca,
        /// Unemployment Insurance Tax Claim or Charge Information (153)
        "CB" => Cb,
        /// Clauses and Provisions (504)
        "CC" => Cc,
        /// Credit/Debit Adjustment (812)
        "CD" => Cd,
        /// Cartage Work Assignment (222)
        "CE" => Ce,
        /// Corporate Financial Adjustment Information (844 and 849)
        "CF" => Cf,
        /// Car Handling Information (420)
        "CH" => Ch,
        /// Consolidated Service Invoice/Statement (811)
        "CI" => Ci,
        /// Manufacturer Coupon Family Code Structure (877)
        "CJ" => Cj,
        /// Manufacturer Coupon Redemption Detail (881)
        "CK" => Ck,
        /// Election Campaign and Lobbyist Reporting (113)
        "CL" => Cl,
        /// Component Parts Content (871)
        "CM" => Cm,
        /// Coupon Notification (887)
        "CN" => Cn,
        /// Cooperative Advertising Agreements (290)
        "CO" => Co,
        /// Electronic Proposal Information (251, 805)
        "CP" => Cp,
        /// Commodity Movement Services Response (874)
        "CQ" => Cq,
        /// Rail Carhire Settlements (414)
        "CR" => Cr,
        /// Cryptographic Service Message (815)
        "CS" => Cs,
        /// Application Control Totals (831)
        "CT" => Ct,
        /// Commodity Movement Services (873)
        "CU" => Cu,
        /// Commercial Vehicle Safety and Credentials Information Exchange (285)
        "CV" => Cv,
        /// Educational Institution Record (133)
        "CW" => Cw,
        /// Contract Completion Status (567)
        "D3" => D3,
        /// Contract Abstract (561)
        "D4" => D4,
        /// Contract Payment Management Report (568)
        "D5" => D5,
        /// Debit Authorization (828)
        "DA" => Da,
        /// Shipment Delivery Discrepancy Information (854)
        "DD" => Dd,
        /// Market Development Fund Allocation (883)
        "DF" => Df,
        /// Dealer Information (128)
        "DI" => Di,
        /// Equipment Order (422)
        "DM" => Dm,
        /// Data Status Tracking (242)
        "DS" => Ds,
        /// Direct Exchange Delivery and Return Information (894, 895)
        "DX" => Dx,
        /// Educational Course Inventory (188)
        "EC" => Ec,
        /// Student Educational Record (Transcript) (130)
        "ED" => Ed,
        /// Railroad Equipment Inquiry or Advice (456)
        "EI" => Ei,
        /// Equipment Inspection
        "EN" => En,
        /// Environmental Compliance Reporting (179)
        "EP" => Ep,
        /// Revenue Receipts Statement (170)
        "ER" => Er,
        /// Notice of Employment Status (540)
        "ES" => Es,
        /// Railroad Event Report (451)
        "EV" => Ev,
        /// Excavation Communication (620)
        "EX" => Ex,
        /// Functional or Implementation Acknowledgment Transaction Sets (997, 999)
        "FA" => Fa,
        /// Freight Invoice (859)
        "FB" => Fb,
        /// Court and Law Enforcement Information (175, 176)
        "FC" => Fc,
        /// Motor Carrier Loading and Route Guide (217)
        "FG" => Fg,
        /// Financial Reporting (821, 827)
        "FR" => Fr,
        /// File Transfer (996)
        "FT" => Ft,
        /// Damage Claim Transaction Sets (920, 924, 925, 926)
        "GC" => Gc,
        /// General Request, Response or Confirmation (814)
        "GE" => Ge,
        /// Response to a Load Tender (990)
        "GF" => Gf,
        /// Intermodal Group Loading Plan (715)
        "GL" => Gl,
        /// Grocery Products Invoice (880)
        "GP" => Gp,
        /// Statistical Government Information (152)
        "GR" => Gr,
        /// Grant or Assistance Application (194)
        "GT" => Gt,
        /// Eligibility, Coverage or Benefit Information (271)
        "HB" => Hb,
        /// Health Care Claim (837)
        "HC" => Hc,
        /// Health Care Services Review Information (278)
        "HI" => Hi,
        /// Health Care Information Status Notification (277)
        "HN" => Hn,
        /// Health Care Claim Payment/Advice (835)
        "HP" => Hp,
        /// Health Care Claim Status Request (276)
        "HR" => Hr,
        /// Eligibility, Coverage or Benefit Inquiry (270)
        "HS" => Hs,
        /// Human Resource Information (132)
        "HU" => Hu,
        /// Health Care Benefit Coordination Verification (269)
        "HV" => Hv,
        /// Air Freight Details and Invoice (110, 980)
        "IA" => Ia,
        /// Inventory Inquiry/Advice (846)
        "IB" => Ib,
        /// Rail Advance Interchange Consist (418)
        "IC" => Ic,
        /// Insurance/Annuity Application Status (273)
        "ID" => Id,
        /// Insurance Producer Administration (252)
        "IE" => Ie,
        /// Individual Insurance Policy and Client Information (111)
        "IF" => If,
        /// Direct Store Delivery Summary Information (882)
        "IG" => Ig,
        /// Commercial Vehicle Safety Reports (284)
        "IH" => Ih,
        /// Report of Injury, Illness or Incident (148)
        "IJ" => Ij,
        /// Motor Carrier Freight Details and Invoice (210, 980)
        "IM" => Im,
        /// Invoice Information (810)
        "IN" => In,
        /// Ocean Shipment Billing Details (310, 312, 980)
        "IO" => Io,
        /// Rail Carrier Freight Details and Invoice (410, 980)
        "IR" => Ir,
        /// Estimated Time of Arrival and Car Scheduling (421)
        "IS" => Is,
        /// Joint Interest Billing and Operating Expense Statement (819)
        "JB" => Jb,
        /// Commercial Vehicle Credentials (286)
        "KM" => Km,
        /// Federal Communications Commission (FCC) License Application (195)
        "LA" => La,
        /// Lockbox (823)
        "LB" => Lb,
        /// Locomotive Information (436)
        "LI" => Li,
        /// Property and Casualty Loss Notification (272)
        "LN" => Ln,
        /// Logistics Reassignment (536)
        "LR" => Lr,
        /// Asset Schedule (851)
        "LS" => Ls,
        /// Student Loan Transfer and Status Verification (144)
        "LT" => Lt,
        /// Motor Carrier Summary Freight Bill Manifest (224)
        "MA" => Ma,
        /// Request for Motor Carrier Rate Proposal (107)
        "MC" => Mc,
        /// Department of Defense Inventory Management (527)
        "MD" => Md,
        /// Mortgage Origination (198, 200, 201, 245, 261, 262, 263, 833, 872)
        "ME" => Me,
        /// Market Development Fund Settlement (884)
        "MF" => Mf,
        /// Mortgage Servicing Transaction Sets (203, 206, 259, 260, 264, 266)
        "MG" => Mg,
        /// Motor Carrier Rate Proposal (106)
        "MH" => Mh,
        /// Motor Carrier Shipment Status Inquiry (213)
        "MI" => Mi,
        /// Secondary Mortgage Market Loan Delivery (202)
        "MJ" => Mj,
        /// Response to a Motor Carrier Rate Proposal (108)
        "MK" => Mk,
        /// Medical Event Reporting (500)
        "MM" => Mm,
        /// Mortgage Note (205)
        "MN" => Mn,
        /// Maintenance Service Order (650)
        "MO" => Mo,
        /// Motion Picture Booking Confirmation (159)
        "MP" => Mp,
        /// Consolidators Freight Bill and Invoice (223)
        "MQ" => Mq,
        /// Multilevel Railcar Load Details (125)
        "MR" => Mr,
        /// Material Safety Data Sheet (848)
        "MS" => Ms,
        /// Electronic Form Structure (868)
        "MT" => Mt,
        /// Material Obligation Validation (517)
        "MV" => Mv,
        /// Rail Waybill Response (427)
        "MW" => Mw,
        /// Material Claim (847)
        "MX" => Mx,
        /// Response to a Cartage Work Assignment (225)
        "MY" => My,
        /// Motor Carrier Package Status (240)
        "MZ" => Mz,
        /// Nonconformance Report (842)
        "NC" => Nc,
        /// Name and Address Lists (101)
        "NL" => Nl,
        /// Notice of Power of Attorney (157)
        "NP" => Np,
        /// Secured Receipt or Acknowledgment (993)
        "NR" => Nr,
        /// Notice of Tax Adjustment or Assessment (149)
        "NT" => Nt,
        /// Cargo Insurance Advice of Shipment (362)
        "OC" => Oc,
        /// Order Group - Grocery (875, 876)
        "OG" => Og,
        /// Organizational Relationships (816)
        "OR" => Or,
        /// Warehouse Shipping Order (940)
        "OW" => Ow,
        /// Price Authorization Acknowledgment/Status (845)
        "PA" => Pa,
        /// Railroad Parameter Trace Registration (455)
        "PB" => Pb,
        /// Purchase Order Change Request - Buyer Initiated (860)
        "PC" => Pc,
        /// Product Activity Data (852)
        "PD" => Pd,
        /// Periodic Compensation (256)
        "PE" => Pe,
        /// Annuity Activity (268)
        "PF" => Pf,
        /// Insurance Plan Description (100)
        "PG" => Pg,
        /// Pricing History (503)
        "PH" => Ph,
        /// Patient Information (275)
        "PI" => Pi,
        /// Project Schedule Reporting (806)
        "PJ" => Pj,
        /// Project Cost Reporting (839) and Contractor Cost Data Reporting (196)
        "PK" => Pk,
        /// Railroad Problem Log Inquiry or Advice (452)
        "PL" => Pl,
        /// Product Source Information (244)
        "PN" => Pn,
        /// Purchase Order (850)
        "PO" => Po,
        /// Property Damage Report (112)
        "PQ" => Pq,
        /// Purchase Order Acknowledgment (855)
        "PR" => Pr,
        /// Planning Schedule with Release Capability (830)
        "PS" => Ps,
        /// Product Transfer and Resale Report (867)
        "PT" => Pt,
        /// Motor Carrier Shipment Pickup Notification (216)
        "PU" => Pu,
        /// Purchase Order Shipment Management Document (250)
        "PV" => Pv,
        /// Healthcare Provider Information (274)
        "PW" => Pw,
        /// Payment Cancellation Request (829)
        "PY" => Py,
        /// Product Information (878, 879, 888, 889, 896)
        "QG" => Qg,
        /// Transportation Carrier Shipment Status Message (214)
        "QM" => Qm,
        /// Ocean Shipment Status Information (313, 315)
        "QO" => Qo,
        /// Payment Order/Remittance Advice (820)
        "RA" => Ra,
        /// Railroad Clearance (470)
        "RB" => Rb,
        /// Receiving Advice/Acceptance Certificate (861)
        "RC" => Rc,
        /// Royalty Regulatory Report (185)
        "RD" => Rd,
        /// Warehouse Stock Receipt Advice (944)
        "RE" => Re,
        /// Request for Routing Instructions (753)
        "RF" => Rf,
        /// Routing Instructions (754)
        "RG" => Rg,
        /// Railroad Reciprocal Switch File (433)
        "RH" => Rh,
        /// Routing and Carrier Instruction (853)
        "RI" => Ri,
        /// Railroad Mark Register Update Activity (434)
        "RJ" => Rj,
        /// Standard Transportation Commodity Code Master (435)
        "RK" => Rk,
        /// Rail Industrial Switch List (423)
        "RL" => Rl,
        /// Railroad Station Master File (431)
        "RM" => Rm,
        /// Requisition Transaction (511)
        "RN" => Rn,
        /// Ocean Booking Information (300, 301, 303)
        "RO" => Ro,
        /// Commission Sales Report (818)
        "RP" => Rp,
        /// Request for Quotation (840) and Procurement Notices (836)
        "RQ" => Rq,
        /// Response to Request For Quotation (843)
        "RR" => Rr,
        /// Order Status Information (869, 870)
        "RS" => Rs,
        /// Report of Test Results (863)
        "RT" => Rt,
        /// Railroad Retirement Activity (429)
        "RU" => Ru,
        /// Railroad Junctions and Interchanges Activity (437)
        "RV" => Rv,
        /// Rail Revenue Waybill (426)
        "RW" => Rw,
        /// Rail Deprescription (432)
        "RX" => Rx,
        /// Request for Student Educational Record (Transcript) (146)
        "RY" => Ry,
        /// Response to Request for Student Educational Record (Transcript) (147)
        "RZ" => Rz,
        /// Air Shipment Information (104)
        "SA" => Sa,
        /// Rail Carrier Services Settlement (424)
        "SB" => Sb,
        /// Price/Sales Catalog (832)
        "SC" => Sc,
        /// Student Loan Pre-Claims and Claims (191)
        "SD" => Sd,
        /// Shipper's Export Declaration (601)
        "SE" => Se,
        /// Ship Notice/Manifest (856)
        "SH" => Sh,
        /// Shipment Information (858)
        "SI" => Si,
        /// Transportation Automatic Equipment Identification (160)
        "SJ" => Sj,
        /// Student Aid Origination Record (135, 139)
        "SL" => Sl,
        /// Motor Carrier Load Tender (204)
        "SM" => Sm,
        /// Rail Route File Maintenance (475)
        "SN" => Sn,
        /// Ocean Shipment Information (304, 309, 311, 317, 319, 322, 323, 324, 325, 326, 350, 352, 353, 354, 355, 356, 357, 358, 361)
        "SO" => So,
        /// Specifications/Technical Information (841)
        "SP" => Sp,
        /// Production Sequence (866)
        "SQ" => Sq,
        /// Rail Carrier Shipment Information (404, 419)
        "SR" => Sr,
        /// Shipping Schedule (862)
        "SS" => Ss,
        /// Railroad Service Commitment Advice (453)
        "ST" => St,
        /// Account Assignment/Inquiry and Service/Status (248)
        "SU" => Su,
        /// Student Enrollment Verification (190)
        "SV" => Sv,
        /// Warehouse Shipping Advice (945)
        "SW" => Sw,
        /// Electronic Filing of Tax Return Data Acknowledgment (151)
        "TA" => Ta,
        /// Trailer or Container Repair Billing (412)
        "TB" => Tb,
        /// Trading Partner Profile (838)
        "TD" => Td,
        /// Tax or Fee Exemption Certification (283)
        "TE" => Te,
        /// Electronic Filing of Tax Return Data (813)
        "TF" => Tf,
        /// Tax Information Exchange (826)
        "TI" => Ti,
        /// Tax Jurisdiction Sourcing (158)
        "TJ" => Tj,
        /// Motor Carrier Delivery Trailer Manifest (212)
        "TM" => Tm,
        /// Tax Rate Notification (150)
        "TN" => Tn,
        /// Real Estate Title Services (197, 199, 265, 485, 486)
        "TO" => To,
        /// Rail Rate Transactions (460, 463, 466, 468, 485, 486, 490, 492, 494)
        "TP" => Tp,
        /// Train Sheet (161)
        "TR" => Tr,
        /// Transportation Services Tender (602)
        "TS" => Ts,
        /// Educational Testing and Prospect Request and Report (138)
        "TT" => Tt,
        /// Trailer Usage Report (227)
        "TU" => Tu,
        /// Text Message (864)
        "TX" => Tx,
        /// Retail Account Characteristics (885)
        "UA" => Ua,
        /// Customer Call Reporting (886)
        "UB" => Ub,
        /// Secured Interest Filing (154)
        "UC" => Uc,
        /// Deduction Research Report (891)
        "UD" => Ud,
        /// Underwriting Information Services (255)
        "UI" => Ui,
        /// Motor Carrier Pickup Manifest (215)
        "UP" => Up,
        /// Insurance Underwriting Requirements Reporting (186)
        "UW" => Uw,
        /// Vehicle Application Advice (126)
        "VA" => Va,
        /// Vehicle Baying Order (127)
        "VB" => Vb,
        /// Vehicle Shipping Order (120)
        "VC" => Vc,
        /// Vehicle Damage (124)
        "VD" => Vd,
        /// Vessel Content Details (109)
        "VE" => Ve,
        /// Vehicle Carrier Rate Update (129)
        "VH" => Vh,
        /// Voter Registration Information (280)
        "VI" => Vi,
        /// Vehicle Service (121)
        "VS" => Vs,
        /// Product Service Transaction Sets (140, 141, 142, 143)
        "WA" => Wa,
        /// Rail Carrier Waybill Interchange (417)
        "WB" => Wb,
        /// Vendor Performance Review (501)
        "WG" => Wg,
        /// Wage Determination (288)
        "WI" => Wi,
        /// Well Information (625)
        "WL" => Wl,
        /// Shipment Weights (440)
        "WR" => Wr,
        /// Rail Waybill Request (425)
        "WT" => Wt,
    }
);
