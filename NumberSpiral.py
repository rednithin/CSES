t = int(input())

while t != 0:
    y, x = [int(x) for x in input().split()]
    big, small = max(x, y), min(x, y)
       
    if (big % 2 == 1 and big == x) or (big % 2 == 0 and big == y):
        print(big ** 2 - (small - 1))
    else:
        print((big - 1) ** 2 + small)
    t -= 1