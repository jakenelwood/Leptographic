# 🚀 blueprint.sh User Manual for Dummies

**The complete step-by-step guide to generating production-ready Leptos components**

## 🎯 Overview: Who Does What

| **Tool** | **Responsibility** |
|----------|-------------------|
| **blueprint.sh** | Generates research prompts, runs quality pipeline |
| **Augment Code** | Executes MCP research, generates Rust code |
| **VS Code** | Real-time error checking, auto-formatting, IntelliSense |
| **You** | Copy prompts, review code, fix issues, make decisions |

---

## 📋 **Complete Workflow: Step-by-Step**

### **Step 1: Start Component Generation**

**Command:**
```bash
./scripts/blueprint.sh <component-name> [phase]
```

**Examples:**
```bash
./scripts/blueprint.sh switch           # Complete workflow (all phases)
./scripts/blueprint.sh button I         # Phase I only
./scripts/blueprint.sh dialog research  # Research only
```

**What Happens:**
- ✅ Script generates research prompt in `/tmp/research_<component>.md`
- ✅ Script pauses and waits for you

---

### **Step 2: Execute Research in Augment Code**

**Your Action:**
1. Open `/tmp/research_<component>.md` 
2. **Copy the entire contents**
3. **Paste into Augment Code chat**
4. **Press Enter**

**Example Research Prompt:**
```markdown
# Research & Generation: switch

## Automated Research
@octocode Search Radix UI primitives for "switch" React implementation
@octocode Find RustForWeb/radix "switch" if exists
@octocode Find Leptix "switch" if exists  
@context7 Get WAI-ARIA patterns for "switch"

## Analysis Needed
1. Existing Rust implementations quality
2. React → Leptos translation patterns
3. ARIA compliance requirements
4. Form integration needs

Generate comprehensive analysis before code generation.
```

**What Augment Code Does:**
- ✅ **@octocode** searches GitHub for Radix UI, RustForWeb, Leptix implementations
- ✅ **@context7** fetches WAI-ARIA specifications
- ✅ **Analyzes** all findings and provides comprehensive research summary
- ✅ **Recommends** implementation approach

**What to Watch For:**
- ✅ Research results from all 4 sources (Radix UI, RustForWeb, Leptix, WAI-ARIA)
- ✅ Clear implementation recommendations
- ✅ Identified patterns and gotchas

**Go Back to Terminal:** Press Enter to continue to Phase I

---

### **Step 3: Phase I - Core Architecture (30 min)**

**Your Action:**
1. Script generates `/tmp/phase1_<component>.md`
2. **Copy the entire contents**
3. **Paste into Augment Code chat**
4. **Add this context:** "Generate Phase I following BLUEPRINT.md patterns exactly."

**Example Phase I Prompt:**
```markdown
# Phase I: Core Architecture - switch

## Research (Automated)
@octocode Search Radix UI "switch" React patterns
@context7 Get WAI-ARIA spec for "switch"

## Generation (BLUEPRINT.md Phase I)
Using proven Phase I patterns:

1. **Critical Imports**
```rust
use leptos::prelude::*;
use leptos::context::Provider;  // ← CRITICAL
```

2. **State Management** (if stateful)
```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SwitchState { /* ... */ }
```

3. **Basic Component Structure**
- Context pattern for state sharing
- Controllable state management  
- Basic interaction handling

Generate Phase I following BLUEPRINT.md patterns exactly.
```

**What Augment Code Does:**
- ✅ **Generates** basic component structure
- ✅ **Creates** context pattern for state sharing
- ✅ **Implements** basic interaction handling
- ✅ **Follows** proven Phase I patterns from BLUEPRINT.md

**What VS Code Does (Automatically):**
- ✅ **rust-analyzer** shows real-time errors/warnings
- ✅ **Auto-imports** suggest missing dependencies
- ✅ **Format on save** keeps code clean
- ✅ **Clippy** highlights potential issues

**What to Watch For:**
- ✅ Component compiles without errors
- ✅ Basic functionality works
- ✅ Context pattern implemented
- ✅ No red squiggles in VS Code

**Go Back to Terminal:** Press Enter to run quality pipeline

---

### **Step 4: Quality Pipeline (Automatic)**

**What blueprint.sh Does:**
```bash
cargo fmt                                    # Format code
cargo fix --allow-dirty --allow-staged      # Auto-fix issues
cargo clippy --all-targets -- -D warnings   # Check for warnings
cargo test                                   # Run tests
```

**What to Watch For:**
- ✅ **All commands succeed** (green checkmarks)
- ❌ **Any failures** = Fix before continuing

**If Quality Pipeline Fails:**

1. **Check VS Code** for rust-analyzer errors
2. **Fix compilation errors** first
3. **Address clippy warnings:**
   ```bash
   cargo clippy --fix --allow-dirty --allow-staged
   ```
4. **Re-run quality pipeline:**
   ```bash
   cargo fmt && cargo clippy && cargo test
   ```

---

### **Step 5: Phase II - Production Features (1-2 hours)**

**Your Action:**
1. Script generates `/tmp/phase2_<component>.md`
2. **Copy the entire contents**
3. **Paste into Augment Code chat**
4. **Add this context:** "Upgrade Phase I to Phase II following BLUEPRINT.md exactly."

**What Augment Code Does:**
- ✅ **Enhances props** for form integration
- ✅ **Adds WAI-ARIA compliance** 
- ✅ **Implements keyboard navigation**
- ✅ **Adds form integration** with hidden inputs

**Repeat Quality Pipeline** → Continue to Phase III

---

### **Step 6: Phase III - Advanced Composition (1-2 hours)**

**Your Action:**
1. Script generates `/tmp/phase3_<component>.md`
2. **Copy the entire contents**
3. **Paste into Augment Code chat**
4. **Add this context:** "Upgrade to Phase III following BLUEPRINT.md patterns."

**What Augment Code Does:**
- ✅ **Advanced props** (MaybeProp, as_child, NodeRef)
- ✅ **Event composition** for multiple handlers
- ✅ **Edge case handling** (errors, loading states)

**Repeat Quality Pipeline** → Continue to Phase IV

---

### **Step 7: Phase IV - Tailwind Styling (1-2 hours)**

**Your Action:**
1. Script generates `/tmp/phase4_<component>.md`
2. **Copy the entire contents**
3. **Paste into Augment Code chat**
4. **Add this context:** "Add complete styling following BLUEPRINT.md Phase IV."

**What Augment Code Does:**
- ✅ **Data-driven styling** with state attributes
- ✅ **Tailwind CSS 4** selectors (`data-[state=active]:`)
- ✅ **Professional polish** (transitions, focus states)

**Final Quality Pipeline** → Component Complete!

---

## 🔧 **Managing Feedback from Tools**

### **rust-analyzer Errors (VS Code)**
**What You See:** Red squiggles, error messages in Problems panel

**How to Help Augment Code:**
1. **Copy the exact error message**
2. **Paste into Augment Code chat**
3. **Add context:** "Fix this rust-analyzer error: [error message]"

**Example:**
```
Fix this rust-analyzer error: 
cannot find function `expect_context` in this scope
help: consider importing this function: `use leptos::context::expect_context;`
```

### **Clippy Warnings**
**What You See:** Yellow warnings in VS Code, clippy output in terminal

**How to Help Augment Code:**
1. **Run clippy to get full output:**
   ```bash
   cargo clippy --all-targets
   ```
2. **Copy warning messages**
3. **Paste into Augment Code chat**
4. **Add context:** "Fix these clippy warnings: [warnings]"

### **Compilation Errors**
**What You See:** Red errors in terminal, failed cargo build

**How to Help Augment Code:**
1. **Copy the full error output**
2. **Include file path and line numbers**
3. **Paste into Augment Code chat**
4. **Add context:** "Fix this compilation error: [error]"

---

## 🎯 **Context Formatting for Augment Code**

### **For Research Phase:**
```
[Paste entire research prompt]

Additional context: I'm building a Leptos component following BLUEPRINT.md patterns. Focus on React → Leptos translation patterns and identify any existing Rust implementations.
```

### **For Code Generation:**
```
[Paste phase prompt]

Additional context: 
- Use Leptos 0.8.2 API
- Follow BLUEPRINT.md Phase [X] patterns exactly
- Ensure rust-analyzer compatibility
- Target file: src/components/[component].rs
```

### **For Error Fixing:**
```
Fix this [rust-analyzer/clippy/compilation] error:

[Paste exact error message]

Context:
- Component: [component name]
- Phase: [current phase]
- File: [file path]
- Goal: [what you're trying to achieve]
```

---

## 🚨 **Troubleshooting Guide**

### **Script Won't Run**
```bash
chmod +x scripts/blueprint.sh
./scripts/blueprint.sh
```

### **Quality Pipeline Fails**
1. **Check rust-analyzer** in VS Code first
2. **Fix compilation errors** before clippy
3. **Run commands individually:**
   ```bash
   cargo fmt
   cargo clippy --fix --allow-dirty --allow-staged
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```

### **Augment Code Doesn't Understand**
- ✅ **Be specific** about what phase you're in
- ✅ **Include file paths** and line numbers
- ✅ **Copy exact error messages**
- ✅ **Mention BLUEPRINT.md patterns**

### **VS Code Issues**
- ✅ **Restart rust-analyzer:** Ctrl+Shift+P → "rust-analyzer: Restart"
- ✅ **Check settings:** Ensure `.vscode/settings.json` is correct
- ✅ **Reload window:** Ctrl+Shift+P → "Developer: Reload Window"

---

## 🎯 **Success Indicators**

### **Research Phase Success:**
- ✅ Found implementations in 3+ sources
- ✅ Clear implementation strategy
- ✅ Identified patterns and gotchas

### **Phase I Success:**
- ✅ Component compiles without errors
- ✅ Basic functionality works
- ✅ No rust-analyzer red squiggles
- ✅ Quality pipeline passes

### **Phase II Success:**
- ✅ WAI-ARIA compliance implemented
- ✅ Form integration working
- ✅ Keyboard navigation functional
- ✅ All Phase I features preserved

### **Phase III Success:**
- ✅ Advanced composition patterns
- ✅ External state control works
- ✅ Event composition functional
- ✅ Edge cases handled

### **Phase IV Success:**
- ✅ Professional styling applied
- ✅ State-driven CSS working
- ✅ Transitions and focus states
- ✅ Production-ready appearance

---

## 🚀 **Quick Reference Commands**

```bash
# Complete workflow
./scripts/blueprint.sh switch

# Phase-specific
./scripts/blueprint.sh button I
./scripts/blueprint.sh dialog II

# Research only
./scripts/blueprint.sh accordion research

# Quality check
cargo fmt && cargo clippy && cargo test

# VS Code task
Ctrl+Shift+P → "Tasks: Run Task" → "Generate Component"
```

---

## 💡 **Pro Tips**

1. **Always run research first** - Don't skip to Phase I
2. **Fix errors immediately** - Don't accumulate technical debt
3. **Use VS Code tasks** for quick component generation
4. **Copy exact error messages** when asking for help
5. **Trust the quality pipeline** - If it passes, you're good
6. **Review generated code** before moving to next phase
7. **Keep terminal and VS Code open** side by side

**Remember**: The script is your conductor, Augment Code is your code generator, VS Code is your quality checker, and you are the decision maker! 🎯