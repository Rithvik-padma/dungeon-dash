use super::*;

pub fn fetch(user_id: &str) -> Result<PlayerAchievements, std::io::Error> {
    let filepath = server::paths::player_achievements(&user_id);
    os::client::watch_file(server::PROGRAM_ID, &filepath)
        .data
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "PlayerAchievements unavailable")
        })
        .and_then(|file| PlayerAchievements::try_from_slice(&file.contents))
}