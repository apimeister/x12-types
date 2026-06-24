//! X12 data elements 1300-1399.

crate::code_enum!(
    /// **1300** Service, Promotion, Allowance, or Charge Code
    ///
    /// - Data element: 1300
    /// - Type: Identifier (ID)
    /// - Length: min 4, max 4
    ///
    /// Code identifying the service, promotion, allowance, or charge. Common codes
    /// named below; any other round-trips as `Unknown`.
    E1300 {
        /// Base Charge
        "A520" => A520,
        /// COD Amount
        "B230" => B230,
        /// Delivery
        "C040" => C040,
        /// Freight
        "D240" => D240,
        /// Fuel Charge
        "D260" => D260,
        /// Handling
        "D500" => D500,
        /// Insurance Fee
        "D920" => D920,
        /// Loading (Labor Charges)
        "E400" => E400,
        /// Promotional Allowance
        "F800" => F800,
        /// Service Charge
        "G740" => G740,
        /// Storage
        "H430" => H430,
        /// Surcharge
        "H550" => H550,
        /// Unloading (Labor Charges)
        "I380" => I380,
    }
);
