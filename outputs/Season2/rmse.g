set terminal png
set output "part6/true_mpf_res_skip02_rmse.png"
plot [][0:8] "part6/true_mpf_res_skip02_rmse.ssv" \
        title "RMSE" \
        with linespoints \
        linetype 1\
        linewidth 1\
        linecolor rgb "dark-green" \
        pointtype 1\
        pointsize 1

