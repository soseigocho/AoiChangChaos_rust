set terminal pdf
set output "input_mpf_x.pdf"
plot "to_print.ssv" \
    title "observation" \
    with lines \
    linecolor rgb "dark-green" \
    linetype 1\
    linewidth 1, \
    "mpf_da.ssv" \
        title "MPF" \
        with lines \
        linecolor rgb "dark-pink" \
        linetype 1 \
        linewidth 1
