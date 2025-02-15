pub enum Biome {
    Grass,
    Desert,
    Snow,
}

pub fn get_biome_by_params(temperature: f64, humidity: f64) -> Biome {
    match (humidity, temperature) {
        (h, t) if h < 0.5 && t >= 0.6 => Biome::Desert,
        (h, t) if h >= 0.5 && t >= 0.3 && t < 0.6 => Biome::Grass,
        (h, t) if h >= 0.5 && t < 0.3 => Biome::Snow,
        _ => Biome::Grass,
    }
}
