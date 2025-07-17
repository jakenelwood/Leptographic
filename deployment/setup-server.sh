#!/bin/bash

# Leptographic Server Setup Script
# Run this script on your new server (178.156.150.128) to set up the deployment environment

set -e

echo "🚀 Setting up Leptographic deployment environment..."

# Update system packages
echo "📦 Updating system packages..."
sudo apt update && sudo apt upgrade -y

# Install required packages
echo "🔧 Installing required packages..."
sudo apt install -y nginx curl wget git certbot python3-certbot-nginx

# Create application directory
echo "📁 Creating application directory..."
sudo mkdir -p /var/www/leptographic
sudo chown www-data:www-data /var/www/leptographic

# Copy systemd service file
echo "⚙️ Setting up systemd service..."
sudo cp leptographic.service /etc/systemd/system/
sudo systemctl daemon-reload

# Setup Nginx configuration (temporary HTTP-only for SSL setup)
echo "🌐 Configuring Nginx..."
sudo cp nginx.conf /etc/nginx/sites-available/leptographic
sudo ln -sf /etc/nginx/sites-available/leptographic /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default

# Create temporary HTTP-only config for SSL certificate generation
sudo tee /etc/nginx/sites-available/leptographic-temp > /dev/null << 'EOF'
server {
    listen 80;
    server_name leptographic.com www.leptographic.com;

    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    location / {
        return 301 https://$server_name$request_uri;
    }
}
EOF

sudo ln -sf /etc/nginx/sites-available/leptographic-temp /etc/nginx/sites-enabled/leptographic
sudo nginx -t
sudo systemctl enable nginx
sudo systemctl restart nginx

# Generate SSL certificate
echo "🔒 Generating SSL certificate..."
sudo certbot --nginx -d leptographic.com -d www.leptographic.com --non-interactive --agree-tos --email admin@leptographic.com

# Switch to full SSL configuration
echo "🔧 Switching to SSL configuration..."
sudo ln -sf /etc/nginx/sites-available/leptographic /etc/nginx/sites-enabled/leptographic
sudo nginx -t
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
echo "   - NEW_SERVER_HOST: 178.156.150.128"
echo "   - NEW_SERVER_USER: root"
echo "   - NEW_SERVER_SSH_KEY: your-private-ssh-key"
echo ""
echo "2. Push to main branch to trigger deployment"
echo ""
echo "3. Access your application at: https://leptographic.com"
echo ""
echo "🔒 SSL Certificate: Automatically configured with Let's Encrypt"
echo "🌐 Domain: leptographic.com (with www redirect)"
