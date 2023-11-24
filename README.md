# space-traders

## Prerequisites
Save your [Space traders API key](https://docs.spacetraders.io/) locally in `~/.config/space-traders/key.txt` (for macOS and Linux).  No Windows support at this time.

## Contribute
- Install all [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- Install the Tauri CLI with `cargo install tauri-cli`
- Run the following to interactively view the project while working:
```bash
npm install
cargo tauri dev
``` 

## Note
I did not write the code for the Rust implemented API to interact with Space Traders.  I used [OpenAPI Generator](https://openapi-generator.tech/) command:
```bash
openapi-generator-cli -g rust -i https://stoplight.io/api/v1/projects/spacetraders/spacetraders/nodes/reference/SpaceTraders.json?fromExportButton=true&snapshotType=http_service&deref=optimizedBundle
```
I did have to make some minor tweaks to get the code to compile.

Anything in the `helpers` folder I wrote.
