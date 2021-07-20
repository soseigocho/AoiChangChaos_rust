set terminal png
set output "enkf_mv_skip02_first_half.png"
plot [][0:]"enkf_mv_skip02_first_half.ssv" \
        using 1:2 \
        title "Member's Variance on Dim[0]" \
        with lines \
        linecolor rgb "dark-green" \
        linetype 1\
        linewidth 1

