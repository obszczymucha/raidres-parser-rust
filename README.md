# Raidres Parser
A parser for https://raidres.fly.dev data for integration with https://github.com/obszczymucha/RollFor.

## How to use
1. Set up the raid and grab the id from the url. Then run:
  ```bash
  cargo run <id>
  ```

  Example:  
  ```bash
  cargo run U22642
  ```

2. Open softres screen in RollFor by typing `/sr`.
3. Paste the hash there.
4. Repeat from step one if **raidres** list was updated.
5. Don't forget to lock the raid in **raidres** website before starting.

## Hacking
The hash is a **json** encoded with **Base64**.  
Run the following to view:
  ```bash
  cargo run <id> | base64 --decode | jq
  ```

Obviously you have to have **base64** and **jq** installed for this.

