mod model;

use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine};
use model::{Item, Metadata, Output, RaidResponse, RaidresResponse, SoftReserve};
use reqwest::Client;

const RAIDRES_URL: &str = "https://raidres.fly.dev/";

async fn fetch_raidres_data(id: &str, client: &Client) -> Result<RaidresResponse> {
    let url = format!("{}/api/events/{}", RAIDRES_URL, id);
    eprintln!("Fetching item reservation data from {}", url);
    client
        .get(&url)
        .send()
        .await?
        .json::<RaidresResponse>()
        .await
        .context("Failed to fetch item reservation data.")
}

async fn fetch_raid_data(raid_id: i32, client: &Client) -> Result<RaidResponse> {
    let url = format!("{}/raids/raid_{}.json", RAIDRES_URL, raid_id);
    eprintln!("Fetching raid item data from {}", url);
    client
        .get(&url)
        .send()
        .await?
        .json::<RaidResponse>()
        .await
        .context("Failed to fetch raid item data.")
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <id>", args[0]);
        std::process::exit(1);
    }

    let id = &args[1];
    let client = reqwest::Client::new();
    let raidres_response = fetch_raidres_data(id, &client).await?;
    let raid_response = fetch_raid_data(raidres_response.raid_id, &client).await?;
    let mut soft_reserves = Vec::new();

    for reservation in raidres_response.reservations {
        if let Some(raid_item) = raid_response
            .raid_items
            .iter()
            .find(|item| item.id == reservation.raid_item_id)
        {
            soft_reserves.push(SoftReserve {
                name: reservation.character.name,
                items: vec![Item {
                    id: raid_item.turtle_db_item_id,
                    quality: raid_item.quality,
                }],
            });
        }
    }

    let output = Output {
        metadata: Metadata {
            id: id.to_string(),
            instance: raidres_response.raid_id,
            instances: vec![raid_response.name],
        },
        softreserves: soft_reserves,
    };

    let json = serde_json::to_string(&output)?;
    let output = general_purpose::STANDARD.encode(json);

    println!("{}", output);
    Ok(())
}
