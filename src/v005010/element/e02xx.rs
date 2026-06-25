//! X12 data elements 0200-0299.

crate::code_enum!(
    /// **208** Hazardous Material Code Qualifier
    ///
    /// - Data element: 208
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code which qualifies the Hazardous Material Class Code (209).
    E208 {
        /// 46 Level DOT Code
        "4" => N4,
        /// Airline Tariff 6D
        "6" => N6,
        /// Title 49, Code of Federal Regulations (CFR)
        "9" => N9,
        /// International Civil Aviation Organization (ICAO) Code
        "A" => A,
        /// Uniform Fire Code (UFC)
        "B" => B,
        /// Hazardous Materials ID, DOT
        "D" => D,
        /// Intergovernmental Maritime Organization (IMO) Code
        "I" => I,
        /// International Air Transport Association Dangerous Goods Code List
        "T" => T,
        /// United Nations
        "U" => U,
        /// Hazard Class or Division
        "X" => X,
    }
);

crate::code_enum!(
    /// **209** Hazardous Material Class Code
    ///
    /// - Data element: 209
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 4
    ///
    /// The hazard class of a material (UN hazard classes 1-9).
    E209 {
        /// Explosives
        "1" => N1,
        /// Gases
        "2" => N2,
        /// Flammable Liquids
        "3" => N3,
        /// Flammable Solids
        "4" => N4,
        /// Oxidizers and Organic Peroxides
        "5" => N5,
        /// Toxic and Infectious Substances
        "6" => N6,
        /// Radioactive Material
        "7" => N7,
        /// Corrosives
        "8" => N8,
        /// Miscellaneous Dangerous Goods
        "9" => N9,
    }
);

crate::code_enum!(
    /// **248** Allowance or Charge Indicator
    ///
    /// - Data element: 248
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code which indicates an allowance or charge for the service specified.
    E248 {
        /// Allowance
        "A" => A,
        /// Charge
        "C" => C,
        /// No Allowance or Charge
        "N" => N,
        /// Charge to be Subtracted from Order
        "S" => S,
    }
);

crate::code_enum!(
    /// **235** Product/Service ID Qualifier
    ///
    /// - Data element: 235
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the type/source of the descriptive number used in
    /// Product/Service ID (234). Common codes named below.
    E235 {
        /// Buyer's Item Number
        "BP" => Bp,
        /// EAN/UCC - 13
        "EN" => En,
        /// EAN/UCC - 8
        "EO" => Eo,
        /// Grade
        "GR" => Gr,
        /// Purchase Order Number
        "PO" => Po,
        /// Style Number
        "SN" => Sn,
        /// UPC Consumer Package Code (1-5-5)
        "UA" => Ua,
        /// UPC/EAN Shipping Container Code (1-2-5-5)
        "UK" => Uk,
        /// UPC Consumer Package Code (1-5-5-1)
        "UP" => Up,
        /// Vendor's (Seller's) Item Number
        "VN" => Vn,
        /// Vendor's Style Number
        "VS" => Vs,
        /// Commodity
        "C3" => C3,
    }
);

crate::num_element!(
    /// **205** Dunnage
    ///
    /// - Data element: 205
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Dunnage.
    E205
);

crate::num_element!(
    /// **212** Unit Price
    ///
    /// - Data element: 212
    /// - Type: Numeric (R)
    /// - Length: min 1, max 17
    ///
    /// Unit Price.
    E212
);

crate::num_element!(
    /// **213** Lading Line Item Number
    ///
    /// - Data element: 213
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 3
    ///
    /// Lading Line Item Number.
    E213
);

crate::num_element!(
    /// **220** Billed/Rated-as Quantity
    ///
    /// - Data element: 220
    /// - Type: Numeric (R)
    /// - Length: min 1, max 11
    ///
    /// Billed/Rated-as Quantity.
    E220
);

crate::num_element!(
    /// **232** Weight Allowance
    ///
    /// - Data element: 232
    /// - Type: Numeric (N0)
    /// - Length: min 2, max 6
    ///
    /// Weight Allowance.
    E232
);

crate::num_element!(
    /// **237** Item List Cost
    ///
    /// - Data element: 237
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Item List Cost.
    E237
);

crate::num_element!(
    /// **258** Quantity Cost
    ///
    /// - Data element: 258
    /// - Type: Numeric (N)
    /// - Length: min 1, max 9
    ///
    /// Quantity Cost.
    E258
);

crate::num_element!(
    /// **267** Net Explosive Quantity
    ///
    /// - Data element: 267
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Net Explosive Quantity.
    E267
);

crate::date_element!(
    /// **275** Authorization Date
    ///
    /// - Data element: 275
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Authorization Date.
    E275
);

crate::date_element!(
    /// **282** Terms Start Date
    ///
    /// - Data element: 282
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Terms Start Date.
    E282
);

crate::num_element!(
    /// **289** Multiple Price Quantity
    ///
    /// - Data element: 289
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 2
    ///
    /// Multiple Price Quantity.
    E289
);

crate::code_enum!(
    /// **201** Business Transaction Status
    ///
    /// - Data element: 201
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Business Transaction Status. Code values verified against the Stedi X12 reference.
    E201 {
        /// B/L Not Received
        "BNR" => Bnr,
        /// Demurrage Completely Paid
        "DCP" => Dcp,
        /// Demurrage Not Paid
        "DNP" => Dnp,
        /// Demurrage Partially Paid
        "DPP" => Dpp,
        /// Freight Completely Paid
        "FCP" => Fcp,
        /// Freight Not Paid
        "FNP" => Fnp,
        /// Freight Partially Paid
        "FPP" => Fpp,
        /// Letter of Guarantee Received
        "LGR" => Lgr,
        /// Original B/L Received
        "OBR" => Obr,
        /// Transaction Completely Rated
        "TCR" => Tcr,
        /// Transaction Not Rated
        "TNR" => Tnr,
        /// Transaction Partially Rated
        "TPR" => Tpr,
    }
);

crate::code_enum!(
    /// **211** Packaging Form Code
    ///
    /// - Data element: 211
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Packaging Form Code. Code values verified against the Stedi X12 reference.
    E211 {
        /// Ammo Pack
        "AMM" => Amm,
        /// Bag
        "BAG" => Bag,
        /// Bale
        "BAL" => Bal,
        /// Barrel
        "BBL" => Bbl,
        /// Bundle
        "BDL" => Bdl,
        /// Beam
        "BEM" => Bem,
        /// Bing Chest
        "BIC" => Bic,
        /// Bin
        "BIN" => Bin,
        /// Bulk Bag
        "BKG" => Bkg,
        /// Bulk
        "BLK" => Blk,
        /// Bobbin
        "BOB" => Bob,
        /// Bottle
        "BOT" => Bot,
        /// Box
        "BOX" => Box,
        /// Barge
        "BRG" => Brg,
        /// Basket or hamper
        "BSK" => Bsk,
        /// Box, with inner container
        "BXI" => Bxi,
        /// Bucket
        "BXT" => Bxt,
        /// Cabinet
        "CAB" => Cab,
        /// Cage
        "CAG" => Cag,
        /// Can
        "CAN" => Can,
        /// Carrier
        "CAR" => Car,
        /// Case
        "CAS" => Cas,
        /// Containers of Bulk Cargo
        "CBC" => Cbc,
        /// Carboy
        "CBY" => Cby,
        /// Can Case
        "CCS" => Ccs,
        /// Cheeses
        "CHE" => Che,
        /// Chest
        "CHS" => Chs,
        /// Car Load, Rail
        "CLD" => Cld,
        /// Household Goods Containers, Wood
        "CNA" => Cna,
        /// Container, MAC-ISO (Military Airlift Container - International Standards Organization) Light Weight 8x8x20 Foot Air
        "CNB" => Cnb,
        /// Container, Navy Cargo Transporter
        "CNC" => Cnc,
        /// Container, Commercial Highway Lift
        "CND" => Cnd,
        /// Engine Container
        "CNE" => Cne,
        /// Multiwall Container Secured to Warehouse Pallet
        "CNF" => Cnf,
        /// Container
        "CNT" => Cnt,
        /// CONEX - Container Express
        "CNX" => Cnx,
        /// Coil
        "COL" => Col,
        /// Cones
        "CON" => Con,
        /// Core
        "COR" => Cor,
        /// Cradle
        "CRD" => Crd,
        /// Crate
        "CRT" => Crt,
        /// Cask
        "CSK" => Csk,
        /// Carton
        "CTN" => Ctn,
        /// Cube
        "CUB" => Cub,
        /// Cylinder
        "CYL" => Cyl,
        /// Dry Bulk
        "DBK" => Dbk,
        /// Double-length Rack
        "DRK" => Drk,
        /// Drum
        "DRM" => Drm,
        /// Double-length Skid
        "DSK" => Dsk,
        /// Double-length Tote Bin
        "DTB" => Dtb,
        /// Duffle Bag
        "DUF" => Duf,
        /// Envelope
        "ENV" => Env,
        /// Firkin
        "FIR" => Fir,
        /// Flo-bin
        "FLO" => Flo,
        /// Liner Bag Liquid
        "FLX" => Flx,
        /// Frame
        "FRM" => Frm,
        /// Flask
        "FSK" => Fsk,
        /// Forward Reel
        "FWR" => Fwr,
        /// Garments on Hangers
        "GOH" => Goh,
        /// Heads of Beef
        "HED" => Hed,
        /// Hogshead
        "HGH" => Hgh,
        /// Hamper
        "HPR" => Hpr,
        /// Hopper Truck
        "HPT" => Hpt,
        /// On Hanger or Rack in Boxes
        "HRB" => Hrb,
        /// Half-standard Rack
        "HRK" => Hrk,
        /// Half-Standard Tote Bin
        "HTB" => Htb,
        /// Jar
        "JAR" => Jar,
        /// Jug
        "JUG" => Jug,
        /// Keg
        "KEG" => Keg,
        /// Kit
        "KIT" => Kit,
        /// Knockdown Rack
        "KRK" => Krk,
        /// Knockdown Tote Bin
        "KTB" => Ktb,
        /// Liquid Bulk
        "LBK" => Lbk,
        /// Lifts
        "LIF" => Lif,
        /// Log
        "LOG" => Log,
        /// Loose
        "LSE" => Lse,
        /// Lug
        "LUG" => Lug,
        /// Lift Van
        "LVN" => Lvn,
        /// MILVAN - Military Van
        "MLV" => Mlv,
        /// Multi-Roll Pack
        "MRP" => Mrp,
        /// MSCVAN - Military Sealift Command Van
        "MSV" => Msv,
        /// Mixed Type Pack
        "MXD" => Mxd,
        /// Noil
        "NOL" => Nol,
        /// Overwrap
        "OVW" => Ovw,
        /// Pail
        "PAL" => Pal,
        /// Packed - not otherwise specified
        "PCK" => Pck,
        /// Pieces
        "PCS" => Pcs,
        /// Pims
        "PIR" => Pir,
        /// Package
        "PKG" => Pkg,
        /// Platform
        "PLF" => Plf,
        /// 463L Air Pallet
        "PLL" => Pll,
        /// Pipeline
        "PLN" => Pln,
        /// Pallet
        "PLT" => Plt,
        /// Private Vehicle
        "POV" => Pov,
        /// Pipe Rack
        "PRK" => Prk,
        /// Quarter of Beef
        "QTR" => Qtr,
        /// Rail (Semiconductor)
        "RAL" => Ral,
        /// Rack
        "RCK" => Rck,
        /// Reel
        "REL" => Rel,
        /// Roll
        "ROL" => Rol,
        /// Reverse Reel
        "RVR" => Rvr,
        /// Sack
        "SAK" => Sak,
        /// Liner Bag Dry
        "SBC" => Sbc,
        /// Suitcase
        "SCS" => Scs,
        /// Shook
        "SHK" => Shk,
        /// Sheet
        "SHT" => Sht,
        /// Side of Beef
        "SID" => Sid,
        /// Skid
        "SKD" => Skd,
        /// Skid, elevating or lift truck
        "SKE" => Ske,
        /// Slip Sheet
        "SLP" => Slp,
        /// Sleeve
        "SLV" => Slv,
        /// Spin Cylinders
        "SPI" => Spi,
        /// Spool
        "SPL" => Spl,
        /// SEAVAN - Sea Van
        "SVN" => Svn,
        /// Tube
        "TBE" => Tbe,
        /// Tote Bin
        "TBN" => Tbn,
        /// Tank Car
        "TKR" => Tkr,
        /// Tank Truck
        "TKT" => Tkt,
        /// Intermodal Trailer/Container Load (Rail)
        "TLD" => Tld,
        /// Tank
        "TNK" => Tnk,
        /// Tierce
        "TRC" => Trc,
        /// Triwall Box
        "TRI" => Tri,
        /// Trunk and Chest
        "TRK" => Trk,
        /// Truck
        "TRU" => Tru,
        /// Tray
        "TRY" => Try,
        /// Trunk, Salesmen Sample
        "TSS" => Tss,
        /// Tote Can
        "TTC" => Ttc,
        /// Tub
        "TUB" => Tub,
        /// Unpacked
        "UNP" => Unp,
        /// Unit
        "UNT" => Unt,
        /// Vehicles
        "VEH" => Veh,
        /// Van Pack
        "VPK" => Vpk,
        /// On Own Wheel
        "WHE" => Whe,
        /// Wheeled Carrier
        "WLC" => Wlc,
        /// Wrapped
        "WRP" => Wrp,
    }
);

crate::code_enum!(
    /// **214** Waybill Request Code
    ///
    /// - Data element: 214
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Waybill Request Code. Code values verified against the Stedi X12 reference.
    E214 {
        /// Electronic Car Movement
        "C" => C,
        /// Electronic Revenue Waybill
        "E" => E,
        /// Electronic Haulage Waybill
        "H" => H,
        /// Paper Revenue Waybill Document
        "P" => P,
    }
);

crate::code_enum!(
    /// **221** Billed/Rated-as Qualifier
    ///
    /// - Data element: 221
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Billed/Rated-as Qualifier. Code values verified against the Stedi X12 reference.
    E221 {
        /// Appurtenance (Enhancements/Additions to Equipment)
        "AR" => Ar,
        /// Barrels
        "BA" => Ba,
        /// Box
        "BX" => Bx,
        /// Cubic Centimeter
        "CC" => Cc,
        /// Cubic Foot
        "CF" => Cf,
        /// Centimeter
        "CM" => Cm,
        /// Kilometers
        "DK" => Dk,
        /// Miles
        "DM" => Dm,
        /// Drivers
        "DR" => Dr,
        /// Each
        "EA" => Ea,
        /// Flat Rate
        "FR" => Fr,
        /// Foot
        "FT" => Ft,
        /// 100 Gallons
        "GC" => Gc,
        /// Gallon
        "GL" => Gl,
        /// Kilogram
        "KG" => Kg,
        /// Pound
        "LB" => Lb,
        /// 100 Pounds
        "LC" => Lc,
        /// 100 Liters
        "LH" => Lh,
        /// Liter
        "LR" => Lr,
        /// Mileage
        "MR" => Mr,
        /// Measurement Ton
        "MT" => Mt,
        /// Monetary Value
        "MV" => Mv,
        /// Barge
        "NB" => Nb,
        /// Car
        "NC" => Nc,
        /// Cord
        "ND" => Nd,
        /// Nights
        "NG" => Ng,
        /// Load
        "NL" => Nl,
        /// Train
        "NN" => Nn,
        /// Piece
        "NP" => Np,
        /// Container
        "NR" => Nr,
        /// Trailer
        "NT" => Nt,
        /// Unit
        "NU" => Nu,
        /// Vehicle
        "NV" => Nv,
        /// Other
        "OR" => Or,
        /// Package
        "PK" => Pk,
        /// Persons
        "PR" => Pr,
        /// Release Value
        "RV" => Rv,
        /// Stops
        "SP" => Sp,
        /// Number of States
        "ST" => St,
        /// Square Yards
        "SY" => Sy,
        /// Days
        "TD" => Td,
        /// Hours
        "TH" => Th,
        /// Tons
        "TN" => Tn,
        /// Time
        "TR" => Tr,
        /// Actual Volume
        "VA" => Va,
        /// Chargeable Volume
        "VC" => Vc,
        /// Volume Metric Unit
        "VM" => Vm,
    }
);

crate::code_enum!(
    /// **226** Section Seven Code
    ///
    /// - Data element: 226
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Section Seven Code. Code values verified against the Stedi X12 reference.
    E226 {
        /// Not in Effect
        "N" => N,
        /// In Effect
        "S" => S,
    }
);

crate::code_enum!(
    /// **231** Cross Reference Type Code
    ///
    /// - Data element: 231
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Cross Reference Type Code. Code values verified against the Stedi X12 reference.
    E231 {
        /// Additional Equipment for Manifest Billing
        "A" => A,
        /// Basket Attached to Chassis (to hold generator set)
        "B" => B,
        /// Prior Load Credit
        "C" => C,
        /// Chassis Attached to Container
        "D" => D,
        /// Conveying Flat Car
        "F" => F,
        /// Reference to Lead Equipment for Manifest Billing
        "G" => G,
        /// Generator Set
        "H" => H,
        /// Clip-on Front-Mounted Generator Unit For Container
        "K" => K,
        /// Lead Car
        "L" => L,
        /// Mated
        "M" => M,
        /// Cryogenic Apparatus
        "N" => N,
        /// Tractor
        "R" => R,
        /// Saddle to Conveying Flatcar
        "S" => S,
        /// Trailer
        "T" => T,
        /// Transfer Load
        "X" => X,
    }
);

crate::code_enum!(
    /// **236** Price Identifier Code
    ///
    /// - Data element: 236
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Price Identifier Code. Code values verified against the Stedi X12 reference.
    E236 {
        /// Actual
        "ACT" => Act,
        /// Average Generic Product Price
        "AGC" => Agc,
        /// Alternate Price
        "ALT" => Alt,
        /// Average Wholesale Price
        "AWP" => Awp,
        /// Balance-Based Price
        "BBP" => Bbp,
        /// Base Charge
        "BCH" => Bch,
        /// Bid Price
        "BID" => Bid,
        /// Contract Tier 1
        "C01" => C01,
        /// Contract Tier 2
        "C02" => C02,
        /// Contract Tier 3
        "C03" => C03,
        /// Contract Tier 4
        "C04" => C04,
        /// Contract Tier 5
        "C05" => C05,
        /// Contract Tier 6
        "C06" => C06,
        /// Contract Tier 7
        "C07" => C07,
        /// Contract Tier 8
        "C08" => C08,
        /// Contract Tier 9
        "C09" => C09,
        /// Contract Tier 10
        "C10" => C10,
        /// Contract Tier 11
        "C11" => C11,
        /// Contract Tier 12
        "C12" => C12,
        /// Contract Tier 13
        "C13" => C13,
        /// Contract Tier 14
        "C14" => C14,
        /// Contract Tier 15
        "C15" => C15,
        /// Contract Tier 16
        "C16" => C16,
        /// Contract Tier 17
        "C17" => C17,
        /// Contract Tier 18
        "C18" => C18,
        /// Contract Tier 19
        "C19" => C19,
        /// Contract Tier 20
        "C20" => C20,
        /// Contract Tier 21
        "C21" => C21,
        /// Contract Tier 22
        "C22" => C22,
        /// Contract Tier 23
        "C23" => C23,
        /// Contract Tier 24
        "C24" => C24,
        /// Contract Tier 25
        "C25" => C25,
        /// Contract Tier 26
        "C26" => C26,
        /// Contract Tier 27
        "C27" => C27,
        /// Contract Tier 28
        "C28" => C28,
        /// Contract Tier 29
        "C29" => C29,
        /// Contract Tier 30
        "C30" => C30,
        /// Cancellation Charge
        "CAN" => Can,
        /// Catalog Price
        "CAT" => Cat,
        /// Central Distribution Facility (Warehouse)
        "CDF" => Cdf,
        /// Current Domestic Value
        "CDV" => Cdv,
        /// Changed Price
        "CHG" => Chg,
        /// Contract Price
        "CON" => Con,
        /// Confirmed Unit Price
        "CUP" => Cup,
        /// Declared Customs Unit Value
        "CUS" => Cus,
        /// Federal Supply Schedule (FSS) Price
        "D01" => D01,
        /// Depot Price
        "D02" => D02,
        /// Distribution and Pricing Agreement (DAPA) Price
        "D03" => D03,
        /// Dealer Adjusted Price
        "DAP" => Dap,
        /// Distributor's Price
        "DIS" => Dis,
        /// Discount Price
        "DPR" => Dpr,
        /// Discount Amount Allowed
        "DSC" => Dsc,
        /// Direct Store Delivery
        "DSD" => Dsd,
        /// Direct Ship Program Price
        "DSP" => Dsp,
        /// Emergency Direct Ship Price (Original Equipment Manufacturer)
        "EDM" => Edm,
        /// Emergency Direct Ship Price
        "EDP" => Edp,
        /// Emergency Direct Ship Price (Supplier)
        "EDS" => Eds,
        /// Emergency Direct Ship Price (Warehouse)
        "EDW" => Edw,
        /// Estimated Landed Cost
        "ELC" => Elc,
        /// Estimated Price
        "EST" => Est,
        /// Expected Unit Price
        "EUP" => Eup,
        /// Flat Charge
        "FCH" => Fch,
        /// First Cost Price
        "FCP" => Fcp,
        /// Frequent Delivery Service
        "FDS" => Fds,
        /// Federal Excise Tax
        "FET" => Fet,
        /// Free Goods Price
        "FGP" => Fgp,
        /// Formula Price
        "FOR" => For,
        /// Free Service Price
        "FSP" => Fsp,
        /// Federal Upper Limit Price (Maximum Allowable Cost Pricing for Drugs)
        "FUL" => Ful,
        /// Firm Price - Do Not Advise
        "FUP" => Fup,
        /// Advertising Price
        "GAP" => Gap,
        /// Display Price
        "GDP" => Gdp,
        /// Government Price
        "GOV" => Gov,
        /// Shelf Price
        "GSP" => Gsp,
        /// Temporary Price Reduction Price
        "GTP" => Gtp,
        /// Unit Price Through Quantity
        "ICL" => Icl,
        /// Industrial Price
        "IND" => Ind,
        /// Institutional Price
        "INS" => Ins,
        /// Invoice Billing Price
        "INV" => Inv,
        /// Labor Rate
        "LAR" => Lar,
        /// Last Cost Price
        "LCP" => Lcp,
        /// Lease to Purchase Price
        "LPP" => Lpp,
        /// List Price
        "LPR" => Lpr,
        /// Mandatory to Advise Unit Price
        "MAP" => Map,
        /// Minimum Activity Surcharge
        "MAS" => Mas,
        /// Maximum Order Quantity Price
        "MAX" => Max,
        /// Minimum Order Quantity Price
        "MIN" => Min,
        /// Minimum Charge
        "MNC" => Mnc,
        /// Minimum Release Quantity Price
        "MNR" => Mnr,
        /// Modal Premium
        "MOD" => Mod,
        /// Maximum Price Reduction
        "MPR" => Mpr,
        /// Manufacturer's Suggested Retail
        "MSR" => Msr,
        /// Maximum Release Quantity Price
        "MXR" => Mxr,
        /// Noncontract Tier 1
        "N01" => N01,
        /// Noncontract Tier 2
        "N02" => N02,
        /// Noncontract Tier 3
        "N03" => N03,
        /// Noncontract Tier 4
        "N04" => N04,
        /// Noncontract Tier 5
        "N05" => N05,
        /// Noncontract Tier 6
        "N06" => N06,
        /// Noncontract Tier 7
        "N07" => N07,
        /// Noncontract Tier 8
        "N08" => N08,
        /// Noncontract Tier 9
        "N09" => N09,
        /// Noncontract Tier 10
        "N10" => N10,
        /// Noncontract Tier 11
        "N11" => N11,
        /// Noncontract Tier 12
        "N12" => N12,
        /// Noncontract Tier 13
        "N13" => N13,
        /// Noncontract Tier 14
        "N14" => N14,
        /// Noncontract Tier 15
        "N15" => N15,
        /// Noncontract Tier 16
        "N16" => N16,
        /// Noncontract Tier 17
        "N17" => N17,
        /// Noncontract Tier 18
        "N18" => N18,
        /// Noncontract Tier 19
        "N19" => N19,
        /// Noncontract Tier 20
        "N20" => N20,
        /// Noncontract Tier 21
        "N21" => N21,
        /// Noncontract Tier 22
        "N22" => N22,
        /// Noncontract Tier 23
        "N23" => N23,
        /// Noncontract Tier 24
        "N24" => N24,
        /// Noncontract Tier 25
        "N25" => N25,
        /// Noncontract Tier 26
        "N26" => N26,
        /// Noncontract Tier 27
        "N27" => N27,
        /// Noncontract Tier 28
        "N28" => N28,
        /// Noncontract Tier 29
        "N29" => N29,
        /// Noncontract Tier 30
        "N30" => N30,
        /// No Charge
        "N31" => N31,
        /// Net Item Price
        "NET" => Net,
        /// Optional to Advise Unit Price
        "OAP" => Oap,
        /// Original Purchase Order Price
        "OPP" => Opp,
        /// Protection Level Price
        "PAP" => Pap,
        /// Price Break Quantity(s)
        "PAQ" => Paq,
        /// Unit Price Beginning Quantity
        "PBQ" => Pbq,
        /// Price Break Purchase Order Count
        "PBR" => Pbr,
        /// Public Health Service Price
        "PHS" => Phs,
        /// Price in Effect at Time of Shipment
        "PIE" => Pie,
        /// Producing Plant Price
        "PLT" => Plt,
        /// Packing Level Price
        "PPA" => Ppa,
        /// Prepaid Freight Charges
        "PPD" => Ppd,
        /// Professional Price
        "PRF" => Prf,
        /// Producer's Price
        "PRO" => Pro,
        /// Promotional price
        "PRP" => Prp,
        /// Purchase
        "PUR" => Pur,
        /// Quote Price
        "QTE" => Qte,
        /// Regular Charge
        "REG" => Reg,
        /// Resale
        "RES" => Res,
        /// Rental Price, Annual
        "RPA" => Rpa,
        /// Rental Price, Monthly
        "RPM" => Rpm,
        /// Replacement Price
        "RPP" => Rpp,
        /// Rush Charge
        "RSH" => Rsh,
        /// Retail
        "RTL" => Rtl,
        /// Service Attempted Charge
        "SAC" => Sac,
        /// Suggested Dealer Net Price
        "SDP" => Sdp,
        /// Suggested Fleet Price
        "SFP" => Sfp,
        /// Ship and Debit
        "SHD" => Shd,
        /// Suggested List Price
        "SLP" => Slp,
        /// Special Price
        "SPC" => Spc,
        /// Single Price (Factors Equalized)
        "SPE" => Spe,
        /// Secondary Supply Plant
        "SSP" => Ssp,
        /// Standard Price
        "STA" => Sta,
        /// Sum of Line Items
        "SUM" => Sum,
        /// Suggested Wholesale Price
        "SWP" => Swp,
        /// Threshold Price
        "THP" => Thp,
        /// Total Invoice Amount Due
        "TOT" => Tot,
        /// Transfer
        "TRF" => Trf,
        /// Unit cost price
        "UCP" => Ucp,
        /// Unsalable Item List Cost
        "ULC" => Ulc,
        /// Public Warehouse Price
        "WAR" => War,
        /// Wholesale
        "WHL" => Whl,
        /// Waived Service Price
        "WSP" => Wsp,
        /// Zone Price
        "ZNP" => Znp,
    }
);

crate::code_enum!(
    /// **240** Car Service Order Code
    ///
    /// - Data element: 240
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 5
    ///
    /// Car Service Order Code. Code values verified against the Stedi X12 reference.
    E240 {
        /// Empty car under directive CSD145
        "145" => N145,
        /// Loaded car under directive CSD145 that is to return to agent at city specified
        "145A" => N145A,
        /// Loaded car under directive CSD145 that is to return to shipper at billed origin
        "145S" => N145S,
        /// Assigned TTX car that is to be returned to shipper at billed origin by reverse of loaded move
        "145X" => N145X,
        /// Unassigned special equipped car that is to be handled in accordance with the provisions of directive CSD150
        "150" => N150,
        /// Unassigned TTX car that is to be handled in accordance with trailer train directive 150
        "150X" => N150X,
        /// Empty car under CSD155
        "155" => N155,
        /// Loaded car under directive CSD155 that is to return to agent at city specified
        "155A" => N155A,
        /// Loaded car under directive CSD155 that is to return to shipper at billed origin
        "155S" => N155S,
        /// Empty car under directive CSD435
        "435" => N435,
        /// Loaded car under directive CSD435 that is to return to agent at city specified
        "435A" => N435A,
        /// Loaded car under directive CSD435 that is to return to shipper at billed origin
        "435S" => N435S,
        /// Special heavy duty flat car that is to be handled under the provisions of directive CSD439
        "439" => N439,
        /// Car being returned per car owner's instructions
        "34617" => N34617,
        /// Car being returned per the pool operator's instructions
        "34618" => N34618,
        /// Car being returned per AAR/ICC instructions
        "34619" => N34619,
        /// Loaded TTX cars under directive CMD1 that are to be returned to the origin road at the city specified
        "CMD1A" => Cmd1a,
        /// Loaded TTX cars under directive CMD1 that are to be returned to shipper at billed origin
        "CMD1S" => Cmd1s,
        /// Empty Car Moving Under Car Service Rule 5 Charges
        "CSR5" => Csr5,
    }
);

crate::code_enum!(
    /// **241** Protective Service Code
    ///
    /// - Data element: 241
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 4
    ///
    /// Protective Service Code. Code values verified against the Stedi X12 reference.
    E241 {
        /// Body Ice
        "B" => B,
        /// Body Ice Consumed or Removed
        "BC" => Bc,
        /// Discontinue Service
        "D" => D,
        /// Do Not Heat
        "HDN" => Hdn,
        /// Do Not Heat in Canada
        "HDNC" => Hdnc,
        /// Standard Heating In Canada
        "HSC" => Hsc,
        /// Standard Mechanical Protective Service
        "M" => M,
        /// Modified Mechanical Protective Service
        "MN" => Mn,
        /// Do Not Operate
        "MNU" => Mnu,
    }
);

crate::code_enum!(
    /// **242** Vent Instruction Code
    ///
    /// - Data element: 242
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 7
    ///
    /// Vent Instruction Code. Code values verified against the Stedi X12 reference.
    E242 {
        /// Standard Ventilation at "--" Degrees (Other than 32 Degrees)
        "V40" => V40,
        /// Vents Closed to Destination
        "VC" => Vc,
        /// Diagonal Ventilation at "--" Degrees (Other than 32 Degrees) - Open Vent Each End of Car
        "VD40" => Vd40,
        /// Diagonal Vents on Irons
        "VDOI" => Vdoi,
        /// Vents Open to Destination
        "VO" => Vo,
        /// Vents on Irons
        "VOI" => Voi,
        /// Standard Ventilation at 32 Degrees
        "VS" => Vs,
        /// Standard Ventilation - Substitute Carrier's Protective Service at First Terminal Train Yard where Heaters Are Available and Outside Temperature is 10 Degrees Above Zero or Lower (PPT #619 Rules 385 and 515)
        "VS10" => Vs10,
    }
);

crate::code_enum!(
    /// **246** Certification/Clause Code
    ///
    /// - Data element: 246
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 4
    ///
    /// Certification/Clause Code. Code values verified against the Stedi X12 reference.
    E246 {
        /// Shipper's Load and Count
        "01" => N01,
        /// Shipper's Load, Stowage and Count
        "02" => N02,
        /// Laden on Board
        "03" => N03,
        /// Laden on Board Vessel
        "04" => N04,
        /// Vessel Not Responsible for Freezing
        "05" => N05,
        /// Container(s) Sealed by Shipper
        "06" => N06,
        /// On Deck at Shipper's Risk
        "07" => N07,
        /// Short-Shipped
        "08" => N08,
        /// Sea Waybill
        "09" => N09,
        /// This Shipment is Effected under a Sea Waybill
        "10" => N10,
        /// Memo Bill of Lading Only
        "11" => N11,
        /// Refrigerated Cargo
        "12" => N12,
        /// Cool Cargo
        "13" => N13,
        /// Freeze Cargo
        "14" => N14,
        /// Inland Transportation Arranged as Agents Only with such Arranged Transportation Being Solely for Account and Risk of Cargo
        "15" => N15,
        /// Sea-Air Cargo
        "16" => N16,
        /// Freight Prepaid
        "17" => N17,
        /// Freight Collect
        "18" => N18,
        /// Freight as Agreed
        "19" => N19,
        /// No Shipper's Export Declaration Required (Section 30.39)
        "20" => N20,
        /// Carrier Reserves the Right to Place Container(s) in Heated Warehouse at a Set Cost
        "21" => N21,
        /// On Board Rail
        "22" => N22,
        /// On Board Truck
        "23" => N23,
        /// On Board Vessel
        "24" => N24,
        /// Received For Shipment
        "25" => N25,
        /// On Board Container
        "26" => N26,
        /// On Board Airplane
        "27" => N27,
        /// On Board Boxcar
        "28" => N28,
        /// Emergency Response Statement
        "29" => N29,
        /// International Maritime Organization Certification
        "30" => N30,
        /// Statement of Correctness
        "31" => N31,
        /// Destination Control Statements
        "32" => N32,
        /// Producing Country of Origin
        "33" => N33,
        /// Laden on Board Named Vessel
        "34" => N34,
        /// Age of Vessel
        "35" => N35,
        /// Kosher
        "36" => N36,
        /// Route
        "37" => N37,
        /// Certification Statements
        "38" => N38,
        /// Destination Country
        "39" => N39,
        /// Title Passage Clause
        "40" => N40,
        /// Container Safety Act
        "41" => N41,
        /// Substantial Transformation
        "42" => N42,
        /// Canada Value Added
        "43" => N43,
        /// Mexican Value Added
        "44" => N44,
        /// General Agreement on Tariff and Trade (GATT)
        "45" => N45,
        /// Prior Damage Remarks
        "46" => N46,
        /// Administrative
        "AM" => Am,
        /// Caribbean Basin Initiative (CBI)
        "CB" => Cb,
        /// Custom
        "CC" => Cc,
        /// Container Packing Certificate
        "CP" => Cp,
        /// Disclaimer
        "DC" => Dc,
        /// Delivery Order Liability Clause
        "DV" => Dv,
        /// Estimate Error
        "EE" => Ee,
        /// Estimate Remarks
        "ER" => Er,
        /// General System of Preferences (GSP)
        "GS" => Gs,
        /// Heading
        "HD" => Hd,
        /// Hidden
        "HN" => Hn,
        /// Israeli Free Trade Agreement
        "IS" => Is,
        /// North American Free Trade Agreement (NAFTA)
        "NF" => Nf,
    }
);

crate::code_enum!(
    /// **256** Manifest Type Code
    ///
    /// - Data element: 256
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Manifest Type Code. Code values verified against the Stedi X12 reference.
    E256 {
        /// Original Manifest From Carrier To Port of Discharge
        "A" => A,
        /// Updating Manifest Prior To Vessel Arrival From Carrier To Port of Discharge
        "B" => B,
        /// Amendment To Manifest From Carrier To Port of Discharge
        "C" => C,
        /// Updating Export Manifest Prior to Vessel Departure From Carrier to U.S. Customs
        "D" => D,
        /// Original Export Manifest from Carrier to U.S. Customs
        "E" => E,
        /// Amendment to Export Manifest From Carrier to U.S. Customs
        "F" => F,
        /// General Order Items from Carriers to US Customs
        "G" => G,
        /// Arrival Notification from Carrier to U.S. Customs
        "H" => H,
        /// Supplementary In-bond Information from Carrier to U.S. Customs
        "I" => I,
        /// Sent from U.S. Customs (Export) to Carriers
        "J" => J,
        /// Departure Notification from Carrier to U.S. Customs
        "K" => K,
        /// Transfer of Liability
        "L" => L,
        /// Driver Information Update
        "M" => M,
        /// Preliminary Manifest from Carrier to U.S. Customs
        "P" => P,
        /// Consist Manifest from Carrier to U.S. Customs
        "S" => S,
        /// Transit
        "T" => T,
        /// Amendment to Manifest Required by US Customs
        "V" => V,
        /// Original Manifest from Carriers to US Customs
        "W" => W,
        /// Updating Manifest Prior to Vessel Arrival from Carriers to US Customs
        "X" => X,
        /// Amendment to Manifest from Carrier to US Customs
        "Y" => Y,
        /// Sent from US Customs to Carriers
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **257** Tariff Application Code
    ///
    /// - Data element: 257
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Tariff Application Code. Code values verified against the Stedi X12 reference.
    E257 {
        /// Intrastate - Intraplant
        "A" => A,
        /// Interstate - Intraplant
        "B" => B,
        /// Commingled
        "C" => C,
        /// Reciprocal
        "D" => D,
        /// Intraterminal
        "E" => E,
        /// Interterminal
        "F" => F,
        /// International
        "I" => I,
        /// Interstate
        "N" => N,
        /// Regional
        "R" => R,
        /// Intrastate
        "S" => S,
    }
);

crate::code_enum!(
    /// **259** Change Type Code
    ///
    /// - Data element: 259
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Change Type Code. Code values verified against the Stedi X12 reference.
    E259 {
        /// Add Rate
        "A" => A,
        /// Original Price Change Transaction
        "B" => B,
        /// Cancel Previous Price Change Transaction
        "C" => C,
        /// Decrease Rate
        "D" => D,
        /// Eliminate/Expire
        "E" => E,
        /// Confirmation Of A Discussion Between Buyer And Sales Representative
        "F" => F,
        /// Add Items To A Previous Change Transaction
        "G" => G,
        /// Delete Items From A Previous Change Transaction
        "H" => H,
        /// Increase Rate
        "I" => I,
        /// Replace All Dates Shown In A Previous Change
        "J" => J,
        /// Replace Prices Shown In A Previous Change Transaction
        "K" => K,
        /// Replace Restrictions (Conditions) At An Item Level Shown In A Previous Change
        "L" => L,
        /// Replace Marketing Area Shown In A Previous Announcement
        "M" => M,
        /// Change Having No Effect on Rates
        "N" => N,
        /// Replace Restrictions (Conditions) At A Transaction Level Shown In A Previous Change
        "O" => O,
        /// Replace Price Area Shown In A Previous Change
        "P" => P,
        /// Replace Previous Change Transaction Entirely Because of Multiple Kinds Of Changes Or To Make Changes Not Specifically Indicated By Another Code In This Data Element
        "Q" => Q,
        /// Replace Allowance Rates Shown In A Previous Announcement
        "R" => R,
        /// Modified
        "T" => T,
        /// New
        "W" => W,
        /// No Modification
        "X" => X,
    }
);

crate::code_enum!(
    /// **262** Geography Qualifier Code
    ///
    /// - Data element: 262
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Geography Qualifier Code. Code values verified against the Stedi X12 reference.
    E262 {
        /// And
        "A" => A,
        /// Between
        "B" => B,
        /// Destination
        "D" => D,
        /// From
        "F" => F,
        /// Intermediate Location
        "I" => I,
        /// Geographic Listing
        "L" => L,
        /// Origin
        "O" => O,
        /// Prior Origin
        "P" => P,
        /// To
        "T" => T,
        /// Ultimate Destination
        "U" => U,
    }
);

crate::code_enum!(
    /// **271** Subsidiary Risk Indicator
    ///
    /// - Data element: 271
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Subsidiary Risk Indicator. Code values verified against the Stedi X12 reference.
    E271 {
        /// Potentially Explosive
        "E" => E,
        /// Potentially Very Damaging To The Eyes
        "I" => I,
    }
);

crate::code_enum!(
    /// **272** Hazardous Certification Code
    ///
    /// - Data element: 272
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Hazardous Certification Code. Code values verified against the Stedi X12 reference.
    E272 {
        /// Abbreviated Hazardous Certification
        "1" => N1,
        /// Long Form of Hazardous Certification
        "2" => N2,
    }
);

crate::code_enum!(
    /// **283** Terms Due Date Qualifier
    ///
    /// - Data element: 283
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Terms Due Date Qualifier. Code values verified against the Stedi X12 reference.
    E283 {
        /// Mailed by Date (Postmark Date)
        "01" => N01,
        /// Received by date
        "02" => N02,
        /// Electronics Funds Transfer Settlement Date
        "03" => N03,
        /// Funds Deposited by Date
        "04" => N04,
    }
);

crate::code_enum!(
    /// **284** Service Level Code
    ///
    /// - Data element: 284
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Service Level Code. Code values verified against the Stedi X12 reference.
    E284 {
        /// Bulk Commodity Train
        "01" => N01,
        /// Three Day Service
        "3D" => N3D,
        /// Premium Surface
        "09" => N09,
        /// 9 A.M.
        "9A" => N9A,
        /// Air Cargo
        "AC" => Ac,
        /// Air Economy
        "AE" => Ae,
        /// A.M.
        "AM" => Am,
        /// Business Class
        "BC" => Bc,
        /// Consignee Billing Service
        "CB" => Cb,
        /// Courier Express
        "CE" => Ce,
        /// Ground
        "CG" => Cg,
        /// Express Service
        "CX" => Cx,
        /// Delivery Scheduled Next Day by Cartage Agent
        "D1" => D1,
        /// Delivery scheduled second day by cartage agent
        "D2" => D2,
        /// Delivery scheduled third day by cartage agent
        "D3" => D3,
        /// Delivery Confirmation
        "DC" => Dc,
        /// Deferred Service
        "DF" => Df,
        /// Delivery Confirmation Return
        "DR" => Dr,
        /// Door Service
        "DS" => Ds,
        /// Delivery Notification Only
        "DT" => Dt,
        /// Expedited Service
        "ES" => Es,
        /// Proof of Delivery (POD) with Signature
        "ET" => Et,
        /// First Class
        "FC" => Fc,
        /// Standard Service
        "G2" => G2,
        /// Express Service Plus
        "GP" => Gp,
        /// Tracking - Ground
        "GT" => Gt,
        /// International Second Day
        "I2" => I2,
        /// IATA
        "IA" => Ia,
        /// Expedited Service - Worldwide
        "IE" => Ie,
        /// International Service
        "IS" => Is,
        /// Express Service - Worldwide
        "IX" => Ix,
        /// Metro
        "ME" => Me,
        /// Multiweight
        "MW" => Mw,
        /// Next Day Air
        "ND" => Nd,
        /// Next Flight Out
        "NF" => Nf,
        /// Next Day Hundred Weight
        "NH" => Nh,
        /// Next Morning
        "NM" => Nm,
        /// Not Served
        "NS" => Ns,
        /// Overnight
        "ON" => On,
        /// Priority Service
        "P1" => P1,
        /// Primary Service Area
        "P2" => P2,
        /// Primary Service Area - Next Day by 10:30 A.M.
        "PA" => Pa,
        /// Priority Mail
        "PB" => Pb,
        /// Primary Service Area - Next Day By 9:30 AM
        "PC" => Pc,
        /// Priority Mail Insured
        "PI" => Pi,
        /// PM
        "PM" => Pm,
        /// Primary Service Area - Next Day by Noon
        "PN" => Pn,
        /// P.O. Box/Zip Code
        "PO" => Po,
        /// Primary Service Area - Next Day by 5:00 P.M.
        "PR" => Pr,
        /// Primary Service Area - Second Day by Noon
        "PS" => Ps,
        /// Premium Service
        "PX" => Px,
        /// Passenger Service
        "R1" => R1,
        /// Quality Intermodal High Speed 70 Miles Per Hour (MPH)
        "R2" => R2,
        /// Other Intermodal and Stack Service
        "R3" => R3,
        /// 60 Miles Per Hour (MPH) Service
        "R4" => R4,
        /// Manifest Freight
        "R5" => R5,
        /// Circus Train
        "R6" => R6,
        /// Work Train
        "R7" => R7,
        /// Commuter Service
        "R8" => R8,
        /// Authorized Return Service
        "RS" => Rs,
        /// Same Day
        "SA" => Sa,
        /// Second Day Air
        "SC" => Sc,
        /// Saturday
        "SD" => Sd,
        /// Second Day
        "SE" => Se,
        /// Standard Ground
        "SG" => Sg,
        /// Second Day Hundred Weight
        "SH" => Sh,
        /// Standard Ground Hundred Weight
        "SI" => Si,
        /// Second Morning
        "SM" => Sm,
        /// Saturday Pickup
        "SP" => Sp,
        /// Standard Class
        "ST" => St,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **288** Pre-priced Option Code
    ///
    /// - Data element: 288
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Pre-priced Option Code. Code values verified against the Stedi X12 reference.
    E288 {
        /// Pre-Priced Prices included and price qualifier applies
        "A" => A,
        /// Bonus Pack
        "B" => B,
        /// Cents-off Pack
        "C" => C,
        /// Feature Price
        "F" => F,
        /// Not Pre-Priced
        "N" => N,
        /// Pre-Priced (Prices Included)
        "Y" => Y,
        /// Pre-Priced (Prices Not Included)
        "Z" => Z,
    }
);

crate::code_enum!(
    /// **290** Price Condition Code
    ///
    /// - Data element: 290
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Price Condition Code. Code values verified against the Stedi X12 reference.
    E290 {
        /// Price Protection Applies
        "01" => N01,
        /// Allocation Applies
        "02" => N02,
        /// Advance Ship Notice/Pallet Tags
        "03" => N03,
        /// Bar Code Case Label
        "04" => N04,
        /// Bottom Load Only
        "05" => N05,
        /// Color Coding
        "06" => N06,
        /// Synchronization of Prices
        "07" => N07,
        /// Dedicated Carrier
        "08" => N08,
        /// Drop Trailer
        "09" => N09,
        /// Financial Electronic Data Interchange (FEDI)
        "10" => N10,
        /// Fax a Manifest
        "11" => N11,
        /// Hand Stack - Special Loading
        "12" => N12,
        /// Modular Pallets
        "13" => N13,
        /// No Diverting
        "14" => N14,
        /// Order Lead Time Requirement
        "15" => N15,
        /// Order Quality (No-Touch EDI)
        "16" => N16,
        /// Pallet Placards
        "17" => N17,
        /// Pick and Pack
        "18" => N18,
        /// Pre-set Appointments
        "19" => N19,
        /// Rapid Deployment (Same Day Ship)
        "20" => N20,
        /// Slipsheet
        "21" => N21,
        /// Strap Packages
        "22" => N22,
        /// Third Party Pallet
        "23" => N23,
        /// Unload Time Required
        "24" => N24,
    }
);

crate::code_enum!(
    /// **291** Price Condition Applies Code
    ///
    /// - Data element: 291
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Price Condition Applies Code. Code values verified against the Stedi X12 reference.
    E291 {
        /// Yes at Transaction Set Level
        "001" => N001,
        /// Yes at Line Item Level
        "002" => N002,
        /// No Conditions Apply
        "003" => N003,
    }
);

crate::code_enum!(
    /// **292** Quantity Basis
    ///
    /// - Data element: 292
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Quantity Basis. Code values verified against the Stedi X12 reference.
    E292 {
        /// Quantity per Customer Allowed
        "001" => N001,
        /// Quantity per Market Allowed
        "002" => N002,
    }
);

crate::code_enum!(
    /// **293** Promotion Condition Qualifier
    ///
    /// - Data element: 293
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Promotion Condition Qualifier. Code values verified against the Stedi X12 reference.
    E293 {
        /// "AND" Relationship Between Current and Immediately Prior Occurrences of the Same Segment
        "01" => N01,
        /// "OR" Relationship Between Current and All Preceding Occurrences of the Same Segment
        "02" => N02,
        /// Refer to Free-form Description
        "99" => N99,
    }
);

crate::code_enum!(
    /// **295** Distance Qualifier
    ///
    /// - Data element: 295
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Distance Qualifier. Code values verified against the Stedi X12 reference.
    E295 {
        /// Air Miles
        "A" => A,
        /// Carrier's Rate Basis Number
        "B" => B,
        /// Carrier's Docket 28300 Miles
        "D" => D,
        /// Air Kilometers
        "F" => F,
        /// Kilometers (Actual)
        "K" => K,
        /// Tariff Kilometers
        "L" => L,
        /// Miles (Actual)
        "M" => M,
        /// Tariff Miles
        "T" => T,
        /// Maximum Miles
        "X" => X,
    }
);

crate::num_element!(
    /// **294** Tariff Distance
    ///
    /// - Data element: 294
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 5
    ///
    /// Tariff Distance.
    E294
);
