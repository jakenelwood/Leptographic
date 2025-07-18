# Leptos-Radix

A comprehensive Leptos component library implementing Radix UI patterns with enterprise-grade security and quality assurance.

## 🚀 Project Structure

```
Leptos-Radix/                  # Project root
├── src/                       # Source code
│   ├── components/            # Radix-compatible components
│   ├── lib.rs                # Library exports
│   └── main.rs               # Application entry point
├── docs/                      # Documentation
│   ├── BLUEPRINT_USER_MANUAL.md    # Complete development guide
│   ├── SECURITY_CHECKLIST.md       # Security guidelines
│   └── WORKFLOW_INTEGRATION.md     # Workflow guides
├── scripts/                   # Quality pipeline tools
│   └── blueprintautomate.sh   # 8-step quality pipeline
├── templates/                 # Generation templates
├── public/                    # Static assets
├── style/                     # CSS and styling
├── Cargo.toml                # Rust dependencies
├── deny.toml                 # Security policies
├── package.json              # Node.js dependencies (Tailwind)
├── README.md                 # This file
├── BLUEPRINT.md              # Development blueprint
├── dev-journal.md            # Development history
└── PHASE_III_TRANSITION_STATUS.md  # Current development status
```

## 🛡️ Security & Quality Features

- **8-step quality pipeline**: format, lint, test, security, docs
- **Security scanning**: cargo-audit, cargo-geiger, cargo-deny, cargo-machete
- **Dependency management**: Comprehensive security policies
- **AI-generated code verification**: Security checklist for all AI contributions

## 🎨 Components

### Checkbox
- Radix-compatible styling and behavior
- Clean unchecked state with subtle hover effects
- Accessibility-compliant focus handling
- Modern Tailwind CSS integration

## 🔧 Development

### Quick Start
```bash
# From project root
cargo run
```

### Quality Pipeline
```bash
./scripts/blueprintautomate.sh
```

### Security Scan
```bash
cargo audit
cargo geiger
cargo deny check
```

## 📚 Documentation

- [Blueprint Enhancement Guide](docs/BLUEPRINT_ENHANCEMENT.md)
- [Security Checklist](docs/SECURITY_CHECKLIST.md)
- [Workflow Integration](docs/WORKFLOW_INTEGRATION.md)

## 🎯 Mission

Building commercial-grade Leptos components with enterprise security standards and comprehensive quality assurance for modern web applications.

---

**Ocean's 11 Level Security** 🕴️💎 - Every component is a heist-worthy masterpiece!
