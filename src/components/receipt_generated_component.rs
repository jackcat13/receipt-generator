use linked_hash_map::LinkedHashMap;
use yew::{function_component, html, Html};
use yew_router::prelude::use_location;

use crate::model::facture_dto::FactureDto;

use super::{constants::*, helpers::local_storage};

#[function_component(ReceiptGeneratedComponent)]
pub fn receipt_generated_component() -> Html {
    let location = use_location().expect("Could not resolve location");
    let queries: LinkedHashMap<String, String> =
        location.query().expect("Could not retrieve get parameters");
    let facture_result = FactureDto::from_queries(&queries);
    if facture_result.is_err() {
        return html! {
            <>
                <div id="generatedReceipt">
                    {facture_result.err().unwrap()}
                </div>
            </>
        };
    }
    let facture = facture_result.unwrap();
    fill_local_storage(&facture);
    let total_amount: f64 = Vec::from_iter(facture.services.values().cloned())
        .iter()
        .sum();
    html! {
        <>
            <table>
                <thead><th><td><div class="emptyHeader"></div></td></th></thead>
                <tbody><tr><td><div class="content">
                    <div id="generatedReceipt">
                    <img id="logo" src={"https://media.discordapp.net/attachments/1074719812871258122/1078671377005027428/image.png"} />
                    if facture.is_paid {
                        <div id="paid">{"PAYÉ"}</div>
                    }
                    <div id="receiptTopRight">
                        <div><h1 class="factureTitle">{"FACTURE"}</h1></div>
                        <div>{"N° FAC-"}{facture.number}</div>
                        <div>{facture.date}</div>
                        <div>{facture.acompte}</div>
                        <div>{"Émis le "}{facture.date_emited}</div>
                    </div>
                    <div><h1 class="factureTitle">{facture.project_name.clone()}</h1></div>
                    <div>{facture.project_adress}</div>
                    <div>{"Email: "}{facture.project_mail}</div>
                    <div>{"Tel: "}{facture.project_tel}</div>
                    <div>{"Site web: "}{facture.project_website}</div>
                    <div id="clientFacturationRight">
                        <div>{facture.client_name}</div>
                        <div>{facture.client_adress}</div>
                        <div>{"Tel: "}{facture.client_tel}</div>
                        <div>{"TVA intracommunautaire: "}{facture.client_tva}</div>
                        <div>{facture.client_devis}</div>
                    </div>
                    <table>
                        <tr class="greenBackground"><th>{"Désignation"}</th><th>{"Qté."}</th><th>{"Prix unitaire HT"}</th><th>{"Montant HT"}</th></tr>
                        {
                            facture.services.into_iter().map( |(name, amount)| {
                                html!{<tr class="designationLine"><td>{name}</td><td>{"1"}</td><td>{amount}</td><td>{amount}</td></tr>}
                            }).collect::<Html>()
                        }
                    </table>
                    <table id="TotalFactureRight">
                        <tr><td>{"Total HT"}</td><td>{total_amount}</td></tr>
                        <tr class="greenBackground"><td>{"Total TTC"}</td><td>{total_amount}</td></tr>
                    </table>
                    <div id="receiptAboveFooter">
                        <div>{"Total net de TVA"}</div>
                        <div>{"Chèque à l'ordre de "}{facture.project_name}</div>
                        <div>{"Virement bancaire à "}{facture.project_bank}{" : IBAN "}{facture.project_iban}{" - BIC "}{facture.project_bic}{" Paiement sécurisé internet"}</div>
                    </div>
                </div>
                </div></td></tr></tbody>
                <tfoot><div class="emptyFooter"></div></tfoot>
            </table>
            <footer id="receiptFooter">
                <div>{"Bien évidemment, nous restons à votre entière disposition et vous remercions de toute votre confiance."}</div>
                <div>{"Siren : 910551126 · Siret : 91055112600022 · TVA non applicable selon l’article 293B du Code Général des Impôts : FR 15910551126 · Ville + RCS :"}</div>
                <div>{"910551126 RCS Annecy · APE/NAF : 7410Z ·"}</div>
                <div>{"En l’absence de règlement au lendemain de la date de paiement figurant sur la facture, des pénalités de retard au taux de 10.0% par an s'appliquent. De plus, pour tout professionnel et conformément à l'article L. 441-6 du code de commerce, une indemnité de 40,00 € s'ajoute."}</div>
            </footer>
        </>
    }
}

fn fill_local_storage(facture: &FactureDto) {
    let local_storage = local_storage();
    local_storage
        .set_item(FACTURE_NUMBER_QUERY, &facture.number.to_string())
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_NAME, &facture.project_name)
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_ADRESS, &facture.project_adress)
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_EMAIL, &facture.project_mail)
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_TEL, &facture.project_tel)
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_WEBSITE, &facture.project_website)
        .unwrap();
    local_storage
        .set_item(FACTURE_CLIENT_NAME, &facture.client_name)
        .unwrap();
    local_storage
        .set_item(FACTURE_CLIENT_ADRESS, &facture.client_adress)
        .unwrap();
    local_storage
        .set_item(FACTURE_CLIENT_TEL, &facture.client_tel)
        .unwrap();
    local_storage
        .set_item(FACTURE_CLIENT_TVA_INTRACOMMUNAUTAIRE, &facture.client_tva)
        .unwrap();
    local_storage
        .set_item(FACTURE_CLIENT_DEVIS, &facture.client_devis)
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_BANK, &facture.project_bank)
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_IBAN, &facture.project_iban)
        .unwrap();
    local_storage
        .set_item(FACTURE_PROJECT_BIC, &facture.project_bic)
        .unwrap();
}
