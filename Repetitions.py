s = input()
last = 'X'
count = 1
max_count = 1

for c in s:
    if c == last:
        count += 1
    else:
        max_count = max(count, max_count)
        count = 1
        last = c

max_count = max(count, max_count)
print(max_count)
