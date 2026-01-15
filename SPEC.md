# GraceX - Grammar of Graphics Plotting Library Specification

## Project Overview

**GraceX** is a grammar of graphics plotting library for Rust, inspired by ggplot2 and the layered grammar of graphics approach. The library is being built from the ground up, starting with low-level primitives and progressing to high-level plotting abstractions.

**Philosophy**: Build incrementally from primitives → renderer → geoms → scales → coordinate systems → full grammar

---

## Minimal Working Example (MWE) Goal

**Capability Target**: Create a scatter plot with aesthetic mappings

**Core Requirements**:
- Accept data from Polars DataFrame, Series, or Vec<f64>
- Map data columns to visual properties (x, y, size, color, etc.)
- Render points to PNG file
- Support both mapped aesthetics (from data) and fixed aesthetics (constant values)

**API Status**: 🔬 **EXPLORATORY** - The exact user-facing API will be discovered through implementation. The examples below show possible directions to explore, not final designs.

### Possible API Patterns to Explore

```rust
// Option A: Method chaining with aes() builder
Plot::new()
    .data(df)
    .geom_point(aes().x("x").y("y").size("size"))
    .save("out.png")?;

// Option B: Separate aesthetic function parameter
Plot::new()
    .data(df)
    .geom_point()
    .aes_x("x")
    .aes_y("y")
    .aes_size("size")
    .save("out.png")?;

// Option C: Macro-based DSL
plot!(data = df, geom_point(x = "x", y = "y", size = "size"))
    .save("out.png")?;

// Option D: Struct-based configuration
let aes = Aesthetic {
    x: Mapping::Column("x"),
    y: Mapping::Column("y"),
    size: Mapping::Column("size"),
    ..Default::default()
};
Plot::new().data(df).geom_point(aes).save("out.png")?;

// Option E: Mix of mapped and fixed aesthetics
Plot::new()
    .data(df)
    .geom_point()
    .map_x("x")           // from data
    .map_y("y")           // from data
    .set_color(Color::RED) // fixed value
    .map_size("size")     // from data
    .save("out.png")?;
```

**Note**: We'll implement the underlying machinery (aesthetic mapping system, data extraction, coordinate transformation) and then experiment with different API surfaces to find what feels most natural in Rust.

---

## Current Implementation Status

### ✅ Completed

- [x] Project structure setup
- [x] Basic primitives module
  - [x] Point struct
  - [x] Color struct (RGBA)
  - [x] Stroke struct
  - [x] DrawCommand enum (Circle, Line, Rectangle, Polygon, Text)
  - [x] Default implementations for primitives

### 🚧 In Progress

- [ ] Renderer implementation

### ⏳ Pending

- [ ] Data abstraction layer
- [ ] Aesthetic mapping system
- [ ] Coordinate mapping system (scales)
- [ ] Geom layer (starting with geom_point)
- [ ] Plot builder API
- [ ] File output system

---

## Core Concepts

### Aesthetic Mapping (The Heart of Grammar of Graphics)

**Aesthetics** are visual properties of geometric objects that can represent data. The grammar of graphics philosophy separates:

1. **What to display** (data)
2. **How to display it** (aesthetic mappings)
3. **Where to display it** (coordinate system)
4. **With what shape** (geometric objects/geoms)

#### Aesthetic Types

**Position aesthetics**: x, y (required for most plots)
**Visual aesthetics**: color, fill, size, shape, alpha, linetype, etc.

#### Mapped vs. Set Aesthetics

```
Mapped aesthetic: property value comes FROM data
  - "map column 'age' to point size"
  - Each row gets a different value based on data
  - Example: larger points for older individuals

Set aesthetic: property value is FIXED
  - "set all points to red"
  - Every element gets the same value
  - Example: all points have size 3
```

#### Why This Matters for Implementation

In ggplot2 (R):
```r
# Mapped: inside aes()
ggplot(data, aes(x = age, y = height, color = gender)) + geom_point()

# Set: outside aes()
ggplot(data, aes(x = age, y = height)) + geom_point(color = "red", size = 3)
```

In Rust, we need to decide how to:
- Represent this distinction (types? enums? traits?)
- Store aesthetic specifications
- Evaluate them at render time
- Handle lifetimes/ownership of data references
- Make the API feel natural despite Rust's constraints

This is what we'll explore through the MWE implementation.

---

## Implementation Roadmap

### Phase 1: Foundation (Current)

**Goal**: Implement a working renderer with tiny-skia

#### 1.1 Renderer Implementation
- [ ] Add tiny-skia dependency to Cargo.toml
- [ ] Implement `Renderer` trait for `PngRenderer`
- [ ] Convert DrawCommand::Circle to tiny-skia path
- [ ] Convert DrawCommand::Line to tiny-skia path
- [ ] Convert DrawCommand::Rectangle to tiny-skia path
- [ ] Convert DrawCommand::Polygon to tiny-skia path
- [ ] Convert DrawCommand::Text to tiny-skia text
- [ ] Implement save to PNG functionality
- [ ] Test renderer with manual DrawCommand creation

**Success Criterion**: Manually create DrawCommands and render them to a PNG file

---

### Phase 2: Data Abstraction

**Goal**: Support multiple data input types (Polars, Vec)

#### 2.1 Data Types
- [ ] Define `DataSource` trait for abstracting data inputs
- [ ] Implement DataSource for `polars::series::Series`
- [ ] Implement DataSource for `polars::dataframe::DataFrame`
- [ ] Implement DataSource for `Vec<f64>`
- [ ] Create data extraction/indexing utilities

#### 2.2 Data Validation
- [ ] Length checking for paired data (x, y)
- [ ] Type validation
- [ ] Missing value handling strategy

**Success Criterion**: Extract x, y data from any supported source

---

### Phase 3: Aesthetic Mapping System

**Goal**: Create a flexible system for mapping data to visual properties

#### 3.1 Aesthetic Concepts
- [ ] Define core aesthetic properties (x, y, color, size, alpha, shape, etc.)
- [ ] Distinguish between **mapped aesthetics** (from data columns) and **set aesthetics** (fixed values)
- [ ] Design aesthetic specification data structure

#### 3.2 Aesthetic Implementation Options

**Core question**: How do we represent "map column 'age' to size" vs "set all points to size 5"?

Possible approaches to explore:
- **Enum-based**: `enum AesValue<T> { Mapped(String), Fixed(T) }`
- **Trait-based**: Separate `MappedAesthetic` and `FixedAesthetic` traits
- **Type-state pattern**: Use generics to enforce mapping at compile time
- **Runtime polymorphism**: Use trait objects for flexibility

#### 3.3 Aesthetic Storage & Evaluation
- [ ] Store aesthetic specifications in geom/layer
- [ ] Evaluate aesthetics: extract values from data source
- [ ] Handle missing data in aesthetic mappings
- [ ] Apply default values for unmapped aesthetics

#### 3.4 Aesthetic Scales (Simplified for MWE)
- [ ] Map continuous data to visual property (e.g., size values → pixel radii)
- [ ] Map categorical data to discrete values (future: color palettes)
- [ ] Define sensible defaults for each aesthetic type

**Success Criterion**: Given data and aesthetic specs, extract and transform values for rendering

**Key Design Questions to Explore**:
1. Should aesthetics be generic over data types or use trait objects?
2. How do we make the API ergonomic for common cases but flexible for advanced uses?
3. What's the right balance between compile-time safety and runtime flexibility?
4. How do we handle the lifetime/ownership of data references?

---

### Phase 4: Coordinate System (Scales)

**Goal**: Map data coordinates to pixel coordinates

#### 4.1 Scales
- [ ] Define `Scale` trait
- [ ] Implement continuous scale (linear mapping)
- [ ] Calculate data extent (min, max)
- [ ] Map data value → pixel coordinate
- [ ] Handle margins and padding

#### 4.2 Coordinate Space
- [ ] Define plot area dimensions
- [ ] Define margin system (top, right, bottom, left)
- [ ] Calculate axis ranges
- [ ] Transform (data_x, data_y) → (pixel_x, pixel_y)

**Success Criterion**: Map arbitrary data values to screen coordinates

---

### Phase 5: Geom Layer

**Goal**: Implement geom_point for scatter plots

#### 5.1 Geom Abstraction
- [ ] Define `Geom` trait
- [ ] Implement trait method: `to_draw_commands()`
- [ ] Receive aesthetic values (already mapped/evaluated)
- [ ] Receive coordinate transformations (scales)

#### 5.2 GeomPoint Implementation
- [ ] Create `GeomPoint` struct
- [ ] Store aesthetic specifications
- [ ] Map data points to Circle DrawCommands
- [ ] Apply aesthetic values (color, size, alpha) to each point
- [ ] Apply coordinate transformations to positions

**Success Criterion**: Convert aesthetic-mapped data into positioned, styled circles

---

### Phase 6: Plot API (Exploratory)

**Goal**: Design and implement user-facing builder API

#### 6.1 Plot Structure
- [ ] Create `Plot` struct
- [ ] Implement builder pattern with chaining
- [ ] Store data reference/ownership
- [ ] Store geom layers
- [ ] Store aesthetic mappings

#### 6.2 API Experimentation
- [ ] Implement basic version (pick one pattern from options above)
- [ ] Test ergonomics with real usage
- [ ] Identify pain points (borrow checker issues, verbosity, clarity)
- [ ] Iterate on design based on experience
- [ ] Document tradeoffs discovered

#### 6.3 Builder Methods (exact API TBD)
- [ ] Constructor pattern
- [ ] Data attachment method
- [ ] Geom addition with aesthetic specification
- [ ] Rendering/save method
- [ ] Error handling with Result types

#### 6.4 Rendering Pipeline
- [ ] Combine all geoms into DrawCommands
- [ ] Evaluate aesthetics against data
- [ ] Apply coordinate transformations
- [ ] Pass commands to renderer
- [ ] Output to file

**Success Criterion**: Create scatter plot through discovered API, with both mapped and fixed aesthetics

---

## Technical Specifications

### Supported Data Types

| Type | Use Case | Status |
|------|----------|--------|
| `polars::series::Series` | Single column of data | ⏳ Planned |
| `polars::dataframe::DataFrame` | Multi-column data | ⏳ Planned |
| `Vec<f64>` | Simple numeric vectors | ⏳ Planned |

### Dependencies

```toml
[dependencies]
tiny-skia = "0.11"        # Rendering backend
polars = "0.44"            # Data structures (optional feature)
```

### File Output Formats

- **Phase 1**: PNG only
- **Future**: SVG, PDF (stretch goals)

---

## Design Decisions & Tradeoffs

### Renderer Choice: tiny-skia
**Rationale**:
- Pure Rust, no system dependencies
- Fast rasterization
- Good text rendering
- Active maintenance

**Tradeoffs**:
- Raster-only (no vector output initially)
- Memory usage for large canvases

### Data Model: Trait-based abstraction
**Rationale**:
- Supports multiple input types
- Extensible for future data sources
- Type-safe at compile time

**Tradeoffs**:
- More complex initial implementation
- Requires understanding trait system

### API Style: To Be Determined Through Exploration
**Status**: 🔬 Exploratory - will be determined during Phase 6

**Candidates being considered**:
- Builder pattern (method chaining)
- Struct-based configuration
- Macro DSL
- Hybrid approaches

**Evaluation criteria**:
- Ergonomics (how natural does it feel?)
- Compile-time safety vs. runtime flexibility
- Borrow checker friendliness
- Discoverability (IDE autocomplete, docs)
- Similarity to ggplot2 (where it makes sense)
- Rust idiomaticity

---

## Testing Strategy

### Unit Tests
- [ ] Primitive types (Point, Color, etc.)
- [ ] Coordinate transformations
- [ ] Scale calculations
- [ ] Data extraction from sources

### Integration Tests
- [ ] Render simple shapes
- [ ] Create scatter plot from Vec<f64>
- [ ] Create scatter plot from Polars DataFrame
- [ ] File output validation

### Visual Tests
- [ ] Generate reference images
- [ ] Manual inspection of outputs
- [ ] (Future) Pixel-based comparison

---

## Example Usage Patterns (Aspirational)

**Note**: These are aspirational examples showing what we want to achieve. The exact API syntax will be determined through Phase 6 exploration.

### Pattern 1: Vec<f64> Input (Simple case)
```rust
use gracex::prelude::*;

let x = vec![1.0, 2.0, 3.0, 4.0];
let y = vec![2.0, 4.0, 3.0, 5.0];

// Exact API TBD - could be any of:
// Option A: .data_xy(x, y)
// Option B: .data_vecs(&x, &y)
// Option C: .aes().x_vec(x).y_vec(y)
Plot::new()
    .data_xy(x, y)  // placeholder
    .geom_point()
    .save("scatter.png")?;
```

### Pattern 2: Polars DataFrame Input with Aesthetic Mapping
```rust
use gracex::prelude::*;
use polars::prelude::*;

let df = df! {
    "age" => &[25.0, 30.0, 35.0, 40.0],
    "height" => &[165.0, 170.0, 175.0, 180.0],
    "weight" => &[60.0, 70.0, 80.0, 90.0],
}?;

// Exact API TBD - exploring how to specify aesthetics ergonomically
// This shows the CONCEPT, not the final syntax
Plot::new()
    .data(df)
    .geom_point(/* aesthetic mappings here */)
    // Somehow map: age -> x, height -> y, weight -> size
    .save("scatter.png")?;
```

### Pattern 3: Mixed Mapped and Fixed Aesthetics
```rust
use gracex::prelude::*;
use polars::prelude::*;

let df = df! {
    "x" => &[1.0, 2.0, 3.0],
    "y" => &[2.0, 4.0, 3.0],
    "category" => &["A", "B", "A"],
}?;

// Goal: map x and y from data, but set all points to red
// Exploring different ways to express this distinction
Plot::new()
    .data(df)
    .geom_point(/* map x, y; set color = red */)
    .save("scatter.png")?;
```

---

## Future Extensions (Post-MWE)

### Additional Geoms
- [ ] geom_line (line plots)
- [ ] geom_bar (bar charts)
- [ ] geom_histogram
- [ ] geom_boxplot

### Statistical Transformations
- [ ] Stat layer abstraction
- [ ] stat_smooth (regression lines)
- [ ] stat_bin (histograms)

### Scales & Axes
- [ ] Categorical scales
- [ ] Logarithmic scales
- [ ] Date/time scales
- [ ] Axis rendering with labels
- [ ] Grid lines

### Aesthetics
- [ ] Color palettes
- [ ] Shape mappings
- [ ] Size scaling
- [ ] Transparency (alpha)

### Faceting
- [ ] facet_wrap
- [ ] facet_grid

### Themes
- [ ] Default theme system
- [ ] Customizable themes
- [ ] Background colors
- [ ] Font customization

---

## Open Questions & Exploration Areas

### API Design
1. **Aesthetic Specification**: How should users specify mapped vs. fixed aesthetics?
   - Inside/outside pattern like ggplot2's `aes()`?
   - Different methods (`.map_x()` vs `.set_x()`)?
   - Type-based distinction?

2. **Method Chaining vs. Configuration Structs**: Which feels more natural in Rust?
   - Builder pattern with long chains?
   - Struct configuration passed to constructor?
   - Hybrid approach?

3. **Aesthetic Storage**: How do we store aesthetic specs before evaluation?
   - `enum AesValue<T> { Mapped(String), Fixed(T) }`?
   - Separate collections for mapped and fixed?
   - Trait objects?

4. **String-based Column References**: Are string column names the right approach?
   - Compile-time column validation possible?
   - Type-safe alternatives?
   - Runtime validation strategy?

### Implementation Details
5. **Memory Management**: How to handle large datasets efficiently?
   - Copy data into Plot or hold references?
   - Lazy evaluation strategies?

6. **Error Handling**: What granularity of error types do we need?
   - Column not found, type mismatch, render errors
   - Custom error types or anyhow?

7. **Trait Bounds**: Should we use trait objects or generic constraints?
   - `Box<dyn DataSource>` vs `impl DataSource` vs `<T: DataSource>`?
   - Performance vs. flexibility tradeoffs?

8. **Ownership**: When should Plot own data vs. borrow references?
   - Lifetime parameters on Plot struct?
   - Impact on API ergonomics?

9. **Rendering Architecture**: Should rendering be synchronous or async?
   - Any use cases for async rendering?
   - Progressive rendering for large plots?

### Aesthetic System Design
10. **Default Values**: How to handle aesthetic defaults?
    - Per-geom defaults or global defaults?
    - Precedence: geom default < mapped aesthetic < set aesthetic?

11. **Scale Application**: When do we transform aesthetic values?
    - During evaluation or during rendering?
    - Size: data value 10 → pixel radius 5?

12. **Type Heterogeneity**: How to handle different aesthetic value types?
    - x, y are numeric; color could be categorical or numeric
    - Single generic approach or specialized handling?

---

## Success Metrics for MWE

### Functional Requirements
- [ ] Can create a scatter plot from Polars DataFrame with column mappings
- [ ] Can create a scatter plot from Vec<f64>
- [ ] Supports aesthetic mapping (map data columns to visual properties)
- [ ] Supports fixed aesthetics (set constant values for visual properties)
- [ ] Points are correctly positioned in coordinate space
- [ ] Output is a valid PNG file that can be opened

### Code Quality
- [ ] Code compiles without warnings
- [ ] Basic error handling exists
- [ ] Core concepts documented with comments

### API Evaluation
- [ ] API feels natural to write
- [ ] Common case (simple scatter) is concise
- [ ] Borrow checker doesn't fight us
- [ ] Aesthetic mapping distinction (mapped vs. fixed) is clear
- [ ] We've explored at least 2-3 API variations and can articulate tradeoffs

---

## References & Inspiration

- **ggplot2**: R plotting library (grammar of graphics)
- **Vega-Lite**: Declarative visualization grammar
- **matplotlib**: Python plotting library
- **plotly**: Interactive plotting
- **The Grammar of Graphics** by Leland Wilkinson

---

**Last Updated**: 2026-01-14
**Current Phase**: Phase 1 - Renderer Implementation
**Next Milestone**: Working PngRenderer with tiny-skia
