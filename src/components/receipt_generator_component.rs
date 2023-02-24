use chrono::Local;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
use yew::{function_component, html, Html, TargetCast};
use yew_router::prelude::use_navigator;

use crate::{components::constants::*, model::facture_dto::FactureDto, Route};

#[function_component(ReceiptGeneratorComponent)]
pub fn receipt_generator_component() -> Html {
    let navigator = use_navigator().expect("Failed to use location");
    let handle_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let form_element: HtmlFormElement = event
            .target_dyn_into()
            .expect("Failed to load form element");
        let form_data = FormData::new_with_form(&form_element).expect("Failed to load Form data");
        let facture = FactureDto::from_form_data(&form_data);
        navigator
            .push_with_query(&Route::Facture, &facture.to_queries())
            .expect("Failed to navigate to facture page");
    };
    html! {
        <>
            <form method="get" onsubmit={handle_submit}>
                <div><label for={FACTURE_NUMBER_QUERY}>{"Numéro de facture"}</label>
                <input id={FACTURE_NUMBER_QUERY} name={FACTURE_NUMBER_QUERY} value={"0"} /></div>
                <div><label for={FACTURE_DATE_QUERY}>{"Date de facture"}</label>
                <input id={FACTURE_DATE_QUERY} name={FACTURE_DATE_QUERY} value={Local::now().format("%d/%m/%Y").to_string()}/></div>
                <div><label for={FACTURE_DATE_EMITED_QUERY}>{"Émis le"}</label>
                <input id={FACTURE_DATE_EMITED_QUERY} name={FACTURE_DATE_EMITED_QUERY} value={Local::now().format("%d/%m/%Y").to_string()}/></div>
                <div><label for={FACTURE_ACOMPTE}>{"Acompte?"}</label>
                <select id={FACTURE_ACOMPTE} name={FACTURE_ACOMPTE}>
                    <option value="comptant">{"comptant"}</option>
                    <option value="avec acompte">{"avec acompte"}</option>
                </select></div>
                <div><label for={FACTURE_IS_PAID}>{"Payé?"}</label>
                <input type="checkbox" id={FACTURE_IS_PAID} name={FACTURE_IS_PAID} checked={false} /></div>
                <div><label for={FACTURE_PROJECT_NAME}>{"Nom du projet"}</label>
                <input id={FACTURE_PROJECT_NAME} name={FACTURE_PROJECT_NAME} /></div>
                <div><label for={FACTURE_PROJECT_ADRESS}>{"Adresse de facturation"}</label>
                <textarea id={FACTURE_PROJECT_ADRESS} name={FACTURE_PROJECT_ADRESS} /></div>
                <div><label for={FACTURE_PROJECT_EMAIL}>{"Email du projet"}</label>
                <input id={FACTURE_PROJECT_EMAIL} name={FACTURE_PROJECT_EMAIL} /></div>
                <div><label for={FACTURE_PROJECT_TEL}>{"Tel du projet"}</label>
                <input id={FACTURE_PROJECT_TEL} name={FACTURE_PROJECT_TEL} /></div>
                <div><label for={FACTURE_PROJECT_WEBSITE}>{"Site web du projet"}</label>
                <input id={FACTURE_PROJECT_WEBSITE} name={FACTURE_PROJECT_WEBSITE} /></div>
                <div><label for={FACTURE_CLIENT_NAME}>{"Nom du client/de l'entreprise"}</label>
                <input id={FACTURE_CLIENT_NAME} name={FACTURE_CLIENT_NAME} /></div>
                <div><label for={FACTURE_CLIENT_ADRESS}>{"Adresse client"}</label>
                <textarea id={FACTURE_CLIENT_ADRESS} name={FACTURE_CLIENT_ADRESS} /></div>
                <div><label for={FACTURE_CLIENT_TEL}>{"Tel client"}</label>
                <input id={FACTURE_CLIENT_TEL} name={FACTURE_CLIENT_TEL} /></div>
                <div><label for={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE}>{"Tva intracommunautaire client"}</label>
                <input id={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE} name={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE} /></div>
                <div><label for={FACTURE_CLIENT_DEVIS}>{"Devis client"}</label>
                <input id={FACTURE_CLIENT_DEVIS} name={FACTURE_CLIENT_DEVIS} /></div>
                <div><label for={FACTURE_AMOUNT}>{"Coût de la prestation"}</label>
                <input id={FACTURE_AMOUNT} name={FACTURE_AMOUNT} value={"40"} /></div>
                <div><input type="submit" id="factureGenerateButton" value="Générer"/></div>
            </form>
        </>
    }
}
