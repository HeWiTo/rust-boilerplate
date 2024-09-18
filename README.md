# Rust SaaS Boilerplate

This is a comprehensive boilerplate built with Rust. It provides a solid foundation for building scalable, secure, and feature-rich applications.

## Features

- **Clean Architecture**: Organized for scalability and maintainability
- **Authentication**: JWT and Google OAuth support
- **Database Integration**: PostgreSQL and MongoDB support
- **API Documentation**: Auto-generated using OpenAPI (Swagger)
- **WebSocket Support**: Real-time communication capabilities
- **Multi-tenant**: Built-in support for multi-tenant applications
- **WASM Support**: Frontend components using Dioxus
- **OpenTelemetry**: Integrated for observability
- **Testing Structure**: Ready for unit, integration, and e2e tests
- **Workspace Management**: Cargo workspace for modular development
- **Error Handling**: Comprehensive error handling structure
- **Input Validation**: API input validation setup

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- MongoDB
- Node.js and npm (for frontend development)

## Getting Started

1. Clone the repository:

   ```
   git clone https://github.com/HeWiTo/rust-boilerplate.git
   cd rust-boilerplate
   ```

2. Set up the environment variables:

   ```
   cp .env.example .env
   ```

   Edit the `.env` file with your configuration.

3. Set up the database:

   ```
   cargo run --bin migrate
   ```

4. Run the application:

   ```
   cargo run
   ```

5. For frontend development:
   ```
   cd frontend
   npm install
   npm run dev
   ```

## Project Structure

```
saas-boilerplate/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── README.md
├── LICENSE
├── docker-compose.yml
├── crates/
│   ├── api/
│   ├── core/
│   ├── infrastructure/
│   └── wasm/
├── templates/
├── migrations/
├── tests/
├── docs/
├── scripts/
└── frontend/
```

## Documentation

API documentation is available at `http://localhost:8080/swagger-ui/` when the application is running.

## Testing

Run the test suite:

```
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/HeWiTo/rust-boilerplate/LICENSE) file for details.
