//! X12 data elements 1600-1699.

crate::code_enum!(
    /// **1600** Freight Rate Qualifier
    ///
    /// - Data element: 1600
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Freight Rate Qualifier. Code values verified against the Stedi X12 reference.
    E1600 {
        /// Per Hundred Weight
        "CW" => Cw,
        /// Per Flat Rate
        "FR" => Fr,
        /// Per Each Unit
        "PE" => Pe,
        /// Per Cubic Foot
        "PF" => Pf,
        /// Per Gallon
        "PG" => Pg,
        /// Per Mile
        "PM" => Pm,
        /// Per Hour
        "PR" => Pr,
        /// Per Trailer
        "PT" => Pt,
        /// Per Square Yard
        "SY" => Sy,
    }
);

crate::code_enum!(
    /// **1601** Rated-as Qualifier
    ///
    /// - Data element: 1601
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Rated-as Qualifier. Code values verified against the Stedi X12 reference.
    E1601 {
        /// As Weight
        "AS" => As,
        /// Bumped Weight
        "BW" => Bw,
        /// Cubic Foot
        "CF" => Cf,
        /// Deficit Weight
        "DW" => Dw,
        /// Flat Rate
        "FR" => Fr,
        /// Gallon
        "GA" => Ga,
        /// Hours
        "HR" => Hr,
        /// Hundred Weight
        "HW" => Hw,
        /// Mile
        "MI" => Mi,
        /// Square Yard
        "SY" => Sy,
        /// Trailer
        "TR" => Tr,
        /// Unit
        "UN" => Un,
    }
);

crate::code_enum!(
    /// **1602** Bill of Lading Charge Code
    ///
    /// - Data element: 1602
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Bill of Lading Charge Code. Code values verified against the Stedi X12 reference.
    E1602 {
        /// Advance Fee
        "ADF" => Adf,
        /// Advance Amount
        "ADV" => Adv,
        /// Blocking and Bracing Charge
        "BLK" => Blk,
        /// Cash on Delivery
        "COD" => Cod,
        /// Fee for Collecting COD
        "COL" => Col,
        /// Delivery Charge
        "DEL" => Del,
        /// Detention of Power Unit
        "DEP" => Dep,
        /// Detention of Trailer
        "DET" => Det,
        /// Diversion and Reconsignment
        "DIB" => Dib,
        /// Discount
        "DSC" => Dsc,
        /// Export and/or Import Charge
        "EIC" => Eic,
        /// Exclusive Use Charge
        "EXC" => Exc,
        /// Flat Charge
        "FLT" => Flt,
        /// Fuel Surcharge
        "FSC" => Fsc,
        /// Inside Delivery
        "IDL" => Idl,
        /// Inside Pickup
        "IPU" => Ipu,
        /// Loading Allowance
        "LDA" => Lda,
        /// Unloading Allowance
        "LDL" => Ldl,
        /// Mileage Charge
        "MIL" => Mil,
        /// Minimum Charge
        "MIN" => Min,
        /// Marking or Tagging Charge
        "MRK" => Mrk,
        /// Miscellaneous Charge
        "MSG" => Msg,
        /// Order Notify Charge
        "ONC" => Onc,
        /// Placement Charge
        "PLA" => Pla,
        /// Pier Charges - Wharfage
        "PWH" => Pwh,
        /// Residential Pickup
        "REP" => Rep,
        /// Residential Delivery
        "RES" => Res,
        /// Stop Charge
        "SOC" => Soc,
        /// Single Pickup
        "SPU" => Spu,
        /// Storage
        "SRG" => Srg,
        /// Sufferance Warehouse
        "SUF" => Suf,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::code_enum!(
    /// **1634** Business Professional Title Code
    ///
    /// - Data element: 1634
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Business Professional Title Code. Code values verified against the Stedi X12 reference.
    E1634 {
        /// Accountant
        "1" => N1,
        /// General Secretary
        "1A" => N1A,
        /// Group Executive
        "1B" => N1B,
        /// Group Controller
        "1C" => N1C,
        /// Limited Partner
        "1D" => N1D,
        /// Manager
        "1E" => N1E,
        /// Managing Director
        "1F" => N1F,
        /// Managing Partner
        "1G" => N1G,
        /// Marketing Manager
        "1H" => N1H,
        /// Member of the Board
        "1J" => N1J,
        /// Member of the Board of Directors
        "1K" => N1K,
        /// Merchant Banker
        "1L" => N1L,
        /// Office Manager
        "1M" => N1M,
        /// Official Liquidator
        "1N" => N1N,
        /// Other Title
        "1O" => N1O,
        /// Owner
        "1P" => N1P,
        /// Partner
        "1Q" => N1Q,
        /// Personnel Manager
        "1R" => N1R,
        /// President
        "1S" => N1S,
        /// Proprietor
        "1T" => N1T,
        /// Purchasing Manager
        "1U" => N1U,
        /// Registrar
        "1V" => N1V,
        /// Sales Director
        "1W" => N1W,
        /// Sales Manager
        "1X" => N1X,
        /// Secretary
        "1Y" => N1Y,
        /// Secretary of the Board of Directors
        "1Z" => N1Z,
        /// Accounting Manager
        "2" => N2,
        /// Secretary-Treasurer
        "2A" => N2A,
        /// Senior Vice President
        "2B" => N2B,
        /// Sole Director
        "2C" => N2C,
        /// Spokesperson
        "2D" => N2D,
        /// Treasurer
        "2E" => N2E,
        /// Unlimited Partner
        "2F" => N2F,
        /// Vice Chairman of the Board
        "2G" => N2G,
        /// Vice President
        "2H" => N2H,
        /// Advertising Manager
        "2I" => N2I,
        /// Alternate Director
        "2J" => N2J,
        /// Commercial Director
        "2K" => N2K,
        /// Company Secretary
        "2L" => N2L,
        /// Delegated Manager
        "2M" => N2M,
        /// Deputy General Manager
        "2N" => N2N,
        /// Export Manager
        "2O" => N2O,
        /// Financial Director
        "2P" => N2P,
        /// Marketing Director
        "2Q" => N2Q,
        /// Operations Manager
        "2R" => N2R,
        /// Personnel Director
        "2S" => N2S,
        /// Production Manager
        "2T" => N2T,
        /// Referee
        "2U" => N2U,
        /// Sole Administrator
        "2V" => N2V,
        /// Technical Director
        "2W" => N2W,
        /// Trustee
        "2X" => N2X,
        /// Administrative Assistant
        "3" => N3,
        /// Ancillary
        "3A" => N3A,
        /// Consultant
        "3B" => N3B,
        /// Examiner
        "3C" => N3C,
        /// Instructor
        "3D" => N3D,
        /// PCP/Gatekeeper
        "3E" => N3E,
        /// Researcher
        "3F" => N3F,
        /// Specialist
        "3G" => N3G,
        /// Vendor
        "3H" => N3H,
        /// Chief Electoral Officer
        "3I" => N3I,
        /// Co-Chairperson
        "3J" => N3J,
        /// Commissioner
        "3K" => N3K,
        /// Government Agent
        "3L" => N3L,
        /// Notary
        "3M" => N3M,
        /// Principal Member
        "3N" => N3N,
        /// Principal Officer
        "3O" => N3O,
        /// Solicitor
        "3P" => N3P,
        /// Sponsor
        "3Q" => N3Q,
        /// Administrator
        "4" => N4,
        /// Assistant Secretary
        "5" => N5,
        /// Assistant Treasurer
        "6" => N6,
        /// Assistant Vice President
        "7" => N7,
        /// Associate
        "8" => N8,
        /// Attorney
        "9" => N9,
        /// Auditor
        "A" => A,
        /// Bookkeeper
        "B" => B,
        /// Branch Manager
        "C" => C,
        /// Cashier
        "D" => D,
        /// Certified Public Accountant
        "E" => E,
        /// Chairman
        "F" => F,
        /// Chairman of the Board
        "G" => G,
        /// Chairman of the Board of Directors
        "H" => H,
        /// Chief Executive Officer
        "I" => I,
        /// Chief Financial Officer
        "J" => J,
        /// Chief Operating Officer
        "K" => K,
        /// Clerk
        "L" => L,
        /// Commercial Manager
        "M" => M,
        /// Comptroller
        "N" => N,
        /// Controller
        "O" => O,
        /// Credit Manager
        "P" => P,
        /// EDP Manager
        "Q" => Q,
        /// Director
        "R" => R,
        /// Executive Director
        "S" => S,
        /// Executive Secretary
        "T" => T,
        /// Executive Vice President
        "U" => U,
        /// General Controller
        "V" => V,
        /// General Counsel
        "W" => W,
        /// General Manager
        "X" => X,
        /// General Partner
        "Y" => Y,
        /// Mutually Defined
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **1658** Shipment or Work Assignment Decline Reason Code
    ///
    /// - Data element: 1658
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Shipment or Work Assignment Decline Reason Code. Code values verified against the Stedi X12 reference.
    E1658 {
        /// Capacity Type
        "CPT" => Cpt,
        /// Capacity Unavailable
        "CPU" => Cpu,
        /// Equipment Type
        "EQT" => Eqt,
        /// Equipment Unavailable
        "EQU" => Equ,
        /// Length of Haul
        "LNH" => Lnh,
        /// Permits
        "PRM" => Prm,
        /// Weight
        "WGT" => Wgt,
    }
);

crate::code_enum!(
    /// **1698** Settlement Type Code
    ///
    /// - Data element: 1698
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Settlement Type Code. Code values verified against the Stedi X12 reference.
    E1698 {
        /// Full Settlement
        "A" => A,
        /// Partial Settlement
        "B" => B,
        /// Disputed Settlement
        "C" => C,
    }
);
