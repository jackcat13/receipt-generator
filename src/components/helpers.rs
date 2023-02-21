use super::social_media_block_component::{SocialMediaBlock, SocialMediaEnum};

pub fn youtube_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Youtube,
        social_media_url: "".to_string(),
    }
}

pub fn github_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Github,
        social_media_url: "https://github.com/jackcat13".to_string(),
    }
}

pub fn twitter_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Twitter,
        social_media_url: "".to_string(),
    }
}

pub fn facebook_link() -> SocialMediaBlock {
    SocialMediaBlock {
        social_media_class: SocialMediaEnum::Facebook,
        social_media_url: "".to_string(),
    }
}
