# Notes

* player character needs to be 1.5 tiles tall and 0.75-1.0 tile wide
* background_items layer cannot be interacted it (only map layer is interacted with)
* level_start marker indicates player initial position
* jump pad turns into a different sprite when stepped on and doubles the player's jump
* player needs to press up or down to grab ladder and finish climb to doorway
* door opens automatically (changes to open door sprite) when player crosses it while holding a key
  * idea: door opens automatically (with a sound) when key is collected
  * open door tiles move to background_tiles2 layer since they can't be interacted with
* door is not crossible until player is holding a key
* player can collect coins
* player can collect diamonds (worth 25 coins?)
* player can climb initial ladder
* viewport stays within the current room (within objects with type viewport_boundary)
  * since the viewport can't always fit entirely within the boundary: the tiles outside the boundary are not shown
  * if you are within multiple viewport boundaries, all objects in all boundaries become visible
  * if you are not within *any* viewport boundaries, all objects around you are visible (no boundary checking)
* objects outside of the current boundary are not added until you enter the boundary and are removed after you leave (?)
* idea: laser colors each have different timing delays of on/off (no switches)
* laser switches of the same color are synced
* laser emitters have a different sprite when off
* switch towards right = ON
* switch towards left = OFF
* idea: each switch toggles each laser (so if the laser was initially off, it becomes on)
  * each laser has different initial conditions
* blade wheel rotates using its second sprite and moves left and right within the surrounding tiles at its configured speed
  * blade wheels can pass through each other but not other tiles
* blue lasers turn on and off at a set interval
* top spikes drop from the ceiling from left to right one at a time, repeats on a set interval
* bottom spikes show and hide at set intervals (all together, not one at a time?)
* green and red worm things come in and out of the ground at set intervals (intervals alternate start times)
* you can jump through the bottoms of platforms and land on top
* spiders have a walk animation
* spiders can walk left and right but must stay within the boundaries
  * when a spider sees you, it begins to move towards you a little faster
  * slows down after a few seconds if you go out of left/right view
  * you can jump on top of them to defeat them
* once you enter a boss_area, you can't leave
* boss always stays within `boss_movement_zone` rectangles and can jump between them
* you can hit the boss by jumping on their head
* boss shoots laser pieces in 3 directions (with a few second delay where the boss can be hit)
* once the boss has been beat, the castle door opens and you walk through
