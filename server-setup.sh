#!/bin/bash

# Server Setup Script for Leptographic.com
# Run this on your server to prepare for deployment

set -e

echo "🔧 Setting up server for Leptographic.com..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Update system
print_status "Updating system packages..."
sudo apt update && sudo apt upgrade -y

# Install Nginx
print_status "Installing Nginx..."
sudo apt install nginx -y

# Install Certbot
print_status "Installing Certbot for SSL certificates..."
sudo apt install certbot python3-certbot-nginx -y

# Create directories
print_status "Creating application directories..."
sudo mkdir -p /var/www/leptographic
sudo mkdir -p /var/www/certbot
sudo chown www-data:www-data /var/www/leptographic

# Configure firewall
print_status "Configuring firewall..."
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw --force enable

# Get SSL certificate
print_status "Getting SSL certificate..."
read -p "Enter your domain (e.g., leptographic.com): " DOMAIN

# Stop nginx temporarily for standalone certificate
sudo systemctl stop nginx

# Get certificate
sudo certbot certonly --standalone -d $DOMAIN -d www.$DOMAIN

# Start nginx
sudo systemctl start nginx

# Setup auto-renewal
print_status "Setting up SSL auto-renewal..."
echo "0 12 * * * /usr/bin/certbot renew --quiet" | sudo crontab -

print_success "Server setup completed!"
print_status "Next steps:"
echo "1. Copy nginx-leptographic.conf to /etc/nginx/sites-available/$DOMAIN"
echo "2. Copy leptographic.service to /etc/systemd/system/"
echo "3. Run your deployment script"
echo ""
echo "🎯 Server is ready for Leptographic deployment!"
