//! Weather domain types as Rust structures: a per-period forecast with units, two language variants.
//! Sources are converted into `Period` (`nws.rs`, `eccc.rs`); the forecast text is just a field.

use serde::{Deserialize, Serialize};

/// Weather language variant: US (°F, mph) or international English (°C, km/h).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Variant {
    Us,
    Intl,
}

impl Variant {
    pub fn name(self) -> &'static str {
        match self {
            Variant::Us => "us",
            Variant::Intl => "intl",
        }
    }
    pub fn temp_unit(self) -> TempUnit {
        match self {
            Variant::Us => TempUnit::F,
            Variant::Intl => TempUnit::C,
        }
    }
    pub fn speed_unit(self) -> SpeedUnit {
        match self {
            Variant::Us => SpeedUnit::Mph,
            Variant::Intl => SpeedUnit::Kmh,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum TempUnit {
    C,
    F,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum SpeedUnit {
    Kmh,
    Mph,
}

/// Temperature in whole degrees.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Temp {
    pub value: i16,
    pub unit: TempUnit,
}

impl Temp {
    /// Conversion with rounding to an integer.
    pub fn to(self, unit: TempUnit) -> Temp {
        let v = self.value as f64;
        let value = match (self.unit, unit) {
            (TempUnit::C, TempUnit::F) => v * 9.0 / 5.0 + 32.0,
            (TempUnit::F, TempUnit::C) => (v - 32.0) * 5.0 / 9.0,
            _ => v,
        };
        Temp { value: value.round() as i16, unit }
    }
}

/// Wind direction: 16 compass points or variable.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Dir {
    N,
    NNE,
    NE,
    ENE,
    E,
    ESE,
    SE,
    SSE,
    S,
    SSW,
    SW,
    WSW,
    W,
    WNW,
    NW,
    NNW,
    Var,
}

impl Dir {
    pub fn parse(s: &str) -> Option<Dir> {
        use Dir::*;
        Some(match s.trim() {
            "N" => N,
            "NNE" => NNE,
            "NE" => NE,
            "ENE" => ENE,
            "E" => E,
            "ESE" => ESE,
            "SE" => SE,
            "SSE" => SSE,
            "S" => S,
            "SSW" => SSW,
            "SW" => SW,
            "WSW" => WSW,
            "W" => W,
            "WNW" => WNW,
            "NW" => NW,
            "NNW" => NNW,
            "VR" | "V" => Var,
            _ => return None,
        })
    }
}

/// Wind: direction, speed (range), gusts.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Wind {
    pub dir: Option<Dir>,
    pub lo: u16,
    pub hi: u16,
    pub gust: Option<u16>,
    pub unit: SpeedUnit,
}

/// Sky (cloud cover).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Default)]
pub enum Sky {
    Clear,
    FewClouds,
    PartlyCloudy,
    MostlyCloudy,
    Overcast,
    Increasing,
    Clearing,
    #[default]
    Unknown,
}

/// Weather phenomenon.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Default)]
pub enum Wx {
    #[default]
    None,
    Showers,
    Rain,
    Drizzle,
    Thunder,
    FreezingRain,
    RainSnow,
    Flurries,
    Snow,
    Blizzard,
    Sleet,
    Fog,
    Haze,
    Smoke,
    Dust,
    Hot,
    Cold,
    Tropical,
}

/// Period condition, shared by both sources: sky, phenomenon, windy.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Default)]
pub struct Cond {
    pub sky: Sky,
    pub wx: Wx,
    pub windy: bool,
}

impl Cond {
    /// From NWS icon codes (`bkn`, `rain_showers,40`, `wind_sct`…): sky from the cloud cover codes,
    /// phenomenon from the code with the highest probability (first one on ties).
    pub fn from_nws(codes: &[(&str, u8)]) -> Cond {
        let mut c = Cond::default();
        let mut best: Option<(Wx, u8)> = None;
        for &(code, pop) in codes {
            let (windy, base) = match code.strip_prefix("wind_") {
                Some(b) => (true, b),
                None => (false, code),
            };
            c.windy |= windy;
            let sky = match base {
                "skc" => Some(Sky::Clear),
                "few" => Some(Sky::FewClouds),
                "sct" => Some(Sky::PartlyCloudy),
                "bkn" => Some(Sky::MostlyCloudy),
                "ovc" => Some(Sky::Overcast),
                _ => None,
            };
            if let Some(s) = sky {
                if c.sky == Sky::Unknown {
                    c.sky = s;
                }
                continue;
            }
            let wx = match base {
                "rain_showers" | "rain_showers_hi" => Wx::Showers,
                "rain" => Wx::Rain,
                "tsra" | "tsra_sct" | "tsra_hi" => Wx::Thunder,
                "snow" => Wx::Snow,
                "rain_snow" => Wx::RainSnow,
                "rain_sleet" | "snow_sleet" | "sleet" => Wx::Sleet,
                "fzra" | "rain_fzra" | "snow_fzra" => Wx::FreezingRain,
                "fog" => Wx::Fog,
                "haze" => Wx::Haze,
                "smoke" => Wx::Smoke,
                "dust" => Wx::Dust,
                "hot" => Wx::Hot,
                "cold" => Wx::Cold,
                "blizzard" => Wx::Blizzard,
                "tornado" | "hurricane" | "tropical_storm" => Wx::Tropical,
                _ => continue,
            };
            if best.is_none_or(|(_, p)| pop > p) {
                best = Some((wx, pop));
            }
        }
        c.wx = best.map_or(Wx::None, |b| b.0);
        c
    }

    /// From an ECCC forecast icon code (table forecast_conditions_icon_code_descriptions_e.csv).
    pub fn from_eccc(code: u8) -> Cond {
        let (sky, wx, windy) = match code {
            0 | 30 => (Sky::Clear, Wx::None, false),
            1 | 31 => (Sky::FewClouds, Wx::None, false),
            2 | 22 | 32 => (Sky::PartlyCloudy, Wx::None, false),
            3 => (Sky::MostlyCloudy, Wx::None, false),
            10 | 33 => (Sky::Overcast, Wx::None, false),
            4 | 34 => (Sky::Increasing, Wx::None, false),
            5 | 35 => (Sky::Clearing, Wx::None, false),
            6 | 36 | 12 => (Sky::Unknown, Wx::Showers, false),
            13 => (Sky::Unknown, Wx::Rain, false),
            28 => (Sky::Unknown, Wx::Drizzle, false),
            14 => (Sky::Unknown, Wx::FreezingRain, false),
            7 | 15 | 37 => (Sky::Unknown, Wx::RainSnow, false),
            8 | 16 | 38 => (Sky::Unknown, Wx::Flurries, false),
            17 => (Sky::Unknown, Wx::Snow, false),
            18 | 40 => (Sky::Unknown, Wx::Blizzard, false),
            27 => (Sky::Unknown, Wx::Sleet, false),
            9 | 19 | 39 => (Sky::Unknown, Wx::Thunder, false),
            20 | 21 | 24 => (Sky::Unknown, Wx::Fog, false),
            23 => (Sky::Unknown, Wx::Haze, false),
            44 => (Sky::Unknown, Wx::Smoke, false),
            43 => (Sky::Unknown, Wx::None, true),
            _ => (Sky::Unknown, Wx::None, false),
        };
        Cond { sky, wx, windy }
    }
}

/// Forecast for a period (day, night) for a place.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Period {
    pub variant: Variant,
    /// place (station code / city-state)
    pub site: String,
    /// period name: «Tonight», «Friday night»
    pub name: String,
    pub day: bool,
    /// period number in the forecast (1 = nearest): what the bureau mentions depends on it
    #[serde(default)]
    pub lead: u8,
    /// sky, phenomenon, windy — from the source's icon codes
    #[serde(default)]
    pub cond: Cond,
    /// daytime high / nighttime low
    pub temp: Option<Temp>,
    /// probability of precipitation, %
    pub pop: Option<u8>,
    pub wind: Option<Wind>,
    /// UV index (ECCC, daytime periods)
    #[serde(default)]
    pub uv: Option<u8>,
    /// humidex (ECCC)
    #[serde(default)]
    pub humidex: Option<i16>,
    /// short description of sky and precipitation (NWS shortForecast / ECCC abbreviatedForecast)
    pub sky: String,
    /// full forecast text
    pub text: String,
}

/// Sentences of a forecast text: the boundary is a period, a space and a capital letter or digit («Cloudy. 30 percent…»).
pub fn sentences(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.trim().chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    for (i, &c) in chars.iter().enumerate() {
        cur.push(c);
        if c == '.' && chars.get(i + 1) == Some(&' ') && chars.get(i + 2).is_some_and(|n| n.is_uppercase() || n.is_ascii_digit()) {
            out.push(cur.trim().to_string());
            cur.clear();
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_and_convert() {
        assert_eq!(sentences("Mainly cloudy. Wind south 20 km/h. High 15."), ["Mainly cloudy.", "Wind south 20 km/h.", "High 15."]);
        assert_eq!(sentences("A 0.5 inch total. Low 3."), ["A 0.5 inch total.", "Low 3."]);
        assert_eq!(sentences("Cloudy. 30 percent chance of showers."), ["Cloudy.", "30 percent chance of showers."]);
        assert_eq!(Temp { value: 72, unit: TempUnit::F }.to(TempUnit::C).value, 22);
        assert_eq!(Temp { value: -40, unit: TempUnit::C }.to(TempUnit::F).value, -40);
    }
}
