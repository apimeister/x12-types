//! X12 data elements 0400-0499.

crate::code_enum!(
    /// **432** Date Qualifier
    ///
    /// - Data element: 432
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying type of date (used in G62 and other grocery date segments).
    E432 {
        /// Cancel After This Date
        "01" => N01,
        /// Invoice Date
        "03" => N03,
        /// Purchase Order Date
        "04" => N04,
        /// Requested Ship Date/Pickup Date
        "10" => N10,
        /// Shipped on This Date
        "11" => N11,
        /// Terms Net Due Date
        "13" => N13,
        /// Estimated Delivery Date
        "17" => N17,
        /// Expiration Date
        "36" => N36,
        /// Requested Delivery Date
        "68" => N68,
        /// Scheduled Delivery Date
        "70" => N70,
        /// Date Issued
        "85" => N85,
    }
);

crate::code_enum!(
    /// **478** Credit/Debit Flag Code
    ///
    /// - Data element: 478
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code indicating whether amount is a credit or debit.
    E478 {
        /// Credit
        "C" => C,
        /// Debit
        "D" => D,
    }
);
