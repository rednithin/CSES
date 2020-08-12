from itertools import permutations

s = input()

ans_set = set()
ans_list = []

s = sorted(s)

for perm in permutations(s):
    ans = "".join(perm)
    if not ans in ans_set:
        ans_set.add(ans)
        ans_list.append(ans)

print(len(ans_set))
print("\n".join(ans_list))

