#!/usr/bin/env bash
# Test-oracle provisioning only: public pinned packages, no root install or private image.
# Debian 13 / amd64 with its GCC 14, make and R shared-library dependencies.
set -euo pipefail
if [[ $# != 1 ]]; then
  echo "usage: $0 ABSOLUTE_RUNTIME_DIRECTORY" >&2
  exit 2
fi
prefix=$1
if [[ ! "$prefix" =~ ^/[A-Za-z0-9._/-]+$ ]]; then
  echo "runtime path must be absolute and contain no spaces or shell metacharacters" >&2
  exit 2
fi
if [[ $(uname -m) != x86_64 ]] || ! grep -q '^VERSION_ID="13"$' /etc/os-release ||
   ! grep -q '^ID=debian$' /etc/os-release; then
  echo "this bounded provisioning recipe requires Debian 13 / amd64" >&2
  exit 2
fi
script_dir=$(cd -- "$(dirname -- "$0")" && pwd)
cache=${LIMMA_REFERENCE_CACHE:-"$prefix/downloads"}
mkdir -p "$cache" "$prefix/root" "$prefix/library"
while IFS=$'\t' read -r sha name url; do
  [[ "$sha" == \#* || -z "$sha" ]] && continue
  if [[ ! -f "$cache/$name" ]]; then
    curl --fail --location --retry 2 --connect-timeout 15 --max-time 120 "$url" -o "$cache/$name"
  fi
  printf '%s  %s\n' "$sha" "$cache/$name" | sha256sum --check --status
  if [[ "$name" == *.deb ]]; then
    dpkg-deb --extract "$cache/$name" "$prefix/root"
  fi
done < "$script_dir/reference-packages.sha256.tsv"
rhome="$prefix/root/usr/lib/R"
# Relocate only Debian's filesystem wrappers/configuration, never R numerical code.
for config in Makeconf Renviron.site Rprofile.site ldpaths repositories; do
  ln -sfn "$prefix/root/etc/R/$config" "$rhome/etc/$config"
done
cp "$rhome/etc/Renviron.ucf" "$prefix/root/etc/R/Renviron"
ln -sfn "$prefix/root/etc/R/Renviron" "$rhome/etc/Renviron"
for item in share include; do
  ln -sfn "$prefix/root/usr/share/R/$item" "$rhome/$item"
done
ln -sfn "$prefix/root/usr/share/doc/r-base-core" "$rhome/doc"
for wrapper in "$prefix/root/usr/bin/R" "$rhome/bin/R"; do
  sed -i \
    -e "s|^R_HOME_DIR=/usr/lib/R$|R_HOME_DIR=$rhome|" \
    -e "s|^R_SHARE_DIR=/usr/share/R/share$|R_SHARE_DIR=$prefix/root/usr/share/R/share|" \
    -e "s|^R_INCLUDE_DIR=/usr/share/R/include$|R_INCLUDE_DIR=$prefix/root/usr/share/R/include|" \
    -e "s|^R_DOC_DIR=/usr/share/R/doc$|R_DOC_DIR=$prefix/root/usr/share/R/doc|" "$wrapper"
done
compiler="$prefix/root/usr/bin/x86_64-linux-gnu-gfortran-14"
gcc_lib="$prefix/root/usr/lib/gcc/x86_64-linux-gnu/14"
printf 'FC = %s -B%s/\nF77 = %s -B%s/\nFLIBS = -L%s -lgfortran -lm -lquadmath\n' \
  "$compiler" "$gcc_lib" "$compiler" "$gcc_lib" "$gcc_lib" > "$prefix/Makevars"
export R_MAKEVARS_USER="$prefix/Makevars"
export R_LIBS_USER="$prefix/library"
export R_INSTALL_VANILLA=1
export TZ=UTC LC_ALL=C
for package in statmod_1.5.2.tar.gz limma_3.68.5.tar.gz; do
  "$prefix/root/usr/bin/R" CMD INSTALL --no-multiarch --library="$prefix/library" "$cache/$package"
done
"$prefix/root/usr/bin/R" --vanilla --slave -e \
  'stopifnot(getRversion() == "4.5.0", packageVersion("limma") == "3.68.5", packageVersion("statmod") == "1.5.2"); print(sessionInfo())'

