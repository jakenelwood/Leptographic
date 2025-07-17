# BLUEPRINT.md Workflow Integration

## 🎯 Single Command Philosophy

**One script. One workflow. Maximum simplicity.**

```bash
# Complete component generation
./scripts/blueprint.sh switch

# Phase-specific generation  
./scripts/blueprint.sh button I
./scripts/blueprint.sh dialog research
```

## 🚀 How It Works

### 1. Research Phase
- Generates comprehensive research prompt
- Uses @octocode and @context7 MCP servers
- Analyzes existing implementations

### 2. Generation Phases (I-IV)
- **Phase I**: Core architecture (30 min)
- **Phase II**: Production features (1-2 hours)  
- **Phase III**: Advanced composition (1-2 hours)
- **Phase IV**: Tailwind styling (1-2 hours)

### 3. Quality Pipeline
- Automatic cargo fmt/clippy/test
- Runs after each phase
- Fails fast on errors

## 💡 VS Code Integration

**Ctrl+Shift+P** → "Tasks: Run Task" → "Generate Component"

## 🎯 Result

From 7 scripts to 1. From complexity to simplicity. From confusion to clarity.

**"Everything should be made as simple as possible, but no simpler" - Einstein**
