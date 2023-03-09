use std::collections::HashMap;

use chrono::NaiveDate;
use regex::Regex;
use web_sys::FormData;

use crate::components::constants::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FactureDto {
    pub number: usize,
    pub date: NaiveDate,
    pub date_emited: NaiveDate,
    pub acompte: String,
    pub is_paid: bool,
    pub project_name: String,
    pub project_adress: String,
    pub project_mail: String,
    pub project_tel: String,
    pub project_website: String,
    pub client_name: String,
    pub client_adress: String,
    pub client_tel: String,
    pub client_tva: String,
    pub client_devis: String,
    pub services: HashMap<String, f64>,
    pub project_bank: String,
    pub project_iban: String,
    pub project_bic: String,
}

impl FactureDto {
    pub fn from_form_data(form_data: &FormData) -> Result<FactureDto, String> {
        let form_data = form_data.clone();
        let number = resolve_number_form(&form_data);
        number.clone()?;
        let date = resolve_date_form(&form_data, FACTURE_DATE_QUERY);
        date.clone()?;
        let date_emited = resolve_date_form(&form_data, FACTURE_DATE_EMITED_QUERY);
        date_emited.clone()?;
        let acompte = resolve_acompte_form(&form_data);
        acompte.clone()?;
        let is_paid = resolve_is_paid_form(&form_data);
        is_paid.clone()?;
        let project_name = resolve_project_name_form(&form_data);
        project_name.clone()?;
        let project_adress = resolve_project_adress_form(&form_data);
        project_adress.clone()?;
        let project_mail = resolve_project_mail_form(&form_data);
        project_mail.clone()?;
        let project_tel = resolve_project_tel_form(&form_data);
        project_tel.clone()?;
        let project_website = resolve_project_website_form(&form_data);
        project_website.clone()?;
        let client_name = resolve_client_name_form(&form_data);
        client_name.clone()?;
        let client_adress = resolve_client_adress_form(&form_data);
        client_adress.clone()?;
        let client_tel = resolve_client_tel_form(&form_data);
        client_tel.clone()?;
        let client_tva = resolve_client_tva_form(&form_data);
        client_tva.clone()?;
        let client_devis = resolve_client_devis_form(&form_data);
        client_devis.clone()?;
        let services = resolve_services_form(&form_data);
        services.clone()?;
        let project_bank = resolve_project_bank_form(&form_data);
        project_bank.clone()?;
        let project_iban = resolve_project_iban_form(&form_data);
        project_iban.clone()?;
        let project_bic = resolve_project_bic_form(&form_data);
        project_bic.clone()?;
        let facture_dto = FactureDto {
            number: number.unwrap(),
            date: date.unwrap(),
            date_emited: date_emited.unwrap(),
            acompte: acompte.unwrap(),
            is_paid: is_paid.unwrap(),
            project_name: project_name.unwrap(),
            project_adress: project_adress.unwrap(),
            project_mail: project_mail.unwrap(),
            project_tel: project_tel.unwrap(),
            project_website: project_website.unwrap(),
            client_name: client_name.unwrap(),
            client_adress: client_adress.unwrap(),
            client_tel: client_tel.unwrap(),
            client_tva: client_tva.unwrap(),
            client_devis: client_devis.unwrap(),
            services: services.unwrap(),
            project_bank: project_bank.unwrap(),
            project_iban: project_iban.unwrap(),
            project_bic: project_bic.unwrap(),
        };
        Ok(facture_dto)
    }

    pub fn from_queries(queries: &HashMap<String, String>) -> Result<FactureDto, String> {
        let queries = queries.clone();
        let number = resolve_number_queries(&queries);
        number.clone()?;
        let date = resolve_date_queries(&queries, FACTURE_DATE_QUERY);
        date.clone()?;
        let date_emited = resolve_date_queries(&queries, FACTURE_DATE_EMITED_QUERY);
        date_emited.clone()?;
        let acompte = resolve_acompte_queries(&queries);
        acompte.clone()?;
        let is_paid = resolve_is_paid_queries(&queries);
        is_paid.clone()?;
        let project_name = resolve_project_name_queries(&queries);
        project_name.clone()?;
        let project_adress = resolve_project_adress_queries(&queries);
        project_adress.clone()?;
        let project_mail = resolve_project_mail_queries(&queries);
        project_mail.clone()?;
        let project_tel = resolve_project_tel_queries(&queries);
        project_tel.clone()?;
        let project_website = resolve_project_website_queries(&queries);
        project_website.clone()?;
        let client_name = resolve_client_name_queries(&queries);
        client_name.clone()?;
        let client_adress = resolve_client_adress_queries(&queries);
        client_adress.clone()?;
        let client_tel = resolve_client_tel_queries(&queries);
        client_tel.clone()?;
        let client_tva = resolve_client_tva_queries(&queries);
        client_tva.clone()?;
        let client_devis = resolve_client_devis_queries(&queries);
        client_devis.clone()?;
        let services = resolve_services_queries(&queries);
        services.clone()?;
        let project_bank = resolve_project_bank_queries(&queries);
        project_bank.clone()?;
        let project_iban = resolve_project_iban_queries(&queries);
        project_iban.clone()?;
        let project_bic = resolve_project_bic_queries(&queries);
        project_bic.clone()?;
        let facture_dto = FactureDto {
            number: number.unwrap(),
            date: date.unwrap(),
            date_emited: date_emited.unwrap(),
            acompte: acompte.unwrap(),
            is_paid: is_paid.unwrap(),
            project_name: project_name.unwrap(),
            project_adress: project_adress.unwrap(),
            project_mail: project_mail.unwrap(),
            project_tel: project_tel.unwrap(),
            project_website: project_website.unwrap(),
            client_name: client_name.unwrap(),
            client_adress: client_adress.unwrap(),
            client_tel: client_tel.unwrap(),
            client_tva: client_tva.unwrap(),
            client_devis: client_devis.unwrap(),
            services: services.unwrap(),
            project_bank: project_bank.unwrap(),
            project_iban: project_iban.unwrap(),
            project_bic: project_bic.unwrap(),
        };
        Ok(facture_dto)
    }

    pub fn to_queries(&self) -> HashMap<String, String> {
        let mut queries = HashMap::new();
        let facture = self.clone();
        queries.insert(FACTURE_NUMBER_QUERY.to_string(), facture.number.to_string());
        queries.insert(
            FACTURE_DATE_QUERY.to_string(),
            facture.date.format(DATE_FORMAT).to_string(),
        );
        queries.insert(
            FACTURE_DATE_EMITED_QUERY.to_string(),
            facture.date_emited.format(DATE_FORMAT).to_string(),
        );
        queries.insert(FACTURE_ACOMPTE.to_string(), facture.acompte);
        queries.insert(FACTURE_IS_PAID.to_string(), facture.is_paid.to_string());
        queries.insert(FACTURE_PROJECT_NAME.to_string(), facture.project_name);
        queries.insert(FACTURE_PROJECT_ADRESS.to_string(), facture.project_adress);
        queries.insert(FACTURE_PROJECT_EMAIL.to_string(), facture.project_mail);
        queries.insert(FACTURE_PROJECT_TEL.to_string(), facture.project_tel);
        queries.insert(FACTURE_PROJECT_WEBSITE.to_string(), facture.project_website);
        queries.insert(FACTURE_CLIENT_NAME.to_string(), facture.client_name);
        queries.insert(FACTURE_CLIENT_ADRESS.to_string(), facture.client_adress);
        queries.insert(FACTURE_CLIENT_TEL.to_string(), facture.client_tel);
        queries.insert(
            FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE.to_string(),
            facture.client_tva,
        );
        queries.insert(FACTURE_CLIENT_DEVIS.to_string(), facture.client_devis);
        queries.insert(
            FACTURE_SERVICES.to_string(),
            serde_json::to_string(&facture.services).unwrap(),
        );
        queries.insert(FACTURE_PROJECT_BANK.to_string(), facture.project_bank);
        queries.insert(FACTURE_PROJECT_IBAN.to_string(), facture.project_iban);
        queries.insert(FACTURE_PROJECT_BIC.to_string(), facture.project_bic);
        queries
    }
}

fn resolve_number_form(form_data: &FormData) -> Result<usize, String> {
    let number_str = form_data.get(FACTURE_NUMBER_QUERY).as_string();
    if let Some(number_str) = number_str {
        number_str
            .parse()
            .or(Err(format!("Failed to parse {number_str} into usize")))
    } else {
        Err(format!("Failed to get {FACTURE_NUMBER_QUERY}"))
    }
}

fn resolve_number_queries(queries: &HashMap<String, String>) -> Result<usize, String> {
    let number_str = queries.get(FACTURE_NUMBER_QUERY);
    if let Some(number_str) = number_str {
        number_str
            .parse()
            .or(Err(format!("Failed to parse {number_str} into usize")))
    } else {
        Err(format!("Failed to get {FACTURE_NUMBER_QUERY}"))
    }
}

fn resolve_date_form(form_data: &FormData, form_field: &str) -> Result<NaiveDate, String> {
    let date_str = form_data.get(form_field).as_string();
    if let Some(date_str) = date_str {
        to_naive_date(&date_str)
    } else {
        Err(format!("Failed to get {form_field}"))
    }
}

fn resolve_date_queries(
    queries: &HashMap<String, String>,
    queries_field: &str,
) -> Result<NaiveDate, String> {
    let date_str = queries.get(queries_field);
    if let Some(date_str) = date_str {
        to_naive_date(date_str)
    } else {
        Err(format!("Failed to get {queries_field}"))
    }
}

fn to_naive_date(date_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_str, DATE_FORMAT).or(Err(format!(
        "Could not parse date from string {}",
        date_str
    )))
}

fn resolve_acompte_form(form_data: &FormData) -> Result<String, String> {
    let acompte_str = form_data.get(FACTURE_ACOMPTE).as_string();
    acompte_check(acompte_str)
}

fn resolve_acompte_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let acompte_str = queries.get(FACTURE_ACOMPTE);
    if let Some(acompte_str) = acompte_str {
        acompte_check(Option::from(acompte_str.to_string()))
    } else {
        Err(format!("Failed to get {FACTURE_ACOMPTE}"))
    }
}

fn acompte_check(acompte_str: Option<String>) -> Result<String, String> {
    if let Some(acompte_str) = acompte_str {
        if acompte_str == "comptant" || acompte_str == "avec acompte" {
            Ok(acompte_str)
        } else {
            Err("Value must be either `acompte` or `avec acompte`".to_string())
        }
    } else {
        Err(format!("Failed to get {FACTURE_ACOMPTE}"))
    }
}

fn resolve_is_paid_form(form_data: &FormData) -> Result<bool, String> {
    let is_paid_str = form_data.get(FACTURE_IS_PAID).as_string();
    if let Some(is_paid_str) = is_paid_str {
        if is_paid_str == "on" {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

fn resolve_is_paid_queries(queries: &HashMap<String, String>) -> Result<bool, String> {
    let is_paid_str = queries.get(FACTURE_IS_PAID);
    if let Some(is_paid_str) = is_paid_str {
        if is_paid_str == "true" {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

fn resolve_project_name_form(form_data: &FormData) -> Result<String, String> {
    let project_name_str = form_data.get(FACTURE_PROJECT_NAME).as_string();
    not_empty_check(project_name_str, FACTURE_PROJECT_NAME)
}

fn resolve_project_name_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let project_name_str = queries.get(FACTURE_PROJECT_NAME);
    if let Some(project_name_str) = project_name_str {
        not_empty_check(
            Option::from(project_name_str.to_string()),
            FACTURE_PROJECT_NAME,
        )
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_NAME}"))
    }
}

fn not_empty_check(field_str: Option<String>, field_name: &str) -> Result<String, String> {
    if let Some(field_str) = field_str {
        if field_str.is_empty() {
            return Err(format!("{field_name} value can't be null"));
        }
        Ok(field_str)
    } else {
        Err(format!("Failed to get {field_name}"))
    }
}

fn resolve_project_adress_form(form_data: &FormData) -> Result<String, String> {
    let project_adress_str = form_data.get(FACTURE_PROJECT_ADRESS).as_string();
    not_empty_check(project_adress_str, FACTURE_PROJECT_ADRESS)
}

fn resolve_project_adress_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let project_adress_str = queries.get(FACTURE_PROJECT_ADRESS);
    if let Some(project_adress_str) = project_adress_str {
        not_empty_check(
            Option::from(project_adress_str.to_string()),
            FACTURE_PROJECT_ADRESS,
        )
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_ADRESS}"))
    }
}

fn resolve_project_mail_form(form_data: &FormData) -> Result<String, String> {
    let project_mail_str = form_data.get(FACTURE_PROJECT_EMAIL).as_string();
    email_check(project_mail_str, FACTURE_PROJECT_EMAIL)
}

fn resolve_project_mail_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let project_mail_str = queries.get(FACTURE_PROJECT_EMAIL);
    if let Some(project_mail_str) = project_mail_str {
        email_check(
            Option::from(project_mail_str.to_string()),
            FACTURE_PROJECT_EMAIL,
        )
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_EMAIL}"))
    }
}

fn email_check(mail_str: Option<String>, field_name: &str) -> Result<String, String> {
    if let Some(mail_str) = mail_str {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if !email_regex.is_match(&mail_str) {
            return Err(format!(
                "{field_name} value must have format like `project@gmail.com`"
            ));
        }
        Ok(mail_str)
    } else {
        Err(format!("Failed to get {field_name}"))
    }
}

fn resolve_project_tel_form(form_data: &FormData) -> Result<String, String> {
    let project_tel_str = form_data.get(FACTURE_PROJECT_TEL).as_string();
    not_empty_check(project_tel_str, FACTURE_PROJECT_TEL)
}

fn resolve_project_tel_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let project_tel_str = queries.get(FACTURE_PROJECT_TEL);
    if let Some(project_tel_str) = project_tel_str {
        not_empty_check(
            Option::from(project_tel_str.to_string()),
            FACTURE_PROJECT_TEL,
        )
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_TEL}"))
    }
}

fn resolve_project_website_form(form_data: &FormData) -> Result<String, String> {
    let project_website_str = form_data.get(FACTURE_PROJECT_WEBSITE).as_string();
    project_website_str.ok_or(format!("Failed to get {FACTURE_PROJECT_WEBSITE}"))
}

fn resolve_project_website_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let project_website_str = queries.get(FACTURE_PROJECT_WEBSITE);
    if let Some(project_website_str) = project_website_str {
        Ok(project_website_str.to_string())
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_WEBSITE}"))
    }
}

fn resolve_client_name_form(form_data: &FormData) -> Result<String, String> {
    let client_name_str = form_data.get(FACTURE_CLIENT_NAME).as_string();
    not_empty_check(client_name_str, FACTURE_CLIENT_NAME)
}

fn resolve_client_name_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let client_name_str = queries.get(FACTURE_CLIENT_NAME);
    if let Some(client_name_str) = client_name_str {
        not_empty_check(
            Option::from(client_name_str.to_string()),
            FACTURE_CLIENT_NAME,
        )
    } else {
        Err(format!("Failed to get {FACTURE_CLIENT_NAME}"))
    }
}

fn resolve_client_adress_form(form_data: &FormData) -> Result<String, String> {
    let client_adress_str = form_data.get(FACTURE_CLIENT_ADRESS).as_string();
    not_empty_check(client_adress_str, FACTURE_CLIENT_ADRESS)
}

fn resolve_client_adress_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let client_adress_str = queries.get(FACTURE_CLIENT_ADRESS);
    if let Some(client_adress_str) = client_adress_str {
        not_empty_check(
            Option::from(client_adress_str.to_string()),
            FACTURE_CLIENT_ADRESS,
        )
    } else {
        Err(format!("Failed to get {FACTURE_CLIENT_ADRESS}"))
    }
}

fn resolve_client_tel_form(form_data: &FormData) -> Result<String, String> {
    let client_tel_str = form_data.get(FACTURE_CLIENT_TEL).as_string();
    not_empty_check(client_tel_str, FACTURE_CLIENT_TEL)
}

fn resolve_client_tel_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let client_tel_str = queries.get(FACTURE_CLIENT_TEL);
    if let Some(client_tel_str) = client_tel_str {
        not_empty_check(Option::from(client_tel_str.to_string()), FACTURE_CLIENT_TEL)
    } else {
        Err(format!("Failed to get {FACTURE_CLIENT_TEL}"))
    }
}

fn resolve_client_tva_form(form_data: &FormData) -> Result<String, String> {
    let client_tva_str = form_data
        .get(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE)
        .as_string();
    not_empty_check(client_tva_str, FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE)
}

fn resolve_client_tva_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let client_tva_str = queries.get(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE);
    if let Some(client_tva_str) = client_tva_str {
        not_empty_check(
            Option::from(client_tva_str.to_string()),
            FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE,
        )
    } else {
        Err(format!(
            "Failed to get {FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE}"
        ))
    }
}

fn resolve_client_devis_form(form_data: &FormData) -> Result<String, String> {
    let client_devis_str = form_data.get(FACTURE_CLIENT_DEVIS).as_string();
    not_empty_check(client_devis_str, FACTURE_CLIENT_DEVIS)
}

fn resolve_client_devis_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let client_devis_str = queries.get(FACTURE_CLIENT_DEVIS);
    if let Some(client_devis_str) = client_devis_str {
        not_empty_check(
            Option::from(client_devis_str.to_string()),
            FACTURE_CLIENT_DEVIS,
        )
    } else {
        Err(format!("Failed to get {FACTURE_CLIENT_DEVIS}"))
    }
}

fn resolve_services_form(form_data: &FormData) -> Result<HashMap<String, f64>, String> {
    let services_names = form_data.get_all(FACTURE_SERVICE);
    let services_amounts = form_data.get_all(FACTURE_SERVICE_AMOUNT);
    if services_names.length() == services_amounts.length() {
        let mut services_map = HashMap::<String, f64>::new();
        let names = services_names.iter();
        let mut amounts = services_amounts.iter();
        for name in names {
            let amount = amounts
                .next()
                .unwrap()
                .as_string()
                .unwrap()
                .parse::<f64>()
                .unwrap();
            services_map.insert(name.as_string().unwrap(), amount);
        }
        Ok(services_map)
    } else {
        Err(format!(
            "Failed to get {FACTURE_SERVICE} and {FACTURE_SERVICE_AMOUNT}"
        ))
    }
}

fn resolve_services_queries(
    queries: &HashMap<String, String>,
) -> Result<HashMap<String, f64>, String> {
    Ok(serde_json::from_str(queries.get(FACTURE_SERVICES).unwrap()).unwrap())
}

fn resolve_project_bank_form(form_data: &FormData) -> Result<String, String> {
    let bank_str = form_data.get(FACTURE_PROJECT_BANK).as_string();
    not_empty_check(bank_str, FACTURE_PROJECT_BANK)
}

fn resolve_project_bank_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let bank_str = queries.get(FACTURE_PROJECT_BANK);
    if let Some(bank_str) = bank_str {
        not_empty_check(Option::from(bank_str.to_string()), FACTURE_PROJECT_BANK)
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_BANK}"))
    }
}

fn resolve_project_iban_form(form_data: &FormData) -> Result<String, String> {
    let iban_str = form_data.get(FACTURE_PROJECT_IBAN).as_string();
    not_empty_check(iban_str, FACTURE_PROJECT_IBAN)
}

fn resolve_project_iban_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let iban_str = queries.get(FACTURE_PROJECT_IBAN);
    if let Some(iban_str) = iban_str {
        not_empty_check(Option::from(iban_str.to_string()), FACTURE_PROJECT_IBAN)
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_IBAN}"))
    }
}

fn resolve_project_bic_form(form_data: &FormData) -> Result<String, String> {
    let bic_str = form_data.get(FACTURE_PROJECT_BIC).as_string();
    not_empty_check(bic_str, FACTURE_PROJECT_BIC)
}

fn resolve_project_bic_queries(queries: &HashMap<String, String>) -> Result<String, String> {
    let bic_str = queries.get(FACTURE_PROJECT_BIC);
    if let Some(bic_str) = bic_str {
        not_empty_check(Option::from(bic_str.to_string()), FACTURE_PROJECT_BIC)
    } else {
        Err(format!("Failed to get {FACTURE_PROJECT_BIC}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn resolve_facture_dto_from_form_should_return_dto() {
        let form = empty_form();
        form.append_with_str(FACTURE_NUMBER_QUERY, "123").unwrap();
        form.append_with_str(FACTURE_DATE_QUERY, "22/02/2022")
            .unwrap();
        form.append_with_str(FACTURE_DATE_EMITED_QUERY, "23/02/2022")
            .unwrap();
        form.append_with_str(FACTURE_ACOMPTE, "comptant").unwrap();
        form.append_with_str(FACTURE_IS_PAID, "on").unwrap();
        form.append_with_str(FACTURE_PROJECT_NAME, "a project")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_ADRESS, "a project adress")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_EMAIL, "project@gmail.com")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_TEL, "0487452734")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_WEBSITE, "project.com")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_NAME, "a client")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_ADRESS, "a client adress")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_TEL, "0487452734")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE, "00000000")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_DEVIS, "DEV01").unwrap();
        form.append_with_str(FACTURE_PROJECT_BANK, "HelloBank!")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_IBAN, "FR76 0000 0000 0000 0000 0000 000 !")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_BIC, "BNPAFRPPXXX")
            .unwrap();
        form.append_with_str(FACTURE_SERVICE, "un service").unwrap();
        form.append_with_str(FACTURE_SERVICE_AMOUNT, "40.12")
            .unwrap();
        let mut services_map = HashMap::<String, f64>::new();
        services_map.insert("un service".to_string(), 40.12);
        let expected_dto = FactureDto {
            number: 123,
            date: NaiveDate::from_ymd_opt(2022, 02, 22).unwrap(),
            date_emited: NaiveDate::from_ymd_opt(2022, 02, 23).unwrap(),
            services: services_map,
            acompte: "comptant".to_string(),
            is_paid: true,
            project_name: "a project".to_string(),
            project_adress: "a project adress".to_string(),
            project_mail: "project@gmail.com".to_string(),
            project_tel: "0487452734".to_string(),
            project_website: "project.com".to_string(),
            client_name: "a client".to_string(),
            client_adress: "a client adress".to_string(),
            client_tel: "0487452734".to_string(),
            client_tva: "00000000".to_string(),
            client_devis: "DEV01".to_string(),
            project_bank: "HelloBank!".to_string(),
            project_iban: "FR76 0000 0000 0000 0000 0000 000 !".to_string(),
            project_bic: "BNPAFRPPXXX".to_string(),
        };
        assert_eq!(FactureDto::from_form_data(&form), Ok(expected_dto));
    }

    #[wasm_bindgen_test]
    fn resolve_facture_dto_from_form_should_return_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_NUMBER_QUERY, "abc").unwrap();
        form.append_with_str(FACTURE_DATE_QUERY, "22/02/2022")
            .unwrap();
        form.append_with_str(FACTURE_DATE_EMITED_QUERY, "23/02/2022")
            .unwrap();
        form.append_with_str(FACTURE_ACOMPTE, "comptant").unwrap();
        form.append_with_str(FACTURE_IS_PAID, "on").unwrap();
        form.append_with_str(FACTURE_PROJECT_NAME, "a project")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_ADRESS, "a project adress")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_EMAIL, "project@gmail.com")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_TEL, "0487452734")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_WEBSITE, "project.com")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_NAME, "a client")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_ADRESS, "a client adress")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_TEL, "0487452734")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE, "00000000")
            .unwrap();
        form.append_with_str(FACTURE_CLIENT_DEVIS, "DEV01").unwrap();
        form.append_with_str(FACTURE_PROJECT_BANK, "HelloBank!")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_IBAN, "FR76 0000 0000 0000 0000 0000 000 !")
            .unwrap();
        form.append_with_str(FACTURE_PROJECT_BIC, "BNPAFRPPXXX")
            .unwrap();
        assert_eq!(
            FactureDto::from_form_data(&form),
            Err("Failed to parse abc into usize".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_number_form_should_return_number() {
        let form = empty_form();
        form.append_with_str(FACTURE_NUMBER_QUERY, "123").unwrap();
        assert_eq!(resolve_number_form(&form), Ok(123));
    }

    #[wasm_bindgen_test]
    fn resolve_number_form_should_return_get_error_when_using_wrong_key() {
        let form = empty_form();
        form.append_with_str("wrong_key", "123").unwrap();
        assert_eq!(
            resolve_number_form(&form),
            Err("Failed to get factureNumber".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_number_form_should_return_parse_error_when_parsing_string() {
        let form = empty_form();
        form.append_with_str(FACTURE_NUMBER_QUERY, "abc").unwrap();
        assert_eq!(
            resolve_number_form(&form),
            Err("Failed to parse abc into usize".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_number_form_should_return_parse_error_when_parsing_float() {
        let form = empty_form();
        form.append_with_str(FACTURE_NUMBER_QUERY, "12.43").unwrap();
        assert_eq!(
            resolve_number_form(&form),
            Err("Failed to parse 12.43 into usize".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_number_queries_should_return_number() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_NUMBER_QUERY.to_string(), "123".to_string());
        assert_eq!(resolve_number_queries(&queries), Ok(123));
    }

    #[wasm_bindgen_test]
    fn resolve_number_queries_should_return_get_error_when_using_wrong_key() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "123".to_string());
        assert_eq!(
            resolve_number_queries(&queries),
            Err("Failed to get factureNumber".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_number_queries_should_return_parse_error_when_parsing_string() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_NUMBER_QUERY.to_string(), "abc".to_string());
        assert_eq!(
            resolve_number_queries(&queries),
            Err("Failed to parse abc into usize".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_number_queries_should_return_parse_error_when_parsing_float() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_NUMBER_QUERY.to_string(), "12.54".to_string());
        assert_eq!(
            resolve_number_queries(&queries),
            Err("Failed to parse 12.54 into usize".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_date_form_should_return_naive_date() {
        let form = empty_form();
        form.append_with_str(FACTURE_DATE_QUERY, "22/01/2022")
            .unwrap();
        assert_eq!(
            resolve_date_form(&form, FACTURE_DATE_QUERY),
            Ok(NaiveDate::from_ymd_opt(2022, 01, 22).unwrap())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_date_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_DATE_QUERY, "22/01/2022")
            .unwrap();
        assert_eq!(
            resolve_date_form(&form, "wrong_key"),
            Err("Failed to get wrong_key".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_date_form_should_return_parse_error_when_wrong_date_format() {
        let form = empty_form();
        form.append_with_str(FACTURE_DATE_QUERY, "22/012/2022")
            .unwrap();
        assert_eq!(
            resolve_date_form(&form, FACTURE_DATE_QUERY),
            Err("Could not parse date from string 22/012/2022".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_date_queries_should_return_naive_date() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_DATE_QUERY.to_string(), "22/01/2022".to_string());
        assert_eq!(
            resolve_date_queries(&queries, FACTURE_DATE_QUERY),
            Ok(NaiveDate::from_ymd_opt(2022, 01, 22).unwrap())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_date_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_DATE_QUERY.to_string(), "22/01/2022".to_string());
        assert_eq!(
            resolve_date_queries(&queries, "wrong_key"),
            Err("Failed to get wrong_key".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_date_queries_should_return_parse_error_when_wrong_date_format() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_DATE_QUERY.to_string(), "22/012/2022".to_string());
        assert_eq!(
            resolve_date_queries(&queries, FACTURE_DATE_QUERY),
            Err("Could not parse date from string 22/012/2022".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_acompte_form_should_return_acompte() {
        let form = empty_form();
        form.append_with_str(FACTURE_ACOMPTE, "comptant").unwrap();
        assert_eq!(resolve_acompte_form(&form), Ok("comptant".to_string()));
    }

    #[wasm_bindgen_test]
    fn resolve_acompte_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "comptant").unwrap();
        assert_eq!(
            resolve_acompte_form(&form),
            Err("Failed to get factureAcompte".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_acompte_form_should_return_error_when_string_is_not_comptant_or_avec_acompte() {
        let form = empty_form();
        form.append_with_str(FACTURE_ACOMPTE, "a string").unwrap();
        assert_eq!(
            resolve_acompte_form(&form),
            Err("Value must be either `acompte` or `avec acompte`".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_acompte_queries_should_return_acompte() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_ACOMPTE.to_string(), "avec acompte".to_string());
        assert_eq!(
            resolve_acompte_queries(&queries),
            Ok("avec acompte".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_acompte_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "comptant".to_string());
        assert_eq!(
            resolve_acompte_queries(&queries),
            Err("Failed to get factureAcompte".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_acompte_queries_should_return_error_when_string_is_not_comptant_or_avec_acompte() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_ACOMPTE.to_string(), "a string".to_string());
        assert_eq!(
            resolve_acompte_queries(&queries),
            Err("Value must be either `acompte` or `avec acompte`".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_is_paid_form_should_return_true() {
        let form = empty_form();
        form.append_with_str(FACTURE_IS_PAID, "on").unwrap();
        assert_eq!(resolve_is_paid_form(&form), Ok(true));
    }

    #[wasm_bindgen_test]
    fn resolve_is_paid_form_should_return_false() {
        let form = empty_form();
        assert_eq!(resolve_is_paid_form(&form), Ok(false));
    }

    #[wasm_bindgen_test]
    fn resolve_is_paid_queries_should_return_true() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_IS_PAID.to_string(), "true".to_string());
        assert_eq!(resolve_is_paid_queries(&queries), Ok(true));
    }

    #[wasm_bindgen_test]
    fn resolve_is_paid_queries_should_return_false() {
        let queries = empty_queries();
        assert_eq!(resolve_is_paid_queries(&queries), Ok(false));
    }

    #[wasm_bindgen_test]
    fn resolve_project_name_form_should_return_name() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_NAME, "A project")
            .unwrap();
        assert_eq!(
            resolve_project_name_form(&form),
            Ok("A project".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_name_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "A project")
            .expect("Failed to add project name in form");
        assert_eq!(
            resolve_project_name_form(&form),
            Err("Failed to get factureProjectName".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_name_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_NAME, "").unwrap();
        assert_eq!(
            resolve_project_name_form(&form),
            Err("factureProjectName value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_name_queries_should_return_name() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_NAME.to_string(), "A project".to_string());
        assert_eq!(
            resolve_project_name_queries(&queries),
            Ok("A project".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_name_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "A project".to_string());
        assert_eq!(
            resolve_project_name_queries(&queries),
            Err("Failed to get factureProjectName".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_name_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_NAME.to_string(), "".to_string());
        assert_eq!(
            resolve_project_name_queries(&queries),
            Err("factureProjectName value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_adress_form_should_return_adress() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_ADRESS, "An adress")
            .unwrap();
        assert_eq!(
            resolve_project_adress_form(&form),
            Ok("An adress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_adress_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "An adress").unwrap();
        assert_eq!(
            resolve_project_adress_form(&form),
            Err("Failed to get factureProjectAdress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_adress_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_ADRESS, "").unwrap();
        assert_eq!(
            resolve_project_adress_form(&form),
            Err("factureProjectAdress value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_adress_queries_should_return_adress() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_ADRESS.to_string(), "An adress".to_string());
        assert_eq!(
            resolve_project_adress_queries(&queries),
            Ok("An adress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_adress_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "An adress".to_string());
        assert_eq!(
            resolve_project_adress_queries(&queries),
            Err("Failed to get factureProjectAdress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_adress_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_ADRESS.to_string(), "".to_string());
        assert_eq!(
            resolve_project_adress_queries(&queries),
            Err("factureProjectAdress value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_mail_form_should_return_mail() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_EMAIL, "project@gmail.com")
            .unwrap();
        assert_eq!(
            resolve_project_mail_form(&form),
            Ok("project@gmail.com".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_mail_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "project@gmail.com")
            .unwrap();
        assert_eq!(
            resolve_project_mail_form(&form),
            Err("Failed to get factureProjectEmail".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_mail_form_should_return_regex_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_EMAIL, "project@gmail.a")
            .unwrap();
        assert_eq!(
            resolve_project_mail_form(&form),
            Err("factureProjectEmail value must have format like `project@gmail.com`".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_mail_queries_should_return_mail() {
        let mut queries = empty_queries();
        queries.insert(
            FACTURE_PROJECT_EMAIL.to_string(),
            "project@gmail.com".to_string(),
        );
        assert_eq!(
            resolve_project_mail_queries(&queries),
            Ok("project@gmail.com".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_mail_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "project@gmail.com".to_string());
        assert_eq!(
            resolve_project_mail_queries(&queries),
            Err("Failed to get factureProjectEmail".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_mail_queries_should_return_regex_error() {
        let mut queries = empty_queries();
        queries.insert(
            FACTURE_PROJECT_EMAIL.to_string(),
            "project@gmail.a".to_string(),
        );
        assert_eq!(
            resolve_project_mail_queries(&queries),
            Err("factureProjectEmail value must have format like `project@gmail.com`".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_tel_form_should_return_tel() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_TEL, "0467873289")
            .unwrap();
        assert_eq!(
            resolve_project_tel_form(&form),
            Ok("0467873289".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_tel_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "0467873289").unwrap();
        assert_eq!(
            resolve_project_tel_form(&form),
            Err("Failed to get factureProjectTel".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_tel_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_TEL, "").unwrap();
        assert_eq!(
            resolve_project_tel_form(&form),
            Err("factureProjectTel value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_tel_queries_should_return_tel() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_TEL.to_string(), "0467873289".to_string());
        assert_eq!(
            resolve_project_tel_queries(&queries),
            Ok("0467873289".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_tel_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "0467873289".to_string());
        assert_eq!(
            resolve_project_tel_queries(&queries),
            Err("Failed to get factureProjectTel".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_tel_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_TEL.to_string(), "".to_string());
        assert_eq!(
            resolve_project_tel_queries(&queries),
            Err("factureProjectTel value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_website_form_should_return_website() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_WEBSITE, "project.com")
            .unwrap();
        assert_eq!(
            resolve_project_website_form(&form),
            Ok("project.com".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_website_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "project.com").unwrap();
        assert_eq!(
            resolve_project_website_form(&form),
            Err("Failed to get factureProjectWebsite".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_website_queries_should_return_website() {
        let mut queries = empty_queries();
        queries.insert(
            FACTURE_PROJECT_WEBSITE.to_string(),
            "project.com".to_string(),
        );
        assert_eq!(
            resolve_project_website_queries(&queries),
            Ok("project.com".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_website_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "project.com".to_string());
        assert_eq!(
            resolve_project_website_queries(&queries),
            Err("Failed to get factureProjectWebsite".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_name_form_should_return_name() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_NAME, "A client")
            .unwrap();
        assert_eq!(resolve_client_name_form(&form), Ok("A client".to_string()));
    }

    #[wasm_bindgen_test]
    fn resolve_client_name_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "A client").unwrap();
        assert_eq!(
            resolve_client_name_form(&form),
            Err("Failed to get factureClientName".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_name_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_NAME, "").unwrap();
        assert_eq!(
            resolve_client_name_form(&form),
            Err("factureClientName value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_name_queries_should_return_name() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_NAME.to_string(), "A client".to_string());
        assert_eq!(
            resolve_client_name_queries(&queries),
            Ok("A client".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_name_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "A client".to_string());
        assert_eq!(
            resolve_client_name_queries(&queries),
            Err("Failed to get factureClientName".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_name_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_NAME.to_string(), "".to_string());
        assert_eq!(
            resolve_client_name_queries(&queries),
            Err("factureClientName value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_adress_form_should_return_adress() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_ADRESS, "An adress")
            .unwrap();
        assert_eq!(
            resolve_client_adress_form(&form),
            Ok("An adress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_adress_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "An adress")
            .expect("Failed to add client adress in form");
        assert_eq!(
            resolve_client_adress_form(&form),
            Err("Failed to get factureClientAdress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_adress_form_should_return_get_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_ADRESS, "").unwrap();
        assert_eq!(
            resolve_client_adress_form(&form),
            Err("factureClientAdress value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_adress_queries_should_return_adress() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_ADRESS.to_string(), "An adress".to_string());
        assert_eq!(
            resolve_client_adress_queries(&queries),
            Ok("An adress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_adress_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "An adress".to_string());
        assert_eq!(
            resolve_client_adress_queries(&queries),
            Err("Failed to get factureClientAdress".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_adress_queries_should_return_get_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_ADRESS.to_string(), "".to_string());
        assert_eq!(
            resolve_client_adress_queries(&queries),
            Err("factureClientAdress value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tel_form_should_return_tel() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_TEL, "0467874532")
            .unwrap();
        assert_eq!(resolve_client_tel_form(&form), Ok("0467874532".to_string()));
    }

    #[wasm_bindgen_test]
    fn resolve_client_tel_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "0467874532").unwrap();
        assert_eq!(
            resolve_client_tel_form(&form),
            Err("Failed to get factureClientTel".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tel_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_TEL, "").unwrap();
        assert_eq!(
            resolve_client_tel_form(&form),
            Err("factureClientTel value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tel_queries_should_return_tel() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_TEL.to_string(), "0467874532".to_string());
        assert_eq!(
            resolve_client_tel_queries(&queries),
            Ok("0467874532".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tel_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "0467874532".to_string());
        assert_eq!(
            resolve_client_tel_queries(&queries),
            Err("Failed to get factureClientTel".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tel_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_TEL.to_string(), "".to_string());
        assert_eq!(
            resolve_client_tel_queries(&queries),
            Err("factureClientTel value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tva_form_should_return_tva() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE, "00000000")
            .unwrap();
        assert_eq!(resolve_client_tva_form(&form), Ok("00000000".to_string()));
    }

    #[wasm_bindgen_test]
    fn resolve_client_tva_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "00000000").unwrap();
        assert_eq!(
            resolve_client_tva_form(&form),
            Err("Failed to get factureClientTvaIntracommunautaire".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tva_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE, "")
            .unwrap();
        assert_eq!(
            resolve_client_tva_form(&form),
            Err("factureClientTvaIntracommunautaire value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tva_queries_should_return_tva() {
        let mut queries = empty_queries();
        queries.insert(
            FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE.to_string(),
            "00000000".to_string(),
        );
        assert_eq!(
            resolve_client_tva_queries(&queries),
            Ok("00000000".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tva_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "00000000".to_string());
        assert_eq!(
            resolve_client_tva_queries(&queries),
            Err("Failed to get factureClientTvaIntracommunautaire".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_tva_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(
            FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE.to_string(),
            "".to_string(),
        );
        assert_eq!(
            resolve_client_tva_queries(&queries),
            Err("factureClientTvaIntracommunautaire value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_devis_form_should_return_devis() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_DEVIS, "DEV01").unwrap();
        assert_eq!(resolve_client_devis_form(&form), Ok("DEV01".to_string()));
    }

    #[wasm_bindgen_test]
    fn resolve_client_devis_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "DEV01").unwrap();
        assert_eq!(
            resolve_client_devis_form(&form),
            Err("Failed to get factureClientDevis".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_devis_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_CLIENT_DEVIS, "").unwrap();
        assert_eq!(
            resolve_client_devis_form(&form),
            Err("factureClientDevis value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_devis_queries_should_return_devis() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_DEVIS.to_string(), "DEV01".to_string());
        assert_eq!(
            resolve_client_devis_queries(&queries),
            Ok("DEV01".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_devis_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "DEV01".to_string());
        assert_eq!(
            resolve_client_devis_queries(&queries),
            Err("Failed to get factureClientDevis".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_client_devis_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_CLIENT_DEVIS.to_string(), "".to_string());
        assert_eq!(
            resolve_client_devis_queries(&queries),
            Err("factureClientDevis value can't be null".to_string())
        );
    }

    // TODO: tests on services

    #[wasm_bindgen_test]
    fn resolve_project_bank_form_should_return_bank() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_BANK, "HelloBank!")
            .unwrap();
        assert_eq!(
            resolve_project_bank_form(&form),
            Ok("HelloBank!".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bank_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "HelloBank!").unwrap();
        assert_eq!(
            resolve_project_bank_form(&form),
            Err("Failed to get factureProjectBank".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bank_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_BANK, "").unwrap();
        assert_eq!(
            resolve_project_bank_form(&form),
            Err("factureProjectBank value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bank_queries_should_return_bank() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_BANK.to_string(), "HelloBank!".to_string());
        assert_eq!(
            resolve_project_bank_queries(&queries),
            Ok("HelloBank!".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bank_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "HelloBank!".to_string());
        assert_eq!(
            resolve_project_bank_queries(&queries),
            Err("Failed to get factureProjectBank".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bank_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_BANK.to_string(), "".to_string());
        assert_eq!(
            resolve_project_bank_queries(&queries),
            Err("factureProjectBank value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_iban_form_should_return_iban() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_IBAN, "FR76 0000 0000 0000 0000 0000 000 !")
            .unwrap();
        assert_eq!(
            resolve_project_iban_form(&form),
            Ok("FR76 0000 0000 0000 0000 0000 000 !".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_iban_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "FR76 0000 0000 0000 0000 0000 000 !")
            .unwrap();
        assert_eq!(
            resolve_project_iban_form(&form),
            Err("Failed to get factureProjectIban".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_iban_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_IBAN, "").unwrap();
        assert_eq!(
            resolve_project_iban_form(&form),
            Err("factureProjectIban value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_iban_queries_should_return_iban() {
        let mut queries = empty_queries();
        queries.insert(
            FACTURE_PROJECT_IBAN.to_string(),
            "FR76 0000 0000 0000 0000 0000 000 !".to_string(),
        );
        assert_eq!(
            resolve_project_iban_queries(&queries),
            Ok("FR76 0000 0000 0000 0000 0000 000 !".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_iban_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert(
            "wrong_key".to_string(),
            "FR76 0000 0000 0000 0000 0000 000 !".to_string(),
        );
        assert_eq!(
            resolve_project_iban_queries(&queries),
            Err("Failed to get factureProjectIban".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_iban_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_IBAN.to_string(), "".to_string());
        assert_eq!(
            resolve_project_iban_queries(&queries),
            Err("factureProjectIban value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bic_form_should_return_bic() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_BIC, "BNPAFRPPXXX")
            .unwrap();
        assert_eq!(
            resolve_project_bic_form(&form),
            Ok("BNPAFRPPXXX".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bic_form_should_return_get_error() {
        let form = empty_form();
        form.append_with_str("wrong_key", "BNPAFRPPXXX").unwrap();
        assert_eq!(
            resolve_project_bic_form(&form),
            Err("Failed to get factureProjectBic".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bic_form_should_return_empty_error() {
        let form = empty_form();
        form.append_with_str(FACTURE_PROJECT_BIC, "").unwrap();
        assert_eq!(
            resolve_project_bic_form(&form),
            Err("factureProjectBic value can't be null".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bic_queries_should_return_bic() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_BIC.to_string(), "BNPAFRPPXXX".to_string());
        assert_eq!(
            resolve_project_bic_queries(&queries),
            Ok("BNPAFRPPXXX".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bic_queries_should_return_get_error() {
        let mut queries = empty_queries();
        queries.insert("wrong_key".to_string(), "BNPAFRPPXXX".to_string());
        assert_eq!(
            resolve_project_bic_queries(&queries),
            Err("Failed to get factureProjectBic".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn resolve_project_bic_queries_should_return_empty_error() {
        let mut queries = empty_queries();
        queries.insert(FACTURE_PROJECT_BIC.to_string(), "".to_string());
        assert_eq!(
            resolve_project_bic_queries(&queries),
            Err("factureProjectBic value can't be null".to_string())
        );
    }

    fn empty_form() -> FormData {
        FormData::new().unwrap()
    }

    fn empty_queries() -> HashMap<String, String> {
        HashMap::<String, String>::new()
    }
}
