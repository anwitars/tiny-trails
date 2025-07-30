def random_string(length: int = 8) -> str:
    """
    Generate a random string of fixed length.
    The characters set includes uppercase and lowercase letters and digits.
    """

    import random
    import string

    letters = string.ascii_letters + string.digits
    return "".join(random.choice(letters) for _ in range(length))
