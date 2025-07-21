# X12-945 Warehouse Shipping Advice

The X12-945 Warehouse Shipping Advice transaction set is used by warehouses and third-party logistics providers (3PLs) to notify suppliers that a shipment has been made. This transaction provides detailed information about what was shipped, when it was shipped, and to whom.

## Key Features

- **Shipment Confirmation**: Confirms that goods have been shipped from the warehouse
- **Reconciliation**: Allows suppliers to reconcile ordered quantities with shipped quantities
- **Tracking Information**: Provides carrier details and tracking information
- **Item Details**: Includes detailed product information for each shipped item

## Structure

The 945 transaction includes the following main components:

### Required Segments
- **ST** - Transaction Set Header
- **W06** - Warehouse Shipment Identification
- **LX** - Assigned Number (for line items)
- **W12** - Warehouse Item Detail
- **SE** - Transaction Set Trailer

### Optional Segments
- **N1/N2/N3/N4** - Name and Address Information (Ship-to, Ship-from, etc.)
- **N9** - Reference Identification (tracking numbers, purchase orders, etc.)
- **G62** - Date/Time Information
- **W27** - Carrier Detail
- **W03** - Total Shipment Information

## Usage Example

```rust
use x12_types::v004010::*;
use x12_types::util::Parser;

// Create a 945 transaction
let shipping_advice = _945 {
    st: ST {
        _01: "945".to_string(),
        _02: "0001".to_string(),
    },
    w06: W06 {
        _01: "F".to_string(), // Full shipment
        _02: Some("ORDER123".to_string()),
        _03: Some("20231215".to_string()), // Ship date
        _06: Some("SHIP001".to_string()),  // Shipment ID
        ..Default::default()
    },
    loop_lx: vec![
        _945LoopLX {
            lx: LX { _01: "1".to_string() },
            w12: W12 {
                _01: "CC".to_string(), // Complete shipment
                _02: Some("10".to_string()), // Quantity shipped
                _07: Some("UP".to_string()), // UPC qualifier
                _08: Some("123456789012".to_string()), // UPC code
                ..Default::default()
            },
            n9: vec![],
        }
    ],
    se: SE {
        _01: "6".to_string(),
        _02: "0001".to_string(),
    },
    ..Default::default()
};

// Serialize to X12 format
let x12_string = format!("{}", shipping_advice);

// Parse from X12 format
let (remaining, parsed) = _945::parse(&x12_string).unwrap();
```

## Common Use Cases

1. **3PL Shipment Notification**: A third-party logistics provider notifies the supplier that goods have been shipped to a customer.

2. **Inventory Reconciliation**: Suppliers use the 945 to reconcile their inventory records with actual shipments.

3. **Invoice Generation**: The 945 provides the data needed to generate accurate invoices based on what was actually shipped.

4. **ASN Creation**: Suppliers can use 945 data to create 856 Advance Ship Notice transactions for their customers.

## Integration with Other Transactions

The 945 is typically used in conjunction with:

- **940 Warehouse Shipping Order**: The initial instruction to ship
- **856 Advance Ship Notice**: Notification to the end customer
- **810 Invoice**: Billing based on shipped quantities
- **997 Functional Acknowledgment**: Confirmation of receipt

## Field Codes

### W06 Reporting Codes
- `F` - Full shipment
- `P` - Partial shipment
- `C` - Complete (final) shipment

### W12 Reporting Codes
- `CC` - Complete shipment of line item
- `CP` - Partial shipment of line item
- `CF` - Final shipment of line item

### N1 Entity Identifier Codes
- `ST` - Ship To
- `SF` - Ship From
- `CN` - Consignee
- `DE` - Depositor

For more detailed information about field codes and requirements, consult the official X12 004010 specification.