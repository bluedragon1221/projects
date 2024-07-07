def bot_gen_range(prev_range: (int, int), prev_guess, guess_grage):
    low = prev_range.0
    high = prev_range.1

    match guess_grade:
        "higher" => high = prev_guess,
        "lower" => low = prev_guess,
        "just_right" => ()

    (low, high)
