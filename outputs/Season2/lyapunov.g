set terminal pdf
set output "lyapunov.pdf"
plot 0 \
        title "0"\
        with lines \
        linecolor rgb "dark-pink"\
        linetype 1\
        linewidth 1 ,\
        "lyapunov_exponent.ssv" \
        title "Lyapunov Exponent" \
        with lines \
        linecolor rgb "dark-green" \
        linetype 1\
        linewidth 1\
