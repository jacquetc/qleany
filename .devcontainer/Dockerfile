FROM fedora:latest


RUN dnf install -y ninja-build gcc cmake extra-cmake-modules gdb clang clang-tools-extra
RUN dnf install -y qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtsvg-devel qcoro-qt6-devel
RUN dnf install -y kf6-kirigami-devel kf6-kcoreaddons-devel kf6-kconfig-devel kf6-ki18n-devel
RUN dnf install -y python3-poetry
RUN dnf install -y git
RUN mkdir .venv

# ARG REINSTALL_CMAKE_VERSION_FROM_SOURCE="3.22.2"

# Optionally install the cmake for vcpkg
# COPY ./reinstall-cmake.sh /tmp/

# RUN if [ "${REINSTALL_CMAKE_VERSION_FROM_SOURCE}" != "none" ]; then \
#         chmod +x /tmp/reinstall-cmake.sh && /tmp/reinstall-cmake.sh ${REINSTALL_CMAKE_VERSION_FROM_SOURCE}; \
#     fi \
#     && rm -f /tmp/reinstall-cmake.sh

# [Optional] Uncomment this section to install additional vcpkg ports.
# RUN su vscode -c "${VCPKG_ROOT}/vcpkg install <your-port-name-here>"

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
