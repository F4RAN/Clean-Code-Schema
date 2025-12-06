# CC-Ex: Clean Architecture Examples

A multi-language repository demonstrating Clean Architecture principles with practical implementations. Each language implementation follows the same domain model and architecture patterns, making it easy to compare approaches across different ecosystems.

## 🎯 Purpose

**This project is designed to be a starting point for your own projects.** 

Use this repository as a template or boilerplate to kickstart your applications with Clean Architecture (also known as Hexagonal Architecture or Ports & Adapters) already in place. Each implementation provides a minimal, production-ready structure that you can extend and customize for your specific needs.

The goal is to help developers:
- **Start new projects quickly** with a proven architecture pattern
- **Understand Clean Architecture** through practical, working examples
- **Compare implementations** across different programming languages
- **Learn best practices** for structuring maintainable applications

## 📚 Implementations

### ✅ Available

- **[Node.js / TypeScript](./node/README.md)**
  - Framework: Express.js
  - Database: MongoDB
  - Features: Full async/await support with native MongoDB driver

- **[Python](./python/README.md)**
  - Framework: FastAPI
  - Database: MongoDB (PyMongo)
  - Features: Async support with sync-to-async decorator pattern (prototyping approach)

- **[Rust](./rust/README.md)**
  - Framework: Axum
  - Database: MongoDB
  - Features: Full async/await support with async-trait for trait objects

## 🏗️ Architecture

All implementations follow the same Clean Architecture structure:

```
Entity -> Port -> Usecase -> Infrastructure -> Controller
```

### Layers

- **Domain**: Entities and Value Objects with business rules
- **Application**: Use Cases and Ports (interfaces)
- **Infrastructure**: Concrete implementations (MongoDB repositories)
- **Presentation**: Controllers and HTTP handlers

### Architecture Diagram

<img width="1024" height="1024" alt="Clean Architecture Diagram" src="https://github.com/user-attachments/assets/6e030a90-98a8-4a22-95c2-4a32dbed8865" />

## 📖 Domain Model

All implementations share the same domain model:

### Entity
- **User**: Represents a user in the system

### Value Objects
- **ID**: Sequential integer identifier
- **PhoneNumber**: 10-digit phone number
- **Sex**: Male or Female
- **Role**: Admin, User, or Guest
- **Age**: Integer from 0 to 120

### Use Case
- **CreateUser**: Creates a new user with validation

## 🚀 Quick Start

### Prerequisites

- **MongoDB** running on `localhost:27017` (or update connection strings in config files)

### Single Command Installation

**Single command Node.js (using wget):**
```bash
wget -qO- https://raw.githubusercontent.com/F4RAN/Clean-Code-Schema/main/bootstrap-node.sh | bash
```

**Single command Python (using wget):**
```bash
wget -qO- https://raw.githubusercontent.com/F4RAN/Clean-Code-Schema/main/bootstrap-python.sh | bash
```

**Single command Rust (using wget):**
```bash
wget -qO- https://raw.githubusercontent.com/F4RAN/Clean-Code-Schema/main/bootstrap-rust.sh | bash
```

### Alternative: Using Git Sparse-Checkout

If you prefer using git (only downloads the selected language directory):

```bash
# Node.js
git clone --filter=blob:none --sparse https://github.com/F4RAN/Clean-Code-Schema.git && cd Clean-Code-Schema && git sparse-checkout init --cone && git sparse-checkout set node && cd node && bash install.sh

# Python
git clone --filter=blob:none --sparse https://github.com/F4RAN/Clean-Code-Schema.git && cd Clean-Code-Schema && git sparse-checkout init --cone && git sparse-checkout set python && cd python && bash install.sh

# Rust
git clone --filter=blob:none --sparse https://github.com/F4RAN/Clean-Code-Schema.git && cd Clean-Code-Schema && git sparse-checkout init --cone && git sparse-checkout set rust && cd rust && bash install.sh
```

**Note:** After installation, you can customize the code before running. To start the server:
- **Node.js**: `npm start` (runs on port 3000)
- **Python**: `source .venv/bin/activate && uvicorn main:app --reload` (runs on port 8000)
- **Rust**: `cargo run` (runs on port 3000)

### Detailed Setup

Each language implementation has its own README with detailed setup instructions:

- [Node.js Setup Guide](./node/README.md)
- [Python Setup Guide](./python/README.md)
- [Rust Setup Guide](./rust/README.md)

## 🧪 Testing

All implementations expose the same API endpoint and can be tested using the same curl commands. The only difference is the port number:

- **Node.js**: `http://localhost:3000`
- **Python**: `http://localhost:8000` (FastAPI default)
- **Rust**: `http://localhost:3000`

### Create User

#### Basic Request
```bash
# Node.js or Rust (port 3000)
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "1234567890",
    "role": "user",
    "age": 25,
    "sex": "male"
  }'

# Python (port 8000)
curl -X POST http://localhost:8000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "1234567890",
    "role": "user",
    "age": 25,
    "sex": "male"
  }'
```

#### One-liner
```bash
# Node.js or Rust
curl -X POST http://localhost:3000/create_user -H "Content-Type: application/json" -d '{"phone_number":"1234567890","role":"user","age":25,"sex":"male"}'

# Python
curl -X POST http://localhost:8000/create_user -H "Content-Type: application/json" -d '{"phone_number":"1234567890","role":"user","age":25,"sex":"male"}'
```

#### Create Admin User
```bash
# Node.js or Rust
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"9876543210","role":"admin","age":30,"sex":"female"}'

# Python
curl -X POST http://localhost:8000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"9876543210","role":"admin","age":30,"sex":"female"}'
```

### Validation Error Examples

All implementations return `400 Bad Request` for validation errors:

#### Invalid Phone Number (not 10 digits)
```bash
# Node.js or Rust
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"123","role":"user","age":25,"sex":"male"}'

# Python
curl -X POST http://localhost:8000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"123","role":"user","age":25,"sex":"male"}'
```

#### Invalid Age (over 120)
```bash
# Node.js or Rust
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"user","age":150,"sex":"male"}'

# Python
curl -X POST http://localhost:8000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"user","age":150,"sex":"male"}'
```

#### Invalid Role
```bash
# Node.js or Rust
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"invalid_role","age":25,"sex":"male"}'

# Python
curl -X POST http://localhost:8000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"invalid_role","age":25,"sex":"male"}'
```

#### Invalid Sex
```bash
# Node.js or Rust
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"user","age":25,"sex":"invalid"}'

# Python
curl -X POST http://localhost:8000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"user","age":25,"sex":"invalid"}'
```

### Pretty Print Response (with jq)

If you have `jq` installed, you can pretty-print the JSON response:

```bash
# Node.js or Rust
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"user","age":25,"sex":"male"}' | jq .

# Python
curl -X POST http://localhost:8000/create_user \
  -H "Content-Type: application/json" \
  -d '{"phone_number":"1234567890","role":"user","age":25,"sex":"male"}' | jq .
```

## 🤝 Contributing

We welcome contributions for additional language implementations! **All contributions must follow the same story and development sequence** as the existing implementations.

### The Story: EPUIC Sequence

Every implementation in this repository follows a strict development sequence called **EPUIC**:

```
Entity -> Port -> Usecase -> Infrastructure -> Controller
```

This sequence ensures consistency across all language implementations and demonstrates Clean Architecture principles.

### The Problem

All implementations solve the same problem:
> **Build a minimal Clean Architecture setup for creating a user with validation and repository storage.**

### The Story Structure

When contributing a new language implementation, you **must** follow this exact story:

#### 1) Entity (Domain Layer)
Start with the domain model and business rules:

**Entities:**
- User has an ID
- User has a phone number
- User has a sex
- User has a role
- User has an age

**Value Objects:**
- ID is a sequential integer
- PhoneNumber is a 10-digit field
- Sex is male or female
- Role can be admin/user/guest
- Age is an integer from 0 to 120

#### 2) Port (Application Layer)
Define the interface without implementation:
- **Port**: `UserRepository` interface/abstract class that specifies user model behavior (e.g., `save(user)`, `findById(id)`)
- **UseCase**: `CreateUser` use case that uses `UserRepository` methods to add functionality in the domain area

> **Key Point**: Ports (Repositories) are interfaces between UseCases and Entities

#### 3) Infrastructure
Implement the concrete repository:
- `MongoUserRepository` implements the `UserRepository` methods
- `UserRepository` specifies **WHAT to do**, `MongoUserRepository` specifies **HOW to do it with MongoDB**
- You can swap MongoDB for PostgreSQL, MySQL, or an in-memory store without changing the application or domain layer

#### 4) Presentation
Connect the framework to the use case:
- Controller connects the framework (Express, FastAPI, etc.) to the `CreateUser` UseCase
- Controllers use UseCases to implement their functionality

#### 5) Aggregation (main file)
Wire everything together:
1. Get DB client (e.g., `MongoClient`)
2. Create infrastructure repo (`MongoUserRepository`)
3. Pass `MongoUserRepository` instance to `CreateUser` UseCase
4. Pass `CreateUser` UseCase to the controller
5. Controller executes the UseCase

### Contribution Steps

1. **Fork the repository**
2. **Create a new directory** for your language (e.g., `rust/`, `go/`, `java/`, `csharp/`, etc.)
3. **Follow the EPUIC sequence** exactly as described above
4. **Implement the same domain model and business rules** (User entity with the same value objects)
5. **Implement the same use case** (`CreateUser`)
6. **Use MongoDB** as the database (or provide clear instructions for alternatives)
7. **Create a README** following the same structure as existing implementations, including:
   - The EPUIC sequence explanation
   - The same business rules
   - The same problem statement
   - Setup instructions
8. **Submit a pull request**

### Guidelines for New Implementations

- ✅ **Must follow EPUIC sequence**: Entity -> Port -> Usecase -> Infrastructure -> Controller
- ✅ **Must follow the same domain model**: User entity with ID, PhoneNumber, Sex, Role, Age value objects
- ✅ **Must implement the same use case**: CreateUser with the same business rules
- ✅ **Must use MongoDB** (or provide clear alternatives)
- ✅ **Must include comprehensive README** with the same structure
- ✅ **Must keep implementation minimal and educational**
- ✅ **Must follow language-specific best practices** and conventions

**Important**: Review the existing [Node.js](./node/README.md), [Python](./python/README.md), and [Rust](./rust/README.md) implementations to understand the exact pattern and story structure before contributing.

### Implementation Status

- [x] **Node.js / TypeScript** ✅
- [x] **Python** ✅
- [x] **Rust** ✅
- [ ] **Go** 🐹
- [ ] **Java** ☕
- [ ] **C#** 🔷
- [ ] **C** 
- [ ] **Ruby** 💎
- [ ] **PHP** 🐘
- [ ] **Kotlin** 📱
- [ ] **Swift** 🍎
- [ ] **Elixir** 💧
- [ ] **Dart** 🎯
- [ ] **Scala** ⚡

## 📋 Using This as a Template

To use this repository as a starting point for your project:

1. **Clone or fork this repository**: `git clone https://github.com/F4RAN/Clean-Code-Schema.git`
2. **Choose your preferred language** (Node.js, Python, or Rust)
3. **Copy the implementation** to your new project
4. **Customize the domain model** - Replace User entity and value objects with your own
5. **Extend the use cases** - Add your business logic following the same patterns
6. **Swap infrastructure** - Change MongoDB to your preferred database if needed
7. **Add features** - Build upon the clean architecture foundation

The structure is designed to scale - start simple and grow your application while maintaining clean boundaries between layers.

## 📝 License

This project is open source and available for educational purposes.

## 🙏 Acknowledgments

This repository is designed to help developers learn Clean Architecture principles through practical, language-agnostic examples and serve as a solid foundation for new projects.

---

**Happy Learning & Building! 🎓🚀**




[Medium Doc](https://medium.com/@parsa.hassani30/clean-architecture-across-languages-why-the-epuic-sequence-changes-everything-d66b70bcffa5)
[Linkedin Post](https://www.linkedin.com/posts/f4ran_github-f4rancc-ex-activity-7402686889226559488-OiJ7https://medium.com/@parsa.hassani30/clean-architecture-across-languages-why-the-epuic-sequence-changes-everything-d66b70bcffa5/)


