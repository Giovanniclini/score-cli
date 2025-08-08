# 🎯 score-cli

**score-cli** is a command-line tool written in Rust for managing board game matches, players, and scores.  
It allows you to add/remove players, record scores, view statistics, and consult the match history.

---

## ✨ Features

- **Player management**
  - `add-player`: adds a new player.
  - `delete-player`: removes a player.

- **Score management**
  - `add-score`: records a new match with the related scores.
  - `delete-score`: removes an existing match by `id`.
  
- **Consultation**
  - `list-games`: lists all matches in chronological order.

---

## 📦 Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/<your-username>/score-cli.git
cd score-cli
cargo build --release
```

The compiled binary will be available in `target/release/score-cli`.

You can also install it locally with:

```bash
cargo install --path .
```

---

## 🚀 Usage

### Add a player

```bash
score-cli add-player <player-name> [--save-dir <path>]
```

Example:

```bash
score-cli add-player giovi98 --save-dir ./data
```

---

### Remove a player

```bash
score-cli delete-player <player-name> [--save-dir <path>]
```

---

### Add a match

```bash
score-cli add-score <game-name> <player1>::<score1> <player2>::<score2> ... [--time <YYYY-MM-DD>] [--save-dir <path>]
```

Example:

```bash
score-cli add-score catan giovi98::100 emma00::2 --time 2025-08-07 --save-dir ./data
```

---

### Remove a match

```bash
score-cli delete-score <game-id> [--save-dir <path>]
```

---

### List matches

```bash
score-cli list-games [--save-dir <path>]
```

Example:

```bash
score-cli list-games --save-dir ./data
```

---

## 📂 Data format

Data is stored in **JSON** format inside the specified directory (or in the current working directory if none is specified).  
Example of a match file:

```json
{
  "games": {
    "849cf74e-0e19-45ce-a630-99916cb3b648": {
      "id": "849cf74e-0e19-45ce-a630-99916cb3b648",
      "game_name": "catan",
      "scores": {
        "giovi98": 100,
        "emma00": 2
      },
      "time": "2025-08-07"
    }
  }
}
```

---

## 🧪 Tests

The project includes automated tests.  
You can run them with:

```bash
cargo test
```

It also includes end-to-end tests using [`assert_cmd`](https://docs.rs/assert_cmd) to verify the CLI behavior.

---

## ⚙️ CI Pipeline

This project uses **GitHub Actions** to automatically run build and tests on every push or pull request to `main` and `develop`.

---

## 📜 License

This project is distributed under the MIT license.  
See the [LICENSE](LICENSE) file for more details.