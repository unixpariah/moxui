# CSS Level 3 and Earlier Features TODO List

## Selectors
- [ ] Universal Selector (`*`)
- [ ] Type Selector (`element`)
- [ ] Class Selector (`.class`)
- [ ] ID Selector (`#id`)
- [ ] Attribute Selectors
  - [ ] `[attr]`
  - [ ] `[attr=value]`
  - [ ] `[attr^=value]`
  - [ ] `[attr$=value]`
  - [ ] `[attr*=value]`
- [ ] Descendant Combinator (`ancestor descendant`)
- [ ] Child Combinator (`parent > child`)
- [ ] Adjacent Sibling (`element1 + element2`)
- [ ] General Sibling (`element1 ~ element2`)
- [ ] Pseudo-classes
  - [ ] `:hover`
  - [ ] `:nth-child()`
  - [ ] `:focus`
  - [ ] Others
- [ ] Pseudo-elements
  - [ ] `::before`
  - [ ] `::after`
  - [ ] Others

## Box Model
- [ ] Width
  - [x] `width`
  - [ ] `min-width`
  - [ ] `max-width`
- [ ] Height
  - [x] `height`
  - [ ] `min-height`
  - [ ] `max-height`
- [x] Padding
- [x] Margin
- [ ] Borders
  - [x] `border`
  - [x] `border-width`
  - [ ] `border-style`
  - [x] `border-color`
- [x] Outline
  - [x] `outline`
  - [x] `outline-offset`
- [x] Box Sizing (`box-sizing`)
- [ ] Overflow
  - [ ] `overflow`
  - [ ] `overflow-x`
  - [ ] `overflow-y`

## Colors
- [ ] Named Colors
- [ ] Hexadecimal Colors
  - [ ] `#RRGGBB`
  - [ ] `#RGB`
- [ ] RGB and RGBA
  - [ ] `rgb()`
  - [ ] `rgba()`
- [ ] HSL and HSLA
  - [ ] `hsl()`
  - [ ] `hsla()`
- [x] Opacity (`opacity`)

## Typography
- [ ] Fonts
  - [ ] `font-family`
  - [ ] `font-size`
  - [ ] `font-style`
  - [ ] `font-weight`
  - [ ] `font-variant`
- [ ] Line Height (`line-height`)
- [ ] Text Alignment (`text-align`)
- [ ] Letter Spacing (`letter-spacing`)
- [ ] Word Spacing (`word-spacing`)
- [ ] Text Decoration
  - [ ] `text-decoration`
  - [ ] `text-decoration-color`
  - [ ] `text-decoration-style`
- [ ] Text Transform (`text-transform`)
- [ ] White Space (`white-space`)
- [ ] Text Shadow (`text-shadow`)

## Backgrounds and Borders
- [x] Background Color (`background-color`)
- [ ] Background Image (`background-image`)
- [ ] Background Repeat (`background-repeat`)
- [ ] Background Position (`background-position`)
- [ ] Background Attachment (`background-attachment`)
- [ ] Background Size (`background-size`)
- [ ] Multiple Backgrounds
- [x] Border Radius (`border-radius`)
- [ ] Border Images (`border-image`)

## Positioning and Layout
- [ ] Display
  - [x] `block`
  - [x] `inline`
  - [x] `inline-block`
  - [ ] `none`
  - [ ] `flex`
  - [ ] `grid`
- [ ] Position
  - [x] `static`
  - [x] `relative`
  - [x] `absolute`
  - [x] `fixed`
  - [ ] `sticky`
- [ ] Float
  - [ ] `float`
  - [ ] `clear`
- [ ] Z-index (`z-index`)
- [ ] Visibility (`visible`, `hidden`)
- [ ] Vertical Align (`vertical-align`)

## Flexible Box Layout (Flexbox)
- [ ] Flex Container (`display: flex`)
- [ ] Flex Direction (`flex-direction`)
- [ ] Justify Content (`justify-content`)
- [ ] Align Items (`align-items`)
- [ ] Align Content (`align-content`)
- [ ] Flex Grow/Shrink/Basis
  - [ ] `flex-grow`
  - [ ] `flex-shrink`
  - [ ] `flex-basis`
- [ ] Align Self (`align-self`)

## Grid Layout
- [ ] Grid Container (`display: grid`)
- [ ] Grid Template Rows/Columns
  - [ ] `grid-template-rows`
  - [ ] `grid-template-columns`
- [ ] Grid Gap (`grid-gap`)
- [ ] Grid Auto Rows/Columns
  - [ ] `grid-auto-rows`
  - [ ] `grid-auto-columns`
- [ ] Justify Items/Content
  - [ ] `justify-items`
  - [ ] `justify-content`
- [ ] Align Items/Content
  - [ ] `align-items`
  - [ ] `align-content`

## Transitions and Animations
- [ ] Transitions
  - [ ] `transition`
  - [ ] `transition-property`
  - [ ] `transition-duration`
- [ ] Keyframe Animations (`@keyframes`)
- [ ] Animation Properties
  - [ ] `animation-name`
  - [ ] `animation-duration`

## Media and Responsive Design
- [ ] Media Queries (`@media`)
- [x] Viewport Units
  - [x] `vh`
  - [x] `vw`
  - [x] `vmin`
  - [x] `vmax`

## Generated Content
- [ ] Content Property (`content`)
- [ ] Counters
  - [ ] `counter-increment`
  - [ ] `counter-reset`

## Filters and Effects
- [x] CSS Filters (`filter`)
- [ ] Backdrop Filters (`backdrop-filter`)
- [ ] Box Shadows (`box-shadow`)

## Lists and Tables
- [ ] List Style
  - [ ] `list-style-type`
  - [ ] `list-style-position`
- [ ] Table Layout
  - [ ] `border-collapse`
  - [ ] `border-spacing`

## Other Features
- [ ] CSS Variables (`--custom-property`)
- [ ] Functions
  - [x] `calc()`
  - [ ] `clamp()`
  - [ ] `min()`
  - [ ] `max()`
- [ ] Clip Path (`clip-path`)
- [ ] Writing Modes
  - [ ] `writing-mode`
  - [ ] `direction`
- [ ] Resize (`resize`)
- [ ] Cursor (`cursor`)
