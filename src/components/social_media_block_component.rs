use strum_macros::Display;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SocialMediaEnum {
    Twitter,
    Facebook,
    Github,
    Youtube,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SocialMediaBlock {
    pub social_media_class: SocialMediaEnum,
    pub social_media_url: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SocialMediaBlockElement {
    pub social_media_block: SocialMediaBlock,
}

#[function_component(SocialMediaBlockComponent)]
pub fn breaking_block_component(social_media_block_element: &SocialMediaBlockElement) -> Html {
    let block = social_media_block_element.clone();
    html! {
        <>
            <div class="socialMediaBlockWrapper">
                <div class="socialMediaBlock" style="width: 50px">
                    <a href={ block.social_media_block.social_media_url } target="_blank">
                        <span></span>
                        <span></span>
                        <span></span>
                        <span></span>
                        <span class={ String::from("fa fa-") + &String::from(block.social_media_block.social_media_class.to_string()) }></span>
                    </a>
                </div>
            </div>
        </>
    }
}
