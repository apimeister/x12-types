//! X12 data elements 1000-1099.

crate::code_enum!(
    /// **1005** Hierarchical Structure Code
    ///
    /// - Data element: 1005
    /// - Type: Identifier (ID)
    /// - Length: min 4, max 4
    ///
    /// Code indicating the hierarchical application structure of a transaction set
    /// that uses the HL segment (BHT-01). Common codes named below.
    E1005 {
        /// Shipment, Order, Packaging, Item
        "0001" => N0001,
        /// Shipment, Order, Item
        "0004" => N0004,
        /// Information Source, Receiver, Provider, Subscriber, Dependent
        "0010" => N0010,
        /// Provider of Service, Subscriber, Dependent
        "0016" => N0016,
        /// Subscriber, Dependent
        "0017" => N0017,
        /// Information Source, Receiver, Provider, Group, Site of Service
        "0024" => N0024,
        /// Group, Member
        "0059" => N0059,
        /// Information Source, Information Receiver, Company/Corporation, Operating Unit
        "0079" => N0079,
        /// Information Source, Employer, Patient
        "0080" => N0080,
        /// Information Source, Receiver, Provider of Service, Patient
        "0085" => N0085,
    }
);

crate::code_enum!(
    /// **1065** Entity Type Qualifier
    ///
    /// - Data element: 1065
    /// - Type: Identifier (ID)
    /// - Length: min 1, max 1
    ///
    /// Code qualifying the type of entity (NM1-02).
    E1065 {
        /// Person
        "1" => N1,
        /// Non-Person Entity
        "2" => N2,
    }
);
