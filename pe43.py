def conc(digit_list):
    '''
    Concatenate the last three digits into a three digit number.
    '''
    t = 0
    for d in digit_list:
        t *= 10
        t += d
    return t


def conc_last_three(digit_list):
    assert len(digit_list) >= 3
    return conc(digit_list[-3:])


def f(*digit_list):
    '''
    Recursive search.
    '''
    if len(digit_list) == 4 and not conc_last_three(digit_list) % 2 == 0:
        return 0
    if len(digit_list) == 5 and not conc_last_three(digit_list) % 3 == 0:
        return 0
    if len(digit_list) == 6 and not conc_last_three(digit_list) % 5 == 0:
        return 0
    if len(digit_list) == 7 and not conc_last_three(digit_list) % 7 == 0:
        return 0
    if len(digit_list) == 8 and not conc_last_three(digit_list) % 11 == 0:
        return 0
    if len(digit_list) == 9 and not conc_last_three(digit_list) % 13 == 0:
        return 0
    if len(digit_list) == 10 and not conc_last_three(digit_list) % 17 == 0:
        return 0

    # print(digit_list)
    if len(digit_list) == 10:
        return conc(digit_list)

    # Descend
    new_digits = set(range(10)) - set(digit_list)
    search_generator = (f(*digit_list, x) for x in new_digits)
    return sum(search_generator)


answer = f()
print(answer)
