//! X12 data elements 0200-0299.

crate::date_element!(
    /// **243** Transaction Reference Date
    ///
    /// - Data element: 243
    /// - Type: Date (DT)
    /// - Length: min 6, max 6
    ///
    /// Transaction Reference Date.
    E243
);

crate::code_enum!(
    /// **202** Correction Indicator
    ///
    /// - Data element: 202
    /// - Type: Identifier (ID)
    /// - Length: min 2, max 2
    ///
    /// Correction Indicator. Code values verified against the Stedi X12 reference.
    E202 {
        /// Adjustment of Previous Freight Bill Charges
        "AD" => Ad,
        /// Supply Additional Information
        "AI" => Ai,
        /// Adding Revenue
        "AR" => Ar,
        /// Bad Order Car
        "B1" => B1,
        /// Bi-lateral Agreement Date not Met
        "BA" => Ba,
        /// Balance Due Billing
        "BD" => Bd,
        /// Billing Error
        "BE" => Be,
        /// Bad Order Setback/Bill Cancelled
        "BO" => Bo,
        /// Cancelled Bill
        "CA" => Ca,
        /// Cancelled Due to Multicars Being Covered on One Waybill
        "CM" => Cm,
        /// Correction
        "CO" => Co,
        /// Correct Container or Consolidation Contents
        "CP" => Cp,
        /// Consolidation
        "CS" => Cs,
        /// Cancel Waybill
        "CT" => Ct,
        /// Covered Under Another Universal Railroad Revenue Waybill Identification
        "CU" => Cu,
        /// Diversion or Reconsignment
        "D1" => D1,
        /// Dispute Over Collect/Prepaid
        "DC" => Dc,
        /// Delete Event
        "DE" => De,
        /// Delete Container or Consolidation Contents
        "DP" => Dp,
        /// Delayed Shipment
        "DS" => Ds,
        /// Error Move
        "EM" => Em,
        /// Miscellaneous Billing
        "MB" => Mb,
        /// Change Settlement Date
        "MD" => Md,
        /// Misroute
        "MR" => Mr,
        /// No Car (Have Not Received Car)
        "NC" => Nc,
        /// No Division Available
        "ND" => Nd,
        /// Past Due Billing
        "PD" => Pd,
        /// Rebilling (Ignore Previous Bill)
        "RB" => Rb,
        /// Revenue Correction
        "RC" => Rc,
        /// Route Dispute
        "RD" => Rd,
        /// Reassignment/Transfer
        "RE" => Re,
        /// Rebuttal, Full (of previously rejected invoice)
        "RF" => Rf,
        /// Rule 11 Domain Dispute
        "RL" => Rl,
        /// Rebuttal, Partial (of previously rejected invoice)
        "RP" => Rp,
        /// Switch Carrier
        "SC" => Sc,
        /// Separated Car From Multicar Shipment
        "SP" => Sp,
        /// Transload
        "TL" => Tl,
        /// Wrecked Car
        "WC" => Wc,
        /// Work Stoppage
        "WS" => Ws,
    }
);
