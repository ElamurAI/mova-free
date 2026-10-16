//! NWS (api.weather.gov, US National Weather Service, public domain): `/points/{lat},{lon}` →
//! `properties.forecast` → 14 periods. Collector and parsing via serde structures; polite (pause between
//! requests), no retries, failures go to the log.

use std::io::Write;
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::data::{Cond, Dir, Period, SpeedUnit, Temp, TempUnit, Variant, Wind};

const UA: &str = "Mova-NLP/0.1 (weather domain research)";

#[derive(Deserialize)]
struct Points {
    properties: PointsProps,
}

#[derive(Deserialize)]
struct PointsProps {
    forecast: String,
}

#[derive(Deserialize)]
pub struct Forecast {
    pub properties: ForecastProps,
}

#[derive(Deserialize)]
pub struct ForecastProps {
    pub periods: Vec<NwsPeriod>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NwsPeriod {
    #[serde(default)]
    pub number: u8,
    pub name: String,
    pub is_daytime: bool,
    pub temperature: Option<i16>,
    pub temperature_unit: Option<String>,
    pub probability_of_precipitation: Option<Pop>,
    pub wind_speed: Option<String>,
    pub wind_direction: Option<String>,
    #[serde(default)]
    pub icon: String,
    pub short_forecast: String,
    pub detailed_forecast: String,
}

#[derive(Deserialize)]
pub struct Pop {
    pub value: Option<f64>,
}

/// A city from the list: name, state, latitude, longitude.
pub struct City {
    pub name: String,
    pub state: String,
    pub lat: String,
    pub lon: String,
}

impl City {
    pub fn id(&self) -> String {
        format!("{}-{}", self.name, self.state).replace([' ', '/'], "_")
    }
}

pub fn read_cities(path: &Path) -> Result<Vec<City>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    Ok(text
        .lines()
        .filter_map(|l| {
            let c: Vec<&str> = l.split(',').collect();
            (c.len() >= 4).then(|| City { name: c[0].to_string(), state: c[1].to_string(), lat: c[2].to_string(), lon: c[3].to_string() })
        })
        .collect())
}

fn get(agent: &ureq::Agent, url: &str) -> Result<String> {
    Ok(agent.get(url).set("User-Agent", UA).call()?.into_string()?)
}

/// Snapshot of forecasts for cities into `out/<city>.json`; existing files are skipped.
pub fn collect(cities: &[City], out: &Path, pause: Duration) -> Result<(usize, usize)> {
    std::fs::create_dir_all(out)?;
    let agent = ureq::AgentBuilder::new().timeout(Duration::from_secs(30)).build();
    let mut log = std::fs::OpenOptions::new().create(true).append(true).open(out.join("collect.log"))?;
    let (mut ok, mut fail) = (0, 0);
    for c in cities {
        let file = out.join(format!("{}.json", c.id()));
        if file.exists() {
            continue;
        }
        let res = get(&agent, &format!("https://api.weather.gov/points/{},{}", c.lat, c.lon))
            .and_then(|b| Ok(serde_json::from_str::<Points>(&b)?.properties.forecast))
            .and_then(|url| {
                std::thread::sleep(pause);
                get(&agent, &url)
            })
            .and_then(|b| {
                serde_json::from_str::<Forecast>(&b)?; // shape check only
                Ok(b)
            });
        match res {
            Ok(body) => {
                std::fs::write(&file, body)?;
                writeln!(log, "ok {}", c.id())?;
                ok += 1;
            }
            Err(e) => {
                writeln!(log, "FAIL {} {e}", c.id())?;
                fail += 1;
            }
        }
        std::thread::sleep(pause);
    }
    Ok((ok, fail))
}

/// «5 to 10 mph», «7 mph» → (from, to).
fn speed(s: &str) -> Option<(u16, u16)> {
    let nums: Vec<u16> = s.split_whitespace().filter_map(|w| w.parse().ok()).collect();
    match nums.as_slice() {
        [a] => Some((*a, *a)),
        [a, b, ..] => Some((*a, *b)),
        _ => None,
    }
}

/// NWS icon codes: `…/land/day/rain_showers,40/sct?size=medium` → [(code, probability)].
pub fn icon_codes(url: &str) -> Vec<(&str, u8)> {
    let path = url.split('?').next().unwrap_or("");
    let rest = path.split("/land/").nth(1).unwrap_or("");
    rest.split('/').skip(1).map(|c| match c.split_once(',') {
        Some((code, p)) => (code, p.parse().unwrap_or(0)),
        None => (c, 0),
    }).collect()
}

/// Periods from the raw forecast JSON.
pub fn periods(site: &str, body: &str) -> Result<Vec<Period>> {
    let f: Forecast = serde_json::from_str(body)?;
    Ok(f.properties
        .periods
        .into_iter()
        .map(|p| {
            let unit = if p.temperature_unit.as_deref() == Some("C") { TempUnit::C } else { TempUnit::F };
            let wind = p.wind_speed.as_deref().and_then(speed).map(|(lo, hi)| Wind {
                dir: p.wind_direction.as_deref().and_then(Dir::parse),
                lo,
                hi,
                gust: None,
                unit: SpeedUnit::Mph,
            });
            Period {
                variant: Variant::Us,
                site: site.to_string(),
                name: p.name,
                day: p.is_daytime,
                temp: p.temperature.map(|value| Temp { value, unit }),
                pop: p.probability_of_precipitation.and_then(|x| x.value).map(|v| v.round() as u8),
                lead: p.number,
                cond: Cond::from_nws(&icon_codes(&p.icon)),
                wind,
                uv: None,
                humidex: None,
                sky: p.short_forecast,
                text: p.detailed_forecast,
            }
        })
        .collect())
}
