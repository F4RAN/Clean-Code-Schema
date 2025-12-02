#!/bin/bash
set -e

REPO="https://raw.githubusercontent.com/F4RAN/Clean-Code-Schema/main"
DIR="python"

echo "🚀 Downloading CC-Ex Python project..."

# Create directory structure
mkdir -p $DIR/{application/{ports,usecases},config,domain/{entities,value_objects},infrastructure/{dbs,utils},presentation/http}

# Download files
wget -q "$REPO/$DIR/main.py" -O "$DIR/main.py"
wget -q "$REPO/$DIR/requirements.txt" -O "$DIR/requirements.txt"
wget -q "$REPO/$DIR/install.sh" -O "$DIR/install.sh"
wget -q "$REPO/$DIR/README.md" -O "$DIR/README.md"

# Application layer
wget -q "$REPO/$DIR/application/ports/user_repository.py" -O "$DIR/application/ports/user_repository.py"
wget -q "$REPO/$DIR/application/usecases/create_user.py" -O "$DIR/application/usecases/create_user.py"

# Config
wget -q "$REPO/$DIR/config/db.py" -O "$DIR/config/db.py"

# Domain layer
wget -q "$REPO/$DIR/domain/__init__.py" -O "$DIR/domain/__init__.py"
wget -q "$REPO/$DIR/domain/entities/__init__.py" -O "$DIR/domain/entities/__init__.py"
wget -q "$REPO/$DIR/domain/entities/user.py" -O "$DIR/domain/entities/user.py"
wget -q "$REPO/$DIR/domain/value_objects/__init__.py" -O "$DIR/domain/value_objects/__init__.py"
wget -q "$REPO/$DIR/domain/value_objects/age.py" -O "$DIR/domain/value_objects/age.py"
wget -q "$REPO/$DIR/domain/value_objects/id.py" -O "$DIR/domain/value_objects/id.py"
wget -q "$REPO/$DIR/domain/value_objects/phone_number.py" -O "$DIR/domain/value_objects/phone_number.py"
wget -q "$REPO/$DIR/domain/value_objects/role.py" -O "$DIR/domain/value_objects/role.py"
wget -q "$REPO/$DIR/domain/value_objects/sex.py" -O "$DIR/domain/value_objects/sex.py"

# Infrastructure layer
wget -q "$REPO/$DIR/infrastructure/dbs/mongo_user_repository.py" -O "$DIR/infrastructure/dbs/mongo_user_repository.py"
wget -q "$REPO/$DIR/infrastructure/utils/__init__.py" -O "$DIR/infrastructure/utils/__init__.py"
wget -q "$REPO/$DIR/infrastructure/utils/decorators.py" -O "$DIR/infrastructure/utils/decorators.py"

# Presentation layer
wget -q "$REPO/$DIR/presentation/http/create_user_controller.py" -O "$DIR/presentation/http/create_user_controller.py"

chmod +x "$DIR/install.sh"

echo "✅ Download complete! Running installation..."
cd $DIR && bash install.sh

