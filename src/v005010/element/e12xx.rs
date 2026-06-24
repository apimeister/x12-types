//! X12 data elements 1200-1299.

crate::code_enum!(
    /// **1250** Date Time Period Format Qualifier
    ///
    /// - Data element: 1250
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code indicating the date format, time format, or date and time format.
    E1250 {
        /// Date Expressed in Format CCYYMMDD
        "D8" => D8,
        /// Date Expressed in Format YYMMDD
        "D6" => D6,
        /// Date and Time Expressed in Format CCYYMMDDHHMM
        "DT" => Dt,
        /// Range of Dates Expressed in Format CCYYMMDD-CCYYMMDD
        "RD8" => Rd8,
        /// Time Expressed in Format HHMM
        "TM" => Tm,
    }
);

crate::code_enum!(
    /// **1270** Code List Qualifier Code
    ///
    /// - Data element: 1270
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Code identifying a specific industry code list. Common codes named below.
    E1270 {
        /// Document Identification Code
        "0" => N0,
        /// Free On Board Site Code
        "1" => N1,
        /// Kind of Contract Code
        "3" => N3,
        /// Type of Contract Code
        "4" => N4,
        /// Transaction Status Indicator Code
        "10" => N10,
        /// Payment Type Code
        "20" => N20,
        /// Transportation Mode or Method Code
        "39" => N39,
        /// Health Care Provider Taxonomy
        "68" => N68,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);
