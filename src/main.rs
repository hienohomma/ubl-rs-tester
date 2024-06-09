mod invoice;
mod party;
mod line;

use std::path::PathBuf;
use std::io::Write;
use std::str::FromStr;

use chrono::NaiveDate;
use ubl_rs::bdndr_unqualifieddatatypes_1_1::AmountType;
use ubl_rs::ubl_commonaggregatecomponents_2_1::{LegalMonetaryTotal, MonetaryTotalArrayOfPayableAmountComponent};

use invoice::*;
use line::Lines;
use party::{new_customer, new_supplier};


fn main() {
    let new_invoice = Invoice::new(
        "123",
        NaiveDate::from_str("2011-09-22").expect("Invalid issue date format"),
        NaiveDate::from_str("2011-08-01").expect("Invalid start date format"), 
        NaiveDate::from_str("2011-08-31").expect("Invalid due date format")
    );

    let mut invoice = match new_invoice {
        Ok(i) => i,
        Err(e) => panic!("Failed to create new invoice: {}", e.to_string()),
    };

    // Invoice supplier, selling party
    let supplier = match new_supplier("CustomCotterPins") {
        Ok(s) => s,
        Err(e) => panic!("Failed to create new supplier: {}", e.to_string()),
    };

    // Invoice customer, buying party
    let customer = match new_customer("NorthAmericanVeeblefetzer") {
        Ok(c) => c,
        Err(e) => panic!("Failed to create new customer: {}", e.to_string()),
    };

    // Add parties to invoice
    invoice.accounting_supplier_party.push(supplier);
    invoice.accounting_customer_party.push(customer);

    // Invoice lines are in Canadian dollars
    let mut lines = Lines::new("CAD");

    // New line
    if let Err(e) = lines.push("Cotterpin,MIL-SPEC", 100.00) {
        panic!("Failed to create new invoice line: {}", e.to_string())
    }

    // Invoice total
    let line_total = serde_json::Number::from_f64(lines.total).unwrap();
    let amount = match AmountType::new(line_total, &lines.currency).get_validated() {
        Ok(a) => a,
        Err(e) => panic!("Failed to create invoice amount: {}", e.to_string())
    };

    let mon_total = match MonetaryTotalArrayOfPayableAmountComponent::new(amount).get_validated() {
        Ok(m) => m,
        Err(e) => panic!("Failed to create invoice monetary total: {}", e.to_string())
    };

    match LegalMonetaryTotal::new(mon_total).get_validated() {
        Ok(g) => invoice.legal_monetary_total.push(g),
        Err(e) => panic!("Failed to create invoice grand total: {}", e.to_string())
    }

    // Add lines to invoice
    invoice.invoice_line = lines.items;

    // Print json_examples/UBL-Invoice-2.1-Example-Trivial.json as a string
    let json_file = PathBuf::from("./json_examples/UBL-Invoice-2.1-Example-Trivial.json");
    let mut json_str = std::fs::read_to_string(json_file).unwrap();
    
    // Remove whitespace from json_str
    json_str.retain(|c| !c.is_whitespace());

    let wrapper = InvoiceWrapper::new(invoice);
    let built_json_str = serde_json::to_string(&wrapper).unwrap();

    // Write both json strings into a file
    let path = PathBuf::from("./json_examples/UBL-Invoice-2.1-Example-Trivial-comparison.json");
    let mut file = std::fs::File::create(path).unwrap();

    writeln!(file, "[\n{},", json_str).unwrap();
    writeln!(file, "{}\n]", built_json_str).unwrap();
}
