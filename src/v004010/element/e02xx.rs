//! X12 data elements 0200-0299.

crate::num_element!(
    /// **203** Cubic Capacity
    ///
    /// - Data element: 203
    /// - Type: Numeric (N0)
    /// - Length: min 2, max 4
    ///
    /// Cubic Capacity.
    E203
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
    /// **233** Weight Capacity
    ///
    /// - Data element: 233
    /// - Type: Numeric (N0)
    /// - Length: min 2, max 3
    ///
    /// Weight Capacity.
    E233
);

crate::date_element!(
    /// **243** Transaction Reference Date
    ///
    /// - Data element: 243
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Transaction Reference Date.
    E243
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

crate::num_element!(
    /// **280** Exchange Rate
    ///
    /// - Data element: 280
    /// - Type: Numeric (R)
    /// - Length: min 1, max 15
    ///
    /// Exchange Rate.
    E280
);

crate::num_element!(
    /// **289** Unit Price
    ///
    /// - Data element: 289
    /// - Type: Numeric (R)
    /// - Length: min 1, max 17
    ///
    /// Unit Price.
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
    /// **216** Metric Qualifier
    ///
    /// - Data element: 216
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Metric Qualifier. Code values verified against the Stedi X12 reference.
    E216 {
        /// Metric Units
        "M" => M,
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
