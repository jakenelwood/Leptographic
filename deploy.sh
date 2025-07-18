#!/bin/bash

# Leptographic.com Deployment Script
# This script builds and deploys the Leptos Radix UI to leptographic.com

set -e  # Exit on any error

echo "🚀 Starting Leptographic.com deployment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOMAIN="leptographic.com"
SERVER_USER="root"  # Adjust as needed
SERVER_IP="178.156.167.70"  # Leptographic.com server
DEPLOY_PATH="/var/www/leptographic"
SERVICE_NAME="leptographic"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if server IP is set
if [ "$SERVER_IP" = "YOUR_SERVER_IP" ]; then
    print_error "Please set your server IP in the deploy.sh script"
    exit 1
fi

# Step 1: Build the project
print_status "Building Leptos project for production..."
cargo leptos build --release

if [ $? -eq 0 ]; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Step 2: Create deployment package
print_status "Creating deployment package..."
mkdir -p dist
cp -r target/site/* dist/
cp target/release/leptos-radix-ui dist/
cp Cargo.toml dist/

# Step 3: Upload to server
print_status "Uploading to server..."
rsync -avz --delete dist/ $SERVER_USER@$SERVER_IP:$DEPLOY_PATH/

# Step 4: Deploy on server
print_status "Deploying on server..."
ssh $SERVER_USER@$SERVER_IP << EOF
    cd $DEPLOY_PATH
    
    # Stop existing service if running
    sudo systemctl stop $SERVICE_NAME || true
    
    # Make binary executable
    chmod +x leptos-radix-ui
    
    # Start service
    sudo systemctl start $SERVICE_NAME
    sudo systemctl enable $SERVICE_NAME
    
    # Check status
    sudo systemctl status $SERVICE_NAME
EOF

print_success "Deployment completed!"
print_status "Your site should be available at: https://$DOMAIN"

# Cleanup
rm -rf dist

echo "🎯 Leptographic.com is now live!"
