//! X12 data elements 0600-0699.

crate::code_enum!(
    /// **668** Line Item Status Code
    ///
    /// - Data element: 668
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the action taken by the seller on a line item requested by
    /// the buyer.
    E668 {
        /// Item Accepted and Shipped
        "AC" => Ac,
        /// Item Accepted and Released for Shipment
        "AR" => Ar,
        /// Item Accepted - Partial Shipment, Balance Backordered
        "BP" => Bp,
        /// Item Accepted - Date Rescheduled
        "DR" => Dr,
        /// Item Accepted
        "IA" => Ia,
        /// Item Backordered
        "IB" => Ib,
        /// Item Accepted - Changes Made
        "IC" => Ic,
        /// Item Deleted
        "ID" => Id,
        /// Item Accepted, Price Pending
        "IE" => Ie,
        /// Item on Hold
        "IH" => Ih,
        /// Item Accepted - Price Changed
        "IP" => Ip,
        /// Item Accepted - Quantity Changed
        "IQ" => Iq,
        /// Item Rejected
        "IR" => Ir,
    }
);

crate::code_enum!(
    /// **640** Transaction Type Code
    ///
    /// - Data element: 640
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the type of transaction. Common codes named below.
    E640 {
        /// Location Address Message
        "01" => N01,
        /// Report Message
        "03" => N03,
        /// Electronic Mail Message
        "04" => N04,
        /// Normal
        "33" => N33,
        /// Emergency Request
        "34" => N34,
        /// Customer Shipment
        "42" => N42,
        /// Audit
        "49" => N49,
        /// Information Only, No Response Required
        "54" => N54,
        /// New Service Order
        "55" => N55,
        /// Sale
        "98" => N98,
        /// Order
        "OR" => Or,
        /// Mutually Defined
        "ZZ" => Zz,
    }
);

crate::code_enum!(
    /// **673** Quantity Qualifier
    ///
    /// - Data element: 673
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Code specifying the type of quantity. Common codes named below; any other
    /// round-trips as `Unknown`.
    E673 {
        /// Discrete Quantity
        "01" => N01,
        /// Cumulative Quantity
        "02" => N02,
        /// Quantity Available to Promise
        "37" => N37,
        /// Total Receipts
        "38" => N38,
        /// Shipped Quantity
        "39" => N39,
        /// Quantity Ordered
        "41" => N41,
        /// Cumulative Quantity Required
        "63" => N63,
        /// Acceptance Quantity
        "72" => N72,
    }
);
