set terminal pdf
set output "input_enkf_x.pdf"
plot "to_print.ssv" \
    title "observation" \
    with lines \
    linecolor rgb "dark-green" \
    linetype 1\
    linewidth 1, \
    "enkf_da.ssv" \
        title "EnKF" \
        with lines \
        linecolor rgb "dark-pink" \
        linetype 1 \
        linewidth 1
