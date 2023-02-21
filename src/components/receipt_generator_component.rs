use chrono::Local;
use yew::{function_component, html, Html};

#[function_component(ReceiptGeneratorComponent)]
pub fn receipt_generator_component() -> Html {
    html! {
        <>
            <div><label for="factureNumber">{"Numéro de facture"}</label>
            <input id="factureNumber" name="factureNumber" /></div>
            <div><label for="factureDate">{"Date de facture"}</label>
            <input id="factureDate" name="factureDate" value={Local::now().format("%d/%m/%Y").to_string()}/></div>
            <div><label for="emitedDate">{"Émis le"}</label>
            <input id="emitedDate" name="emitedDate" value={Local::now().format("%d/%m/%Y").to_string()}/></div>
            <div><label for="prestationAmount">{"Coût de la prestation"}</label>
            <input id="prestationAmount" name="prestationAmount" /></div>
            <div><button id="factureGenerateButton">{"Générer"}</button></div>
        </>
    }
}
