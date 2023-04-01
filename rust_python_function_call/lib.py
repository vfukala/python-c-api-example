def give_two():
    return 2

def give_five():
    return 5

def take_five(x):
    if x != 5:
        raise "NOT FIVE!!!"

def give_list_a():
    return [ 8, 7, 5, 6, 1, 2, 3 ]

def get_sorted_list(li):
    return sorted(li)

def get_cubed(x):
    return x ** 3

def get_binary(n):
    return bin(n)
