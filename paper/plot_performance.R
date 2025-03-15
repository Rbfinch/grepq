# Load necessary libraries
library(ggplot2)

# Create a data frame with the values from the table
data <- data.frame(
    tool = c("grepq", "fqgrep", "ripgrep", "seqkit grep", "grep", "awk", "gawk"),
    mean = c(0.19, 0.34, 3.56, 122.05, 344.79, 165.45, 287.66),
    sd = c(0.0021, 0.01, 0.01, 0.90, 1.24, 1.59, 1.68)
)

# Calculate speedup relative to grep, ripgrep, and awk
data$speedup_grep <- max(data$mean) / data$mean
data$speedup_ripgrep <- data$mean[data$tool == "ripgrep"] / data$mean
data$speedup_awk <- data$mean[data$tool == "awk"] / data$mean

data$speedup_grep_sd <- data$speedup_grep * (data$sd / data$mean)
data$speedup_ripgrep_sd <- data$speedup_ripgrep * (data$sd / data$mean)
data$speedup_awk_sd <- data$speedup_awk * (data$sd / data$mean)

# Create the bar plot for speedup relative to grep
ggplot(data, aes(x = tool, y = speedup_grep)) +
    geom_bar(stat = "identity", fill = "#000000") +
    geom_errorbar(aes(ymin = speedup_grep - speedup_grep_sd, ymax = speedup_grep + speedup_grep_sd), width = 0.2) +
    scale_y_log10() +
    labs(
        title = "Speedup of tools relative to grep",
        x = "",
        y = expression(Speedup ~ (log[10] ~ seconds))
    ) +
    theme_minimal()

# Save the plot to a PDF file in the paper folder
ggsave("./paper/performance_speedup_plot_grep.pdf")

# Create the bar plot for speedup relative to ripgrep
ggplot(data, aes(x = tool, y = speedup_ripgrep)) +
    geom_bar(stat = "identity", fill = "#000000") +
    geom_errorbar(aes(ymin = speedup_ripgrep - speedup_ripgrep_sd, ymax = speedup_ripgrep + speedup_ripgrep_sd), width = 0.2) +
    scale_y_log10() +
    labs(
        title = "Speedup of tools relative to ripgrep",
        x = "",
        y = expression(Speedup ~ (log[10] ~ seconds))
    ) +
    theme_minimal()

# Save the plot to a PDF file in the paper folder
ggsave("./paper/performance_speedup_plot_ripgrep.pdf")

# Create the bar plot for speedup relative to awk
ggplot(data, aes(x = tool, y = speedup_awk)) +
    geom_bar(stat = "identity", fill = "#000000") +
    geom_errorbar(aes(ymin = speedup_awk - speedup_awk_sd, ymax = speedup_awk + speedup_awk_sd), width = 0.2) +
    scale_y_log10() +
    labs(
        title = "Speedup of tools relative to awk",
        x = "",
        y = expression(Speedup ~ (log[10] ~ seconds))
    ) +
    theme_minimal()

# Save the plot to a PDF file in the paper folder
ggsave("./paper/performance_speedup_plot_awk.pdf")
