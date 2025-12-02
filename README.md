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

### 🚧 Coming Soon

- **Rust** - Implementation in progress

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

**Single command Node.js:**
```bash
wget -qO- https://github.com/F4RAN/Clean-Code-Schema/archive/refs/heads/main.zip && unzip -q main.zip && cd Clean-Code-Schema-main/node && bash install.sh && npm start
```

**Single command Python:**
```bash
wget -qO- https://github.com/F4RAN/Clean-Code-Schema/archive/refs/heads/main.zip && unzip -q main.zip && cd Clean-Code-Schema-main/python && bash install.sh && source .venv/bin/activate && uvicorn main:app --reload
```

**Single command Rust:**
```bash
# Coming soon - Rust implementation in progress
```

### Alternative: Using Git Clone

If you prefer using git:

```bash
# Node.js
git clone https://github.com/F4RAN/Clean-Code-Schema.git && cd Clean-Code-Schema/node && bash install.sh && npm start

# Python
git clone https://github.com/F4RAN/Clean-Code-Schema.git && cd Clean-Code-Schema/python && bash install.sh && source .venv/bin/activate && uvicorn main:app --reload
```

### Detailed Setup

Each language implementation has its own README with detailed setup instructions:

- [Node.js Setup Guide](./node/README.md)
- [Python Setup Guide](./python/README.md)

## 🤝 Contributing

We welcome contributions for additional language implementations! If you'd like to add a Clean Architecture example in your favorite language, please:

1. **Fork the repository**
2. **Create a new directory** for your language (e.g., `rust/`, `go/`, `java/`, `csharp/`, etc.)
3. **Follow the same architecture pattern**:
   - Domain layer with Entity and Value Objects
   - Application layer with Use Cases and Ports
   - Infrastructure layer with MongoDB repository
   - Presentation layer with HTTP controller
4. **Create a README** following the same structure as existing implementations
5. **Submit a pull request**

### Guidelines for New Implementations

- ✅ Follow the same domain model and business rules
- ✅ Implement the same use case (CreateUser)
- ✅ Use MongoDB as the database (or provide clear instructions for alternatives)
- ✅ Include a comprehensive README with setup instructions
- ✅ Keep the implementation minimal and educational
- ✅ Follow language-specific best practices and conventions

### Languages We'd Love to See

- Rust 🦀
- Go 🐹
- Java ☕
- C# 🔷
- Ruby 💎
- PHP 🐘
- Kotlin 📱
- Swift 🍎
- Elixir 💧
- And more!

## 📋 Using This as a Template

To use this repository as a starting point for your project:

1. **Clone or fork this repository**: `git clone https://github.com/F4RAN/Clean-Code-Schema.git`
2. **Choose your preferred language** (Node.js, Python, or wait for Rust)
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

