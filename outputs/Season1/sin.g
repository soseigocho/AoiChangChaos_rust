set terminal pdf
set output "sin.pdf"
plot "to_print.ssv" \
        title "observation" \
        with lines \
        linecolor rgb "dark-green" \
        linetype 1\
        linewidth 1\
