# Leptos-Radix

A comprehensive Leptos component library implementing Radix UI patterns with enterprise-grade security and quality assurance.

## 🚀 Project Structure

```
Leptos-Radix/
├── leptos-radix-ui/           # Main component library
│   ├── src/components/        # Radix-compatible components
│   ├── Cargo.toml            # Rust dependencies
│   ├── deny.toml             # Security policies
│   └── dev-journal.md        # Development history
├── docs/                      # Documentation
│   ├── BLUEPRINT_ENHANCEMENT.md    # Development blueprint
│   ├── SECURITY_CHECKLIST.md       # Security guidelines
│   └── WORKFLOW_INTEGRATION.md     # Workflow guides
├── scripts/                   # Quality pipeline tools
│   ├── blueprint.sh          # 8-step quality pipeline
│   └── cleanup.sh            # Cleanup utilities
└── templates/                 # Generation templates
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
cd leptos-radix-ui
cargo run
```

### Quality Pipeline
```bash
./scripts/blueprint.sh
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
