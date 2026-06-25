//! X12 interchange-control data elements (the `I`-series, `I01`–`I16`).
//!
//! The interchange envelope (`ISA`/`IEA`) draws its elements from this fixed,
//! self-contained series rather than the general numeric element dictionary.
//! Code-list elements are enums (with an `Unknown` catch-all for unpublished codes);
//! `I08`/`I09` carry interchange date/time text with typed views; `I12`/`I16` are
//! numerics. The free-text elements (`I02`, `I04`, `I06`, `I07`) and the component
//! separator (`I15`) have no typed form and remain `String` in the segments.

crate::code_enum!(
    /// **I01** Authorization Information Qualifier
    ///
    /// - Data element: I01
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the type of information in the Authorization Information (I02).
    I01 {
        /// No Authorization Information Present (No Meaningful Information in I02)
        "00" => N00,
        /// UCS Communications ID
        "01" => N01,
        /// EDX Communications ID
        "02" => N02,
        /// Additional Data Identification
        "03" => N03,
        /// Rail Communications ID
        "04" => N04,
        /// Department of Defense (DoD) Communication Identifier
        "05" => N05,
        /// United States Federal Government Communication Identifier
        "06" => N06,
    }
);

crate::code_enum!(
    /// **I03** Security Information Qualifier
    ///
    /// - Data element: I03
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code identifying the type of information in the Security Information (I04).
    I03 {
        /// No Security Information Present (No Meaningful Information in I04)
        "00" => N00,
        /// Password
        "01" => N01,
    }
);

crate::code_enum!(
    /// **I05** Interchange ID Qualifier
    ///
    /// - Data element: I05
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Qualifier designating the system/method of code structure used to designate the
    /// sender or receiver ID. Common codes named below; any other round-trips as
    /// `Unknown`.
    I05 {
        /// Duns (Dun & Bradstreet)
        "01" => N01,
        /// Standard Carrier Alpha Code (SCAC)
        "02" => N02,
        /// Federal Maritime Commission (FMC)
        "03" => N03,
        /// International Air Transport Association (IATA)
        "04" => N04,
        /// UCC EDI Communications ID (Comm ID)
        "08" => N08,
        /// X.121 (CCITT)
        "09" => N09,
        /// Department of Defense (DoD) Activity Address Code
        "10" => N10,
        /// Drug Enforcement Administration (DEA)
        "11" => N11,
        /// Telephone Companies
        "12" => N12,
        /// Duns Plus Suffix
        "14" => N14,
        /// Petroleum Accountants Society of Canada Company Codes
        "15" => N15,
        /// Duns Number With 4-Character Suffix
        "16" => N16,
        /// American Bankers Association (ABA) Transit Routing Number (including check digit, 9 digit)
        "17" => N17,
        /// Association of American Railroads (AAR) Standard Distribution Code
        "18" => N18,
        /// Health Industry Number (HIN)
        "20" => N20,
        /// Carrier Identification Number as assigned by Health Care Financing Administration (HCFA)
        "27" => N27,
        /// Fiscal Intermediary Identification Number as assigned by Health Care Financing Administration (HCFA)
        "28" => N28,
        /// Medicare Provider and Supplier Identification Number as assigned by Health Care Financing Administration (HCFA)
        "29" => N29,
        /// U.S. Federal Tax Identification Number
        "30" => N30,
        /// National Association of Insurance Commissioners Company Code (NAIC)
        "33" => N33,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::date_element!(
    /// **I08** Interchange Date
    ///
    /// - Data element: I08
    /// - Type: Date (DT)
    /// - Length: min 6, max 6
    ///
    /// Date of the interchange, in `YYMMDD` format.
    I08
);

crate::time_element!(
    /// **I09** Interchange Time
    ///
    /// - Data element: I09
    /// - Type: Time (TM)
    /// - Length: min 4, max 4
    ///
    /// Time of the interchange, in `HHMM` format.
    I09
);

crate::code_enum!(
    /// **I10** Interchange Control Standards Identifier
    ///
    /// - Data element: I10
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code identifying the agency responsible for the control standard used by the
    /// message enclosed by the interchange header and trailer.
    I10 {
        /// U.S. EDI Community of ASC X12, TDCC, and UCS
        "U" => U,
    }
);

crate::code_enum!(
    /// **I11** Interchange Control Version Number
    ///
    /// - Data element: I11
    /// - Type: Identifier (ID)
    /// - Length: min 5, max 5
    ///
    /// Code specifying the version number of the interchange control segments. Common
    /// codes named below; any other round-trips as `Unknown`.
    I11 {
        "00200" => N00200,
        "00201" => N00201,
        "00204" => N00204,
        "00300" => N00300,
        "00301" => N00301,
        "00302" => N00302,
        "00303" => N00303,
        "00304" => N00304,
        "00305" => N00305,
        "00306" => N00306,
        "00307" => N00307,
        "00400" => N00400,
        "00401" => N00401,
        "00402" => N00402,
        "00403" => N00403,
        "00404" => N00404,
        "00405" => N00405,
        "00406" => N00406,
        "00500" => N00500,
        "00501" => N00501,
        "00502" => N00502,
        "00503" => N00503,
        "00504" => N00504,
        "00505" => N00505,
        "00510" => N00510,
        "00601" => N00601,
        "00602" => N00602,
        "00603" => N00603,
        "00604" => N00604,
        "00605" => N00605,
    }
);

crate::num_element!(
    /// **I12** Interchange Control Number
    ///
    /// - Data element: I12
    /// - Type: Numeric (N0)
    /// - Length: min 9, max 9
    ///
    /// A control number assigned by the interchange sender.
    I12
);

crate::code_enum!(
    /// **I13** Acknowledgment Requested
    ///
    /// - Data element: I13
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code sent by the sender to request an interchange acknowledgment (TA1).
    I13 {
        /// No Interchange Acknowledgment Requested
        "0" => N0,
        /// Interchange Acknowledgment Requested (TA1)
        "1" => N1,
    }
);

crate::code_enum!(
    /// **I14** Usage Indicator
    ///
    /// - Data element: I14
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code to indicate whether data enclosed by this interchange envelope is test,
    /// production or information. Used in ISA field 15.
    I14 {
        /// Information
        "I" => Information,
        /// Production Data
        "P" => Production,
        /// Test Data
        "T" => Test,
    }
);

crate::num_element!(
    /// **I16** Number of Included Functional Groups
    ///
    /// - Data element: I16
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 5
    ///
    /// A count of the number of functional groups included in an interchange.
    I16
);
