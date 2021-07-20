set terminal png
set output "true_obs_rmse.png"
plot [][0:8] "true_obs_rmse.ssv" \
        title "RMSE" \
        with linespoints \
        linetype 1\
        linewidth 1\
        linecolor rgb "dark-green" \
        pointtype 1\
        pointsize 1

