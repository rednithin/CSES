n = int(input())

answer = 1
mod = 10 ** 9 + 7
for i in range(n):
    answer = (answer * 2) % mod

print(answer)