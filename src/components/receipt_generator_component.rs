use chrono::Local;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
use yew::{function_component, html, use_state, Html, TargetCast};
use yew_router::prelude::use_navigator;

use crate::{components::constants::*, model::facture_dto::FactureDto, Route};

#[function_component(ReceiptGeneratorComponent)]
pub fn receipt_generator_component() -> Html {
    let navigator = use_navigator().expect("Failed to use location");
    let facture_error_handle = use_state(|| "".to_string());
    let facture_error = (*facture_error_handle).clone();
    let handle_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let form_element: HtmlFormElement = event
            .target_dyn_into()
            .expect("Failed to load form element");
        let form_data = FormData::new_with_form(&form_element).expect("Failed to load Form data");
        let facture = FactureDto::from_form_data(&form_data);
        if let Ok(facture) = facture {
            navigator
                .push_with_query(&Route::Facture, &facture.to_queries())
                .expect("Failed to navigate to facture page");
        } else {
            event.cancel_bubble();
            facture_error_handle.set(facture.err().unwrap());
        }
    };
    html! {
        <>
            <form id="factureForm" onsubmit={handle_submit}>
                <div><label for={FACTURE_NUMBER_QUERY}>{"Numéro de facture"}</label></div>
                <div><input id={FACTURE_NUMBER_QUERY} name={FACTURE_NUMBER_QUERY} value={"0"}/></div>
                <div><label for={FACTURE_DATE_QUERY}>{"Date de facture"}</label></div>
                <div><input id={FACTURE_DATE_QUERY} name={FACTURE_DATE_QUERY} value={Local::now().format("%d/%m/%Y").to_string()}/></div>
                <div><label for={FACTURE_DATE_EMITED_QUERY}>{"Émis le"}</label></div>
                <div><input id={FACTURE_DATE_EMITED_QUERY} name={FACTURE_DATE_EMITED_QUERY} value={Local::now().format("%d/%m/%Y").to_string()}/></div>
                <div><label for={FACTURE_ACOMPTE}>{"Acompte?"}</label></div>
                <div><select id={FACTURE_ACOMPTE} name={FACTURE_ACOMPTE}>
                    <option value="comptant">{"comptant"}</option>
                    <option value="avec acompte">{"avec acompte"}</option>
                </select></div>
                <div><label for={FACTURE_IS_PAID}>{"Payé?"}</label></div>
                <div><input type="checkbox" class="checkbox" id={FACTURE_IS_PAID} name={FACTURE_IS_PAID} checked={false} /></div>
                <div><label for={FACTURE_PROJECT_NAME}>{"Nom du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_NAME} name={FACTURE_PROJECT_NAME} /></div>
                <div><label for={FACTURE_PROJECT_ADRESS}>{"Adresse de facturation"}</label></div>
                <div><textarea id={FACTURE_PROJECT_ADRESS} name={FACTURE_PROJECT_ADRESS} /></div>
                <div><label for={FACTURE_PROJECT_EMAIL}>{"Email du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_EMAIL} name={FACTURE_PROJECT_EMAIL} placeholder="de la forme project@site.com"/></div>
                <div><label for={FACTURE_PROJECT_TEL}>{"Tel du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_TEL} name={FACTURE_PROJECT_TEL} /></div>
                <div><label for={FACTURE_PROJECT_WEBSITE}>{"Site web du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_WEBSITE} name={FACTURE_PROJECT_WEBSITE} /></div>
                <div><label for={FACTURE_CLIENT_NAME}>{"Nom du client/de l'entreprise"}</label></div>
                <div><input id={FACTURE_CLIENT_NAME} name={FACTURE_CLIENT_NAME} /></div>
                <div><label for={FACTURE_CLIENT_ADRESS}>{"Adresse client"}</label></div>
                <div><textarea id={FACTURE_CLIENT_ADRESS} name={FACTURE_CLIENT_ADRESS} /></div>
                <div><label for={FACTURE_CLIENT_TEL}>{"Tel client"}</label></div>
                <div><input id={FACTURE_CLIENT_TEL} name={FACTURE_CLIENT_TEL} /></div>
                <div><label for={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE}>{"Tva intracommunautaire client"}</label></div>
                <div><input id={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE} name={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE} /></div>
                <div><label for={FACTURE_CLIENT_DEVIS}>{"Devis client"}</label></div>
                <div><input id={FACTURE_CLIENT_DEVIS} name={FACTURE_CLIENT_DEVIS} /></div>
                <div><label for={FACTURE_AMOUNT}>{"Coût de la prestation"}</label></div>
                <div><input id={FACTURE_AMOUNT} name={FACTURE_AMOUNT} value={"40.0"} /></div>
                <div><input type="submit" id="factureGenerateButton" class="button" value="Générer"/></div>
                <div id="factureError">{facture_error.to_string()}</div>
            </form>
        </>
    }
}
