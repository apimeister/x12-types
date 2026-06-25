//! X12 data elements 1300-1399.

crate::code_enum!(
    /// **1373** Measurement Method or Device
    ///
    /// - Data element: 1373
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 4
    ///
    /// Measurement Method or Device. Code values verified against the Stedi X12 reference.
    E1373 {
        /// Bellows Meter
        "BM" => Bm,
        /// Based on 120 Days List to Contract
        "BO" => Bo,
        /// Displacement Meter
        "DM" => Dm,
        /// Flange Tap
        "FT" => Ft,
        /// Hubometer
        "HN" => Hn,
        /// Measured Over Six Months
        "MA" => Ma,
        /// Mercury Meter
        "MM" => Mm,
        /// Orifice Meter
        "OM" => Om,
        /// Odometer
        "OU" => Ou,
        /// Pipe Tap
        "PT" => Pt,
        /// Turbine Meter
        "TM" => Tm,
        /// Valued at 90 Day Marketing Time Method
        "VA" => Va,
        /// Valued at 120 Day Marketing Time Method
        "VB" => Vb,
        /// Valued at 180 Day Marketing Time Method
        "VC" => Vc,
    }
);
