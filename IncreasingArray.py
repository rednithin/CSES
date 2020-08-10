n = int(input())
arr = [int(x) for x in input().split(' ')]

increments = 0

for i in range(1, n):
    diff = arr[i-1] - arr[i]
    if diff > 0:
        increments += diff
        arr[i] = arr[i-1]

print(increments)

