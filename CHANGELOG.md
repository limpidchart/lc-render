# Changelog

## [0.2.0] - 2021-05-18

### Added

- Added `horizontal bar` view
- Added `line` view
- Added `area` view
- Added `scatter` view
- Added `area` shape
- Added `point` shape
- Added examples for all views that are currently implemented
- Added benchmarks

### Fixed

- Fixed all Clippy warnings and added `-D` parameter in CI to fail on any warning

## [0.1.0] - 2021-05-10

First release of lc-render library that allows to create SVG vertical bar charts

### Added

- Added `math` path with basic normalization and interpolation
- Added `render` path with constants and helpers to create SVG
- Added `scale` path with band and linear axis scale implementations
- Added `value` path with bar value implementation
- Added `view` path that contains vertical bar view
- Added `color` path to create colors from hex and rgb
- Added `error` path with custom error type
- Added `chart` path that uses all the paths above to create SVG charts
- Added vertical bar chart example