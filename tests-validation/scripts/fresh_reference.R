#!/usr/bin/env Rscript
# Independent, synthetic LM01a oracle. No old golden or patient data are read.
# Usage: Rscript --vanilla tests-validation/scripts/fresh_reference.R OUTPUT_DIRECTORY
# Requires the exact versions in reference-environment.json; does not install packages.
args <- commandArgs(trailingOnly = TRUE)
if (length(args) != 1L) stop("supply one empty output directory")
out <- args[[1L]]
if (dir.exists(out) && length(list.files(out, all.files = TRUE, no.. = TRUE))) {
    stop("output directory must be empty; never overwrite a reference")
}
if (getRversion() != "4.5.0") stop("reference requires R 4.5.0")
if (as.character(packageVersion("limma")) != "3.68.5") stop("requires limma 3.68.5")
if (as.character(packageVersion("statmod")) != "1.5.2") stop("requires statmod 1.5.2")
suppressPackageStartupMessages(library(limma))
suppressPackageStartupMessages(library(statmod))
dir.create(out, recursive = TRUE, showWarnings = FALSE)
write.csv(data.frame(package = c("R", "limma", "statmod"),
                     version = c(as.character(getRversion()), "3.68.5", "1.5.2")),
          file.path(out, "versions.csv"), row.names = FALSE)
writeLines(capture.output(sessionInfo()), file.path(out, "sessionInfo.txt"))
writeLines("RNGkind: Mersenne-Twister, Inversion, Rejection; seed: 20261122",
           file.path(out, "rng.txt"))

write_mat <- function(m, dir, file) {
    m <- as.matrix(m)
    rn <- rownames(m)
    if (is.null(rn)) rn <- as.character(seq_len(nrow(m)))
    cn <- colnames(m)
    if (is.null(cn)) cn <- paste0("V", seq_len(ncol(m)))
    vals <- lapply(seq_len(ncol(m)), function(j) {
        ifelse(is.na(m[, j]), "NA", sprintf("%.17g", m[, j]))
    })
    names(vals) <- cn
    tab <- data.frame(row = rn, vals, check.names = FALSE)
    write.csv(tab, file.path(dir, file), row.names = FALSE, quote = 1)
}

write_fit <- function(id, y, x, cm, downstream = TRUE) {
    dir <- file.path(out, id)
    dir.create(dir)
    write_mat(y, dir, "input_log2.csv")
    write_mat(x, dir, "design.csv")
    write_mat(cm, dir, "contrasts.csv")
    fit <- lmFit(y, x)
    write_mat(fit$coefficients, dir, "lmfit_coefficients.csv")
    write_mat(fit$stdev.unscaled, dir, "lmfit_stdev_unscaled.csv")
    write_mat(cbind(sigma = fit$sigma, df_residual = fit$df.residual,
                    Amean = fit$Amean), dir, "lmfit_scalars.csv")
    write_mat(fit$cov.coefficients, dir, "lmfit_cov_coefficients.csv")
    write_mat(cbind(pivot = fit$pivot - 1L), dir, "lmfit_pivot.csv")
    write_mat(matrix(fit$rank, dimnames = list("design", "rank")), dir, "lmfit_rank.csv")
    cf <- contrasts.fit(fit, cm)
    write_mat(cf$coefficients, dir, "contrasts_coefficients.csv")
    write_mat(cf$stdev.unscaled, dir, "contrasts_stdev_unscaled.csv")
    write_mat(cf$cov.coefficients, dir, "contrasts_cov_coefficients.csv")
    if (!downstream) return(invisible(fit))

    n <- nrow(y)
    for (robust in c(FALSE, TRUE)) for (trend in c(FALSE, TRUE)) {
        tag <- sprintf("ebayes_rob%s_trend%s", robust, trend)
        eb <- eBayes(cf, robust = robust, trend = trend)
        write_mat(eb$t, dir, paste0(tag, "_t.csv"))
        write_mat(eb$p.value, dir, paste0(tag, "_p.csv"))
        write_mat(eb$lods, dir, paste0(tag, "_lods.csv"))
        write_mat(cf$stdev.unscaled * sqrt(eb$s2.post), dir, paste0(tag, "_moderated_se.csv"))
        write_mat(cbind(s2_post = eb$s2.post, df_total = eb$df.total,
                        F = eb$F, F_p = eb$F.p.value,
                        s2_prior = rep_len(eb$s2.prior, n),
                        df_prior = rep_len(eb$df.prior, n)),
                  dir, paste0(tag, "_scalars.csv"))
        for (j in seq_len(ncol(cm))) {
            write_mat(topTable(eb, coef = j, number = Inf, sort.by = "none",
                               adjust.method = "BH", confint = 0.95),
                      dir, sprintf("%s_toptable_%d.csv", tag, j))
        }
        write_mat(topTable(eb, coef = seq_len(ncol(cm)), number = Inf,
                           sort.by = "none", adjust.method = "BH"),
                  dir, paste0(tag, "_toptable_F.csv"))
    }
    invisible(fit)
}

RNGkind("Mersenne-Twister", "Inversion", "Rejection")
set.seed(20261122)
ngenes <- 96L
group <- rep(c("A", "B", "C"), each = 4L)
covariate <- rep(c(-1.5, -0.5, 0.5, 1.5), 3L)
x <- cbind(intercept = 1, condition_B = as.numeric(group == "B"),
           condition_C = as.numeric(group == "C"), covariate = covariate)
rownames(x) <- sprintf("array_%02d", seq_along(group))
cm <- matrix(c(0, 1, 0, 0, 0, 0, 1, 0), nrow = 4L,
             dimnames = list(colnames(x), c("B_minus_A", "C_minus_A")))
g <- seq_len(ngenes)
mu <- 4 + g / 24
signal <- cbind(mu, (g %% 5 - 2) / 3, (g %% 7 - 3) / 4, cos(g) / 5)
noise <- matrix(rnorm(ngenes * length(group)), nrow = ngenes)
scale <- sqrt(0.02 + (g %% 9) / 30)
scale[c(4, 21)] <- scale[c(4, 21)] * 6
y <- signal %*% t(x) + noise * scale
dimnames(y) <- list(sprintf("synthetic_%03d", g), rownames(x))
write_fit("complete", y, x, cm)

missing <- y
missing[1, group != "A"] <- NA # per-feature rank 2, B/C not estimable
missing[2, group == "C"] <- NA # per-feature rank 3, C not estimable
missing[3, c(2, 6, 10)] <- NA # full rank with unequal residual df
missing[4, c(1, 5, 8, 12)] <- NA
write_fit("missing", missing, x, cm)

edges <- y[1:5, ]
edges[1, ] <- NA              # all missing: df=0, NA coefficient/SD/mean
edges[2, -1] <- NA            # one observation: residual df=0
edges[3, group != "A"] <- NA  # rank loss
edges[4, group == "C"] <- NA
write_fit("missing_edges", edges, x, cm, downstream = FALSE)

singular_x <- cbind(x, redundant_B = 2 * x[, "condition_B"])
singular_cm <- rbind(cm, redundant_B = 0)
singular_fit <- write_fit("singular_design", y[1:8, ], singular_x, singular_cm,
                          downstream = FALSE)
invalid_cm <- matrix(c(0, 0, 0, 0, 1), ncol = 1L,
                     dimnames = list(colnames(singular_x), "non_estimable"))
write_mat(invalid_cm, file.path(out, "singular_design"), "invalid_contrasts.csv")
err <- tryCatch({
    contrasts.fit(singular_fit, invalid_cm)
    stop("oracle unexpectedly accepted a non-estimable contrast")
}, error = function(e) conditionMessage(e))
stopifnot(identical(err, "trying to take contrast of non-estimable coefficient"))
writeLines(err, file.path(out, "singular_design", "invalid_contrast_error.txt"))
cat("Generated four synthetic cases; full eBayes robust/trend grid for complete and missing.\n")

