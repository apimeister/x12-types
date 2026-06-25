//! X12 data elements 1700-1799.

crate::code_enum!(
    /// **1703** Commodity Characteristic Codes
    ///
    /// - Data element: 1703
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Commodity Characteristic Codes. Code values verified against the Stedi X12 reference.
    E1703 {
        /// Foodstuff
        "FS" => Fs,
        /// Freezable
        "FZ" => Fz,
        /// Hazardous Material
        "HZ" => Hz,
        /// Poisonous
        "PN" => Pn,
        /// Refrigerated
        "RF" => Rf,
    }
);
