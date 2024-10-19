mod model;

use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine};
use model::{
    Item, Metadata, Output, RaidItem, RaidResponse, RaidresResponse, ReservationData, SoftReserve,
};
use reqwest::Client;

const RAIDRES_URL: &str = "https://raidres.fly.dev";

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

fn get_soft_reserves(
    reservations: &Vec<ReservationData>,
    raid_items: &[RaidItem],
) -> Vec<SoftReserve> {
    let mut result = Vec::new();

    for reservation in reservations {
        let item_id = reservation.raid_item_id;

        if item_id.is_none() {
            result.push(SoftReserve {
                name: reservation.character.name.clone(),
                items: vec![Item { id: 0, quality: 0 }],
            });

            continue;
        }

        if let Some(raid_item) = raid_items.iter().find(|item| item.id == item_id.unwrap()) {
            result.push(SoftReserve {
                name: reservation.character.name.clone(),
                items: vec![Item {
                    id: raid_item.turtle_db_item_id,
                    quality: raid_item.quality,
                }],
            });
        }
    }

    result
}

fn get_hard_reserves(reservations: &Vec<i32>, raid_items: &[RaidItem]) -> Vec<Item> {
    let mut result = Vec::new();

    for item_id in reservations {
        if let Some(raid_item) = raid_items.iter().find(|item| item.id == *item_id) {
            result.push(Item {
                id: raid_item.turtle_db_item_id,
                quality: raid_item.quality,
            });
        }
    }

    result
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

    let output = Output {
        metadata: Metadata {
            id: id.to_string(),
            instance: raidres_response.raid_id,
            instances: vec![raid_response.name],
        },
        softreserves: get_soft_reserves(&raidres_response.reservations, &raid_response.raid_items),
        hardreserves: get_hard_reserves(
            &raidres_response.disabled_raid_item_ids,
            &raid_response.raid_items,
        ),
    };

    let json = serde_json::to_string(&output)?;
    let output = general_purpose::STANDARD.encode(json);

    println!("{}", output);
    Ok(())
}
