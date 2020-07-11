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

## Characters - Players & Enemies

Goal: have characters use the proper sprites for walking, jumping, getting hit,
etc. while also ensuring that the behaviour is natural and as the user expectes.

Examples/Test Cases:

* when a character is walking, their walk cycle plays on a loop but only while
  they are touching something below them (no walking on air)
  * enhancement: walk cycle is synced to movement speed
* when a character jumps while also walking, they switch to a jump sprite
* when a character walks off of a ledge, their walk cycle stops and they start
  to use a fall sprite
  * if a character jumps while falling (e.g. if a character has wings or
    something), the jump should take precedence because it was the last action
    that took place
* regardless of whether a character is walking, jumping, falling, etc., if they
  are hit by something that causes them damage, they should change to a hit
  sprite and then freeze momentarily or die if their health reaches zero
  * freeze for a player == not responding to keyboard input
  * death might mean taking an animated fall off the screen (still not
    responding to keyboard input)
* taking a hit should force you back away from the thing you hit (so you don't
  lose all your health immediately)
* when attacking something by jumping on top of it, you should be repelled away
  so you don't immediately land all the hits needed to destroy something
  * this should switch your sprite from a fall sprite to a jump sprite if you
    were previously falling

### Implementation

Systems can generate events and put them into a resource that allows other
systems further down the line handle those events.

Examples:

* Walking and stopping
  1. Keyboard system sets the player's velocity or force
     * The force or velocity change will not be applied if the player is
       currently frozen because of a hit or something
  2. Collision system makes a list of everything that is colliding with
     everything else
  2. Animation system sees that the player is colliding with the ground and
     their velocity on the x-axis is non-zero and then sets the animation to a
     walk cycle + sets flip parameters to match their direction

* Attacking an enemy
  1. Keyboard system generates an Attack event for the player
  2. Collision system makes a list of everything that is colliding with
     everything else
     * Player will have a slightly wider attack hit box so they don't have to be
       directly touching another entity to attack it
  3. Animation system freezes the player (so they can't keep walking) and plays
     the attack animation
  4. Combat system notices attack event, checks if attack hit box is colliding
     with something that has a health component, and then decreases that health
     component
     * Choosing to attack anything with a health component means you can make
       both enemies and obstacles (e.g. a door) attackable

* Hitting an obstacle or enemy
  1. Collision system makes a list of everything that is colliding with
     everything else
  2. Combat system notices that the player is colliding with something that has
     a damage component, then subtracts from the health of the player and
     generates a force backwards to prevent the player from hitting the obstacle
     again
     * Player is momentarily frozen
     * Might want to consider making the player momentarily invincible too
       so they have time to get away
