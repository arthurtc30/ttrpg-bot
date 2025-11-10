# 🎲 TTRPG Roll Bot

A simple Discord bot built with Rust (using the `poise` framework) to roll dice for various TTRPG systems.

## 🚀 Available Commands

### ✅ `/config language`

Sets the bot's language for this server. This command requires **Administrator Permissions**.

**Arguments:**

  * `lang_choice` (Required): The language to use.
      * `Português (BR)`
      * `English`

### ✅ `/help`

Displays the bot's help menu. Use subcommands to get help for specific command groups.

**Subcommands:**

  * `/help roll`: Shows help for all dice rolling commands.
  * `/help config`: Shows help for all configuration commands.

### ✅ `/roll daggerheart`

Rolls the Duality Dice (Hope/Fear) for the Daggerheart system. This command is **fully functional**.

**Arguments:**

  * `modifier` (Required): The modifier to add (e.g., `3`, `-1`).
  * `difficulty` (Optional): The target difficulty. If provided, the bot will declare the roll a Success or Failure.
  * `adv_dis` (Optional): Choose to roll with Advantage (`+1d6`) or Disadvantage (`-1d6`).

**Examples:**
* `/roll daggerheart modifier: 2`
* `/roll daggerheart modifier: 3 adv_dis: Advantage (+1d6) difficulty: 15`
* `/roll daggerheart modifier: 1 adv_dis: Disadvantage (-1d6)`

### ✅ `/roll dnd2014`

Rolls D\&D-style dice strings, including complex modifiers, advantage (`>`), and disadvantage (`<`).

**Examples:**

  * `1d20 + 5`
  * `2d10 + 3 - 1`
  * `2>d20 + 4` (Rolls 2d20, takes the highest, then adds 4)
  * `2<d10 - 2` (Rolls 2d10, takes the lowest, then subtracts 2)
  * `2d10 + 3 + 1d6`
  * `2>d10 + 3 + 1d6` (Rolls 2d10, takes the highest, then adds 3, then rolls 1d6 and adds it)

> _Note: some of these rolls are not used in regular DnD, but they are included for completeness._

-----

## 🛠️ How to Run Locally

You must have the [Rust](https://rustup.rs/) toolchain (including `cargo`) installed.

**1. Clone the Repository**

```bash
git clone https://github.com/arthurtc30/ttrpg-bot.git
cd ttrpg-bot
```

**2. Configure Environment**
Create a file named `.env` in the root of the project. See the `Configuration` section below for details.

**3. Compile and Run**
Cargo will download and compile all dependencies (this may take a while on the first run).

```bash
cargo run
```

The bot will compile and connect to Discord. If your `.env` is configured for debug mode, the commands will register instantly on your test server.

-----

## ⚙️ Configuration

Create a `.env` file in the project's root directory. The following environment variables are supported:

```.env
# Your bot's secret token from the Discord Developer Portal
DISCORD_TOKEN=SECRET_BOT_TOKEN_HERE

# (Optional) Your test server's ID for instant command registration (Debug Mode)
DEBUG_SERVER_ID=TEST_SERVER_ID_HERE
```

### Debug vs. Production Mode

The bot's startup behavior changes based on the `DEBUG_SERVER_ID` variable:

  * **Debug Mode (Recommended for Development):**
    If `DEBUG_SERVER_ID` is set, the bot will start in **Debug Mode**. It will clear all global commands and register its commands *instantly* and *only* on the server (guild) specified by that ID.

  * **Production Mode:**
    If `DEBUG_SERVER_ID` is **not set** (or the line is commented out), the bot will start in **Production Mode**. It will register its commands globally, which allows it to work on any server it's invited to.

    > **⚠️ Warning:** Registering commands globally can take **up to 1 hour** for Discord to update and show them. This is not ideal for testing.