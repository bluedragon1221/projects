from adventurelib import *

places = {
    "house": Room("""
        You stand before your quaint cottage, its chimney puffing wisps of fragrant smoke.
        The door is slightly ajar, revealing glimpses of a warm, golden glow within.
        Soft, inviting sounds of gentle laughter and clinking dishes drift out,
        accompanied by the aroma of freshly baked bread.
        A well-worn welcome mat greets your feet, beckoning you to step inside
        and leave your worries at the threshold.
    """),

    "forest": Room("""
        You stand at the edge of an ancient forest, its canopy a tapestry of emerald and gold.
        Towering trees stretch as far as the eye can see, their gnarled roots weaving intricate patterns on the forest floor.
        A chorus of birdsong and rustling leaves fills the air, punctuated by the occasional snap of a twig.
        Shafts of sunlight pierce through the dense foliage, illuminating patches of vibrant wildflowers and curious mushrooms.
        The path before you winds deeper into the woods, promising adventure and secrets waiting to be uncovered.
    """),

    "blacksmith": Room("""
        The blacksmith's forge glows with an intense, orange light, casting dancing shadows across the soot-stained walls.
        The air is thick with the acrid smell of hot metal and coal, while the rhythmic clang of hammer on anvil echoes through the workshop.
        Racks of finely crafted weapons and armor line the walls, each piece a testament to the smith's skill and artistry.
        The blacksmith, arms rippling with muscle, stands before the anvil, sweat glistening on their brow as they shape glowing metal into deadly beauty.
        Do you want to ask him anything?
    """),

    "store": Room("""
        You step into a cozy, cluttered shop, where shelves bow under the weight of countless curiosities and essentials.
        The air is rich with the mingled scents of herbs, leather, and parchment, while motes of dust dance in beams of light filtering through grimy windows.
        Glass jars filled with colorful potions and peculiar ingredients line one wall, their labels barely legible in spidery handwriting.
        A wizened shopkeeper peers at you from behind a crowded counter, surrounded by trinkets, maps, and mysterious artifacts of unknown origin.
    """),
}

current_location = places["house"]

knife = Item('a sharp knife', 'knife')
knife.max_hp = 300
knife.hp = 300
knife.damage = 3
knife.tags = ["attack"]

axe = Item("a woodcutter's axe", "axe")
axe.max_hp = 250
axe.hp = 250
axe.damage = 5
axe.tags = ["chop", "attack"]

bedroll = Item("an traveler's bedroll", "bedroll")

inventory = Bag([bedroll, knife, axe])

hp = 50
gold = 300

@when("where am i")
def get_location():
    print(current_location)

@when("walk to PLACE")
@when("go to PLACE")
def go_to(place):
    if place in places.keys():
        global current_location
        current_location = places[place]
        set_context(current_location)
        get_location()
    else:
        print(f"That place doesn't exist (yet...). Here are the available places:")
        for i in places.keys():
            print(f"- {i}")

@when("repair TOOL", context="blacksmith")
def repair(tool):
    selected_tool = inventory.find(tool)
    percent = selected_tool.max_hp / selected_tool.hp

    print("Your tool is at {percent*100} hp. Do you want to repair it?")
    print("I can repair your tool, but it will cost you ")
    
@when("upgrade TOOL", context="blacksmith")
def upgrade(tool):
    pass

# @when("buy ITEM", context="shop")
# def buy(item):
#     pass

# @when("sell ITEM", context="shop")
# def sell(item):
#     pass

def main():
    start()
