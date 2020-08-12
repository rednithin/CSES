n = int(input())
apples = [int(x) for x in input().split(' ')]
summ = sum(apples)

min_diff = summ

for i in range(1 << n):
    sub_sum = 0
    for j in range(n):
        if (1 << j & i):
            sub_sum += apples[j]
    
    min_diff = min(min_diff, abs(summ - 2 * sub_sum))

print(min_diff)
    