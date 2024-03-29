use chrono::Local;
use gloo::utils::document;
use web_sys::{Element, FormData, HtmlFormElement, Node, SubmitEvent};
use yew::{function_component, html, use_state, Html, TargetCast};
use yew_router::prelude::use_navigator;

use crate::{
    components::{constants::*, helpers::may_resolve_local_field},
    model::facture_dto::FactureDto,
    Route,
};

const NOM_SERVICE: &str = "Nom du service";

#[function_component(ReceiptGeneratorComponent)]
pub fn receipt_generator_component() -> Html {
    let navigator = use_navigator().expect("Failed to use location");
    let facture_error_handle = use_state(|| "".to_string());
    let facture_error = (*facture_error_handle).clone();
    let handle_add_service = move |_| {
        let element = document().get_element_by_id(FACTURE_FORM);
        add_service_in_form(element);
    };
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
            <form id={FACTURE_FORM} onsubmit={handle_submit}>
                <div><label for={FACTURE_LOGO_URL}>{"Url du logo"}</label></div>
                <div><input id={FACTURE_LOGO_URL} name={FACTURE_LOGO_URL} value={may_resolve_local_field(FACTURE_LOGO_URL)}/></div>
                <div><label for={FACTURE_NUMBER_QUERY}>{"Numéro de facture"}</label></div>
                <div><input id={FACTURE_NUMBER_QUERY} name={FACTURE_NUMBER_QUERY} value={may_resolve_local_field(FACTURE_NUMBER_QUERY)}/></div>
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
                <div><input id={FACTURE_PROJECT_NAME} name={FACTURE_PROJECT_NAME} value={may_resolve_local_field(FACTURE_PROJECT_NAME)} /></div>
                <div><label for={FACTURE_PROJECT_ADRESS}>{"Adresse de facturation"}</label></div>
                <div><textarea id={FACTURE_PROJECT_ADRESS} name={FACTURE_PROJECT_ADRESS} value={may_resolve_local_field(FACTURE_PROJECT_ADRESS)} /></div>
                <div><label for={FACTURE_PROJECT_EMAIL}>{"Email du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_EMAIL} name={FACTURE_PROJECT_EMAIL} value={may_resolve_local_field(FACTURE_PROJECT_EMAIL)} placeholder="de la forme project@site.com"/></div>
                <div><label for={FACTURE_PROJECT_TEL}>{"Tel du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_TEL} name={FACTURE_PROJECT_TEL} value={may_resolve_local_field(FACTURE_PROJECT_TEL)} /></div>
                <div><label for={FACTURE_PROJECT_WEBSITE}>{"Site web du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_WEBSITE} name={FACTURE_PROJECT_WEBSITE} value={may_resolve_local_field(FACTURE_PROJECT_WEBSITE)} /></div>
                <div><label for={FACTURE_CLIENT_NAME}>{"Nom du client/de l'entreprise"}</label></div>
                <div><input id={FACTURE_CLIENT_NAME} name={FACTURE_CLIENT_NAME} value={may_resolve_local_field(FACTURE_CLIENT_NAME)} /></div>
                <div><label for={FACTURE_CLIENT_ADRESS}>{"Adresse client"}</label></div>
                <div><textarea id={FACTURE_CLIENT_ADRESS} name={FACTURE_CLIENT_ADRESS} value={may_resolve_local_field(FACTURE_CLIENT_ADRESS)} /></div>
                <div><label for={FACTURE_CLIENT_TEL}>{"Tel client"}</label></div>
                <div><input id={FACTURE_CLIENT_TEL} name={FACTURE_CLIENT_TEL} value={may_resolve_local_field(FACTURE_CLIENT_TEL)} /></div>
                <div><label for={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE}>{"Tva intracommunautaire client"}</label></div>
                <div><input id={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE} name={FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE} value={may_resolve_local_field(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE)} /></div>
                <div><label for={FACTURE_CLIENT_DEVIS}>{"Devis client"}</label></div>
                <div><input id={FACTURE_CLIENT_DEVIS} name={FACTURE_CLIENT_DEVIS} value={may_resolve_local_field(FACTURE_CLIENT_DEVIS)} /></div>
                <div><label for={FACTURE_PROJECT_BANK}>{"Banque du project"}</label></div>
                <div><input id={FACTURE_PROJECT_BANK} name={FACTURE_PROJECT_BANK} value={may_resolve_local_field(FACTURE_PROJECT_BANK)} /></div>
                <div><label for={FACTURE_PROJECT_IBAN}>{"Iban du project"}</label></div>
                <div><input id={FACTURE_PROJECT_IBAN} name={FACTURE_PROJECT_IBAN} value={may_resolve_local_field(FACTURE_PROJECT_IBAN)} /></div>
                <div><label for={FACTURE_PROJECT_BIC}>{"Bic du projet"}</label></div>
                <div><input id={FACTURE_PROJECT_BIC} name={FACTURE_PROJECT_BIC} value={may_resolve_local_field(FACTURE_PROJECT_BIC)} /></div>
                <div></div><div></div><div><span onclick={handle_add_service} class="button">{"Ajouter service"}</span></div><div></div><div></div><div></div>
                <div><label for={FACTURE_SERVICE}>{"Nom du service"}</label></div>
                <div><input class={FACTURE_SERVICE} name={FACTURE_SERVICE} /></div>
                <div><label for={FACTURE_SERVICE_AMOUNT}>{MONTANT}</label></div>
                <div><input class={FACTURE_SERVICE_AMOUNT} name={FACTURE_SERVICE_AMOUNT} /></div>
                <div id={FACTURE_GENERATE_BUTTON_DIV}><input type="submit" id={FACTURE_GENERATE_BUTTON} class="button" value="Générer"/></div>
                <div id="factureError">{facture_error.to_string()}</div>
            </form>
        </>
    }
}

fn add_service_in_form(form_element: Option<Element>) {
    if let Some(form_element) = form_element {
        let document = document();
        let generate_button: Node = document
            .get_element_by_id(FACTURE_GENERATE_BUTTON_DIV)
            .unwrap()
            .into();
        let label_service = document.create_element("label").unwrap();
        label_service.set_attribute("for", FACTURE_SERVICE).unwrap();
        label_service.set_inner_html(NOM_SERVICE);
        let input_service = document.create_element("input").unwrap();
        input_service
            .set_attribute("name", FACTURE_SERVICE)
            .unwrap();
        input_service.set_class_name(FACTURE_SERVICE);
        let label_amount = document.create_element("label").unwrap();
        label_amount
            .set_attribute("for", FACTURE_SERVICE_AMOUNT)
            .unwrap();
        label_amount.set_inner_html(MONTANT);
        let input_amount = document.create_element("input").unwrap();
        input_amount
            .set_attribute("name", FACTURE_SERVICE_AMOUNT)
            .unwrap();
        input_amount.set_class_name(FACTURE_SERVICE_AMOUNT);
        form_element
            .insert_before(&label_service, Option::from(&generate_button))
            .unwrap();
        form_element
            .insert_before(&input_service, Option::from(&generate_button))
            .unwrap();
        form_element
            .insert_before(&label_amount, Option::from(&generate_button))
            .unwrap();
        form_element
            .insert_before(&input_amount, Option::from(&generate_button))
            .unwrap();
    }
}
