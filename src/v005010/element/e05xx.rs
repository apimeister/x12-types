//! X12 data elements 0500-0599.

crate::code_enum!(
    /// **571** Tare Qualifier Code
    ///
    /// - Data element: 571
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code identifying the type of tare weight.
    E571 {
        /// Actual
        "A" => A,
        /// Marked
        "M" => M,
    }
);

crate::code_enum!(
    /// **591** Payment Method Code
    ///
    /// - Data element: 591
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Code identifying the method for the movement of payment instructions (BPR-04).
    E591 {
        /// Automated Clearing House (ACH)
        "ACH" => Ach,
        /// Check
        "CHK" => Chk,
        /// Direct Deposit
        "DDP" => Ddp,
        /// Financial Institution Option
        "BOP" => Bop,
        /// Clearing House Interbank Payments System (CHIPS) Funds/Wire Transfer
        "CWT" => Cwt,
        /// Federal Reserve Funds/Wire Transfer - Nonrepetitive
        "FWT" => Fwt,
        /// Society for Worldwide Interbank Financial Telecommunications (S.W.I.F.T.)
        "SWT" => Swt,
        /// International Electronic Funds Transfer
        "IWT" => Iwt,
        /// Non-Payment Data
        "NON" => Non,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);

crate::code_enum!(
    /// **559** Agency Qualifier Code
    ///
    /// - Data element: 559
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the agency assigning the code values. Common codes named
    /// below; any other round-trips as `Unknown`.
    E559 {
        /// American Medical Association
        "AM" => Am,
        /// American Petroleum Institute
        "AP" => Ap,
        /// Department of Defense
        "DD" => Dd,
        /// Centers for Medicare and Medicaid Services
        "HC" => Hc,
        /// International Standards Organization
        "IS" => Is,
        /// Dun & Bradstreet
        "93" => N93,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **522** Amount Qualifier Code
    ///
    /// - Data element: 522
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 3
    ///
    /// Code to qualify amount. Common codes named below; any other round-trips as
    /// `Unknown`.
    E522 {
        /// Line Item Total
        "1" => N1,
        /// Batch Total
        "2" => N2,
        /// Deposit Total
        "3" => N3,
        /// Total Invoice Amount
        "5" => N5,
        /// Amount Subject to Total Monetary Discount
        "6" => N6,
        /// Discount Amount Due
        "7" => N7,
        /// Total Monetary Discount Amount
        "8" => N8,
        /// Total Operational Statement Amount
        "9" => N9,
        /// Deductible Amount
        "D2" => D2,
        /// Interest
        "I" => I,
        /// Premium Amount
        "P3" => P3,
        /// Tax
        "T" => T,
        /// Net
        "N" => N,
        /// Coverage Amount
        "AU" => Au,
        /// Balance Due
        "BD" => Bd,
    }
);
