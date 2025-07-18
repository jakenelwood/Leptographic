#!/bin/bash
# BLUEPRINT.md Universal Component Generator
# "Everything should be made as simple as possible, but no simpler" - Einstein

COMPONENT_NAME=$1
PHASE=${2:-"all"}

if [ -z "$COMPONENT_NAME" ]; then
    echo "🚀 BLUEPRINT.md Component Generator"
    echo ""
    echo "Usage:"
    echo "  $0 <component> [phase]"
    echo ""
    echo "Examples:"
    echo "  $0 switch           # Generate complete switch component"
    echo "  $0 button I         # Generate Phase I only"
    echo "  $0 dialog II        # Generate Phase II only"
    echo ""
    echo "Phases: I, II, III, IV, all (default)"
    exit 1
fi

echo "🏗️ BLUEPRINT.md Generator: $COMPONENT_NAME (Phase $PHASE)"

# Function: Generate research prompt
generate_research() {
    cat << EOF > /tmp/research_${COMPONENT_NAME}.md
# Research & Generation: $COMPONENT_NAME

## Automated Research
@octocode Search Radix UI primitives for "$COMPONENT_NAME" React implementation
@octocode Find RustForWeb/radix "$COMPONENT_NAME" if exists
@octocode Find Leptix "$COMPONENT_NAME" if exists  
@context7 Get WAI-ARIA patterns for "$COMPONENT_NAME"

## Analysis Needed
1. Existing Rust implementations quality
2. React → Leptos translation patterns
3. ARIA compliance requirements
4. Form integration needs

Generate comprehensive analysis before code generation.
EOF
    echo "📋 Research prompt: /tmp/research_${COMPONENT_NAME}.md"
}

# Function: Generate phase-specific prompts
generate_phase() {
    local phase=$1
    case $phase in
        "I")
            cat << EOF > /tmp/phase1_${COMPONENT_NAME}.md
# Phase I: Core Architecture - $COMPONENT_NAME

## Research (Automated)
@octocode Search Radix UI "$COMPONENT_NAME" React patterns
@context7 Get WAI-ARIA spec for "$COMPONENT_NAME"

## Generation (BLUEPRINT.md Phase I)
Using proven Phase I patterns:

1. **Critical Imports**
\`\`\`rust
use leptos::prelude::*;
use leptos::context::Provider;  // ← CRITICAL
\`\`\`

2. **State Management** (if stateful)
\`\`\`rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ${COMPONENT_NAME}State { /* ... */ }
\`\`\`

3. **Basic Component Structure**
- Context pattern for state sharing
- Controllable state management  
- Basic interaction handling

Generate Phase I following BLUEPRINT.md patterns exactly.
EOF
            ;;
        "II")
            cat << EOF > /tmp/phase2_${COMPONENT_NAME}.md
# Phase II: Production Features - $COMPONENT_NAME

## Enhancement Requirements
Apply BLUEPRINT.md Phase II patterns:

1. **Enhanced Props** - Form integration, events, accessibility
2. **WAI-ARIA Compliance** - Full keyboard navigation
3. **Form Integration** - Hidden inputs, validation support

Upgrade Phase I to Phase II following BLUEPRINT.md exactly.
EOF
            ;;
        "III")
            cat << EOF > /tmp/phase3_${COMPONENT_NAME}.md
# Phase III: Advanced Composition - $COMPONENT_NAME

## Advanced Requirements
Apply BLUEPRINT.md Phase III patterns:

1. **Advanced Props** - MaybeProp, as_child, NodeRef
2. **Event Composition** - Multiple handler composition
3. **Edge Cases** - Error handling, loading states

Upgrade to Phase III following BLUEPRINT.md patterns.
EOF
            ;;
        "IV")
            cat << EOF > /tmp/phase4_${COMPONENT_NAME}.md
# Phase IV: Tailwind Styling - $COMPONENT_NAME

## Styling Requirements
Apply BLUEPRINT.md Phase IV patterns:

1. **Data-Driven Styling**
\`\`\`rust
data-state=move || match state.get() {
    ComponentState::Active => "active",
    ComponentState::Inactive => "inactive",
}
\`\`\`

2. **Tailwind CSS 4** - data-[state=active]: selectors
3. **Professional Polish** - Transitions, focus states

Add complete styling following BLUEPRINT.md Phase IV.
EOF
            ;;
    esac
    echo "📋 Phase $phase prompt: /tmp/phase${phase}_${COMPONENT_NAME}.md"
}

# Function: Run quality pipeline
run_quality() {
    echo "🔧 Quality Pipeline"

    # Code formatting and basic fixes
    echo "📝 Formatting and basic fixes..."
    cargo fmt
    cargo fix --allow-dirty --allow-staged

    # Static analysis
    echo "🔍 Static analysis..."
    cargo clippy --all-targets -- -D warnings

    # Testing
    echo "🧪 Running tests..."
    cargo test

    # Performance benchmarking (if benches exist)
    echo "⚡ Performance benchmarking..."
    if [ -d "benches" ] || grep -q "\[\[bench\]\]" Cargo.toml 2>/dev/null; then
        cargo bench --no-run 2>/dev/null || echo "ℹ️  No benchmarks configured"
    else
        echo "ℹ️  No benchmarks found"
    fi

    # Documentation generation
    echo "📚 Generating documentation..."
    cargo doc --no-deps --document-private-items

    # Security scanning (Inner Loop)
    echo "🛡️ Security Scan (Inner Loop)"

    # Check for known vulnerabilities in dependencies
    echo "  🔍 Checking for known vulnerabilities..."
    cargo audit

    # Detect unsafe code usage
    echo "  ⚠️  Checking for unsafe code..."
    cargo geiger --deny-unsound || echo "⚠️  Unsafe code detected - review required"

    # Comprehensive dependency checks
    echo "  📦 Checking dependencies, licenses, and bans..."
    cargo deny check

    # Check for unused dependencies
    echo "  🧹 Checking for unused dependencies..."
    cargo machete

    echo "✅ Quality pipeline complete"
}

# Main execution
case $PHASE in
    "research")
        generate_research
        ;;
    "quality")
        echo "🔧 Running quality pipeline for $COMPONENT_NAME"
        run_quality
        ;;
    "I"|"II"|"III"|"IV")
        generate_phase $PHASE
        echo "Copy prompt to Augment Code, then press Enter..."
        read -p ""
        run_quality
        ;;
    "all")
        echo "🚀 Complete workflow for $COMPONENT_NAME"
        generate_research
        echo "1️⃣ Copy research prompt to Augment Code, then press Enter..."
        read -p ""

        for phase in I II III IV; do
            echo "${phase}️⃣ Generating Phase $phase..."
            generate_phase $phase
            echo "Copy Phase $phase prompt to Augment Code, then press Enter..."
            read -p ""
            run_quality
            if [ $? -ne 0 ]; then
                echo "❌ Phase $phase failed - fix issues before continuing"
                exit 1
            fi
            echo "✅ Phase $phase complete!"
        done
        echo "🎯 $COMPONENT_NAME complete!"
        ;;
    *)
        echo "❌ Invalid phase: $PHASE"
        echo "Valid phases: research, quality, I, II, III, IV, all"
        exit 1
        ;;
esac