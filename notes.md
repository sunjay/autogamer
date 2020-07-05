# Autogamer Design Notes

## Python Interface

* There are sort of two interfaces:
  1. For defining the levels and the UI
  2. For defining the entities and components inside a level

* Most entities are created automatically by finding the objects defined in the
  level map
* Player can use the Python API to query for entities and then add components to
  them
* Many entities have default components for things like position, size, etc.
  * Custom properties on tiles may also result in components being added

## Implementation

* System Resources:
  * Game state (Running, Paused, etc.)
  * Viewport (position of camera)

* Viewport system
  * Updates the viewport position based on `ViewportTarget` component (if necessary)
  * Update the current `viewport_boundary` (if necessary)
  * Update the viewport to be within the current `viewport_boundary` (if any)

* Event handling
  * Events are given to the current screen
  * If the current screen is a level screen:
    * Events are passed to the HUD first
    * Events are then passed to the level where they can only be accessed via a
      System
