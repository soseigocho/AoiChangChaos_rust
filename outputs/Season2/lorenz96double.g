set terminal pdf
set output "1e_minus3_vs_1e_minus3_plus_1e_minus4.pdf"
splot "1e_minus3.ssv" \
        using 2:3:4 \
        title "Lorenz96 Dim[0] + 1e-3" \
        with lines \
        linecolor rgb "dark-green" \
        linetype 1\
        linewidth 1, \
        "plus_1e_minus4.ssv" \
                using 2:3:4 \
                title "Lorenz96 Dim[0] + 1e-3 + 1e-4" \
                with lines \
                linecolor rgb "dark-pink" \
                linetype 1\
                linewidth 1
