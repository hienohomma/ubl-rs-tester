use anyhow::Result;
use ubl_rs::bdndr_unqualifieddatatypes_1_1::{AmountType, IdentifierType, TextType};
use ubl_rs::bdndr_ccts_cct_schemamodule_1_1::{TextType as QalifiedTextType, IdentifierType as QualifiedIdentifierType};
use ubl_rs::ubl_commonaggregatecomponents_2_1::{
    InvoiceLine, InvoiceLineArrayOfIDComponent,
    InvoiceLineArrayOfItemComponent, InvoiceLineArrayOfLineExtensionAmountComponent, Item, ItemArrayOfDescriptionComponent
};

pub struct Lines {
    pub currency: String,
    pub total: f64,
    pub items: Vec<InvoiceLine>,
}

impl Lines {
    pub fn new<T>(currency: T) -> Self where T: Into<String> {
        Lines {
            currency: currency.into(),
            items: vec![],
            total: 0.0,
        }
    }
    pub fn push(&mut self, name: &str, total: f64) -> Result<()> {
        // Invoice line item description
        let item_name = TextType(QalifiedTextType::new(name).get_validated()?);
        let mut item = Item::new();
        let item_comp = item.as_mut();

        let description = ItemArrayOfDescriptionComponent::new(item_name).get_validated()?;
        item_comp.description = Some(description);

        let line_item = InvoiceLineArrayOfItemComponent::new(item.get_validated()?);
        let id_count = self.items.len() + 1;

        // Invoice line ID
        let identifier = IdentifierType(QualifiedIdentifierType::new(id_count.to_string()).get_validated()?);
        let line_id = InvoiceLineArrayOfIDComponent::new(identifier).get_validated()?;

        // Invoice line amount
        let amount = serde_json::Number::from_f64(total).unwrap();
        let line_amount = AmountType::new(amount, &self.currency).get_validated()?;

        // Line amount lives in an array
        let line_ext_amount = InvoiceLineArrayOfLineExtensionAmountComponent::new(line_amount).get_validated()?;

        // Build invoice line
        let newline = InvoiceLine::new(
            line_id,
            line_item.get_validated()?,
            line_ext_amount
        );

        self.items.push(newline.get_validated()?);

        self.total += total;

        Ok(())
    }
}