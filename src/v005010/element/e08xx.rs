//! X12 data elements 0800-0899.

crate::code_enum!(
    /// **812** Payment Format Code
    ///
    /// - Data element: 812
    /// - Type: Identifier (ID)
    /// - Length: min 3, max 3
    ///
    /// Code identifying the payment format to be used (BPR-05).
    E812 {
        /// Cash Concentration/Disbursement (CCD)
        "CCD" => Ccd,
        /// Cash Concentration/Disbursement plus Addenda (CCD+)
        "CCP" => Ccp,
        /// Corporate Trade Exchange (CTX)
        "CTX" => Ctx,
        /// Prearranged Payment and Deposit (PPD)
        "PPD" => Ppd,
        /// Mutually Defined
        "ZZZ" => Zzz,
    }
);
