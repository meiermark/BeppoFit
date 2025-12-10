# BeppoFit

A fitness application built with a **Rust (Axum)** backend and an **Angular (Ionic)** frontend.

## 🚀 Quick Start (Docker)

The easiest way to run the application is with Docker Compose. This starts the Database, Backend (API), Frontend (Nginx), and Mailhog (SMTP Mock).

1.  **Configure Environment**:
    Make sure you have a `.env` file in the root directory (one has been created for you).
    ```bash
    cp .env.example .env # If needed
    ```

2.  **Run the Stack**:
    ```bash
    docker-compose up --build
    ```

3.  **Access the App**:
    - **App**: [http://localhost](http://localhost) (Port 80)
    - **Mailhog (Emails)**: [http://localhost:8025](http://localhost:8025)
    - **API (Direct)**: [http://localhost:8080](http://localhost:8080)

## 🛠 Development

### Backend (`beppo-fit-backend`)
- **Framework**: Axum, SQLx, Tokio.
- **Database**: PostgreSQL.
- **Offline Mode**: We use `SQLX_OFFLINE=true` for Docker builds.
  - **Important**: If you modify SQL queries, you **must** run `cargo sqlx prepare` locally (inside the backend folder) and commit the generated `.sqlx` directory.
  ```bash
  cd beppo-fit-backend
  cargo sqlx prepare
  ```

### Frontend (`beppo-fit-app`)
- **Framework**: Angular + Ionic.
- **Proxy**:
  - In **Docker**, Nginx proxies `/api` requests to the `backend` service.
  - In **Dev (`npm start`)**, Angular CLI proxies `/api` to `localhost:8080` using `proxy.conf.json`.

## ✅ Testing

### End-to-End (Playwright)
Run these from the `beppo-fit-app` directory. Ensure the app is running (locally or via Docker).
```bash
cd beppo-fit-app
npx playwright test
```
To view the test report:
```bash
npx playwright show-report
```

### Backend Unit Tests
```bash
cd beppo-fit-backend
cargo test
```

### Frontend Unit Tests
```bash
cd beppo-fit-app
ng test
```

## 📦 Deployment Configuration

The application is configured via **Environment Variables**. Important variables to set in production:

| Variable | Description | Default (Local) |
|----------|-------------|-----------------|
| `DATABASE_URL` | Postgres Connection String | `postgres://beppo...` |
| `JWT_SECRET` | Secret for signing tokens | (Change this!) |
| `FRONTEND_URL` | URL where app is hosted | `http://localhost:4200` |
| `GOOGLE_CLIENT_ID` | OAuth2 Client ID | `dummy...` |
| `GOOGLE_CLIENT_SECRET` | OAuth2 Secret | `dummy...` |
| `GOOGLE_REDIRECT_URL` | OAuth2 Callback URL | `.../auth/google/callback` |

**Note**: In the Docker setup, these can be set in the root `.env` file, which `docker-compose.yml` reads and passes to the containers.
