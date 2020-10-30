use rosu::Osu;
use rosu::backend::UserRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SigQuery {
    pub name: String,
    #[serde(default="default_color")]
    pub color: String
}

fn default_color() -> String { "#FFF".to_string() }

#[derive(Serialize, Debug, Clone)]
pub struct SigUserData {
    pub name: String,
    pub accuracy: f32,
    pub play_count: u32,
    pub ranking: u32,
    pub level: u16,
    pub country_url: String,
    pub avatar_url: String
}

impl SigUserData {
    async fn for_user(osu: &Osu, name: &String) -> Option<SigUserData> {
        let user = UserRequest::with_username(name)
            .unwrap()
            .queue_single(osu)
            .await
            .unwrap();
        match user {
            Some(user) => {
                Some(SigUserData {
                    name: user.username,
                    accuracy: user.accuracy,
                    play_count: user.playcount,
                    ranking: user.pp_rank,
                    level: user.level as u16,
                    country_url: format!("https://osu.ppy.sh/images/flags/{}.png", user.country),
                    avatar_url: format!("https://a.ppy.sh/{}", user.user_id)
                })
            }
            None => None
        }
    }
}

pub struct OsuManager {
    pub osu: Osu
}

impl OsuManager {
    pub async fn request_user_data(&self, name: &String) -> Option<SigUserData> {
        SigUserData::for_user(&self.osu, name).await
    }
}