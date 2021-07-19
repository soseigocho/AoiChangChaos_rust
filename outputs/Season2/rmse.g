set terminal png
set output "true_origin_1e_minus3_rmse.png"
plot [][0:8] "true_origin_plus_1e_minus3_rmse.ssv" \
        title "RMSE" \
        with points \
        pointtype 1\
        pointsize 1

