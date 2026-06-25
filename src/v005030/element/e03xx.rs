//! X12 data elements 0300-0399.

crate::time_element!(
    /// **337** Time
    ///
    /// - Data element: 337
    /// - Type: Time (TM)
    /// - Length: min 4, max 8
    ///
    /// Time.
    E337
);

crate::date_element!(
    /// **373** Date
    ///
    /// - Data element: 373
    /// - Type: Date (DT)
    /// - Length: min 8, max 8
    ///
    /// Date.
    E373
);
