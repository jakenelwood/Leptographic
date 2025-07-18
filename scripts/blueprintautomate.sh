#!/bin/bash
# 🎯 Leptographic Hook-First Component Factory
# "Everything should be made as simple as possible, but no simpler" - Einstein
#
# 🚀 Hook-First Revolution: 10x faster development using proven hooks
# 🎯 For complete workflow: See master-workflow.md
# 🎯 For hook patterns: See BLUEPRINT.md

COMPONENT_NAME=$1
PHASE=${2:-"all"}

if [ -z "$COMPONENT_NAME" ]; then
    echo "🎯 Leptographic Hook-First Component Factory"
    echo ""
    echo "🚀 REVOLUTION: 10x faster development using proven hooks!"
    echo ""
    echo "📋 See master-workflow.md for complete instructions"
    echo ""
    echo "Usage:"
    echo "  $0 <component> [phase]"
    echo ""
    echo "Examples:"
    echo "  $0 switch           # Generate complete switch component (3 phases)"
    echo "  $0 button 0         # Generate Phase 0 (Hook Selection) only"
    echo "  $0 button I         # Generate Phase I (Component Composition) only"
    echo "  $0 dialog II        # Generate Phase II (Polish & Production) only"
    echo ""
    echo "SIMPLIFIED PHASES:"
    echo "  0  - Hook Selection (5-15 min) - Choose the right hooks"
    echo "  I  - Component Composition (15-30 min) - Compose hooks into component"
    echo "  II - Polish & Production (15-30 min) - Styling and final polish"
    echo "  deploy - Production Deployment - Deploy to live server"
    echo ""
    echo "AVAILABLE HOOKS:"
    echo "  ✅ use_controllable_state    - Universal state management"
    echo "  ✅ use_checkbox_state        - Complete checkbox logic"
    echo "  ✅ use_switch_state          - Complete switch logic"
    echo "  ✅ use_id_generator          - Unique IDs for accessibility"
    echo "  ✅ use_escape_key            - Handle escape key presses"
    echo "  ✅ use_previous              - Track previous values"
    echo ""
    echo "🚨 STYLING POLICY:"
    echo "  ✅ TAILWIND CSS 4 ONLY      - Data-driven utility classes"
    echo "  ❌ NO CUSTOM CSS            - No external stylesheets"
    echo "  ❌ NO INLINE STYLES         - No style= attributes"
    echo ""
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
        "0")
            cat << EOF > /tmp/phase0_${COMPONENT_NAME}.md
# Phase 0: Hook Selection - $COMPONENT_NAME

## Research & Analysis
@octocode Search Radix UI "$COMPONENT_NAME" React implementation
@context7 Get WAI-ARIA patterns for "$COMPONENT_NAME"

## Hook Selection Decision Tree
Based on component needs, select from our hook library:

**State Management:**
- [ ] use_controllable_state (universal pattern)
- [ ] use_${COMPONENT_NAME,,}_state (if exists in our library)
- [ ] Custom state hook needed?

**Interactions:**
- [ ] use_escape_key (close on escape)
- [ ] use_outside_click (close on outside click) - TODO: Fix NodeRef issues
- [ ] use_focus_trap (modal/dialog focus) - TODO: Fix NodeRef issues

**Utilities:**
- [ ] use_id_generator (accessibility IDs)
- [ ] use_previous (animations/transitions)

## Output
List of hooks to use and their configuration for $COMPONENT_NAME component.
EOF
            echo "📋 Phase 0 prompt: /tmp/phase0_${COMPONENT_NAME}.md"
            ;;
        "I")
            cat << EOF > /tmp/phase1_${COMPONENT_NAME}.md
# Phase I: Component Composition - $COMPONENT_NAME

## Hook Integration
Using hooks selected in Phase 0, create component following BLUEPRINT.md patterns:

**Template:**
\`\`\`rust
#[component]
pub fn ${COMPONENT_NAME^}(
    // Props based on selected hooks
    #[prop(into, optional)] checked: MaybeProp<T>,
    #[prop(into, optional)] default_checked: MaybeProp<T>,
    #[prop(into, optional)] on_checked_change: Option<Callback<T>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    // Phase 0: Compose hooks
    let state = use_${COMPONENT_NAME,,}_state(checked, default_checked, on_checked_change);
    let ids = use_related_ids("${COMPONENT_NAME,,}");

    // Context for child components
    let context_value = ${COMPONENT_NAME^}ContextValue {
        state: state.checked,
        disabled: disabled.into(),
    };

    view! {
        <Provider value=context_value>
            <button
                id=ids.trigger_id
                role="{aria_role}"
                aria-checked=move || state.get_aria_checked.get()
                data-state=move || state.get_state_attr.get()
                data-disabled=move || disabled.get().then_some("")
                disabled=move || disabled.get().unwrap_or(false)
                on:click=move |_| state.toggle.run(())
            >
                {children()}
            </button>
        </Provider>
    }
}
\`\`\`

## Success Criteria:
- [ ] Component compiles and renders
- [ ] Hooks provide all necessary functionality
- [ ] ARIA attributes are correct
- [ ] Context system works for child components
- [ ] Basic interactions work (click, keyboard)
EOF
            echo "📋 Phase I prompt: /tmp/phase1_${COMPONENT_NAME}.md"
            ;;
        "II")
            cat << EOF > /tmp/phase2_${COMPONENT_NAME}.md
# Phase II: Polish & Production - $COMPONENT_NAME

## 🚨 STYLING INTEGRATION - TAILWIND CSS 4 ONLY
**CRITICAL**: Use ONLY Tailwind CSS 4 utility classes. NO custom CSS allowed.

\`\`\`rust
// ✅ CORRECT: Tailwind CSS 4 data-driven styling
class="
    relative inline-flex h-5 w-5 items-center justify-center
    rounded border-2 border-gray-300 bg-white
    data-[state=checked]:border-blue-500 data-[state=checked]:bg-blue-500
    data-[state=unchecked]:border-gray-300 data-[state=unchecked]:bg-white
    data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed
    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
    transition-colors duration-150 ease-in-out
    hover:border-gray-400 data-[state=checked]:hover:bg-blue-600
"

// ❌ FORBIDDEN: Custom CSS, inline styles, external libraries
// style="custom: styles"  // NO
// class="custom-class"     // NO (unless Tailwind)
\`\`\`

## Form Integration
Add hidden input for form submission:

\`\`\`rust
// Hidden input for form submission
view! {
    <input
        type="hidden"
        name=move || name.get()
        value=move || state.get_form_value.get()
        form=move || form.get()
    />
}
\`\`\`

## Child Components
Create indicator/content child components using context:

\`\`\`rust
#[component]
pub fn ${COMPONENT_NAME^}Indicator(children: ChildrenFn) -> impl IntoView {
    let context = expect_context::<${COMPONENT_NAME^}ContextValue>();

    view! {
        <Show when=move || context.state.get() == SomeState::Active>
            <div data-state=move || /* context-driven state */>
                {children()}
            </div>
        </Show>
    }
}
\`\`\`

## Success Criteria:
- [ ] 🚨 TAILWIND CSS 4 ONLY - No custom CSS anywhere
- [ ] Styling responds to all states using data-[state=*]: selectors
- [ ] Form integration works
- [ ] Child components use context correctly
- [ ] Component is production-ready

EOF
            echo "📋 Phase II prompt: /tmp/phase2_${COMPONENT_NAME}.md"
            ;;
        *)
            echo "❌ Invalid phase: $phase"
            echo "Valid phases: 0, I, II"
            exit 1
            ;;
    esac
}

# Main execution
case $PHASE in
    "0")
        generate_phase "0"
        ;;
    "I")
        generate_phase "I"
        ;;
    "II")
        generate_phase "II"
        ;;
    "all")
        echo "🚀 Generating all phases for $COMPONENT_NAME..."
        generate_research
        generate_phase "0"
        generate_phase "I"
        generate_phase "II"
        echo ""
        echo "✅ All prompts generated!"
        echo "📋 Research: /tmp/research_${COMPONENT_NAME}.md"
        echo "📋 Phase 0: /tmp/phase0_${COMPONENT_NAME}.md"
        echo "📋 Phase I: /tmp/phase1_${COMPONENT_NAME}.md"
        echo "📋 Phase II: /tmp/phase2_${COMPONENT_NAME}.md"
        echo ""
        echo "🎯 Next Steps:"
        echo "1. Copy prompts to Augment Code"
        echo "2. Augment Code will use our hook library"
        echo "3. Run quality pipeline: cargo fmt && cargo clippy && cargo test"
        echo "4. Deploy production-ready component in 1-2 hours!"
        ;;
    "deploy")
        echo "🚀 PRODUCTION DEPLOYMENT"
        echo ""
        echo "📋 Pre-deployment Checklist:"
        echo "✅ All components tested and working"
        echo "✅ Responsive design validated"
        echo "✅ Theme transitions smooth"
        echo "✅ Quality pipeline passed"
        echo ""
        echo "🔧 Deployment Commands:"
        echo "# Build for production"
        echo "cargo leptos build --release"
        echo ""
        echo "# Deploy to server (if configured)"
        echo "./deploy.sh"
        echo ""
        echo "# Verify deployment"
        echo "curl -I https://your-domain.com"
        echo ""
        echo "📊 Post-deployment Validation:"
        echo "- SSL certificate valid"
        echo "- Performance metrics good"
        echo "- All components functional"
        echo "- Mobile responsiveness working"
        echo ""
        echo "🌐 Live Example: https://leptographic.com"
        ;;
    *)
        echo "❌ Invalid phase: $PHASE"
        echo "Valid phases: 0, I, II, all, deploy"
        exit 1
        ;;
esac
