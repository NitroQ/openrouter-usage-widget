# openrouter-usage-widget

A desktop widget for monitoring OpenRouter API usage in real-time. Built with Tauri 2, React 19, and TypeScript, it sits on top of your screen as a compact always-on-top overlay, giving you instant visibility into your API spending, remaining credits, and usage trends.

---

## Features

**Real-time Usage Monitoring**
- Remaining account credits and percentage used
- Today's usage and current month's usage
- Overall usage against monthly limits
- In-widget chart view with daily (last 7 days) and monthly (last 6 months) toggle
- Daily bars show date labels (MM-DD); monthly bars show abbreviated month names (Jan, Feb, etc.)
- Chart automatically includes today's usage from the live dashboard data

**Key Mode Support**
- Standard API keys with BYOK (Bring Your Own Key) usage tracking
- Management API keys with a full dashboard: key summaries (total, active, disabled, near-limit), account credit summary, and daily activity details

**Widget Design**
- Compact frameless transparent window (320x220)
- Always-on-top with configurable opacity (30%--100%)
- Drag-to-reposition with saved window state
- Close-to-tray behavior with system tray icon
- Light, dark, and system theme support
- Compact mode for reduced footprint

**Configuration**
- Refresh intervals: 30s, 1m, 2m, 5m with exponential backoff on failures
- History retention: 30, 90, 365 days, or unlimited
- UTC-based history tracking aligned with OpenRouter's daily reset at 00:00 UTC
- Auto-launch at startup
- Start minimized
- Show in taskbar
- Refresh on launch
- CSV export of usage history

**Security**
- API keys stored in Windows Credential Manager via the system keyring -- they never leave your device except for OpenRouter API calls
- No telemetry or analytics
- All data processed locally

---

## Screenshots

> Screenshots coming soon. The widget renders as a compact overlay on the desktop with a summary view and a toggleable 7-day spend chart.

---

## Installation

### Prerequisites

- **OpenRouter API key** -- sign up at [openrouter.ai](https://openrouter.ai)
- **Windows 10 or 11** with WebView2 runtime (pre-installed on most systems; if missing, download from [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/))
- For Windows builds: Rust/Cargo, Visual Studio C++ build tools, and the Tauri Windows prerequisites
- For Linux builds: Rust/Cargo, Node.js, `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, and `librsvg2-dev`

### Install the widget

1. Download the latest release installer (`.exe`) or standalone executable from the releases page.
2. Run the installer or launch the standalone executable.
3. On first launch, enter your OpenRouter API key in the setup screen.
4. The widget appears on your desktop and begins polling for usage data.

The NSIS installer can install per-user or system-wide because `installMode` is set to `both`. The installed program is normally under one of these locations:

- Per-user: `%LOCALAPPDATA%\Programs\openrouter-widget\`
- System-wide: `%ProgramFiles%\openrouter-widget\`

The standalone `.exe` can run from any folder and does not install itself. Application data is stored separately from the executable under `%APPDATA%`, so reinstalling or replacing the `.exe` does not automatically reset settings, credentials, or history.

---

## Configuration

All settings are accessible from the Settings window (right-click the system tray icon or click the gear icon on the widget).

| Setting | Options | Default | Description |
|---|---|---|---|
| Key Mode | `standard`, `management` | `standard` | Standard key for personal use; management key for account-wide dashboard |
| Refresh Interval | 30s, 1m, 2m, 5m | 1m | How often the widget polls the OpenRouter API |
| Theme | `system`, `light`, `dark` | `system` | UI color theme |
| Opacity | 30%--100% | 90% | Widget window transparency |
| Compact Mode | on / off | on | Reduces widget size |
| Always on Top | on / off | on | Keeps the widget above other windows |
| Close to Tray | on / off | on | Minimize to system tray instead of exiting |
| Launch at Startup | on / off | on | Start the widget automatically with your system |
| Start Minimized | on / off | on | Launch directly to the system tray |
| Show in Taskbar | on / off | off | Show the widget in the Windows taskbar |
| Refresh on Launch | on / off | on | Poll the API immediately when the app starts |
| Restore Position | on / off | on | Remember the last window position |
| History Retention | 30d, 90d, 365d, unlimited | 365d | How long to keep local usage history in SQLite |
| History Timezone | UTC only | UTC | OpenRouter resets daily usage at 00:00 UTC, so the widget uses UTC to keep stored totals and chart dates aligned with the API |
| Auto-Update | Check GitHub Releases | enabled | Checks NitroQ/openrouter-usage-widget releases every 15 days; downloads and installs a verified Windows x64 `.exe` or Linux x64 `.deb` |

---

### Update release requirements

Updates are accepted only from the `NitroQ/openrouter-usage-widget` GitHub repository. Supported packages must be named with the product, version, operating system, and architecture, for example `openrouter-widget_1.2.3_windows-x64.exe` or `openrouter-widget_1.2.3_linux-amd64.deb`. Each package must have a matching detached Ed25519 signature asset with the additional `.sig` suffix. The application rejects unsigned, ambiguous, malformed, unsupported, or oversized packages before installation.

## Architecture

```
openrouter-widget/
├── src/                          # React + TypeScript frontend
│   ├── app/                      # App component with router
│   ├── pages/                    # Widget, Details, Settings, Setup views
│   ├── components/               # Reusable UI components
│   ├── hooks/                    # useDashboard, useSettings
│   ├── lib/                      # Tauri command wrappers, formatters
│   └── types/                    # TypeScript type definitions
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── commands/             # Tauri command handlers
│   │   │   ├── app_state.rs      # Application state management
│   │   │   ├── credentials.rs    # Keyring credential operations
│   │   │   ├── dashboard.rs      # Dashboard data aggregation
│   │   │   ├── history.rs        # Usage history and CSV export
│   │   │   ├── settings.rs       # Settings persistence
│   │   │   └── windows.rs        # Window management
│   │   ├── openrouter/           # OpenRouter API client
│   │   │   ├── client.rs         # HTTP client (reqwest + rustls)
│   │   │   ├── standard.rs       # Standard key endpoints
│   │   │   ├── management.rs     # Management key endpoints
│   │   │   └── models.rs         # API response types
│   │   ├── storage/              # Local persistence layer
│   │   │   ├── credentials.rs    # Windows Credential Manager
│   │   │   ├── database.rs       # SQLite (rusqlite)
│   │   │   └── settings.rs       # JSON settings file
│   │   ├── tray.rs               # System tray setup
│   │   ├── error.rs              # Error types
│   │   ├── lib.rs                # Library root
│   │   └── main.rs               # Entry point
│   ├── migrations/               # SQLite schema migrations
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
└── vitest.config.ts
```

**Frontend stack:** React 19, TypeScript (strict mode), Vite 7, Tailwind CSS 4, React Router 7

**Backend stack:** Rust, Tauri 2, reqwest (rustls-tls), rusqlite (bundled), keyring (Windows native), serde

**Data flow:** The React frontend invokes Tauri commands, which handle API calls, local storage, and credential management on the Rust side. Settings are persisted as JSON on disk. Usage history is stored in a local SQLite database. API keys are stored in the OS credential manager and never written to disk in plaintext.

---

## OpenRouter API Endpoints

| Endpoint | Description | Key Type |
|---|---|---|
| `GET /api/v1/auth/key` | Key info (limit, usage, rate limit) | Standard |
| `GET /api/v1/credits` | Account credit balance | Standard |
| `GET /api/v1/keys` | List of managed keys | Management |
| `GET /api/v1/activity` | Usage activity history | Management |

---

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install) toolchain
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) (system dependencies, WebView2 on Windows)
- Recommended IDE: [VS Code](https://code.visualstudio.com/) with the [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extensions

### Getting started

```bash
git clone <repository-url>
cd openrouter-widget
npm install
npm run tauri dev
```

This starts the Vite dev server and opens the Tauri window with hot-reload for the frontend. Rust changes recompile automatically.

### Project scripts

| Command | Description |
|---|---|
| `npm run dev` | Start Vite dev server (frontend only) |
| `npm run tauri dev` | Start full Tauri development environment |
| `npm run tauri build` | Build production binaries |
| `npm run test` | Run frontend tests (Vitest) |
| `npm run test:watch` | Run frontend tests in watch mode |
| `cargo test` | Run Rust unit tests (from `src-tauri/`) |

---

## Build

### Production build

```bash
cd openrouter-widget
npm run tauri build
```

Output binaries are placed in `src-tauri/target/release/bundle/`.

### Reset local setup

The application stores setup state in the current Windows user profile. The recommended way to reset setup is from **Settings → Reset**:

- **Remove Key and Keep History** removes the API key, deactivates the credential profile, resets preferences, and preserves local SQL usage history.
- **Clear Key and History** removes the API key, deletes credential profiles and local usage data, resets preferences, and restarts the application on the initial setup screen.

Both actions display a confirmation warning before changing data. If the application cannot be opened, close the widget and remove these items manually:

- `%APPDATA%\openrouter-widget\settings.json`
- `%APPDATA%\com.nitroq.openrouter-usage-widget\openrouter-monitor.db`
- `%APPDATA%\com.nitroq.openrouter-usage-widget\.window-state.json`
- The Windows Credential Manager entry for the `openrouter-widget` service and `openrouter_api_key` account

Deleting the database removes local usage history. The API key must also be removed from Credential Manager because the app checks that store independently of the settings file. Replacing the executable does not reset user-profile data.

### Build targets

| Platform | Output |
|---|---|
| Windows | NSIS installer (`.exe`) and standalone executable |
| Linux | `.deb` package (Ubuntu/Debian-based; build on Linux) |

The NSIS installer supports both per-user and system-wide installation.

Build the standalone Windows executable and NSIS installer on Windows:

```bash
npm ci
npm run tauri build -- --bundles nsis
```

Outputs:

```text
src-tauri/target/release/openrouter-widget.exe
src-tauri/target/release/bundle/nsis/*.exe
```

Build the `.deb` package from a Linux machine, Linux CI runner, or compatible WSL environment with the Tauri Linux prerequisites installed:

```bash
npm ci
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
npm run tauri build -- --bundles deb
```

The package is written to `src-tauri/target/release/bundle/deb/`. A Windows environment cannot directly produce a Linux `.deb` package.

---
 Widget
## Testing

The project includes comprehensive test coverage across both the frontend and backend:

- **Rust backend:** Unit tests covering command handlers, API client logic, storage operations, and utility functions
- **Frontend:** 64 tests using Vitest and Testing Library covering components, hooks, formatters, setup/reset states, and type utilities
- **TypeScript:** strict mode enabled for full type safety

Run all tests:

```bash
# Frontend
npm run test

# Backend (from src-tauri/)
cargo test
```

---

## Security

- **Credential storage:** API keys are stored in the Windows Credential Manager via the OS-native keyring. They are never written to configuration files, environment variables, or any plaintext storage.
- **Local-only data:** All usage history, settings, and cached data remain on your local machine in SQLite and JSON files.
- **No telemetry:** The application does not phone home, collect analytics, or report usage statistics.
- **Network access:** The only outbound network requests are to the OpenRouter API (`openrouter.ai`) for the sole purpose of retrieving your account data.
- **Open source:** Full source code is available for audit.

---

## License

This is an open-source community project. See the repository for license details.
