use std::collections::HashMap;

use chrono::NaiveDate;
use web_sys::FormData;

use crate::components::constants::*;

#[derive(Clone)]
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
    pub amount: f64,
}

impl FactureDto {
    pub fn from_form_data(form_data: &FormData) -> FactureDto {
        let form_data = form_data.clone();
        FactureDto {
            number: form_data
                .get(FACTURE_NUMBER_QUERY)
                .as_string()
                .expect("Failed to init facture number")
                .parse()
                .expect("Failed to parse facture number"),
            date: to_naive_date(
                &form_data
                    .get(FACTURE_DATE_QUERY)
                    .as_string()
                    .expect("Failed to init facture date"),
            ),
            date_emited: to_naive_date(
                &form_data
                    .get(FACTURE_DATE_EMITED_QUERY)
                    .as_string()
                    .expect("Failed to init facture date emited"),
            ),
            acompte: form_data
                .get(FACTURE_ACOMPTE)
                .as_string()
                .expect("Failed to init facture acompte"),
            is_paid: form_data.get(FACTURE_IS_PAID).as_string() == Some("on".to_string()),
            project_name: form_data
                .get(FACTURE_PROJECT_NAME)
                .as_string()
                .expect("Failed to init project name"),
            project_adress: form_data
                .get(FACTURE_PROJECT_ADRESS)
                .as_string()
                .expect("Failed to init project adress"),
            project_mail: form_data
                .get(FACTURE_PROJECT_EMAIL)
                .as_string()
                .expect("Failed to init project mail"),
            project_tel: form_data
                .get(FACTURE_PROJECT_TEL)
                .as_string()
                .expect("Failed to init project tel"),
            project_website: form_data
                .get(FACTURE_PROJECT_WEBSITE)
                .as_string()
                .expect("Failed to init project website"),
            client_name: form_data
                .get(FACTURE_CLIENT_NAME)
                .as_string()
                .expect("Failed to init client name"),
            client_adress: form_data
                .get(FACTURE_CLIENT_ADRESS)
                .as_string()
                .expect("Failed to init client adress"),
            client_tel: form_data
                .get(FACTURE_CLIENT_TEL)
                .as_string()
                .expect("Failed to init client tel"),
            client_tva: form_data
                .get(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE)
                .as_string()
                .expect("Failed to init tva client"),
            client_devis: form_data
                .get(FACTURE_CLIENT_DEVIS)
                .as_string()
                .expect("Failed to init client devis"),
            amount: form_data
                .get(FACTURE_AMOUNT)
                .as_string()
                .expect("Failed to init facture amount")
                .parse()
                .expect("Failed to parse amount"),
        }
    }

    pub fn from_queries(queries: &HashMap<String, String>) -> FactureDto {
        let queries = queries.clone();
        FactureDto {
            number: queries
                .get(FACTURE_NUMBER_QUERY)
                .expect("Failed to init facture number")
                .parse()
                .expect("Failed to parse facture number"),
            date: to_naive_date(
                queries
                    .get(FACTURE_DATE_QUERY)
                    .expect("Failed to init facture date"),
            ),
            date_emited: to_naive_date(
                queries
                    .get(FACTURE_DATE_EMITED_QUERY)
                    .expect("Failed to init facture date emited"),
            ),
            acompte: queries
                .get(FACTURE_ACOMPTE)
                .expect("Failed to init facture acompte")
                .to_string(),
            is_paid: queries
                .get(FACTURE_IS_PAID)
                .expect("Failed to init facture is paid")
                .parse::<bool>()
                .expect("Failed to parse facture is paid"),
            project_name: queries
                .get(FACTURE_PROJECT_NAME)
                .expect("Failed to init project name")
                .to_string(),
            project_adress: queries
                .get(FACTURE_PROJECT_ADRESS)
                .expect("Failed to init project adress")
                .to_string(),
            project_mail: queries
                .get(FACTURE_PROJECT_EMAIL)
                .expect("Failed to init project mail")
                .to_string(),
            project_tel: queries
                .get(FACTURE_PROJECT_TEL)
                .expect("Failed to init project tel")
                .to_string(),
            project_website: queries
                .get(FACTURE_PROJECT_WEBSITE)
                .expect("Failed to init project website")
                .to_string(),
            client_name: queries
                .get(FACTURE_CLIENT_NAME)
                .expect("Failed to init client name")
                .to_string(),
            client_adress: queries
                .get(FACTURE_CLIENT_ADRESS)
                .expect("Failed to init client adress")
                .to_string(),
            client_tel: queries
                .get(FACTURE_CLIENT_TEL)
                .expect("Failed to init client tel")
                .to_string(),
            client_tva: queries
                .get(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE)
                .expect("Failed to init tva client")
                .to_string(),
            client_devis: queries
                .get(FACTURE_CLIENT_DEVIS)
                .expect("Failed to init client devis")
                .to_string(),
            amount: queries
                .get(FACTURE_AMOUNT)
                .expect("Failed to init facture amount")
                .to_string()
                .parse()
                .expect("Failed to parse amount"),
        }
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
        queries.insert(FACTURE_AMOUNT.to_string(), facture.amount.to_string());
        queries
    }
}

fn to_naive_date(expect: &str) -> NaiveDate {
    NaiveDate::parse_from_str(expect, DATE_FORMAT).expect("Failed to parse date")
}
