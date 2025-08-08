# 🎯 score-cli

**score-cli** è uno strumento da riga di comando scritto in Rust per gestire partite, giocatori e punteggi di giochi da tavolo.  
Permette di aggiungere/rimuovere giocatori, registrare punteggi, visualizzare statistiche e consultare la cronologia delle partite.

---

## ✨ Funzionalità

- **Gestione giocatori**
  - `add-player`: aggiunge un nuovo giocatore.
  - `delete-player`: rimuove un giocatore.

- **Gestione punteggi**
  - `add-score`: registra una nuova partita con i relativi punteggi.
  - `delete-score`: rimuove una partita esistente tramite `id`.
  
- **Consultazione**
  - `list-games`: elenca tutte le partite in ordine cronologico.

---

## 📦 Installazione

Clona il repository e compila con Cargo:

```bash
git clone https://github.com/<tuo-username>/score-cli.git
cd score-cli
cargo build --release
```

Il binario compilato sarà disponibile in `target/release/score-cli`.

Puoi anche installarlo localmente con:

```bash
cargo install --path .
```

---

## 🚀 Utilizzo

### Aggiungere un giocatore

```bash
score-cli add-player <player-name> [--save-dir <path>]
```

Esempio:

```bash
score-cli add-player giovi98 --save-dir ./data
```

---

### Rimuovere un giocatore

```bash
score-cli delete-player <player-name> [--save-dir <path>]
```

---

### Aggiungere una partita

```bash
score-cli add-score <game-name> <player1>::<score1> <player2>::<score2> ... [--time <YYYY-MM-DD>] [--save-dir <path>]
```

Esempio:

```bash
score-cli add-score catan giovi98::100 emma00::2 --time 2025-08-07 --save-dir ./data
```

---

### Rimuovere una partita

```bash
score-cli delete-score <game-id> [--save-dir <path>]
```

---

### Elencare le partite

```bash
score-cli list-games [--save-dir <path>]
```

Esempio:

```bash
score-cli list-games --save-dir ./data
```

---

## 📂 Formato dei dati

I dati vengono salvati in formato **JSON** all’interno della directory specificata (o nella cartella di lavoro corrente se non specificata).  
Esempio di file di partite:

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

## 🧪 Test

Il progetto include test automatici.  
Puoi eseguirli con:

```bash
cargo test
```

Sono presenti anche test end-to-end con [`assert_cmd`](https://docs.rs/assert_cmd) per verificare il comportamento del CLI.

---

## ⚙️ Pipeline CI

Questo progetto utilizza **GitHub Actions** per eseguire automaticamente build e test a ogni push o pull request su `main` e `develop`.

---

## 📜 Licenza

Questo progetto è distribuito sotto licenza MIT.  
Vedi il file [LICENSE](LICENSE) per maggiori dettagli.
