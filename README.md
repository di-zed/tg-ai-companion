# 🤖 Telegram AI Companion (Rust + LocalAI)

**Telegram AI Companion** is a Rust-based application that integrates a Telegram bot with a local AI model (e.g.,
Mistral via LocalAI).
The project demonstrates how to build an asynchronous web application using Actix Web, and it can operate without
relying on external APIs like OpenAI.

---

## ✨ Features

* ✉️ Telegram bot that receives messages from users
* 🤖 Integration with a local language model via [LocalAI](https://github.com/mudler/LocalAI)
* 🔁 Optional support for OpenAI API — simply configure parameters in `.env`
* ⌛ Asynchronous web server using Actix Web
* 🔌 REST API
* 🔬 Test coverage
* 📆 Ready to run in Docker containers

---

## 📁 Project Structure

```
├── src/                           # Main application code
│   ├── handlers/                  # HTTP handlers
│   ├── middleware/                # Middleware (e.g., authorization)
│   ├── models/                    # Structs for Telegram, chat, etc.
│   ├── routes/                    # Route definitions
│   └── services/                  # Business logic and API integrations
├── tests/                         # Integration tests
├── images/                        # Dockerfiles
├── models/                        # Models for LocalAI (.gguf + .yaml)
├── volumes/                       # Config files for containers
├── docker-compose.yml             # Docker Compose configuration
├── .env                           # Environment variables
├── Cargo.toml / Cargo.lock        # Rust dependencies
```

---

## 🧠 Using LLM (LocalAI or OpenAI)

By default, LocalAI is used. You can switch to OpenAI by changing `.env`:

### `.env`

```env
OPEN_AI_URL=http://localai:8080                       # or https://api.openai.com
OPEN_AI_MODEL=mistral                                 # or gpt-3.5-turbo / gpt-4
OPEN_AI_API_KEY=your_openai_key                       # required if using OpenAI
```

---

## 📥 Downloading a Model for LocalAI

1. Navigate to the `models/` directory.

2. Download the model (e.g., Mistral):

   ```bash
   wget https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf
   ```

3. Create a `mistral.yaml` config file, for example:

```yaml
name: mistral
backend: llama
parameters:
  model: mistral-7b-instruct-v0.2.Q4_K_M.gguf
  temperature: 0.7
  top_p: 0.9
  top_k: 40
  n_ctx: 4096
```

---

## 🌐 Quick Start

### ⚡ System Requirements

* Docker + Docker Compose
* [Rust](https://www.rust-lang.org/tools/install) (if running outside Docker)

### 🧬 Environment Setup

Copy `.env.sample` to `.env`:

```bash
cp .env.sample .env
```

Edit the `.env` file to match your setup.

### 🐚 Terminal History Setup

```bash
cp volumes/root/.bash_history.sample volumes/root/.bash_history
```

### 🚀 Start with Docker Compose

```bash
docker-compose up --build
```

Once running, the app will be available at `localhost:80`, and LocalAI at `localhost:8080`.

### 🐳 Containers

**Docker Compose runs two containers:**

* `rust-tac`: the Rust application
* `localai-tac`: the language model server

They communicate via an internal Docker network. The Rust app communicates with LocalAI at `http://localai:8080`
(configured via `OPEN_AI_URL` in `.env`).

**Docker volumes:**

* `./models` — contains `.gguf` and `.yaml` model files for LocalAI
* `./volumes/` — used for bash history, autocomplete, and cargo registry cache

### 🧑‍💻 Entering the Container and Running the App

```bash
docker-compose exec rust bash
cargo run
```

Or build a binary and run:

```bash
cargo build --release
./target/release/tg_ai_companion
```

---

## 🔗 API Endpoints

### `POST /telegram/webhook`

* Accepts updates from Telegram (Webhook)
* Processes incoming messages and replies back via Telegram API

> You must configure the Telegram webhook for your bot

**Sample Telegram request body:**

```json
{
  "update_id": 123456789,
  "message": {
    "message_id": 1,
    "chat": {
      "id": 987654321
    },
    "text": "Hello bot"
  }
}
```

The bot response will be sent back to the user via the Telegram API.

---

### `POST /chat`

* Accepts JSON with a `prompt`
* Returns a model response (LocalAI or OpenAI)
* Can be used directly (outside of Telegram), for example in custom UIs or API clients

Example:

```json
{
  "prompt": "Hi, who are you?"
}
```

> You can use either LocalAI or OpenAI depending on your configuration

---

## ✅ Tests

```bash
docker-compose exec rust bash
cargo test
```

Test coverage includes:

* `Telegram` and `Chat` handlers
* `ChatApi` and `TelegramApi` services
* External API integration using `httpmock`

---

## 🚀 Future Plans

* Add conversation memory support
* Create a web interface

---

## ✍️ License

This project is licensed under the MIT License. See [LICENSE](LICENSE).

---

## 🙏 Acknowledgments

* [LocalAI](https://github.com/mudler/LocalAI) for providing an excellent OpenAI-compatible alternative
