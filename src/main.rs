use std::{fs::File, io::BufWriter};

use anyhow::Result;
use csv::WriterBuilder;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct Data {
    #[serde(rename = "Id")]
    id: i64,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Series")]
    series: String,
    #[serde(rename = "Hidden")]
    hidden: bool,
}

fn main() -> Result<()> {
    let output = File::create("chives.csv")?;
    let mut writer = WriterBuilder::new().from_writer(BufWriter::new(output));

    let url = "https://raw.githubusercontent.com/Dimbreath/StarRailData/master/";

    let text_map: Value = ureq::get(&format!("{url}TextMap/TextMapEN.json"))
        .call()?
        .into_json()?;
    let achievement_data: Value = ureq::get(&format!("{url}ExcelOutput/AchievementData.json"))
        .call()?
        .into_json()?;
    let achievement_series: Value = ureq::get(&format!("{url}ExcelOutput/AchievementSeries.json"))
        .call()?
        .into_json()?;

    for (_, value) in achievement_data.as_object().unwrap() {
        let id = value["AchievementID"].as_i64().unwrap();

        let title_hash = value["AchievementTitle"]["Hash"]
            .as_i64()
            .unwrap()
            .to_string();
        let title = text_map[title_hash].as_str().unwrap().to_string();

        let series_id = value["SeriesID"].as_i64().unwrap().to_string();
        let series_hash = achievement_series[series_id]["SeriesTitle"]["Hash"]
            .as_i64()
            .unwrap()
            .to_string();
        let series = text_map[series_hash].as_str().unwrap().to_string();

        let hidden = value["ShowType"] == json!("ShowAfterFinish");

        let data = Data {
            id,
            title,
            series,
            hidden,
        };
        writer.serialize(data)?;
    }

    Ok(())
}
