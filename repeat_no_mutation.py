def repeat(a, times):
    if times > 0:
        a()
        repeat(a, times - 1)
       
repeat(lambda: print("hello"), 5)
