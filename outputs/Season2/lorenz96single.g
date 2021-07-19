set terminal png
set output "part6/mpf_res_all.png"
splot "part6/mpf_res_all.ssv" \
        using 2:3:4 \
        title "Lorenz96" \
        with lines \
        linecolor rgb "dark-green" \
        linetype 1\
        linewidth 1

