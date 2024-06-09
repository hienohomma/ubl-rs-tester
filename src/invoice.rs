use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ubl_rs::bdndr_ccts_cct_schemamodule_1_1::IdentifierType;
use ubl_rs::ubl_commonaggregatecomponents_2_1::{AccountingCustomerParty, AccountingSupplierParty, InvoiceLine, LegalMonetaryTotal};
use ubl_rs::ubl_commonaggregatecomponents_2_1::{InvoicePeriod, PeriodArrayOfEndDateComponent, PeriodArrayOfStartDateComponent};
use ubl_rs::ubl_commonbasiccomponents_2_1::{EndDate, StartDate, IssueDate};
use ubl_rs::FormattedValue;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceWrapper {
    #[serde(rename = "_D")]
    _d: String,
    #[serde(rename = "_A")]
    _a: String,
    #[serde(rename = "_B")]
    _b: String,
    pub invoice: Vec<Invoice>,
}

impl InvoiceWrapper {
    pub fn new(invoice: Invoice) -> Self {
        InvoiceWrapper {
            _d: "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2".to_string(),
            _a: "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2".to_string(),
            _b: "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2".to_string(),
            invoice: vec![invoice],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Invoice {
    #[serde(rename(serialize = "ID"))]
    id: Vec<IdentifierType>,
    issue_date: Vec<IssueDate>,
    invoice_period: Vec<InvoicePeriod>,
    pub accounting_supplier_party: Vec<AccountingSupplierParty>,
    pub accounting_customer_party: Vec<AccountingCustomerParty>,
    pub legal_monetary_total: Vec<LegalMonetaryTotal>,
    pub invoice_line: Vec<InvoiceLine>,
}

impl Invoice {
    pub fn new(id: &str, issue_date: NaiveDate, start_date: NaiveDate, due_date: NaiveDate) -> Result<Self> {
        let identifier = IdentifierType::new(id)
            .get_validated()
            .map_err(|e| anyhow::anyhow!("Invalid invoice identifier: {}", e))?;


        let issued_date = FormattedValue::new_date(issue_date);
        let period_start_date = FormattedValue::new_date(start_date);
        let period_end_date = FormattedValue::new_date(due_date);

        // Invoice period component initialization
        let mut period = InvoicePeriod::new();
        let component = period.as_mut();

        // Period start date component + validation
        let date = StartDate::new(period_start_date)
            .get_validated()
            .map_err(|e| anyhow!("Invalid start date format: {}", e))?;

        // Add start date to the period component
        component.start_date = Some(PeriodArrayOfStartDateComponent::new(date)
            .get_validated()
            .map_err(|e| anyhow!("Invalid start date format: {}", e))?
        );

        // Period end date component + validation
        let date = EndDate::new(period_end_date)
            .get_validated()
            .map_err(|e| anyhow!("Invalid end date format: {}", e))?;

        // Add end date to the period component
        component.end_date = Some(PeriodArrayOfEndDateComponent::new(date)
            .get_validated()
            .map_err(|e| anyhow!("Invalid end date format: {}", e))?
        );

        // Issue date component + validation
        let issue_date = IssueDate::new(issued_date)
            .get_validated()
            .map_err(|e| anyhow!("Invalid issue date format: {}", e))?;

        // Invoice period to validated component
        let period = period.get_validated()
            .map_err(|e| anyhow!("Invoice period validation failed: {}", e))?;

        Ok(Invoice {
            id: vec![identifier],
            issue_date: vec![issue_date],
            invoice_period: vec![period],
            accounting_supplier_party: Vec::new(),
            accounting_customer_party: Vec::new(),
            legal_monetary_total: Vec::new(),
            invoice_line: Vec::new(),
        })
    }
}
