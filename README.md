# Armor Trims
This program produces every possible combination of armor item, armor trim, and trim material.

It works by reading in the structure file armor.nbt and then editing the nbt to give every armor stand a different combination.

It produces 6 structure files named `armor_[armor material].nbt` that can then be loaded into a world via datapacks.

The provided armor_showcase.zip has the 6 armor showcase structures I generated plus one structure `quartz:armor_showcase` that combines all of them

If you want to use your own armor.nbt structure you'll have to edit the math I used to find each armor stand and chest.