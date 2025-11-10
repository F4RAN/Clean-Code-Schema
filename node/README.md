# Cheatsheet
- ✅ Domain (Entity + Value Object)
  - Business Rules (Pure english)
- ✅ Application (Use Case + Port)
- ✅ Infrastructure (Fake Repository)
- ✅ Presentation (Simple controller function)

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

- Port: *UserRepository* is an interface which specify user's model behaviour without the implementation.
e.g. save(user) or findById(id)

- UseCase: *CreateUser* is an usecase which use the UserRepository's methods to add some functionalities on domain area.

> *Ports (Repositories)* in fact are an interface between the *Usecases* and *Entities*

---

### 3) Infrastructure

*MongoUserRepository* in fact implements the UserRepository methods. The *UserRepository* specifies **WHAT to do** and *MongoUserRepository* specifies **HOW to do it with MongoDB**.

> You can swap MongoDB for PostgreSQL, MySQL, or an in-memory store without causing any change in application or domain layer.
---

### 4) Presentation

*createUserController* connects the framework to the *CreateUser* Usecase so **Controllers** use the UseCases to implement their functionality

---

### 5) Aggregation (main.ts)

1) Get DB client (e.g. *MongoClient*)
2) Create infrastructure repo (*MongoUserRepository*)
3) Pass *MongoUserRepository*'s instance to *CreateUser* UseCase (It's possible because that infrastructure repository implements port repository)
4) Pass the *CreateUser* UseCase to the *createUserController*
5) Controller execute the UseCase

---


<img width="1024" height="1024" alt="image" src="https://github.com/user-attachments/assets/6e030a90-98a8-4a22-95c2-4a32dbed8865" />


