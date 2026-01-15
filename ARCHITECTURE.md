# GraceX Architecture

## Overview: The Grammar of Graphics Stack

GraceX follows a layered architecture inspired by ggplot2, where each layer builds upon the previous one. The system transforms high-level declarative specifications into low-level rendering commands.

```
┌─────────────────────────────────────────────────────────────┐
│                    USER API LAYER                           │
│  Plot builder, method chaining, aesthetic specifications    │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   GRAMMAR LAYER                             │
│  Layers, Geoms, Stats, Scales, Coords, Facets, Theme       │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                  TRANSFORMATION LAYER                       │
│  Data → Aesthetics → Scaled Values → Coordinates            │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   RENDERING LAYER                           │
│  DrawCommands → Primitives → tiny-skia → PNG               │
└─────────────────────────────────────────────────────────────┘
```

---

## Core Components and Their Relationships

### 1. Plot - The Container (Top-level orchestrator)

```rust
pub struct Plot {
    // Data
    data: Option<Box<dyn DataSource>>,

    // Layers (each layer = geom + stat + position + data)
    layers: Vec<Layer>,

    // Scales (transform data → aesthetic values)
    scales: Scales,

    // Coordinate system
    coord: Box<dyn CoordSystem>,

    // Faceting specification
    facet: Option<Box<dyn Facet>>,

    // Theme (visual styling)
    theme: Theme,

    // Guides (legends, axes)
    guides: Guides,

    // Labels (title, axis labels, etc.)
    labels: Labels,

    // Layout configuration
    layout: Layout,
}
```

**Responsibilities**:
- Store all plot specifications
- Orchestrate the rendering pipeline
- Combine all layers
- Apply faceting
- Position guides and labels

---

### 2. Layer - The Building Block

A layer represents one geometric representation of data. You can have multiple layers in a plot (e.g., points + line + smooth).

```rust
pub struct Layer {
    // Geom: HOW to draw (point, line, bar, etc.)
    geom: Box<dyn Geom>,

    // Stat: WHAT to compute (identity, bin, smooth, etc.)
    stat: Box<dyn Stat>,

    // Data: Can override plot-level data
    data: Option<Box<dyn DataSource>>,

    // Aesthetic mappings (column → aesthetic)
    mapping: AestheticMapping,

    // Fixed aesthetic values
    fixed_aes: FixedAesthetics,

    // Position adjustment (dodge, jitter, stack)
    position: Box<dyn Position>,
}
```

**Responsibilities**:
- Encapsulate one visual representation
- Know its own data (or inherit from plot)
- Store aesthetic mappings and fixed values
- Coordinate geom + stat interaction

---

### 3. AestheticMapping - The Heart of the Grammar

This is what makes the grammar of graphics powerful - the separation of data from visual representation.

```rust
pub struct AestheticMapping {
    // Maps aesthetic name → data column
    mappings: HashMap<Aesthetic, ColumnSpec>,
}

pub enum Aesthetic {
    // Position aesthetics (required for most geoms)
    X,
    Y,

    // Visual aesthetics
    Color,
    Fill,
    Size,
    Alpha,
    Shape,
    Linetype,
    Linewidth,

    // Text aesthetics
    Label,

    // Group aesthetic (for connecting points)
    Group,
}

pub enum ColumnSpec {
    Named(String),           // Reference to column name
    Computed(String),        // Reference to stat-computed column (e.g., "..count..")
    Expression(Box<dyn Fn(&DataSource) -> Vec<f64>>),  // Future: computed columns
}

pub struct FixedAesthetics {
    // Aesthetic name → fixed value
    values: HashMap<Aesthetic, AestheticValue>,
}

pub enum AestheticValue {
    Numeric(f64),
    Color(Color),
    String(String),
    // ... other types
}
```

**Key Concepts**:
- **Mapped aesthetic**: Value comes from data column (varies per observation)
- **Fixed aesthetic**: Value is constant (same for all observations)
- **Computed aesthetic**: Value comes from statistical transformation

---

### 4. Scales - Data → Visual Property Transformation

Scales control how data values map to visual properties. Each aesthetic typically has a scale.

```rust
pub struct Scales {
    // One scale per aesthetic
    scales: HashMap<Aesthetic, Box<dyn Scale>>,
}

pub trait Scale {
    /// Get the data range this scale should cover
    fn train(&mut self, data: &[f64]);

    /// Map data value → aesthetic value
    fn map(&self, value: f64) -> f64;

    /// Map aesthetic value → data value (for axes)
    fn inverse(&self, value: f64) -> f64;

    /// Generate breaks (tick positions)
    fn breaks(&self) -> Vec<f64>;

    /// Generate labels for breaks
    fn labels(&self) -> Vec<String>;
}

// Example implementations:
pub struct ContinuousScale {
    data_range: (f64, f64),
    visual_range: (f64, f64),
    trans: Box<dyn Transform>,  // identity, log, sqrt, etc.
}

pub struct DiscreteScale {
    levels: Vec<String>,
    palette: Vec<Color>,  // or shapes, sizes, etc.
}

pub struct ColorScale {
    gradient: ColorGradient,
    // or discrete palette
}
```

**Responsibilities**:
- Determine data range (domain)
- Determine visual range (range)
- Provide transformation function
- Generate axis breaks and labels

---

### 5. Geom - Geometric Representation

Geoms define how to represent data visually.

```rust
pub trait Geom {
    /// Convert aesthetic values into draw commands
    fn draw(
        &self,
        data: &ProcessedData,
        coords: &dyn CoordSystem,
    ) -> Vec<DrawCommand>;

    /// What aesthetics does this geom require?
    fn required_aes(&self) -> Vec<Aesthetic>;

    /// What aesthetics does this geom understand?
    fn optional_aes(&self) -> Vec<Aesthetic>;

    /// Default aesthetic values
    fn default_aes(&self) -> FixedAesthetics;
}

// Example implementations:
pub struct GeomPoint { /* point-specific config */ }
pub struct GeomLine { /* line-specific config */ }
pub struct GeomBar { /* bar-specific config */ }
pub struct GeomBoxplot { /* boxplot-specific config */ }

// Processed data contains aesthetic values already extracted and scaled
pub struct ProcessedData {
    // Each row is one observation
    // Each column is one aesthetic's scaled values
    x: Vec<f64>,
    y: Vec<f64>,
    color: Option<Vec<Color>>,
    size: Option<Vec<f64>>,
    // ... other aesthetics
}
```

**Responsibilities**:
- Define visual representation
- Declare aesthetic requirements
- Convert scaled aesthetic values → DrawCommands
- Apply coordinate transformations

---

### 6. Stat - Statistical Transformation

Stats compute derived data before plotting.

```rust
pub trait Stat {
    /// Transform input data
    fn compute(
        &self,
        data: &dyn DataSource,
        mapping: &AestheticMapping,
    ) -> StatResult;

    /// What new columns does this stat create?
    fn computed_vars(&self) -> Vec<String>;
}

pub struct StatResult {
    // Original data + computed columns
    data: DataFrame,
}

// Example implementations:
pub struct StatIdentity;     // No transformation (default)
pub struct StatBin;          // Histogram binning (..count.., ..density..)
pub struct StatSmooth;       // Smoothing curves (..ymin.., ..ymax.., ..se..)
pub struct StatBoxplot;      // Boxplot statistics (..lower.., ..upper.., ..middle..)
pub struct StatCount;        // Count observations per group
```

**Responsibilities**:
- Transform raw data
- Create computed variables
- Prepare data for geom

---

### 7. CoordSystem - Coordinate Transformations

Coordinate systems control how (x, y) positions map to the canvas.

```rust
pub trait CoordSystem {
    /// Transform data coordinates → display coordinates
    fn transform(&self, x: f64, y: f64) -> (f64, f64);

    /// Inverse transformation
    fn inverse(&self, x: f64, y: f64) -> (f64, f64);

    /// Determine aspect ratio
    fn aspect_ratio(&self) -> Option<f64>;
}

// Example implementations:
pub struct CoordCartesian {
    xlim: Option<(f64, f64)>,
    ylim: Option<(f64, f64)>,
}

pub struct CoordFlip;  // Swap x and y
pub struct CoordPolar { theta: Aesthetic, r: Aesthetic }
pub struct CoordFixed { ratio: f64 }  // Fixed aspect ratio
pub struct CoordMap { projection: Projection }  // Map projections
```

**Responsibilities**:
- Position transformations
- Maintain aspect ratios
- Handle coordinate system specifics

---

### 8. Facet - Small Multiples

Facets split data into panels and create multiple sub-plots.

```rust
pub trait Facet {
    /// Split data into panels
    fn split_data(&self, data: &dyn DataSource) -> Vec<Panel>;

    /// Calculate panel layout
    fn layout(&self) -> FacetLayout;
}

pub struct Panel {
    data: Box<dyn DataSource>,
    label: String,
    position: (usize, usize),  // row, col in grid
}

pub struct FacetLayout {
    rows: usize,
    cols: usize,
    panel_positions: Vec<PanelPosition>,
}

// Example implementations:
pub struct FacetWrap {
    facets: Vec<String>,  // column names to facet by
    ncol: Option<usize>,
}

pub struct FacetGrid {
    rows: Vec<String>,
    cols: Vec<String>,
}
```

**Responsibilities**:
- Partition data into subsets
- Calculate grid layout
- Create strip labels

---

### 9. Theme - Visual Styling

Theme controls all non-data visual elements.

```rust
pub struct Theme {
    // Plot area
    plot_background: Fill,
    plot_margin: Margin,

    // Panel (data area)
    panel_background: Fill,
    panel_border: Border,
    panel_grid_major: Line,
    panel_grid_minor: Line,

    // Axes
    axis_line: Line,
    axis_ticks: Line,
    axis_text: Text,
    axis_title: Text,

    // Legend
    legend_position: Position,
    legend_background: Fill,
    legend_text: Text,
    legend_title: Text,

    // Strip (facet labels)
    strip_background: Fill,
    strip_text: Text,

    // Title
    plot_title: Text,
    plot_subtitle: Text,
    plot_caption: Text,
}

pub struct Text {
    font_family: String,
    font_size: f32,
    color: Color,
    face: FontFace,  // normal, bold, italic
    hjust: f32,      // horizontal justification
    vjust: f32,      // vertical justification
}
```

**Responsibilities**:
- Style all visual elements
- Provide defaults
- Allow user customization

---

### 10. Guides - Legends and Axes

Guides help readers decode aesthetic mappings.

```rust
pub struct Guides {
    legends: Vec<Legend>,
    x_axis: Axis,
    y_axis: Axis,
}

pub struct Legend {
    aesthetic: Aesthetic,
    title: Option<String>,
    breaks: Vec<f64>,
    labels: Vec<String>,
    position: LegendPosition,
}

pub struct Axis {
    title: Option<String>,
    breaks: Vec<f64>,
    labels: Vec<String>,
    position: AxisPosition,
}
```

**Responsibilities**:
- Create legends for mapped aesthetics
- Create axes for position aesthetics
- Position and style guides

---

### 11. Layout - Component Positioning

Layout manages the spatial arrangement of all plot components.

```rust
pub struct Layout {
    canvas_size: (u32, u32),
    plot_area: Rectangle,
    panel_areas: Vec<Rectangle>,
    legend_area: Rectangle,
    title_area: Rectangle,
    x_axis_area: Rectangle,
    y_axis_area: Rectangle,
}

pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
```

**Responsibilities**:
- Calculate sizes and positions
- Handle margins and padding
- Coordinate facet layout

---

## The Rendering Pipeline

Here's how data flows through the system:

```
1. USER SPECIFICATION
   Plot::new()
     .data(df)
     .geom_point(aes().x("age").y("height").color("gender"))
     .scale_color_manual(values = ["red", "blue"])
     .theme_minimal()

2. DATA PREPARATION
   - Extract data from DataSource
   - For each layer:
     - Apply stat transformation
     - Evaluate aesthetic mappings
     - Extract aesthetic columns

3. SCALING
   - For each aesthetic:
     - Train scale on data range
     - Map data values → aesthetic values
     - x: [20, 30, 40] → [50px, 250px, 450px]
     - color: ["M", "F"] → [Color::BLUE, Color::RED]

4. FACETING (if specified)
   - Split data by facet variables
   - Calculate panel layout
   - Assign each subset to a panel

5. COORDINATE TRANSFORMATION
   - Apply coordinate system
   - Transform aesthetic positions
   - (x_data, y_data) → (x_screen, y_screen)

6. GEOM RENDERING
   - For each layer in each panel:
     - Call geom.draw(scaled_data, coord_system)
     - Geom produces DrawCommands
     - DrawCommands use screen coordinates

7. GUIDE GENERATION
   - Extract scales that were used
   - Generate legends for non-position aesthetics
   - Generate axes for position aesthetics
   - Convert to DrawCommands

8. LAYOUT COMPOSITION
   - Calculate panel sizes
   - Position guides
   - Position title/labels
   - Assign rendering regions

9. THEME APPLICATION
   - Add background elements
   - Add grid lines
   - Style all text elements
   - Convert to DrawCommands

10. RENDERING
    - Collect all DrawCommands
    - Pass to Renderer
    - Renderer converts to pixels
    - Save as PNG/SVG/etc.
```

---

## Extension Points: How to Add Features

### Adding a New Geom

```rust
pub struct GeomViolin {
    // violin-specific parameters
}

impl Geom for GeomViolin {
    fn required_aes(&self) -> Vec<Aesthetic> {
        vec![Aesthetic::X, Aesthetic::Y]
    }

    fn draw(&self, data: &ProcessedData, coords: &dyn CoordSystem) -> Vec<DrawCommand> {
        // Create violin shape from data
        // Return Polygon DrawCommands
        todo!()
    }
}
```

### Adding a New Scale

```rust
pub struct ScaleColorViridis {
    n_colors: usize,
}

impl Scale for ScaleColorViridis {
    fn map(&self, value: f64) -> f64 {
        // Map value to viridis color
        todo!()
    }
}
```

### Adding a New Coordinate System

```rust
pub struct CoordPolar {
    theta: Aesthetic,
    start: f64,
}

impl CoordSystem for CoordPolar {
    fn transform(&self, x: f64, y: f64) -> (f64, f64) {
        // Convert cartesian → polar
        todo!()
    }
}
```

---

## MWE vs Full System

### What We're Building in MWE:

```
Plot (simplified)
  ├── Data (Polars DataFrame/Vec)
  ├── Layer (single)
  │   ├── GeomPoint
  │   ├── StatIdentity (no transformation)
  │   └── AestheticMapping (x, y, maybe size/color)
  ├── Scales (simple linear scales for x, y)
  └── CoordCartesian (simple)

→ DrawCommands → Renderer → PNG
```

### What Full System Would Have:

```
Plot
  ├── Data
  ├── Layers (multiple)
  │   ├── Geom (point, line, bar, boxplot, etc.)
  │   ├── Stat (identity, bin, smooth, boxplot, etc.)
  │   ├── AestheticMapping (all aesthetics)
  │   └── Position (identity, dodge, stack, jitter)
  ├── Scales (continuous, discrete, color, size, shape)
  ├── Coord (cartesian, polar, map, flipped)
  ├── Facet (wrap, grid)
  ├── Theme (complete styling)
  ├── Guides (legends, axes)
  ├── Labels (titles, captions)
  └── Layout (positioning system)

→ DrawCommands → Renderer → PNG/SVG/PDF
```

---

## Key Architectural Decisions

### 1. Trait-Based Extensibility

Use traits for major extension points:
- `Geom` trait → add new geometric objects
- `Stat` trait → add new statistical transformations
- `Scale` trait → add new scaling functions
- `CoordSystem` trait → add new coordinate systems
- `Facet` trait → add new faceting strategies
- `Renderer` trait → add new output formats

### 2. Separation of Concerns

- **Data layer**: Knows nothing about visuals
- **Grammar layer**: Knows nothing about pixels
- **Rendering layer**: Knows nothing about data
- **User API layer**: Provides ergonomics, delegates to grammar layer

### 3. Pipeline Architecture

Each stage transforms and passes data forward:
- Data → Aesthetics
- Aesthetics → Scales
- Scaled → Coordinates
- Coordinates → DrawCommands
- DrawCommands → Pixels

### 4. Immutability Where Possible

- Plot building is immutable (builder pattern returns new Plot)
- DrawCommands are immutable once created
- Data is not mutated by plotting

### 5. Error Handling Strategy

- Use Result types throughout
- Custom error types for different stages
- Fail early with clear messages

---

## Module Organization

Proposed structure for full system:

```
gracex/
├── src/
│   ├── lib.rs
│   ├── prelude.rs              // Re-exports for users
│   │
│   ├── primitives/             // Low-level drawing
│   │   ├── mod.rs
│   │   ├── point.rs
│   │   ├── color.rs
│   │   └── command.rs
│   │
│   ├── renderer/               // Output backends
│   │   ├── mod.rs
│   │   ├── trait.rs
│   │   ├── png.rs
│   │   └── svg.rs
│   │
│   ├── data/                   // Data abstraction
│   │   ├── mod.rs
│   │   ├── source.rs
│   │   ├── polars.rs
│   │   └── vec.rs
│   │
│   ├── aes/                    // Aesthetics
│   │   ├── mod.rs
│   │   ├── mapping.rs
│   │   ├── aesthetic.rs
│   │   └── value.rs
│   │
│   ├── scale/                  // Scales
│   │   ├── mod.rs
│   │   ├── trait.rs
│   │   ├── continuous.rs
│   │   ├── discrete.rs
│   │   └── color.rs
│   │
│   ├── geom/                   // Geoms
│   │   ├── mod.rs
│   │   ├── trait.rs
│   │   ├── point.rs
│   │   ├── line.rs
│   │   ├── bar.rs
│   │   └── ...
│   │
│   ├── stat/                   // Stats
│   │   ├── mod.rs
│   │   ├── trait.rs
│   │   ├── identity.rs
│   │   ├── bin.rs
│   │   └── smooth.rs
│   │
│   ├── coord/                  // Coordinate systems
│   │   ├── mod.rs
│   │   ├── trait.rs
│   │   ├── cartesian.rs
│   │   └── polar.rs
│   │
│   ├── facet/                  // Faceting
│   │   ├── mod.rs
│   │   ├── trait.rs
│   │   ├── wrap.rs
│   │   └── grid.rs
│   │
│   ├── theme/                  // Theming
│   │   ├── mod.rs
│   │   ├── theme.rs
│   │   ├── element.rs
│   │   └── defaults.rs
│   │
│   ├── guide/                  // Guides
│   │   ├── mod.rs
│   │   ├── legend.rs
│   │   └── axis.rs
│   │
│   ├── layout/                 // Layout
│   │   ├── mod.rs
│   │   └── layout.rs
│   │
│   ├── layer.rs                // Layer abstraction
│   └── plot.rs                 // Plot orchestrator
```

---

## Summary

The key insight of the grammar of graphics is **separation of concerns**:

1. **Data** is independent of **visuals**
2. **Aesthetics** separate **what to show** from **how to show it**
3. **Scales** separate **data space** from **visual space**
4. **Geoms** separate **shape** from **data**
5. **Stats** separate **transformation** from **visualization**

This separation makes the system:
- **Composable**: Mix and match components
- **Extensible**: Add new geoms, scales, stats without changing core
- **Declarative**: Describe what you want, not how to draw it
- **Powerful**: Complex plots from simple combinations

Your current renderer and primitives are the **foundation**. Everything else will be built on top, transforming high-level specifications down to those low-level DrawCommands.
