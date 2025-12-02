#!/bin/bash
set -e

REPO="https://raw.githubusercontent.com/F4RAN/Clean-Code-Schema/main"
DIR="node"

echo "🚀 Downloading CC-Ex Node.js project..."

# Create directory structure
mkdir -p $DIR/{application/{ports,usecases},config,domain/user/{entities,value_objects},infrastructure/dbs,presentation/http}

# Download files
wget -q "$REPO/$DIR/main.ts" -O "$DIR/main.ts"
wget -q "$REPO/$DIR/package.json" -O "$DIR/package.json"
wget -q "$REPO/$DIR/install.sh" -O "$DIR/install.sh"
wget -q "$REPO/$DIR/README.md" -O "$DIR/README.md"

# Application layer
wget -q "$REPO/$DIR/application/ports/UserRepository.ts" -O "$DIR/application/ports/UserRepository.ts"
wget -q "$REPO/$DIR/application/usecases/CreateUser.ts" -O "$DIR/application/usecases/CreateUser.ts"

# Config
wget -q "$REPO/$DIR/config/db.ts" -O "$DIR/config/db.ts"

# Domain layer
wget -q "$REPO/$DIR/domain/user/entities/User.ts" -O "$DIR/domain/user/entities/User.ts"
wget -q "$REPO/$DIR/domain/user/value_objects/Age.ts" -O "$DIR/domain/user/value_objects/Age.ts"
wget -q "$REPO/$DIR/domain/user/value_objects/ID.ts" -O "$DIR/domain/user/value_objects/ID.ts"
wget -q "$REPO/$DIR/domain/user/value_objects/PhoneNumber.ts" -O "$DIR/domain/user/value_objects/PhoneNumber.ts"
wget -q "$REPO/$DIR/domain/user/value_objects/Role.ts" -O "$DIR/domain/user/value_objects/Role.ts"
wget -q "$REPO/$DIR/domain/user/value_objects/Sex.ts" -O "$DIR/domain/user/value_objects/Sex.ts"

# Infrastructure layer
wget -q "$REPO/$DIR/infrastructure/dbs/MongoUserRepository.ts" -O "$DIR/infrastructure/dbs/MongoUserRepository.ts"

# Presentation layer
wget -q "$REPO/$DIR/presentation/http/CreateUserController.ts" -O "$DIR/presentation/http/CreateUserController.ts"

chmod +x "$DIR/install.sh"

echo "✅ Download complete! Running installation..."
cd $DIR && bash install.sh

