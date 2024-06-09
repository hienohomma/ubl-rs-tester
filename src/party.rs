use anyhow::{anyhow, Result};
use ubl_rs::bdndr_unqualifieddatatypes_1_1::NameType;
use ubl_rs::ubl_commonaggregatecomponents_2_1::{AccountingCustomerParty, AccountingSupplierParty, CustomerPartyArrayOfPartyComponent, Party, PartyArrayOfPartyNameComponent, PartyName, PartyNameArrayOfNameComponent, SupplierPartyArrayOfPartyComponent};
use ubl_rs::bdndr_ccts_cct_schemamodule_1_1::TextType;


pub fn new_supplier(name: &str) -> Result<AccountingSupplierParty> {
    let party = new(name)?;

    // Supplier party lives in an array
    let supplier_parties = SupplierPartyArrayOfPartyComponent::new(party)
        .get_validated()?;

    // Build accounting supplier party with party
    let mut supplier = AccountingSupplierParty::new();
    
    let supplier_comp = supplier.as_mut();
    supplier_comp.party = Some(supplier_parties);

    // Validate and return supplier created above
    supplier.get_validated().map_err(|e| anyhow!("Failed to validate supplier: {}", e))
}

pub fn new_customer(name: &str) -> Result<AccountingCustomerParty> {
    let party = new(name)?;

    // Customer party lives in an array
    let customer_parties = CustomerPartyArrayOfPartyComponent::new(party)
        .get_validated()?;

    // Build accounting customer party
    let mut customer = AccountingCustomerParty::new();
    
    let customer_comp = customer.as_mut();
    customer_comp.party = Some(customer_parties);

    // Validate and return supplier created above
    customer.get_validated().map_err(|e| anyhow!("Failed to validate customer: {}", e))
}

fn new(name: &str) -> Result<Party> {
    // Make name for the party member
    let name = TextType::new(name).get_validated()?;
    let name_type = NameType(name);
    
    // Build party with member name
    let mut party = Party::new();
    let party_comp = party.as_mut();

    let party_names = PartyNameArrayOfNameComponent::new(name_type).get_validated()?;
    let party_name = PartyName::new(party_names).get_validated()?;
    party_comp.party_name = Some(PartyArrayOfPartyNameComponent::new(party_name).get_validated()?);

    party.get_validated().map_err(|e| anyhow!("Failed to validate party: {}", e))
}
