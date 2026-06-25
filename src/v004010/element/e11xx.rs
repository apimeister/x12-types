//! X12 data elements 1100-1199.

crate::code_enum!(
    /// **1109** Race or Ethnicity Code
    ///
    /// - Data element: 1109
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Race or Ethnicity Code. Code values verified against the Stedi X12 reference.
    E1109 {
        /// Not Provided
        "7" => N7,
        /// Not Applicable
        "8" => N8,
        /// Asian or Pacific Islander
        "A" => A,
        /// Black
        "B" => B,
        /// Caucasian
        "C" => C,
        /// Subcontinent Asian American
        "D" => D,
        /// Other Race or Ethnicity
        "E" => E,
        /// Asian Pacific American
        "F" => F,
        /// Native American
        "G" => G,
        /// Hispanic
        "H" => H,
        /// American Indian or Alaskan Native
        "I" => I,
        /// Native Hawaiian
        "J" => J,
        /// Black (Non-Hispanic)
        "N" => N,
        /// White (Non-Hispanic)
        "O" => O,
        /// Pacific Islander
        "P" => P,
        /// Black or African American (Office of Management and Budget 1997)
        "Q" => Q,
        /// Hispanic or Latino (Office of Management and Budget 1997)
        "R" => R,
        /// White (Office of Management and Budget 1997)
        "S" => S,
        /// American Indian or Alaska Native (Office of Management and Budget 1997)
        "T" => T,
        /// Asian (Office of Management and Budget 1997)
        "U" => U,
        /// Native Hawaiian or Other Pacific Islander (Office of Management and Budget 1997)
        "V" => V,
        /// Not Hispanic or Latino (Office of Management and Budget 1997)
        "W" => W,
        /// Mutually Defined
        "Z" => Z,
    }
);
