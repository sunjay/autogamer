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
