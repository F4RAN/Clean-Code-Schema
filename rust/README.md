# Cheatsheet
- ✅ Domain (Entity + Value Object)
  - Business Rules (Pure english)
- ✅ Application (Use Case + Port)
- ✅ Infrastructure (MongoDB Repository)
- ✅ Presentation (Axum controller)

# Problem
Build a minimal Clean Architecture setup for creating a user with validation and repository storage.

# Solution

## Sequence
### EPUIC
Development sequence must be like this

Entity -> Port -> Usecase -> Infrastructure -> Controller

---

### 1) Entity

Business Rules:
-  *Entities*
1) User has an ID
2) User has a phone number
3) User has a sex
4) User has a role
5) User has an age

- *Value Objects*
1) ID is a sequencial integer
2) PhoneNumber is a 10 digit field
3) Sex is male or female
4) Role can be admin/user/guest
5) Age is an integer from 0 to 120

--- 

### 2) Port

- Port: *UserRepository* is a trait which specify user's model behaviour without the implementation.
e.g. save(user) or find_by_id(id)

- UseCase: *CreateUser* is an usecase which use the UserRepository's methods to add some functionalities on domain area.

> *Ports (Repositories)* in fact are an interface between the *Usecases* and *Entities*

---

### 3) Infrastructure

*MongoUserRepository* in fact implements the UserRepository methods. The *UserRepository* specifies **WHAT to do** and *MongoUserRepository* specifies **HOW to do it with MongoDB**.

> You can swap MongoDB for PostgreSQL, MySQL, or an in-memory store without causing any change in application or domain layer.

#### Async Traits

This project uses `async-trait` crate to enable async methods in traits. This is required because Rust's async traits are not yet stable in the standard library. The `async-trait` crate provides a macro that transforms async methods into ones that return boxed futures, making them compatible with trait objects (`dyn Trait`).

**How it works:**
```rust
#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> Result<User, String>;
}
```

The `#[async_trait]` macro transforms the async method to return `Pin<Box<dyn Future<Output = ...>>>`, making it compatible with `Box<dyn UserRepository>`.

---

### 4) Presentation

*create_user_controller* connects Axum to the *CreateUser* Usecase so **Controllers** use the UseCases to implement their functionality

The controller follows a higher-order function pattern (similar to Python/Node implementations), where it takes the use case and returns a handler function that Axum can use.

---

### 5) Aggregation (main.rs)

1) Get DB client (e.g. *MongoClient*)
2) Create infrastructure repo (*MongoUserRepository*)
3) Pass *MongoUserRepository*'s instance to *CreateUser* UseCase (It's possible because that infrastructure repository implements port repository)
4) Pass the *CreateUser* UseCase to the *create_user_controller*
5) Controller execute the UseCase

---

## 🚀 Setup

### Prerequisites

- **Rust** (latest stable version)
- **MongoDB** running on `localhost:27017`

### Installation

1. Navigate to the rust directory:
```bash
cd rust
```

2. Build the project:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:3000`

---

## 🧪 Testing with cURL

### Create User

#### Basic Request
```bash
curl -X POST http://localhost:3000/create_user \
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
curl -X POST http://localhost:3000/create_user -H "Content-Type: application/json" -d '{"phone_number":"1234567890","role":"user","age":25,"sex":"male"}'
```

#### Create Admin User
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "9876543210",
    "role": "admin",
    "age": 30,
    "sex": "female"
  }'
```

#### Create Guest User
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "5555555555",
    "role": "guest",
    "age": 18,
    "sex": "male"
  }'
```

### Validation Error Examples

#### Invalid Phone Number (not 10 digits)
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "123",
    "role": "user",
    "age": 25,
    "sex": "male"
  }'
```
**Expected Response:** `400 Bad Request`

#### Invalid Age (over 120)
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "1234567890",
    "role": "user",
    "age": 150,
    "sex": "male"
  }'
```
**Expected Response:** `400 Bad Request`

#### Invalid Age (negative)
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "1234567890",
    "role": "user",
    "age": -5,
    "sex": "male"
  }'
```
**Expected Response:** `400 Bad Request`

#### Invalid Role
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "1234567890",
    "role": "invalid_role",
    "age": 25,
    "sex": "male"
  }'
```
**Expected Response:** `400 Bad Request`

#### Invalid Sex
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "1234567890",
    "role": "user",
    "age": 25,
    "sex": "invalid"
  }'
```
**Expected Response:** `400 Bad Request`

#### Phone Number with Non-Digit Characters
```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "123456789a",
    "role": "user",
    "age": 25,
    "sex": "male"
  }'
```
**Expected Response:** `400 Bad Request`

### Pretty Print Response (with jq)

If you have `jq` installed, you can pretty-print the JSON response:

```bash
curl -X POST http://localhost:3000/create_user \
  -H "Content-Type: application/json" \
  -d '{
    "phone_number": "1234567890",
    "role": "user",
    "age": 25,
    "sex": "male"
  }' | jq .
```

---

## 📦 Dependencies

- **axum**: Web framework for Rust
- **tokio**: Async runtime
- **mongodb**: MongoDB driver for Rust
- **serde**: Serialization/deserialization framework
- **async-trait**: Enables async methods in traits

---

CC Architecture

<img width="1024" height="1024" alt="image" src="https://github.com/user-attachments/assets/6e030a90-98a8-4a22-95c2-4a32dbed8865" />

---

CC Implementation

<img width="1024" height="1536" alt="image" src="https://github.com/user-attachments/assets/301ca17e-04a9-4fa6-a5ea-c1e5daf16d22" />

