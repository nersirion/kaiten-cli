# Kaiten CLI

Kaiten CLI is a command-line interface for interacting with the [Kaiten API](https://developers.kaiten.ru). It provides various commands to manage cards, columns, users, tags, lanes, spaces, boards, comments, links, and more.

## Getting Started

### Prerequisites

Before using Kaiten CLI, make sure you set the following environment variables:

- `API_URL`: The URL for the Kaiten API. https://<your_domain>.kaiten.ru/api/latest
- `KT`: The Bearer token for authentication.

### Installation

```bash
# Clone the repository
git clone https://github.com/nersirion/kaiten-cli.git
# Navigate to the project directory
cd kaiten-cli
# Build the CLI tool
cargo build --release
```

## Usage

```bash
# Display help message
kaiten-cli --help

# Initialize and download long-term entity information
kaiten-cli init

# Create a new config file for filters
kaiten-cli config new
# Show config values
kaiten-cli config show
# Set config values for automaticly filters in query
kaiten-cli config set --space-id 1 --board-id 1 --exclude-column-ids 1,2

# Example: Get cards with specified filters, ignoring the configuration values.
kaiten-cli cards --space-id <SPACE_ID> --board-id <BOARD_ID> --ignore-config

# Example: Card workflow
# List available cards for the user
kaiten-cli cards ls
# List cards filtered by custom company properties value
kaiten-cli cards ls --properties-value-id 123
# Get full information about the card
kaiten-cli cards get 123
# Edit card with added description
kaiten-cli cards edit --add-description "$(cat description.md)" 123
# Move the card and set responsible
kaiten-cli cards mv 123 --column-id 2 --lane-id 2 user
# Add a link to the card
kaiten-cli link new --link https://something --description dev-stand
# Add comment to the card
kaiten-cli comments new 123 "@user review" 
```

## TODO

- [ ] Add custom properties in the long-term entity
- [ ] Add checklists functional
- [ ] Add comments in the code
- [ ] Add blocker functional
- [ ] Table formatting base on terminal size
- [ ] Extend edit card functional
- [ ] Tests
