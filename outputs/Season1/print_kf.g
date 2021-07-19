set terminal png
set output "input_kf_x.png"
plot "to_print.ssv" \
    title "observation" \
    with lines \
    linecolor rgb "dark-green" \
    linetype 1\
    linewidth 1, \
    "kf_da.ssv" \
        title "KF" \
        with lines \
        linecolor rgb "dark-pink" \
        linetype 1 \
        linewidth 1
