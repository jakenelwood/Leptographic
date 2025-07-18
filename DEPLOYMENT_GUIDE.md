# 🚀 Leptographic.com Deployment Guide

This guide will help you deploy the Leptographic UI system to leptographic.com with SSL certificate.

## Prerequisites

- Ubuntu/Debian server with root access
- Domain `leptographic.com` pointing to your server IP
- SSH access to your server

## Step 1: Server Setup

### 1.1 Install Required Software

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Nginx
sudo apt install nginx -y

# Install Certbot for Let's Encrypt
sudo apt install certbot python3-certbot-nginx -y

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 1.2 Create Application Directory

```bash
sudo mkdir -p /var/www/leptographic
sudo chown root:root /var/www/leptographic
sudo mkdir -p /var/www/certbot
```

## Step 2: SSL Certificate Setup

### 2.1 Get Let's Encrypt Certificate

```bash
# Stop nginx temporarily
sudo systemctl stop nginx

# Get certificate
sudo certbot certonly --standalone -d leptographic.com -d www.leptographic.com

# Start nginx
sudo systemctl start nginx
```

### 2.2 Setup Auto-Renewal

```bash
# Test renewal
sudo certbot renew --dry-run

# Add to crontab for auto-renewal
echo "0 12 * * * /usr/bin/certbot renew --quiet" | sudo crontab -
```

## Step 3: Configure Nginx

### 3.1 Install Nginx Configuration

```bash
# Copy the nginx configuration
sudo cp nginx-leptographic.conf /etc/nginx/sites-available/leptographic.com

# Enable the site
sudo ln -s /etc/nginx/sites-available/leptographic.com /etc/nginx/sites-enabled/

# Remove default site
sudo rm -f /etc/nginx/sites-enabled/default

# Test configuration
sudo nginx -t

# Reload nginx
sudo systemctl reload nginx
```

## Step 4: Setup Systemd Service

### 4.1 Install Service File

```bash
# Copy service file
sudo cp leptographic.service /etc/systemd/system/

# Reload systemd
sudo systemctl daemon-reload

# Enable service
sudo systemctl enable leptographic
```

## Step 5: Deploy Application

### 5.1 Update Deployment Script

Edit `deploy.sh` and set your server IP:

```bash
SERVER_IP="YOUR_ACTUAL_SERVER_IP"
```

### 5.2 Make Script Executable and Run

```bash
chmod +x deploy.sh
./deploy.sh
```

## Step 6: Verify Deployment

### 6.1 Check Service Status

```bash
sudo systemctl status leptographic
```

### 6.2 Check Nginx Status

```bash
sudo systemctl status nginx
```

### 6.3 Test SSL Certificate

```bash
curl -I https://leptographic.com
```

## Step 7: Monitoring and Logs

### 7.1 View Application Logs

```bash
sudo journalctl -u leptographic -f
```

### 7.2 View Nginx Logs

```bash
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

## Troubleshooting

### Common Issues

1. **Port 3000 already in use**
   ```bash
   sudo lsof -i :3000
   sudo kill -9 <PID>
   ```

2. **Permission issues**
   ```bash
   sudo chown -R root:root /var/www/leptographic
   ```

3. **SSL certificate issues**
   ```bash
   sudo certbot certificates
   sudo certbot renew --force-renewal
   ```

## Security Checklist

- ✅ SSL certificate installed and auto-renewing
- ✅ HTTP redirects to HTTPS
- ✅ Security headers configured
- ✅ Application runs as non-root user
- ✅ Firewall configured (ports 80, 443, 22 only)

## Performance Optimizations

- ✅ Gzip compression enabled
- ✅ Static asset caching (1 year)
- ✅ HTTP/2 enabled
- ✅ Brotli compression for JS/CSS/WASM

## 🎯 Success!

Your Leptographic UI system should now be live at:
- **https://leptographic.com**
- **https://www.leptographic.com**

The deployment includes:
- ✅ Production-optimized Leptos build
- ✅ SSL certificate with auto-renewal
- ✅ Nginx reverse proxy with security headers
- ✅ Systemd service for automatic startup
- ✅ Performance optimizations
- ✅ Monitoring and logging setup
