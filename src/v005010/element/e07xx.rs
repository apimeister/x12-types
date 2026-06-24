//! X12 data elements 0700-0799.

crate::code_enum!(
    /// **735** Hierarchical Level Code
    ///
    /// - Data element: 735
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 2
    ///
    /// Code defining the characteristic of a level in a hierarchical structure
    /// (HL-03). Common codes named below; any other round-trips as `Unknown`.
    E735 {
        /// Shipment
        "S" => S,
        /// Order
        "O" => O,
        /// Item
        "I" => I,
        /// Pack
        "P" => P,
        /// Transportation Equipment
        "E" => E,
        /// Component
        "F" => F,
        /// Bill of Materials
        "H" => H,
        /// Provider of Service
        "19" => N19,
        /// Information Source
        "20" => N20,
        /// Information Receiver
        "21" => N21,
        /// Subscriber
        "22" => N22,
        /// Dependent
        "23" => N23,
        /// Patient
        "PT" => Pt,
        /// Employer
        "EM" => Em,
    }
);

crate::code_enum!(
    /// **736** Hierarchical Child Code
    ///
    /// - Data element: 736
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code indicating if there are hierarchical child data segments subordinate to
    /// the level being described (HL-04).
    E736 {
        /// No Subordinate HL Segment in This Hierarchical Structure
        "0" => N0,
        /// Additional Subordinate HL Data Segment in This Hierarchical Structure
        "1" => N1,
    }
);

crate::code_enum!(
    /// **750** Product/Process Characteristic Code
    ///
    /// - Data element: 750
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 3
    ///
    /// Code identifying the general class of a product or process characteristic
    /// (PID-02). Common codes named below.
    E750 {
        /// Limiting Operation
        "01" => N01,
        /// General Product Form
        "02" => N02,
        /// Preservative
        "03" => N03,
        /// Parameter Specification
        "04" => N04,
        /// Category Classification
        "05" => N05,
        /// Product Type Identifier
        "08" => N08,
        /// Major Grade
        "10" => N10,
        /// Color
        "35" => N35,
        /// Grade Level
        "38" => N38,
        /// Chemistry
        "68" => N68,
        /// Mechanical
        "71" => N71,
    }
);

crate::code_enum!(
    /// **737** Measurement Reference ID Code
    ///
    /// - Data element: 737
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the broad category to which a measurement applies.
    ///
    /// The variants below are a working subset; the published list has 196 codes.
    /// Add the remainder here as `/// description` + `"CODE" => Variant,` lines.
    E737 {
        /// Average Balance
        "AB" => Ab,
        /// Maximum Balance
        "AC" => Ac,
        /// Capacity
        "AM" => Am,
        /// Work
        "AN" => An,
        /// Average Reading
        "AV" => Av,
        /// Relative Humidity
        "AW" => Aw,
        /// Billed Dimensions
        "BD" => Bd,
        /// Concentration
        "CO" => Co,
        /// Counts
        "CT" => Ct,
        /// Finished Dimensions
        "FD" => Fd,
        /// Load Planning Dimensions
        "LD" => Ld,
        /// Accuracy
        "MA" => Ma,
        /// Ordered Dimensions
        "OD" => Od,
        /// Physical Dimensions
        "PD" => Pd,
        /// Package Dimensions
        "PK" => Pk,
        /// Position
        "PO" => Po,
        /// Shipped Dimensions
        "SD" => Sd,
        /// Surface
        "SU" => Su,
        /// Temperature
        "TE" => Te,
        /// Time
        "TI" => Ti,
        /// Total Dimensions
        "TO" => To,
        /// Test Results
        "TR" => Tr,
        /// Single Test Limits
        "TS" => Ts,
        /// Weights
        "WT" => Wt,
    }
);

crate::code_enum!(
    /// **738** Measurement Qualifier
    ///
    /// - Data element: 738
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Code identifying a specific product or process characteristic to which a
    /// measurement applies.
    ///
    /// The variants below are a working subset; the published list has ~600 codes.
    /// Add the remainder here as `/// description` + `"CODE" => Variant,` lines.
    E738 {
        /// Consolidated Weight
        "A" => A,
        /// Ambient Temperature
        "AD" => Ad,
        /// Billed Weight
        "B" => B,
        /// Brightness
        "BR" => Br,
        /// Caliper
        "CA" => Ca,
        /// Diameter
        "DI" => Di,
        /// Density
        "DN" => Dn,
        /// Elongation
        "EA" => Ea,
        /// Gross Weight
        "G" => G,
        /// Height
        "HT" => Ht,
        /// Length
        "LN" => Ln,
        /// Minimum
        "MI" => Mi,
        /// Maximum
        "MX" => Mx,
        /// Actual Net Weight
        "N" => N,
        /// Outside Diameter
        "OD" => Od,
        /// Tare Weight
        "T" => T,
        /// Tenacity
        "TE" => Te,
        /// Thickness
        "TH" => Th,
        /// Volume
        "VOL" => Vol,
        /// Width
        "WD" => Wd,
        /// Weight
        "WT" => Wt,
    }
);

crate::num_element!(
    /// **739** Measurement Value
    ///
    /// - Data element: 739
    /// - Type: Decimal number (R)
    /// - Length: min 1, max 20
    ///
    /// The value of the measurement. `as_f64()`/`as_i64()` give typed views; the
    /// raw text is preserved so precision and formatting round-trip exactly.
    E739
);

crate::num_element!(
    /// **740** Range Minimum
    ///
    /// - Data element: 740
    /// - Type: Decimal number (R)
    /// - Length: min 1, max 20
    ///
    /// The minimum value of a measurement range.
    E740
);

crate::num_element!(
    /// **741** Range Maximum
    ///
    /// - Data element: 741
    /// - Type: Decimal number (R)
    /// - Length: min 1, max 20
    ///
    /// The maximum value of a measurement range.
    E741
);
