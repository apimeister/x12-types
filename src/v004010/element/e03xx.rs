//! X12 data elements 0300-0399.

crate::num_element!(
    /// **330** Quantity Ordered
    ///
    /// - Data element: 330
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Quantity Ordered.
    E330
);

crate::num_element!(
    /// **332** Rate
    ///
    /// - Data element: 332
    /// - Type: Numeric (R)
    /// - Length: min 1, max 9
    ///
    /// Rate.
    E332
);

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

crate::num_element!(
    /// **347** Hash Total
    ///
    /// - Data element: 347
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Hash Total.
    E347
);

crate::num_element!(
    /// **354** Number of Line Items
    ///
    /// - Data element: 354
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 6
    ///
    /// Number of Line Items.
    E354
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

crate::num_element!(
    /// **380** Quantity
    ///
    /// - Data element: 380
    /// - Type: Numeric (R)
    /// - Length: min 1, max 15
    ///
    /// Quantity.
    E380
);

crate::num_element!(
    /// **382** Number of Units Shipped
    ///
    /// - Data element: 382
    /// - Type: Numeric (R)
    /// - Length: min 1, max 10
    ///
    /// Number of Units Shipped.
    E382
);

crate::num_element!(
    /// **387** Equipment Number Check Digit
    ///
    /// - Data element: 387
    /// - Type: Numeric (N0)
    /// - Length: min 1, max 1
    ///
    /// Equipment Number Check Digit.
    E387
);

crate::num_element!(
    /// **395** Unit Weight
    ///
    /// - Data element: 395
    /// - Type: Numeric (R)
    /// - Length: min 1, max 8
    ///
    /// Unit Weight.
    E395
);

crate::code_enum!(
    /// **372** Lading Liability Code
    ///
    /// - Data element: 372
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Lading Liability Code. Code values verified against the Stedi X12 reference.
    E372 {
        /// Full Liability
        "F" => F,
        /// Limited Liability
        "L" => L,
    }
);

crate::code_enum!(
    /// **393** Amendment Code
    ///
    /// - Data element: 393
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Amendment Code. Code values verified against the Stedi X12 reference.
    E393 {
        /// Not laden aboard per evidence from foreign shipper, or amended bill of lading
        "01" => N01,
        /// Error in manifesting, not laden on this carrier. Laden on subsequent carrier for transportation to United States, per evidence in files
        "02" => N02,
        /// Clerical error in manifesting per bill of lading in files
        "03" => N03,
        /// Pilfered or prematurely landed prior to arrival in United States per signed statement of master or his agent or vessel log extract in our file
        "04" => N04,
        /// Erroneously duplicated by another bill of lading on the same manifest
        "05" => N05,
        /// Prematurely landed or overcarried to another United States port where proper disposition was made per evidence in our files
        "06" => N06,
        /// Inadvertently retained on board and taken foreign per master's or his agent's statement, amended bill of lading, landing certificate, in our files
        "07" => N07,
        /// Container stripped under Customs supervision; Foreign seals affixed abroad were intact, as per evidence in our files
        "08" => N08,
        /// Merchandise apparently pilfered on dock while in custody of carrier
        "09" => N09,
        /// Inadvertently delivered without customs release; Goods will be redelivered intact or duty and taxes will be paid by carrier
        "10" => N10,
        /// Overage - Omitted from manifest through clerical error
        "11" => N11,
        /// Overage - Manifested for discharge at another port and inadvertently discharged at this port
        "12" => N12,
        /// Proper entry filed or place in general order per entry or general order number
        "13" => N13,
        /// Merchandise inadvertently delivered to consignee without customs release; Merchandise will be redelivered intact or liquidated damages paid
        "14" => N14,
        /// Merchandise cannot be located and has apparently been lost; Liquidated damages will be paid
        "15" => N15,
        /// Error in quantity manifested at port of origin; Customs form 5931 will be filed at origin to correct in-bond entry; A copy will be delivered to this port within 90 days or duty and taxes will be paid
        "16" => N16,
        /// Merchandise removed from original container and re-stuffed prior to moving in-bond
        "17" => N17,
    }
);

crate::code_enum!(
    /// **399** Pallet Exchange Code
    ///
    /// - Data element: 399
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Pallet Exchange Code. Code values verified against the Stedi X12 reference.
    E399 {
        /// No Exchange/No Return
        "1" => N1,
        /// Exchange Pallets
        "2" => N2,
        /// Return Pallets
        "3" => N3,
        /// Pallets to be Purchased by Customer
        "4" => N4,
        /// Third-Party Pallet Exchange
        "5" => N5,
    }
);

crate::num_element!(
    /// **315** Compensation Paid
    ///
    /// - Data element: 315
    /// - Type: Numeric (R)
    /// - Length: min 2, max 5
    ///
    /// Compensation Paid.
    E315
);

crate::num_element!(
    /// **317** Total Compensation Amount
    ///
    /// - Data element: 317
    /// - Type: Numeric (N0)
    /// - Length: min 3, max 10
    ///
    /// Total Compensation Amount.
    E317
);
