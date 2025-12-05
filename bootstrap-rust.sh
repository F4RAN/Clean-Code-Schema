#!/bin/bash
set -e

REPO="https://raw.githubusercontent.com/F4RAN/Clean-Code-Schema/main"
DIR="rust"

echo "🚀 Downloading CC-Ex Rust project..."

# Create directory structure
mkdir -p $DIR/src/{application/{ports,usecases},config,domain/{entities,value_objects},infrastracture/dbs,presentation/http}

# Download files
wget -q "$REPO/$DIR/Cargo.toml" -O "$DIR/Cargo.toml"
wget -q "$REPO/$DIR/install.sh" -O "$DIR/install.sh"
wget -q "$REPO/$DIR/README.md" -O "$DIR/README.md"

# Main
wget -q "$REPO/$DIR/src/main.rs" -O "$DIR/src/main.rs"

# Application layer
wget -q "$REPO/$DIR/src/application/mod.rs" -O "$DIR/src/application/mod.rs"
wget -q "$REPO/$DIR/src/application/ports/mod.rs" -O "$DIR/src/application/ports/mod.rs"
wget -q "$REPO/$DIR/src/application/ports/user_repository.rs" -O "$DIR/src/application/ports/user_repository.rs"
wget -q "$REPO/$DIR/src/application/usecases/mod.rs" -O "$DIR/src/application/usecases/mod.rs"
wget -q "$REPO/$DIR/src/application/usecases/create_user.rs" -O "$DIR/src/application/usecases/create_user.rs"

# Config
wget -q "$REPO/$DIR/src/config/mod.rs" -O "$DIR/src/config/mod.rs"
wget -q "$REPO/$DIR/src/config/db.rs" -O "$DIR/src/config/db.rs"

# Domain layer
wget -q "$REPO/$DIR/src/domain/mod.rs" -O "$DIR/src/domain/mod.rs"
wget -q "$REPO/$DIR/src/domain/entities/mod.rs" -O "$DIR/src/domain/entities/mod.rs"
wget -q "$REPO/$DIR/src/domain/entities/user.rs" -O "$DIR/src/domain/entities/user.rs"
wget -q "$REPO/$DIR/src/domain/value_objects/mod.rs" -O "$DIR/src/domain/value_objects/mod.rs"
wget -q "$REPO/$DIR/src/domain/value_objects/age.rs" -O "$DIR/src/domain/value_objects/age.rs"
wget -q "$REPO/$DIR/src/domain/value_objects/id.rs" -O "$DIR/src/domain/value_objects/id.rs"
wget -q "$REPO/$DIR/src/domain/value_objects/phone_number.rs" -O "$DIR/src/domain/value_objects/phone_number.rs"
wget -q "$REPO/$DIR/src/domain/value_objects/role.rs" -O "$DIR/src/domain/value_objects/role.rs"
wget -q "$REPO/$DIR/src/domain/value_objects/sex.rs" -O "$DIR/src/domain/value_objects/sex.rs"

# Infrastructure layer
wget -q "$REPO/$DIR/src/infrastracture/mod.rs" -O "$DIR/src/infrastracture/mod.rs"
wget -q "$REPO/$DIR/src/infrastracture/dbs/mod.rs" -O "$DIR/src/infrastracture/dbs/mod.rs"
wget -q "$REPO/$DIR/src/infrastracture/dbs/mongo_user_repository.rs" -O "$DIR/src/infrastracture/dbs/mongo_user_repository.rs"

# Presentation layer
wget -q "$REPO/$DIR/src/presentation/mod.rs" -O "$DIR/src/presentation/mod.rs"
wget -q "$REPO/$DIR/src/presentation/http/mod.rs" -O "$DIR/src/presentation/http/mod.rs"
wget -q "$REPO/$DIR/src/presentation/http/create_user_controller.rs" -O "$DIR/src/presentation/http/create_user_controller.rs"

chmod +x "$DIR/install.sh"

echo "✅ Download complete! Running installation..."
cd $DIR && bash install.sh

