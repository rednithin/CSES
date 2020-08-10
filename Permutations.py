import math

n = int(input())

if n == 1:
    print(1)
elif n == 4:
    print("2 4 1 3 ")
elif n < 4:
    print("NO SOLUTION")
else:
    half = int(math.ceil(n/2))
    answer = []
    for (a, b) in zip(range(1,half+1), range(half+1,2*half+1)):
        answer.append(str(a))
        if b <= n:
            answer.append(str(b))
    
    print(" ".join(answer))
