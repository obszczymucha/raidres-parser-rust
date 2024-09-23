# Raidres Parser
A parser for [raidres](https://raidres.fly.dev) softres data for integration with [RollFor](https://github.com/obszczymucha/roll-for-vanilla) WoW addon.

## NOTE
This is a temporary solution until [raidres](https://raidres.fly.dev) implements proper data export.  
I already talked to **itamedruids** who is the author of the website and he promised he'll do it.  
Ask him kindly to do it if you like his work and this addon. Find him on Discord, details at raidres website.  
In the meantime, continue reading.

## How to use
1. Install **Rust** using [rustup](https://rustup.rs). It will install **cargo** to build this project.  
2. Clone or download (and unzip) this **repository**.
3. Set up the raid at [raidres](https://raidres.fly.dev) and grab raid **id** from the url.  
4. Open the **repository**'s directory in your command line and run:
  ```bash
  cargo run <id>
  ```

  Example:  
  ```bash
  cargo run U22642
  ```

The program will download the list of softres items and print a **hash**. Copy it.

4. Open **RollFor**'s softres screen in WoW by typing `/sr` or clicking minimap icon.
5. Paste the **hash** there.
6. Repeat from step **4** if *raidres* list was updated.
7. Don't forget to **lock** the raid in *raidres* website before starting.
8. Don't forget to set yourselt to **Master Looter** and enable **Master Loot**.

## Hacking
The **hash** is a **json** encoded with **Base64**.  
Run the following to view:
  ```bash
  cargo run <id> | base64 --decode | jq
  ```

Obviously you have to have **base64** and **jq** installed for this.

