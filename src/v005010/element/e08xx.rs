//! X12 data elements 0800-0899.

crate::code_enum!(
    /// **812** Payment Format Code
    ///
    /// - Data element: 812
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Code identifying the payment format to be used (BPR-05).
    E812 {
        /// Cash Concentration/Disbursement (CCD)
        "CCD" => Ccd,
        /// Cash Concentration/Disbursement plus Addenda (CCD+)
        "CCP" => Ccp,
        /// Corporate Trade Exchange (CTX)
        "CTX" => Ctx,
        /// Prearranged Payment and Deposit (PPD)
        "PPD" => Ppd,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::num_element!(
    /// **810** Inner Pack
    ///
    /// - Data element: 810
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Inner Pack.
    E810
);

crate::num_element!(
    /// **826** Owners Share
    ///
    /// - Data element: 826
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Owners Share.
    E826
);

crate::num_element!(
    /// **857** Pre-Price Quantity Designator
    ///
    /// - Data element: 857
    /// - Type: Numeric (N)
    /// - Length: min 1, max 9
    ///
    /// Pre-Price Quantity Designator.
    E857
);

crate::num_element!(
    /// **858** Retail Pre-Price
    ///
    /// - Data element: 858
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Retail Pre-Price.
    E858
);

crate::num_element!(
    /// **895** Availability
    ///
    /// - Data element: 895
    /// - Type: Numeric (R)
    /// - Length: min 1, max 6
    ///
    /// Availability.
    E895
);

crate::code_enum!(
    /// **808** Hazardous Material Shipment Information Qualifier
    ///
    /// - Data element: 808
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Hazardous Material Shipment Information Qualifier. Code values verified against the Stedi X12 reference.
    E808 {
        /// Additional Descriptive Information Not Required by Regulation but Desired to Accompany the Movement by the Shipper
        "ADI" => Adi,
        /// Cargo Airlift Only
        "CAO" => Cao,
        /// "DOT - 113, Do Not Bump or Cut Off Car While in Motion" Declaration
        "D13" => D13,
        /// Damaged Car Number
        "DRC" => Drc,
        /// "Dangerous When Wet" Declaration
        "DWW" => Dww,
        /// Emergency Schedule (EMS) Page Number
        "EMS" => Ems,
        /// Conveyed Equipment Identification
        "EQP" => Eqp,
        /// Fumigation Declaration
        "FUM" => Fum,
        /// Identifies Products in a Heated Molten State
        "HOT" => Hot,
        /// Hazardous Substance Constituents
        "HZC" => Hzc,
        /// Inhalation Hazard
        "INH" => Inh,
        /// "Limited Quantity" Declaration
        "LQY" => Lqy,
        /// Medical First Aid Guide (MFAG) Page Number
        "MFA" => Mfa,
        /// Maximum Operating Speed
        "MOS" => Mos,
        /// Marine Pollutant
        "MPI" => Mpi,
        /// "Poison - Inhalation Hazard" Declaration
        "PIH" => Pih,
        /// "Poison" Declaration
        "POI" => Poi,
        /// Radioactive Material Data
        "RAM" => Ram,
        /// Technical or Chemical Group Name
        "TEC" => Tec,
        /// Trade Name
        "TNM" => Tnm,
        /// Waste Declaration
        "WST" => Wst,
    }
);

crate::code_enum!(
    /// **829** Fuel Type
    ///
    /// - Data element: 829
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Fuel Type. Code values verified against the Stedi X12 reference.
    E829 {
        /// Gasohol
        "A" => A,
        /// Butane
        "B" => B,
        /// Carbon Dioxide
        "C" => C,
        /// Diesel
        "D" => D,
        /// Auxiliary Electricity
        "E" => E,
        /// Electric
        "F" => F,
        /// Gas
        "G" => G,
        /// Compressed Natural Gas
        "H" => H,
        /// Liquid Natural Gas
        "I" => I,
        /// Ethanol
        "J" => J,
        /// Kerosene
        "K" => K,
        /// Liquefied Gases
        "L" => L,
        /// Methanol
        "M" => M,
        /// Natural Gas
        "N" => N,
        /// Other Unlisted Type of Fuel
        "O" => O,
        /// Propane
        "P" => P,
        /// E-85
        "Q" => Q,
        /// M-85
        "R" => R,
        /// Steam
        "S" => S,
        /// A55
        "T" => T,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **835** Supplemental Inspection Code
    ///
    /// - Data element: 835
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Supplemental Inspection Code. Code values verified against the Stedi X12 reference.
    E835 {
        /// Yes
        "Y" => Y,
    }
);

crate::code_enum!(
    /// **836** Vehicle Deck Position Code
    ///
    /// - Data element: 836
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Vehicle Deck Position Code. Code values verified against the Stedi X12 reference.
    E836 {
        /// Bottom level first position from front end.
        "A1" => A1,
        /// Bottom level second position from front end.
        "A2" => A2,
        /// Bottom level third position from front end.
        "A3" => A3,
        /// Bottom level fourth position from front end
        "A4" => A4,
        /// Bottom level fifth position from front end.
        "A5" => A5,
        /// Bottom level sixth position from front end.
        "A6" => A6,
        /// Bottom level seventh position from front end
        "A7" => A7,
        /// Bottom level eighth position from front end
        "A8" => A8,
        /// Bottom level ninth position from front end
        "A9" => A9,
        /// Bottom level twelfth position from front end
        "AX" => Ax,
        /// Bottom level eleventh position from front end
        "AY" => Ay,
        /// Bottom level tenth position from front end
        "AZ" => Az,
        /// Second level first position from front end.
        "B1" => B1,
        /// Second level second position from front end.
        "B2" => B2,
        /// Second level third position from front end.
        "B3" => B3,
        /// Second level fourth position from front end.
        "B4" => B4,
        /// Second level fifth position from front end.
        "B5" => B5,
        /// Second level sixth position from front end.
        "B6" => B6,
        /// Second level seventh position from front end.
        "B7" => B7,
        /// Second level eighth position from front end
        "B8" => B8,
        /// Second level ninth position from front end
        "B9" => B9,
        /// Second level twelfth position from front end
        "BX" => Bx,
        /// Second level eleventh position from front end
        "BY" => By,
        /// Second level tenth position from front end
        "BZ" => Bz,
        /// Third level first position from front end.
        "C1" => C1,
        /// Third level second position from front end.
        "C2" => C2,
        /// Third level third position from front end.
        "C3" => C3,
        /// Third level fourth position from front end.
        "C4" => C4,
        /// Third level fifth position from front end.
        "C5" => C5,
        /// Third level sixth position from front end.
        "C6" => C6,
        /// Third level seventh position from front end.
        "C7" => C7,
        /// Third level eighth position from front end
        "C8" => C8,
        /// Third level ninth position from front end
        "C9" => C9,
        /// Third level twelfth position from front end
        "CX" => Cx,
        /// Third level eleventh position from front end
        "CY" => Cy,
        /// Third level tenth position from front end
        "CZ" => Cz,
    }
);

crate::code_enum!(
    /// **837** Vehicle Type Code
    ///
    /// - Data element: 837
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Vehicle Type Code. Code values verified against the Stedi X12 reference.
    E837 {
        /// Automobile
        "1" => N1,
        /// Truck
        "2" => N2,
        /// Others
        "3" => N3,
        /// Used Vehicles
        "4" => N4,
        /// Military
        "5" => N5,
        /// Passenger Car
        "C" => C,
        /// Small Passenger Car
        "I" => I,
        /// Multipurpose Vehicle
        "M" => M,
        /// Light Truck
        "T" => T,
        /// Large Van
        "V" => V,
        /// Extended Minivan
        "X" => X,
    }
);

crate::code_enum!(
    /// **844** Inbound Condition Hold Code
    ///
    /// - Data element: 844
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Inbound Condition Hold Code. Code values verified against the Stedi X12 reference.
    E844 {
        /// QC Hold
        "01" => N01,
        /// Hold for further information
        "02" => N02,
        /// Committed hold
        "03" => N03,
    }
);

crate::code_enum!(
    /// **846** Contract Status Code
    ///
    /// - Data element: 846
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Contract Status Code. Code values verified against the Stedi X12 reference.
    E846 {
        /// Address Change
        "AC" => Ac,
        /// Annuitized
        "AN" => An,
        /// Awaiting Initial Premium
        "AW" => Aw,
        /// Contract Award
        "CA" => Ca,
        /// Cancelled by IRS
        "CB" => Cb,
        /// Contract Cancelled
        "CC" => Cc,
        /// Closed contract - all quantities taken
        "CL" => Cl,
        /// Contract Modified
        "CM" => Cm,
        /// Contract Renewed
        "CR" => Cr,
        /// Contract Cancelled - Refund Due
        "CX" => Cx,
        /// Disabled
        "DA" => Da,
        /// Death
        "DE" => De,
        /// Deleted Contract
        "DL" => Dl,
        /// Expired Contract
        "EX" => Ex,
        /// Free Look Period
        "FL" => Fl,
        /// General Suspension
        "GS" => Gs,
        /// Hold
        "HO" => Ho,
        /// Inactive
        "IA" => Ia,
        /// Internal Exchange
        "IE" => Ie,
        /// Invalid Contract Due To Terms
        "IN" => In,
        /// Matured
        "MA" => Ma,
        /// Contract Provider No Longer in Business
        "NB" => Nb,
        /// Not Taken
        "NT" => Nt,
        /// Original Contract
        "OC" => Oc,
        /// Paid Up
        "PA" => Pa,
        /// Pending
        "PB" => Pb,
        /// Pending Death Notification
        "PC" => Pc,
        /// Pending Exchange Transfer
        "PD" => Pd,
        /// Prepaid Service Authorization
        "PP" => Pp,
        /// Proposed Contract
        "PR" => Pr,
        /// Reissue
        "RA" => Ra,
        /// Rescinded
        "RB" => Rb,
        /// Restricted
        "RC" => Rc,
        /// Reopened Contract
        "RO" => Ro,
        /// Standard Contract
        "SC" => Sc,
        /// Surrendered
        "SU" => Su,
        /// Terminated
        "TA" => Ta,
        /// Contract Transferred
        "TR" => Tr,
        /// Valid Open Contract
        "VA" => Va,
    }
);

crate::code_enum!(
    /// **847** Order/Item Code
    ///
    /// - Data element: 847
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Order/Item Code. Code values verified against the Stedi X12 reference.
    E847 {
        /// All orders - items with changed status
        "AO" => Ao,
        /// All Orders - All Items
        "CA" => Ca,
        /// All Orders - Shipped Items
        "CI" => Ci,
        /// All Orders - Unshipped Items
        "CO" => Co,
        /// Selected Orders - All Items
        "PA" => Pa,
        /// Selected Orders - Shipped Orders
        "PI" => Pi,
        /// Selected Orders - Unshipped Items
        "PO" => Po,
        /// Selected Orders - Selected Items
        "PP" => Pp,
    }
);

crate::code_enum!(
    /// **848** Product/Date Code
    ///
    /// - Data element: 848
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Product/Date Code. Code values verified against the Stedi X12 reference.
    E848 {
        /// Selected Products and Selected Date Parameters
        "PD" => Pd,
        /// Selected Date Parameters
        "SD" => Sd,
        /// Selected Products
        "SP" => Sp,
    }
);

crate::code_enum!(
    /// **849** Location Code
    ///
    /// - Data element: 849
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Location Code. Code values verified against the Stedi X12 reference.
    E849 {
        /// Selected Supplier Location(s)
        "SB" => Sb,
        /// Selected Buyer Location(s)
        "SL" => Sl,
        /// Selected Supplier and Buyer Locations
        "SS" => Ss,
    }
);

crate::code_enum!(
    /// **853** Damage Reason Code
    ///
    /// - Data element: 853
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Damage Reason Code. Code values verified against the Stedi X12 reference.
    E853 {
        /// Bulkhead Doors Not Secured
        "BD" => Bd,
        /// Case Crushing (Exterior/Secondary Packaging)
        "CC" => Cc,
        /// Case Design
        "CD" => Cd,
        /// Damage Loading
        "DL" => Dl,
        /// Defective Pallets of Slip Sheets
        "DP" => Dp,
        /// Flaps Loose
        "FL" => Fl,
        /// Hidden Damage (Suspected or Actual)
        "HD" => Hd,
        /// Improper Doorway Protection
        "ID" => Id,
        /// Improper Loading
        "IL" => Il,
        /// Improper Unloading
        "IU" => Iu,
        /// Lack of Cardboard Dividers/Dunnage
        "LD" => Ld,
        /// Load Shift
        "LS" => Ls,
        /// Nails or Other Hardware
        "NO" => No,
        /// Not Reported
        "NR" => Nr,
        /// Pallet Pattern
        "PP" => Pp,
        /// Punctured Bailers/Cases
        "PS" => Ps,
        /// Side Fillers Not Extended
        "SF" => Sf,
        /// Temperature Exposure
        "TE" => Te,
        /// Unsuitable Equipment
        "UE" => Ue,
        /// Vehicle Impact
        "VI" => Vi,
        /// Wet or Stained (Shipping Containers or Secondary Packaging)
        "WC" => Wc,
    }
);

crate::code_enum!(
    /// **875** Maintenance Type Code
    ///
    /// - Data element: 875
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Maintenance Type Code. Code values verified against the Stedi X12 reference.
    E875 {
        /// Change
        "001" => N001,
        /// Delete
        "002" => N002,
        /// Add Full Item Detail
        "003" => N003,
        /// Withdraw Item (Temporary)
        "004" => N004,
        /// Restore Item
        "005" => N005,
        /// Membership Type
        "006" => N006,
        /// Purchase Eligibility
        "007" => N007,
        /// Verified
        "008" => N008,
        /// Not Verified
        "009" => N009,
        /// Periodic Report
        "010" => N010,
        /// All Items Refresh
        "011" => N011,
        /// Medical Noncompliance
        "012" => N012,
        /// Administrative Noncompliance
        "013" => N013,
        /// Suspended Pending Settlement Approval
        "014" => N014,
        /// Suspended Pending Appeal or Judicial Review
        "015" => N015,
        /// Injury Report Creation
        "016" => N016,
        /// Illness Report Creation
        "017" => N017,
        /// Transfer of Beneficial Rights
        "018" => N018,
        /// Record Security Instrument
        "019" => N019,
        /// Assignment
        "020" => N020,
        /// Addition
        "021" => N021,
        /// Change in Status
        "022" => N022,
        /// Change in Rate Factors
        "023" => N023,
        /// Cancellation or Termination
        "024" => N024,
        /// Reinstatement
        "025" => N025,
        /// Correction
        "026" => N026,
        /// Policy Adjustment
        "028" => N028,
        /// Inquiry
        "029" => N029,
        /// Audit or Compare
        "030" => N030,
        /// Medical Examination Authorization
        "031" => N031,
        /// Employee Information Not Applicable
        "032" => N032,
        /// Release of Interim Funding Interest
        "033" => N033,
        /// Original
        "050" => N050,
        /// Denial
        "051" => N051,
        /// Initial Payment
        "052" => N052,
        /// Change in Benefit Amount
        "053" => N053,
        /// Change in Benefit Type
        "054" => N054,
        /// Reinstatement of Benefit
        "055" => N055,
        /// Reduced Earnings
        "056" => N056,
        /// Final
        "057" => N057,
        /// Suspension Medically Determined or Qualified to Return to Work
        "059" => N059,
        /// Non-compliance of Medical Requirements
        "060" => N060,
        /// Non-compliance of Administrative Requirements
        "061" => N061,
        /// Compensable Death
        "062" => N062,
        /// Incarceration
        "063" => N063,
        /// Claimant's Whereabouts Unknown
        "064" => N064,
        /// Volunteer
        "065" => N065,
        /// Status Request
        "066" => N066,
        /// Benefits Exhausted
        "067" => N067,
        /// Jurisdictional Change
        "068" => N068,
        /// Payment Notification
        "069" => N069,
        /// Status Request Response
        "070" => N070,
        /// Re-issue Identification Card(s)
        "071" => N071,
        /// Suspension Due to Claimant Death
        "072" => N072,
        /// Partial Denial
        "073" => N073,
        /// Partial Suspension, Returned to Work, or Medically Determined or Qualified to Return to Work
        "074" => N074,
        /// Partial Suspension, Medical Non-compliance
        "075" => N075,
        /// Partial Suspension, Administrative Non-compliance
        "076" => N076,
        /// Partial Suspension, Claimant Death
        "077" => N077,
        /// Partial Suspension, Incarceration
        "078" => N078,
        /// Partial Suspension, Claimant's whereabouts Unknown
        "079" => N079,
        /// Partial Suspension, Benefits Exhausted
        "080" => N080,
        /// Partial Suspension, Jurisdiction Change
        "081" => N081,
        /// Partially Suspended, Benefits Pending Settlement Approval
        "082" => N082,
        /// Partially Suspended, Pending Appeal or Judicial Review
        "083" => N083,
        /// Lump Sum Benefit Payment
        "084" => N084,
        /// Suspension Denial
        "085" => N085,
        /// Sale of Mortgage Change
        "100" => N100,
        /// Servicer Change
        "101" => N101,
        /// Mortgagor Change
        "102" => N102,
        /// Prepayment Termination
        "104" => N104,
        /// Non-conveyance Termination
        "105" => N105,
        /// Voluntary Action Termination
        "106" => N106,
        /// Loan Refinanced Termination
        "107" => N107,
        /// Transfer Due to Interest and Servicing Sold
        "108" => N108,
        /// Retain Servicing
        "109" => N109,
        /// Transfer Servicing
        "110" => N110,
        /// Assign Servicing
        "111" => N111,
        /// Servicer Loan Number Change
        "112" => N112,
        /// Merger
        "113" => N113,
        /// Acquisition
        "114" => N114,
        /// Receivership
        "115" => N115,
        /// Loan Maturity Maintenance
        "116" => N116,
        /// Binder
        "117" => N117,
        /// New Policy
        "118" => N118,
        /// Renewal Policy
        "119" => N119,
        /// Canceled by Administrator
        "123" => N123,
        /// Canceled by Insured
        "124" => N124,
        /// Re-Write
        "125" => N125,
        /// Non-renewal
        "126" => N126,
        /// Termination of All Endorsements
        "127" => N127,
        /// Termination of Lessor Interest
        "128" => N128,
        /// Termination of Additional Insured Interest
        "129" => N129,
        /// Termination of Lienholder or Loss Payee
        "130" => N130,
        /// Addition of Lessor Endorsement
        "131" => N131,
        /// Addition of Lienholder Endorsement
        "132" => N132,
        /// Addition of Additional Insured Endorsement
        "133" => N133,
        /// Change of Lessor Endorsement
        "134" => N134,
        /// Change of Lienholder Endorsement
        "135" => N135,
        /// Change of Additional Insured Endorsement
        "136" => N136,
        /// Acquired Unallocated
        "137" => N137,
        /// Acquired Payment
        "138" => N138,
        /// Investigation Pending
        "139" => N139,
        /// Concurrent Benefit
        "140" => N140,
        /// Abbreviated First Report on an Acquired Claim
        "141" => N141,
        /// Employer Paid
        "142" => N142,
        /// Employer Reinstatement
        "143" => N143,
        /// Suspension Directed by Agency
        "144" => N144,
        /// Occupational Safety and Health Administration Injury and Illness Log (OSHA-300)
        "145" => N145,
        /// Occupational Injury Survey
        "146" => N146,
        /// Bureau of Labor and Statistics Survey
        "147" => N147,
        /// First Payment on an Acquired Claim
        "148" => N148,
        /// First Report on an Acquired Claim
        "149" => N149,
        /// Impairment Report
        "150" => N150,
        /// Return to Work Report
        "151" => N151,
        /// Office of Workers Compensation Program Injury Claim Report (OWCP CA-1)
        "152" => N152,
        /// Office of Workers Compensation Program Illness Claim Report (OWCP CA-2)
        "153" => N153,
        /// Occupational Safety and Health Administration Injury and Illness Record (OSHA-301)
        "154" => N154,
        /// First Aid Injury Only
        "155" => N155,
        /// Change Excluding Price
        "CEP" => Cep,
        /// Price Change Only
        "PRI" => Pri,
        /// To Be Verified
        "TBV" => Tbv,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::code_enum!(
    /// **894** Batch Type Code
    ///
    /// - Data element: 894
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Batch Type Code. Code values verified against the Stedi X12 reference.
    E894 {
        /// Regular
        "01" => N01,
        /// Modified
        "02" => N02,
        /// Errors
        "03" => N03,
        /// Late
        "04" => N04,
        /// Returns
        "05" => N05,
        /// Advance or Deposit
        "06" => N06,
        /// Collection Status
        "07" => N07,
        /// Cashless
        "09" => N09,
        /// Resubmission
        "10" => N10,
        /// Miscellaneous Debit
        "11" => N11,
        /// Miscellaneous Credit
        "12" => N12,
        /// Insufficient Information
        "13" => N13,
    }
);

crate::code_enum!(
    /// **897** Vessel Code Qualifier
    ///
    /// - Data element: 897
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Vessel Code Qualifier. Code values verified against the Stedi X12 reference.
    E897 {
        /// U.S. Bureau of Census
        "B" => B,
        /// Ship's Radio Call Signal
        "C" => C,
        /// Lloyd's Register of Shipping
        "L" => L,
        /// Mutually Defined
        "Z" => Z,
    }
);
