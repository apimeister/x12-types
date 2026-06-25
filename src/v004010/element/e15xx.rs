//! X12 data elements 1500-1599.

crate::code_enum!(
    /// **1525** Request Category Code
    ///
    /// - Data element: 1525
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Request Category Code. Code values verified against the Stedi X12 reference.
    E1525 {
        /// Admission Review
        "AR" => Ar,
        /// Batch
        "BA" => Ba,
        /// Health Services Review
        "HS" => Hs,
        /// Individual
        "IN" => In,
        /// Program Referral
        "PR" => Pr,
        /// Recurring
        "RE" => Re,
        /// Specialty Care Review
        "SC" => Sc,
    }
);

crate::code_enum!(
    /// **1576** Inspected/Weighed Indicator Code
    ///
    /// - Data element: 1576
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Inspected/Weighed Indicator Code. Code values verified against the Stedi X12 reference.
    E1576 {
        /// Both Inspected and Weighed
        "IB" => Ib,
        /// Inspected
        "II" => Ii,
        /// Weighed
        "IW" => Iw,
    }
);

crate::code_enum!(
    /// **1578** Export Exception Code
    ///
    /// - Data element: 1578
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Export Exception Code. Code values verified against the Stedi X12 reference.
    E1578 {
        /// Shipment Information Filed Through the Automated Export System (AES)
        "AE" => Ae,
        /// Ultimate Destination Canada
        "CN" => Cn,
        /// Diplomatic Pouches
        "DP" => Dp,
        /// Shipment to U.S. Agencies
        "GS" => Gs,
        /// Household Shipments
        "HH" => Hh,
        /// Human Remains
        "HR" => Hr,
        /// U.S. Military Supplies
        "MS" => Ms,
        /// Personal Property
        "PP" => Pp,
        /// Shipments between U.S. Possessions except the Virgin Islands
        "UP" => Up,
        /// Unreported Low-value Shipments
        "UR" => Ur,
        /// Foreign Trade Zone, Duty Deferral Shipment
        "ZD" => Zd,
    }
);
