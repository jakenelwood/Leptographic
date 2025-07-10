#!/bin/bash

# Leptos Radix UI Server Setup Script
# Run this script on your Hetzner server (5.78.68.209) to set up the deployment environment

set -e

echo "🚀 Setting up Leptos Radix UI deployment environment..."

# Update system packages
echo "📦 Updating system packages..."
sudo apt update && sudo apt upgrade -y

# Install required packages
echo "🔧 Installing required packages..."
sudo apt install -y nginx curl wget git

# Create application directory
echo "📁 Creating application directory..."
sudo mkdir -p /var/www/leptos-radix-ui
sudo chown www-data:www-data /var/www/leptos-radix-ui

# Copy systemd service file
echo "⚙️ Setting up systemd service..."
sudo cp leptos-radix-ui.service /etc/systemd/system/
sudo systemctl daemon-reload

# Setup Nginx configuration
echo "🌐 Configuring Nginx..."
sudo cp nginx.conf /etc/nginx/sites-available/leptos-radix-ui
sudo ln -sf /etc/nginx/sites-available/leptos-radix-ui /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default

# Test Nginx configuration
sudo nginx -t

# Enable and start Nginx
sudo systemctl enable nginx
sudo systemctl restart nginx

# Setup firewall (if ufw is available)
if command -v ufw &> /dev/null; then
    echo "🔒 Configuring firewall..."
    sudo ufw allow 22/tcp
    sudo ufw allow 80/tcp
    sudo ufw allow 443/tcp
    sudo ufw --force enable
fi

echo "✅ Server setup complete!"
echo ""
echo "Next steps:"
echo "1. Add GitHub secrets to your repository:"
echo "   - HETZNER_HOST: 5.78.68.209"
echo "   - HETZNER_USER: your-username"
echo "   - HETZNER_SSH_KEY: your-private-ssh-key"
echo ""
echo "2. Push to main branch to trigger deployment"
echo ""
echo "3. Access your application at: http://5.78.68.209"
