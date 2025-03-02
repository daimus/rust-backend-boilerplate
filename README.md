# Rust Backend Boilerplate 🍳
A robust and scalable backend boilerplate built with Rust and Axum, designed to kickstart your next project with best practices in mind.

## Features
- **Axum**: A lightweight, fast, and ergonomic web framework for Rust.
- **Clean Architecture**: Clear separation of concerns to ensure maintainability and scalability.
- **OpenAPI**: Automatically generated API documentation for seamless integration.
- **ORM**: Database abstraction for simplified and type-safe querying.
- **Authentication**: Secure your routes using JWKS (JSON Web Key Set) for robust authentication.

## Installation

1. Clone the repository:
   ```sh
   git clone <repo_url>
   cd <project_name>
   ```
2. Run environment support:
   ```sh
   docker compose up -d
   ```
3. Run database migration:
   ```sh
   diesel migrate run
   ```
4. Run unit test:
   ```sh
   cargo test
   ```
5. Run the server:
   ```sh
   cargo run
   ```
## TODO
### Core Features
- [x] Implement authentication & authorization
- [x] Unit test
- [x] Enhance database migrations
- [x] API versioning support
- [ ] Add request validation middleware
- [ ] Improve logging & monitoring
- [ ] Implement caching mechanism
- [ ] Implement configuration management
- [ ] Rate limiting & security hardening
- [ ] Role-based access control (RBAC)
- [ ] Support multi-environment configurations
- [ ] Improve error handling strategy
- [ ] Implement background job processing
- [ ] WebSocket or SSE support