//! Environment and Climate Change Canada — citypage XML (MSC Datamart, dd.weather.gc.ca;
//! ECCC Data Servers End-use Licence: free use, including commercial, with attribution «Data Source:
//! Environment and Climate Change Canada»). English, °C, km/h — the international variant.
//! For each station, the freshest `_en.xml` file of today; parsing via serde structures.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use serde::Deserialize;

use crate::data::{Cond, Dir, Period, SpeedUnit, Temp, TempUnit, Variant, Wind};

const UA: &str = "Mova-NLP/0.1 (weather domain research)";
const ROOT: &str = "https://dd.weather.gc.ca/today/citypage_weather";

fn get(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>> {
    let mut v = Vec::new();
    std::io::Read::read_to_end(&mut agent.get(url).set("User-Agent", UA).call()?.into_reader(), &mut v)?;
    Ok(v)
}

/// Links from an Apache directory index: `href="…"`.
fn links(html: &[u8]) -> Vec<String> {
    let s = String::from_utf8_lossy(html);
    s.split("href=\"").skip(1).filter_map(|x| x.split('"').next()).map(str::to_string).collect()
}

/// Snapshot: the freshest English file of each station → `out/<province>_<station>.xml`.
pub fn collect(out: &Path, pause: Duration) -> Result<(usize, usize)> {
    std::fs::create_dir_all(out)?;
    let agent = ureq::AgentBuilder::new().timeout(Duration::from_secs(30)).build();
    let mut log = std::fs::OpenOptions::new().create(true).append(true).open(out.join("collect.log"))?;
    let provs: Vec<String> = links(&get(&agent, &format!("{ROOT}/"))?)
        .into_iter()
        .filter(|l| l.len() == 3 && l.ends_with('/') && l[..2].bytes().all(|b| b.is_ascii_uppercase()) && l != "HEF/")
        .map(|l| l[..2].to_string())
        .collect();
    let (mut ok, mut fail) = (0, 0);
    for prov in provs {
        std::thread::sleep(pause);
        let mut hours: Vec<String> = links(&get(&agent, &format!("{ROOT}/{prov}/"))?).into_iter().filter(|l| l.len() == 3 && l[..2].bytes().all(|b| b.is_ascii_digit())).collect();
        hours.sort();
        hours.reverse();
        // station → (hour, file): the freshest hour, and within it the latest file
        let mut latest: BTreeMap<String, (String, String)> = BTreeMap::new();
        for h in &hours {
            std::thread::sleep(pause);
            let files = match get(&agent, &format!("{ROOT}/{prov}/{h}")) {
                Ok(b) => links(&b),
                Err(e) => {
                    writeln!(log, "FAIL list {prov}/{h} {e}")?;
                    continue;
                }
            };
            let mut here: BTreeMap<String, String> = BTreeMap::new();
            for f in files.into_iter().filter(|f| f.ends_with("_en.xml")) {
                let Some(site) = f.split('_').rev().nth(1).map(str::to_string) else { continue };
                let e = here.entry(site).or_default();
                if f > *e {
                    *e = f;
                }
            }
            for (site, f) in here {
                latest.entry(site).or_insert((h.clone(), f));
            }
        }
        for (site, (h, f)) in latest {
            let file = out.join(format!("{prov}_{site}.xml"));
            if file.exists() {
                continue;
            }
            std::thread::sleep(pause);
            match get(&agent, &format!("{ROOT}/{prov}/{h}{f}")) {
                Ok(body) => {
                    std::fs::write(&file, body)?;
                    ok += 1;
                }
                Err(e) => {
                    writeln!(log, "FAIL {prov}/{h}{f} {e}")?;
                    fail += 1;
                }
            }
        }
        writeln!(log, "{prov}: done")?;
    }
    Ok((ok, fail))
}

#[derive(Deserialize, Default)]
struct Text {
    #[serde(rename = "$text", default)]
    v: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SiteData {
    location: Option<Location>,
    forecast_group: Option<ForecastGroup>,
}

#[derive(Deserialize)]
struct Location {
    name: Option<Text>,
}

#[derive(Deserialize)]
struct ForecastGroup {
    #[serde(default)]
    forecast: Vec<EcForecast>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EcForecast {
    period: EcPeriod,
    #[serde(default)]
    text_summary: String,
    abbreviated_forecast: Option<Abbrev>,
    temperatures: Option<Temps>,
    winds: Option<Winds>,
    uv: Option<EcUv>,
    humidex: Option<Text>,
}

#[derive(Deserialize)]
struct EcUv {
    index: Option<Text>,
}

#[derive(Deserialize)]
struct EcPeriod {
    #[serde(rename = "@textForecastName")]
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Abbrev {
    icon_code: Option<Text>,
    pop: Option<Text>,
    #[serde(default)]
    text_summary: String,
}

#[derive(Deserialize)]
struct Temps {
    #[serde(default)]
    temperature: Vec<EcTemp>,
}

#[derive(Deserialize)]
struct EcTemp {
    #[serde(rename = "@class")]
    class: Option<String>,
    #[serde(rename = "$text", default)]
    v: String,
}

#[derive(Deserialize)]
struct Winds {
    #[serde(default)]
    wind: Vec<EcWind>,
}

#[derive(Deserialize)]
struct EcWind {
    #[serde(rename = "@rank")]
    rank: Option<String>,
    speed: Option<Text>,
    gust: Option<Text>,
    direction: Option<Text>,
}

/// Periods from a station's raw XML; the second value is the place name.
pub fn periods(site: &str, body: &str) -> Result<(String, Vec<Period>)> {
    let d: SiteData = quick_xml::de::from_str(body)?;
    let place = d.location.and_then(|l| l.name).map(|n| n.v).unwrap_or_default();
    let fs = d.forecast_group.map(|g| g.forecast).unwrap_or_default();
    let out = fs
        .into_iter()
        .enumerate()
        .map(|(i, f)| {
            let night = f.period.name.to_lowercase().contains("night");
            let want = if night { "low" } else { "high" };
            let temp = f.temperatures.and_then(|t| {
                t.temperature.into_iter().find(|x| x.class.as_deref() == Some(want)).and_then(|x| x.v.trim().parse().ok()).map(|value| Temp { value, unit: TempUnit::C })
            });
            let abbrev = f.abbreviated_forecast;
            let cond = abbrev.as_ref().and_then(|a| a.icon_code.as_ref()).and_then(|c| c.v.trim().parse::<u8>().ok()).map(Cond::from_eccc).unwrap_or_default();
            let uv = f.uv.and_then(|u| u.index).and_then(|i| i.v.trim().parse().ok());
            let humidex = f.humidex.and_then(|h| h.v.trim().parse().ok());
            let pop = abbrev.as_ref().and_then(|a| a.pop.as_ref()).and_then(|p| p.v.trim().parse().ok());
            let wind = f.winds.and_then(|w| w.wind.into_iter().filter(|x| x.rank.as_deref() != Some("minor")).max_by_key(|x| x.speed.as_ref().and_then(|s| s.v.trim().parse::<u16>().ok()).unwrap_or(0))).and_then(|w| {
                let sp: u16 = w.speed.as_ref()?.v.trim().parse().ok()?;
                let gust: Option<u16> = w.gust.as_ref().and_then(|g| g.v.trim().parse().ok()).filter(|&g| g > 0);
                Some(Wind { dir: w.direction.as_ref().and_then(|d| Dir::parse(&d.v)), lo: sp, hi: sp, gust, unit: SpeedUnit::Kmh })
            });
            Period {
                variant: Variant::Intl,
                site: site.to_string(),
                name: f.period.name,
                day: !night,
                temp,
                pop,
                lead: (i + 1) as u8,
                cond,
                wind,
                uv,
                humidex,
                sky: abbrev.map(|a| a.text_summary).unwrap_or_default(),
                text: f.text_summary,
            }
        })
        .collect();
    Ok((place, out))
}
