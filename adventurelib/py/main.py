from ad import Object, Container, Player
from env import Environment

player = Player("Collin")

middle_meadow = Environment("middle meadow")

@middle_meadow.command("Go to the patch of flowers")
def patch_of_flowers():
    print("""You walk over to the patch of flowers.
There are beautiful and you can't help but wonder if your mom would like them.""")

    flower_patch = Environment("flower patch")

    @flower_patch.command("Pick them")
    def pick_flowers():
        print("You pick the flowers and head back to the middle of the meadow")
        flowers = Object.new_item("flowers x5")
        player.give_item(flowers)

    @flower_patch.command("Don't")
    def dont_pick_flowers():
        print("You turn around and head back to the middle of the meadow")

    flower_patch.show_menu("Do you want to pick them?")

    middle_meadow.rm_command("Go to the patch of flowers")

@middle_meadow.command("Go to the dark cluster of trees")
def tree_cluster():
    print("""You walk over to the cluster of trees.
Looking around further, you see an old, rusty sword hidden in the log of the tree.""")

    tree_cluster = Environment("tree cluster")

    @tree_cluster.command("Take it")
    def take_it():
        print("You stuff the sword in your backpack and return to the middle of the meadow")
        sword = Object.new_weapon("rusty sword")
        player.give_item(sword)

    @tree_cluster.command("Dont")
    def dont_take_it():
        print("You don't take the sword, instead returning back to the middle of the meadow")

    tree_cluster.show_menu("Do you want to take it?")

    middle_meadow.rm_command("Go to the dark cluster of trees")

if __name__ == "__main__":
    print("""You arive in a meadow, surounded by a dense, lushous forest, tangled with vines and thick grasses.
To the left, you see a patch of flowers.
To the right there is a dark, cluster of trees that almost looks like it's hiding something...""")
    while True:
        if not middle_meadow.show_menu("What do you do?"):
            print("Thanks for playing!")
            break
    
