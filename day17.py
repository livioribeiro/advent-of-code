from itertools import combinations
m = [43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38]

# Part 1 - Sum the number of all combinations that hold 150 L.
total = 0
for i in range(1, len(m)+1):
    total += len([x for x in combinations(m, i) if sum(x) == 150])
print(total)

# Part 2 - Break at the first combination that holds 150 L,
#    that's the smallest. Then just print the total as that's
#    the number of ways you can store 150 L in those containers.
total = 0
for i in range(1, len(m)+1):
    total += len([x for x in combinations(m, i) if sum(x) == 150])
    if total:
        break
print(total)
