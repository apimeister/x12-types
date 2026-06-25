//! X12 data elements 0900-0999.

crate::num_element!(
    /// **953** Interest Rate
    ///
    /// - Data element: 953
    /// - Type: Numeric (R)
    /// - Length: min 1, max 6
    ///
    /// Interest Rate.
    E953
);

crate::num_element!(
    /// **954** Percent
    ///
    /// - Data element: 954
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Percent.
    E954
);

crate::code_enum!(
    /// **952** Adjustment Application Code
    ///
    /// - Data element: 952
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Adjustment Application Code. Code values verified against the Stedi X12 reference.
    E952 {
        /// Float
        "F" => F,
        /// Ledger
        "L" => L,
        /// Service Charge
        "S" => S,
    }
);

crate::code_enum!(
    /// **964** Cost Code
    ///
    /// - Data element: 964
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Cost Code. Code values verified against the Stedi X12 reference.
    E964 {
        /// Final Net Acquisition Cost
        "FNC" => Fnc,
        /// Prototype Handling Cost
        "HAN" => Han,
        /// Labor Cost
        "LAB" => Lab,
        /// Material Cost
        "MTL" => Mtl,
        /// Sender's Cost
        "OCT" => Oct,
        /// Other Cost
        "OTH" => Oth,
        /// Overhead Cost
        "OVR" => Ovr,
        /// Packaging Cost
        "PKG" => Pkg,
        /// Raw Material per Part Cost
        "RMP" => Rmp,
        /// Raw Material per Unit of Measure Cost
        "RMU" => Rmu,
        /// Prototype Set-Up Cost
        "SET" => Set,
        /// Total Die Model Cost
        "TDI" => Tdi,
        /// Total Gage Cost
        "TGA" => Tga,
        /// Line Item Tooling Cost
        "TLN" => Tln,
        /// Total Material Cost
        "TML" => Tml,
        /// Total Material Including Purchased Components Cost
        "TMP" => Tmp,
        /// Total Other Tooling Cost
        "TOL" => Tol,
        /// Total Purchased Components Cost
        "TPU" => Tpu,
        /// Total Tooling Cost
        "TTL" => Ttl,
    }
);

crate::code_enum!(
    /// **983** Hazardous Class Qualifier
    ///
    /// - Data element: 983
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Hazardous Class Qualifier. Code values verified against the Stedi X12 reference.
    E983 {
        /// Primary
        "P" => P,
        /// Secondary
        "S" => S,
    }
);

crate::code_enum!(
    /// **984** Hazardous Material Shipping Name Qualifier
    ///
    /// - Data element: 984
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Hazardous Material Shipping Name Qualifier. Code values verified against the Stedi X12 reference.
    E984 {
        /// Canadian Shipping Name
        "C" => C,
        /// Domestic (United States) Shipping Name
        "D" => D,
        /// International Shipping Name
        "I" => I,
    }
);

crate::code_enum!(
    /// **985** N.O.S. Indicator Code
    ///
    /// - Data element: 985
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// N.O.S. Indicator Code. Code values verified against the Stedi X12 reference.
    E985 {
        /// N.O.S. Regulatory Requirements Apply
        "NOS" => Nos,
    }
);

crate::code_enum!(
    /// **986** Special Commodity Indicator Code
    ///
    /// - Data element: 986
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Special Commodity Indicator Code. Code values verified against the Stedi X12 reference.
    E986 {
        /// Positive Indicator
        "S" => S,
    }
);
