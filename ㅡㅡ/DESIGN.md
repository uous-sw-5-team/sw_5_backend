---
name: Chronos Study System
colors:
  surface: '#f8f9fa'
  surface-dim: '#d9dadb'
  surface-bright: '#f8f9fa'
  surface-container-lowest: '#ffffff'
  surface-container-low: '#f3f4f5'
  surface-container: '#edeeef'
  surface-container-high: '#e7e8e9'
  surface-container-highest: '#e1e3e4'
  on-surface: '#191c1d'
  on-surface-variant: '#44474e'
  inverse-surface: '#2e3132'
  inverse-on-surface: '#f0f1f2'
  outline: '#75777f'
  outline-variant: '#c5c6cf'
  surface-tint: '#4e5e81'
  primary: '#000000'
  on-primary: '#ffffff'
  primary-container: '#081b3a'
  on-primary-container: '#7384a8'
  inverse-primary: '#b6c6ee'
  secondary: '#006a6a'
  on-secondary: '#ffffff'
  secondary-container: '#9deeed'
  on-secondary-container: '#0b6e6e'
  tertiary: '#000000'
  on-tertiary: '#ffffff'
  tertiary-container: '#171c1e'
  on-tertiary-container: '#808587'
  error: '#ba1a1a'
  on-error: '#ffffff'
  error-container: '#ffdad6'
  on-error-container: '#93000a'
  primary-fixed: '#d8e2ff'
  primary-fixed-dim: '#b6c6ee'
  on-primary-fixed: '#081b3a'
  on-primary-fixed-variant: '#364768'
  secondary-fixed: '#a0f0f0'
  secondary-fixed-dim: '#84d4d3'
  on-secondary-fixed: '#002020'
  on-secondary-fixed-variant: '#004f4f'
  tertiary-fixed: '#dfe3e6'
  tertiary-fixed-dim: '#c3c7ca'
  on-tertiary-fixed: '#171c1e'
  on-tertiary-fixed-variant: '#42484a'
  background: '#f8f9fa'
  on-background: '#191c1d'
  surface-variant: '#e1e3e4'
  error-red: '#ba1a1a'
  success-teal: '#006a6a'
typography:
  display-lg:
    fontFamily: Inter
    fontSize: 48px
    fontWeight: '700'
    lineHeight: 56px
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '600'
    lineHeight: 40px
    letterSpacing: -0.01em
  headline-lg-mobile:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
  title-md:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
  body-lg:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
  body-sm:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  label-md:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '600'
    lineHeight: 16px
    letterSpacing: 0.05em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  unit: 8px
  gutter: 32px
  margin-mobile: 16px
  margin-desktop: 40px
  container-max-width: 1536px
---

## Brand & Style
The Chronos Study System embodies a **Modern Corporate** aesthetic with a focus on high-utility and clarity. It is designed for students and educators who require a reliable, structured environment to manage complex schedules. 

The brand personality is authoritative yet encouraging, utilizing a professional "Fidelity" color strategy that balances deep, trust-evoking navies with vibrant, energetic teals. The visual language is disciplined—relying on a strict grid and clear typographic hierarchy—to reduce cognitive load during intensive planning sessions.

## Colors
The palette is rooted in a high-contrast functional hierarchy. 
- **Primary (#031635):** A deep midnight navy used for core branding, primary actions, and major headings to establish authority.
- **Secondary (#006a6a):** A vibrant teal used for status indicators, active states (like the current date selection), and completion metaphors.
- **Neutral Surface System:** Employs a multi-tiered grayscale (`#ffffff` to `#f3f4f5`) to separate the global background from interactive containers.
- **Semantic Accents:** Use "Error Red" (`#ba1a1a`) sparingly for alerts or critical deadlines (e.g., Sunday indicators).

## Typography
The system uses **Inter** exclusively to maintain a utilitarian, Swiss-inspired clarity. 
- **Headlines:** Use tight letter spacing and heavy weights (600-700) to create a strong visual anchor.
- **Labels:** Utilize `label-md` with increased letter spacing (0.05em) and uppercase styling for auxiliary information like calendar headers or badges.
- **Body:** Standardized at 16px for primary reading and 14px for metadata/descriptions to ensure legibility across dense data views.

## Layout & Spacing
The system follows a **Fixed Grid** philosophy for the main content canvas, centering the container at a maximum width of 1536px.
- **Grid System:** Uses a 12-column logic. On desktop, the layout splits into an 8-column main area (Calendar) and a 4-column side panel (Tasks). 
- **Gutter & Rhythm:** A generous 32px gutter separates major sections, while internal component spacing is strictly derived from an 8px base unit.
- **Breakpoints:** On mobile, the grid collapses to a single column with 16px side margins. Headlines scale down to `headline-lg-mobile` to maintain balance.

## Elevation & Depth
Depth is communicated through **Tonal Layering** combined with subtle **Ambient Shadows**.
- **Base:** The `background` (#f8f9fa) acts as the lowest layer.
- **Surface:** Main cards use `surface-container-lowest` (#ffffff) with a `shadow-sm` or `shadow-md` to provide separation.
- **Interactive Depth:** Floating elements (like the Date Picker overlay) utilize `shadow-2xl` to indicate high-z-index priority.
- **Focus States:** Selected items (e.g., the current day) use a 2px inset ring of the secondary color rather than a shadow, maintaining a clean, architectural feel.

## Shapes
The shape language is **Rounded**, conveying approachability without sacrificing the professional tone.
- **Standard Containers:** Use `0.75rem` (12px) for cards and section blocks.
- **Large Components:** Use `1rem` (16px) for the main calendar and task sections.
- **Interactive Elements:** Buttons and input fields use `0.5rem` (8px), while badges and specific icons (like "Prev/Next") use `full` (pill) rounding.

## Components
- **Buttons:** Primary buttons feature a solid fill (`primary`) with white text. Secondary/Ghost buttons use `primary` text with a `surface-container-low` hover state.
- **Chips/Badges:** Use a pill-shaped background with high-contrast text (e.g., `secondary-container` background with `on-secondary-container` text).
- **Calendar Cells:** Strictly defined by `outline-variant` borders. Hover states should trigger a slight background shift or text color change to `primary`.
- **Input Fields:** Use transparent backgrounds with bottom-only borders for inline editing, or full `outline-variant` borders for standard data entry. Focus states should use the `primary` color.
- **Task Cards:** Utilize a vertical stack with a checkbox, title, and metadata. Completed states are indicated by both a checkbox check and a 60% opacity reduction with line-through text.
- **Thumbnails:** Interactive images within the calendar should have a `thumbnail-hover` effect: a significant scale transform (up to 3x) and a deep shadow to allow for quick inspection without navigation.