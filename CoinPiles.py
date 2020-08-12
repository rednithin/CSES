# 2x + y = a
# x + 2y = b

t = int(input())

while t != 0:
    a, b = [int(x) for x in input().split(' ')]
    diff = (2 * b - a)
    if  diff % 3 == 0 and diff >= 0:
        y = diff // 3
        
        second_diff = (a - y)
        if second_diff % 2 == 0 and second_diff >= 0:
            print("YES")
        else:
            print("NO")
    else:
        print("NO")
    t -= 1