# Contributing to Cleanux

Thank you for your interest in contributing to Cleanux!

## Setup Instructions

### Prerequisites

- **Rust** (latest stable) - [Install via rustup](https://rustup.rs/)
- **Node.js** 22+ - [Install via nodejs.org](https://nodejs.org/) or via nvm
- **Bun** - Install via: `curl -fsSL https://bun.sh/install | bash`

### Initial Setup

1. Clone the repository:

```bash
git clone https://github.com/rusnakdima/Cleanux.git
cd Cleanux
```

2. Install frontend dependencies:

```bash
bun install
```

3. Verify Rust toolchain:

```bash
rustc --version
cargo --version
```

## Development

### Running the Development Server

Start the frontend dev server:

```bash
bun run start
```

Or run the full Tauri app in dev mode:

```bash
bun run tauri:dev
```

### Building

Build frontend only:

```bash
bun run build
```

Build Tauri app:

```bash
bun run tauri:build
```

Build optimized (frontend + Tauri):

```bash
bun run build:optimized
```

## Testing

### Rust Tests

Run all Rust tests:

```bash
cd src-tauri && cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Run specific test:

```bash
cargo test test_app_error_io
```

### Frontend Tests

Currently, the project has limited frontend test infrastructure. Build check serves as the primary validation:

```bash
bun run ng build
```

## Code Quality

### Rust Formatting

Format Rust code:

```bash
cd src-tauri && cargo fmt
```

Check formatting:

```bash
cargo fmt --all -- --check
```

### Rust Linting

Run clippy:

```bash
cd src-tauri && cargo clippy -- -D warnings
```

### TypeScript Formatting

The project uses Prettier for TypeScript/Angular code. Configuration is in `.prettierrc`.

Format all files:

```bash
# Install prettier if needed
bun add -D prettier

# Format
bun prettier --write "src/**/*.{ts,html,scss}"
```

### Pre-commit Hooks

It's recommended to run before committing:

```bash
cargo fmt
cargo clippy -- -D warnings
bun run ng build
```

## Project Structure

```
Cleanux/
├── src/                    # Angular frontend
│   ├── app/               # Angular components and services
│   ├── assets/            # Static assets
│   └── environments/      # Environment configurations
├── src-tauri/             # Rust backend (Tauri)
│   ├── src/
│   │   ├── commands/      # Tauri command handlers
│   │   ├── errors/        # Error types
│   │   ├── helpers/       # Utility functions
│   │   ├── models/        # Data models
│   │   ├── routes/        # Route handlers
│   │   ├── security/      # Security utilities
│   │   └── services/      # Business logic
│   └── tests/             # Integration tests
├── .github/workflows/      # CI/CD pipelines
└── package.json           # Frontend dependencies
```

## Coding Standards

### Rust

- Follow Rust idioms and conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` before submitting
- Write tests for new functionality
- Keep functions focused and small

### TypeScript/Angular

- Use PascalCase for components and services
- Use camelCase for variables and functions
- Follow Angular style guide
- Use strict TypeScript where possible

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes
3. Run `cargo fmt` and `cargo clippy`
4. Ensure all tests pass
5. Submit a pull request with a clear description

## Common Issues

### Build Failures

If build fails, try:

```bash
cd src-tauri && cargo clean
cargo build
```

### Tauri Build Errors

Make sure you have the required system dependencies:

```bash
sudo apt-get install -y libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

### Node Module Issues

Remove node_modules and reinstall:

```bash
rm -rf node_modules bun.lock
bun install
```
