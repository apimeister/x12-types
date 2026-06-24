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
