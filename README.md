# 🎲 TTRPG Roll Bot

A simple Discord bot built with Rust (using the poise framework) to roll dice for various TTRPG systems.

## 🚀 Available Commands

### ✅ `/config language`

Sets the bot's language for this server. This command requires Administrator Permissions.

Arguments:

    lang_choice (Required): The language to use.

        Português (BR)
        English

### ✅ `/roll daggerheart`

Rolls the Duality Dice (Hope/Fear) for the Daggerheart system. This command is fully functional.

Arguments:

```
    modifier (required): The modifier to add (e.g., 3, -1).
    difficulty (optional): The target difficulty. If provided, the bot will declare the roll a Success or Failure.
```

### 🚧 `/roll dnd2014`

_(in development)_

A planned command to roll D&D-style dice strings (e.g., "2d20+5").

## 🛠️ How to Run Locally

You must have the Rust toolchain (including cargo) installed.

1. Clone the Repository (Remember to replace with your own repository URL)

```bash
git clone https://github.com/YOUR-USER/YOUR-REPO.git
cd YOUR-REPO
```

2. Configure Environment Create a file named .env in the root of the project. See the Configuration section below for details.

3. Compile and Run Cargo will download and compile all dependencies (this may take a while on the first run).

```bash
cargo run
```

The bot will compile and connect to Discord. If your .env is configured for debug mode, the commands will register instantly on your test server.

## ⚙️ Configuration

Create a .env file in the project's root directory. The following environment variables are supported:

```
DISCORD_TOKEN=SECRET_BOT_TOKEN_HERE (required)
DEBUG_SERVER_ID=TEST_SERVER_ID_HERE (optional)
```

## Debug vs. Production Mode

The bot's startup behavior changes based on the DEBUG_SERVER_ID variable:

    Debug Mode (Recommended for Development): If DEBUG_SERVER_ID is set, the bot will start in Debug Mode. It will clear all global commands and register its commands instantly and only on the server (guild) specified by that ID.

    Production Mode: If DEBUG_SERVER_ID is not set (or the line is commented out), the bot will start in Production Mode. It will register its commands globally, which allows it to work on any server it's invited to.

    ⚠️ Warning: Registering commands globally can take up to 1 hour for Discord to update and show them. This is not ideal for testing.