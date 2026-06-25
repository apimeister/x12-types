//! X12 data elements 0800-0899.

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
    /// **845** Chassis Type
    ///
    /// - Data element: 845
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Chassis Type. Code values verified against the Stedi X12 reference.
    E845 {
        /// Eight Pin Combo
        "8P" => N8P,
        /// Drop Frame
        "DF" => Df,
        /// Flush Back
        "FB" => Fb,
        /// Gooseneck
        "GN" => Gn,
        /// Straight Frame
        "SF" => Sf,
        /// Slider
        "SL" => Sl,
        /// Tri-axle
        "TX" => Tx,
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
