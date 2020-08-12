# for i in range(1, 30):
#     sum = i * (i + 1) // 2
#     if sum % 2 == 0:
#         print(i, "YES")
#     else:
#         print(i, "NO")

n = int(input())
sum = n * (n + 1) / 2

if (sum % 2 != 0):
    print("NO")
    exit(0)

if n % 4 == 0:
    v1 = ["1", "4"]
    v2 = ["2", "3"]
    offset = 4
else:
    v1 = ["1", "2"]
    v2 = ["3"]
    offset = 3

for i in range(0, (n - 1) // 4):
    v1.append(str(4 * i + 1 + offset))
    v1.append(str(4 * i + 4 + offset))
    v2.append(str(4 * i + 2 + offset))
    v2.append(str(4 * i + 3 + offset))

print("YES")
print(len(v1))
print(" ".join(v1))
print(len(v2))
print(" ".join(v2))