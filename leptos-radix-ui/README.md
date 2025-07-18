# Leptos-Radix

A comprehensive Leptos component library implementing Radix UI patterns with enterprise-grade security and quality assurance.

## 🚀 Project Structure

```
leptos-radix-ui/               # Consolidated project root
├── src/components/            # Radix-compatible components
├── docs/                      # Documentation
│   ├── BLUEPRINT_USER_MANUAL.md    # Complete development guide
│   ├── SECURITY_CHECKLIST.md       # Security guidelines
│   └── WORKFLOW_INTEGRATION.md     # Workflow guides
├── scripts/                   # Quality pipeline tools
│   └── blueprintautomate.sh   # 8-step quality pipeline
├── templates/                 # Generation templates
├── Cargo.toml                # Rust dependencies
├── deny.toml                 # Security policies
├── dev-journal.md            # Development history
├── README.md                 # This file
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
# From project root (leptos-radix-ui/)
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

- [Blueprint User Manual](docs/BLUEPRINT_USER_MANUAL.md) - Complete development guide with security enhancements
- [Security Checklist](docs/SECURITY_CHECKLIST.md)
- [Workflow Integration](docs/WORKFLOW_INTEGRATION.md)
- [Phase III Transition Status](PHASE_III_TRANSITION_STATUS.md) - Current development status

## 🎯 Mission

Building commercial-grade Leptos components with enterprise security standards and comprehensive quality assurance for modern web applications.

---

**Ocean's 11 Level Security** 🕴️💎 - Every component is a heist-worthy masterpiece!
