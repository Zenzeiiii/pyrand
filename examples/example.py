from pyrand import set_seed, randfloat, randint, uniform, choice, sample, shuffle, randint_array, random_array

if __name__ == "__main__":
    set_seed(42)

    print("Random float:", randfloat())
    print("Random int 1-10:", randint(1, 10))
    print("Uniform 0-5:", uniform(0, 5))

    lst = [1,2,3,4,5]
    print("Choice:", choice(lst))
    print("Sample:", sample(lst, 3))
    shuffle(lst)
    print("Shuffled:", lst)

    arr = randint_array(1, 100, 10)
    print("Randint array:", arr)
    arr2 = random_array(5)
    print("Random array:", arr2)
    