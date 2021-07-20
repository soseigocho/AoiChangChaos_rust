set terminal png
set output "part6/true_enkf_res_skip02_each_and_first_half_rmse.png"
plot [][0:8] "part6/true_enkf_res_skip02_each_rmse.ssv" \
        title "Each" \
        with linespoints \
        linetype 1\
        linewidth 1\
        linecolor rgb "dark-green" \
        pointtype 1\
        pointsize 1, \
        "part6/true_enkf_res_skip02_first_half_rmse.ssv" \
            title "First-half" \
            with linespoints \
            linetype 1\
            linewidth 1\
            linecolor rgb "dark-pink" \
            pointtype 1\
            pointsize 1
