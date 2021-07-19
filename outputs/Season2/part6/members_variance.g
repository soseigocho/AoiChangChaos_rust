set terminal png
set output "mpf_mv_all.png"
plot [][0:]"mpf_mv_all.ssv" \
        using 1:2 \
        title "Member's Variance on Dim[0]" \
        with lines \
        linecolor rgb "dark-green" \
        linetype 1\
        linewidth 1

