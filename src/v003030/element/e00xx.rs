//! X12 data elements 0000-0099.

crate::num_element!(
    /// **28** Group Control Number
    ///
    /// - Data element: 28
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 9
    ///
    /// Group Control Number.
    E28
);

crate::num_element!(
    /// **96** Number of Included Segments
    ///
    /// - Data element: 96
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 10
    ///
    /// Number of Included Segments.
    E96
);

crate::num_element!(
    /// **97** Number of Transaction Sets Included
    ///
    /// - Data element: 97
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Number of Transaction Sets Included.
    E97
);
