from collections import Counter

s = input()

f = Counter(s)

first_half = ""
second_half = ""

single = ""

# print(f.values())
valid_set = [x for x in f.values() if x % 2]
if len(valid_set) > 1:
    print("NO SOLUTION")
    exit(0)

for a, b in f.items():
    if b % 2 == 0:
        t = a * (b // 2)
        first_half += t 
        second_half = t + second_half
    else:
        single = a * b

print(first_half+ single +second_half)