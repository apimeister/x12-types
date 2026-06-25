//! X12 data elements 1500-1599.

crate::code_enum!(
    /// **1543** Equipment Orientation Code
    ///
    /// - Data element: 1543
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Equipment Orientation Code. Code values verified against the Stedi X12 reference.
    E1543 {
        /// Opposite End of Car from Brake
        "A" => A,
        /// Brake End of Car
        "B" => B,
    }
);

crate::code_enum!(
    /// **1577** Hazardous Material Regulations Exception Code
    ///
    /// - Data element: 1577
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Hazardous Material Regulations Exception Code. Code values verified against the Stedi X12 reference.
    E1577 {
        /// No Exception
        "1" => N1,
        /// Excepted, Ground Transport Less than Limited Quantity
        "2" => N2,
        /// Excepted, Ground Transport Less than Limited Quantity and Consumer Commodity (Other Regulated Materials - Group D (ORM-D))
        "3" => N3,
    }
);
