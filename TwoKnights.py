n = int(input())

for i in range(1, n+1):
    answer = (i ** 2) * (i ** 2 - 1) \
        - 4 * 2 \
        - 4 * 2 * 3 \
        - 4 * (i - 4) * 4 \
        - 4 * 4 \
        - 4 * (i - 4) * 6 \
        - (i - 4) ** 2 * 8
    print(answer//2)