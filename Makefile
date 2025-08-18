## The cross compilation targets assume execution on macOS with the necessary tools installed.
## This Makefile is designed to work with Rust projects and provides commands for building, testing, packaging, and more.
## It includes targets for:
## - creating macOS .pkg installers for both Intel and ARM architectures, 
## - building zip-installers for Windows
## - creating an RPM package for Fedora/RHEL Linux.
## 
## It also includes commands for linting, formatting, and generating coverage reports.
SHELL := /bin/bash

## ----------------------------------------------------------------------------
## You should adjust BUNDLE_ID per your needs
## ----------------------------------------------------------------------------
## The bundle unique ID to use in Apple install package
BUNDLE_ID="nu.aditus.foss"


## ----------------------------------------------------------------------------
## NO NEED TO CHANGE ANYTHING BELOW THIS LINE!
## ----------------------------------------------------------------------------

## Name and email to use in RPM Spec file
RPM_USER := $(shell git config --get user.name)
RPM_USER_EMAIL := $(shell git config --get user.email)

## Check so that RPM command is available if we are running on Linux
ifeq ($(shell uname),Linux)
ifeq ($(shell command -v rpm),)
$(error "RPM command not found. Please install RPM tools to build the RPM package.")
else
RPM_DIST=$(shell rpm --eval '%{?dist}')
## The release number for the RPM package, increment this for each new build of the same version
RPM_RELEASE := "1$(RPM_DIST)"
endif
endif

ifeq ($(shell uname),Linux)
APP_NAME_PKG_PATH := $(shell cargo pkgid | cut -d# -f1)
APP_VERSION_PKG := $(shell cargo pkgid | cut -d# -f2)
else ifeq ($(shell uname),Darwin)
APP_NAME_PKG_PATH := $(shell cargo pkgid | cut -d '\#' -f1)
APP_VERSION_PKG := $(shell cargo pkgid | cut -d '\#' -f2)
else
$(error "Unsupported platform detected. Makefile can only be used on macOS or Linux.")
endif

## Unique ID for Apple OSX packages
BUNDLE_ID_PKG := "$(BUNDLE_ID).$(APP_NAME_PKG)"

## As RPM does not allow dashes in the version number, we replace them with underscores.
## and as Cargo does not allow underscores in the version number we must keep two versions. Sigh.
APP_VERSION_PKG_RPM := $(subst -,_,$(APP_VERSION_PKG))

APP_NAME_PKG := "$(shell basename $(APP_NAME_PKG_PATH))"
INSTALL_LOCATION_PKG := "/usr/local/bin"
TARGET_ARCH_INTEL_PKG := "x86_64-apple-darwin"
TARGET_ARCH_ARM_PKG := "aarch64-apple-darwin"
TARGET_ARCH_INTEL_WIN := "x86_64-pc-windows-gnu"
TARGET_ARCH_LINUX := "x86_64-unknown-linux-gnu"
TARGET_ARCH_LINUX := "x86_64-unknown-linux-gnu"
OUTPUT_DIR_PKG := "target/pkg"
OUTPUT_PKG_NAME_INTEL := "$(APP_NAME_PKG)-$(APP_VERSION_PKG)-intel.pkg"
OUTPUT_PKG_NAME_ARM := "$(APP_NAME_PKG)-$(APP_VERSION_PKG)-arm.pkg"
OUTPUT_PKG_NAME_INTEL_WIN := "$(APP_NAME_PKG)-$(APP_VERSION_PKG)-windows.zip"
OUTPUT_RPM_NAME := "$(APP_NAME_PKG)-$(APP_VERSION_PKG_RPM)-$(RPM_RELEASE).x86_64.rpm"

## Setup PHONY targets for better readability and to avoid conflicts with file names
.PHONY: help, all, all-bin, clean, b, br, test, r, rr, lint, fmt, cov-html, cov, \
tst-pkg, pkg, pkg-intel, pkg-arm, win-exe, rpm, b-linux, br-linux, bump, install-pkg, \
uninstall-pkg, uninstall-pkg, check, fonts, scc

.DEFAULT_GOAL := b ## Build debug version

help: ## Display this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

## Build and create installers
## Depending on what platform we run on we can only build a subset of the installers.
## If we are on Apple OSX we can build macOS and Windows installers, and if we are
## on fedora Linux we can build a RPM installer package.
ifeq ($(shell uname),Darwin)
installer: pkg win-zip 
else ifeq ($(shell uname),Linux)
## This is a mind f..k as grep will return exit status 0 on successful matching.
ifneq ($(shell cat /etc/system-release | grep -i fedora),)
installer: rpm 
else
installer: ## Build all Linux installers (no supported platforms detected)
	@echo "Unsupported Linux distribution detected. Building RPM is only supported on Fedora Linux."
endif
else
installer:
	@echo "Unsupported platform detected. Installer can only be created on macOS, or Fedora Linux."
endif

clean: ## Clean the project using cargo
	cargo clean

b: ## Build host arch debug profile using cargo
	@if echo "$(APP_VERSION_PKG)" | grep -qE '^(.*)-(alpha|beta)-([0-9]{6})$$'; then \
		echo "alpha/beta build detected, $(APP_VERSION_PKG), will bump build number..."; \
		$(MAKE) bump-build; \
	fi
	cargo build

br: ## Build host arch release profile using cargo
	@if echo "$(APP_VERSION_PKG)" | grep -qE '^(.*)-(alpha|beta)-([0-9]{6})$$'; then \
		echo "alpha/beta build detected, $(APP_VERSION_PKG), will bump build number..."; \
		$(MAKE) bump-build; \
	fi
	cargo build --release

fonts: ## Generate font data files
	@cd scripts && ./gen_font_data.sh

scc: ## Generate complexity report
	@scc -n font_dejavusans 

scc-md: ## Generate complexity report
	@scc -n font_dejavusans -f html-table | pandoc -f html -t markdown_strict > docs/scc.md
	@echo "Complexity report generated at docs/scc.md"

test: ## Test the project on host arch using cargo
	cargo test

check: ## Check the project on host arch using cargo
	cargo check

r: ## Run the project on host arch using debug profile using cargo
	cargo run

rr: ## Run the project using release profile on host arch using cargo
	cargo run --release

lint: ## Lint the project using cargo
	cargo clippy

fmt: ## Format the project using cargo
	cargo fmt

cov-html: ## Generate coverage report using llvm
	@cargo llvm-cov --html --ignore-filename-regex='main.rs'
	open target/llvm-cov/html/index.html

cov: ## Generate coverage summary to terminal using llvm
	@cargo llvm-cov --summary-only --ignore-filename-regex='main.rs'

debug-vars: ## Test the package creation process so that vars are set correctly
	@echo "---------------------------------------"
	@echo "---------- Package Variables ----------"
	@echo "---------------------------------------"
	@echo "RPM_USER: '$(RPM_USER)'"
	@echo "RPM_USER_EMAIL: '$(RPM_USER_EMAIL)'"
	@echo "RPM_DIST: '$(RPM_DIST)'"
	@echo "APP_NAME_PKG: '$(APP_NAME_PKG)'"
	@echo "APP_VERSION_PKG: '$(APP_VERSION_PKG)'"
	@echo "APP_VERSION_PKG_RPM: '$(APP_VERSION_PKG_RPM)'"
	@echo "BUNDLE_ID_PKG: '$(BUNDLE_ID_PKG)'"
	@echo "TARGET_ARCH_INTEL_PKG: '$(TARGET_ARCH_INTEL_PKG)'"
	@echo "TARGET_ARCH_INTEL_WIN: '$(TARGET_ARCH_INTEL_WIN)'"
	@echo "TARGET_ARCH_ARM_PKG: '$(TARGET_ARCH_ARM_PKG)'"
	@echo "TARGET_ARCH_LINUX: '$(TARGET_ARCH_LINUX)'"
	@echo "OUTPUT_DIR_PKG: '$(OUTPUT_DIR_PKG)'"
	@echo "OUTPUT_PKG_NAME_INTEL: '$(OUTPUT_PKG_NAME_INTEL)'"
	@echo "OUTPUT_PKG_NAME_ARM: '$(OUTPUT_PKG_NAME_ARM)'"
	@echo "OUTPUT_RPM_NAME: '$(OUTPUT_RPM_NAME)'"
	@echo "OUTPUT_PKG_NAME_INTEL_WIN: '$(OUTPUT_PKG_NAME_INTEL_WIN)'"
	@echo "INSTALL_LOCATION_PKG: '$(INSTALL_LOCATION_PKG)'"
	@echo ""

ifeq ($(shell uname),Linux)
pkg: ## Create a universal .pkg installer for both ARM and Intel Macs
	@echo "The 'pkg' target is only available on macOS."
else
pkg: pkg-intel pkg-arm
	@echo "--- Creating Universal macOS Package ---"
	@echo "Building ARM and Intel binaries..."
	@cargo build --release --target $(TARGET_ARCH_INTEL_PKG)
	@cargo build --release --target $(TARGET_ARCH_ARM_PKG)
	@echo "Combining binaries with lipo..."
	@mkdir -p $(OUTPUT_DIR_PKG)/universal
	@lipo -create \
        "target/$(TARGET_ARCH_INTEL_PKG)/release/$(APP_NAME_PKG)" \
        "target/$(TARGET_ARCH_ARM_PKG)/release/$(APP_NAME_PKG)" \
        -output "$(OUTPUT_DIR_PKG)/universal/$(APP_NAME_PKG)"
	@echo "Preparing staging directory..."
	@rm -rf "$(OUTPUT_DIR_PKG)/staging_universal"
	@mkdir -p "$(OUTPUT_DIR_PKG)/staging_universal/$(INSTALL_LOCATION_PKG)"
	@cp "$(OUTPUT_DIR_PKG)/universal/$(APP_NAME_PKG)" "$(OUTPUT_DIR_PKG)/staging_universal/$(INSTALL_LOCATION_PKG)/"
	@echo "Building universal .pkg..."
	@pkgbuild --root "$(OUTPUT_DIR_PKG)/staging_universal" \
        --identifier "$(BUNDLE_ID_PKG)" \
        --version "$(APP_VERSION_PKG)" \
        "$(OUTPUT_DIR_PKG)/$(APP_NAME_PKG)-$(APP_VERSION_PKG)-universal.pkg"
	@echo "Cleaning up staging directory..."
	@rm -rf "$(OUTPUT_DIR_PKG)/staging_universal"
	@echo ""
	@echo "------------------------------------"
	@echo "Universal macOS package created at: $(OUTPUT_DIR_PKG)/$(APP_NAME_PKG)-$(APP_VERSION_PKG)-universal.pkg"
	@echo "------------------------------------"
endif

b-intel: ## Build Intel arch debug profile using cargo
	cargo build --target $(TARGET_ARCH_INTEL_PKG)

br-intel: ## Build Intel arch release profile using cargo
	cargo build --release --target $(TARGET_ARCH_INTEL_PKG)

ifeq ($(shell uname),Linux)
pkg-intel: ## Create a .pkg installer for Intel Macs (x86_64)
	@echo "The 'pkg-intel' target is only available on macOS."
else
pkg-intel:
	@echo "--- Creating Intel macOS Package ---"
	@echo "Checking for Rust target $(TARGET_ARCH_INTEL_PKG)..."
	@if ! rustup target list --installed | grep -q $(TARGET_ARCH_INTEL_PKG); then \
		echo "Error: Rust target $(TARGET_ARCH_INTEL_PKG) not installed."; \
		echo "Please run: rustup target add $(TARGET_ARCH_INTEL_PKG)"; \
		exit 1; \
	fi
	@echo "Checking for pkgbuild command..."
	@if ! command -v pkgbuild >/dev/null 2>&1; then \
		echo "Error: pkgbuild command not found."; \
		echo "Please install Xcode Command Line Tools (run: xcode-select --install)."; \
		exit 1; \
	fi
	@echo "Building release binary for $(APP_NAME_PKG) version $(APP_VERSION_PKG) for $(TARGET_ARCH_INTEL_PKG)......"
	@cargo build --release --target $(TARGET_ARCH_INTEL_PKG)
	@echo "Creating package directory $(OUTPUT_DIR_PKG) ..."
	@mkdir -p $(OUTPUT_DIR_PKG)
	@echo "Creating Intel macOS package $(OUTPUT_PKG_NAME_INTEL)"
	@echo "Preparing staging directory..."
	@rm -rf "$(OUTPUT_DIR_PKG)/staging_intel"
	@mkdir -p "$(OUTPUT_DIR_PKG)/staging_intel/$(INSTALL_LOCATION_PKG)"
	@cp "target/$(TARGET_ARCH_INTEL_PKG)/release/$(APP_NAME_PKG)" "$(OUTPUT_DIR_PKG)/staging_intel/$(INSTALL_LOCATION_PKG)/"
	@echo "Building package..."
	@pkgbuild --root "$(OUTPUT_DIR_PKG)/staging_intel" \
        --identifier "$(BUNDLE_ID_PKG)" \
        --version "$(APP_VERSION_PKG)" \
        "$(OUTPUT_DIR_PKG)/$(OUTPUT_PKG_NAME_INTEL)"
	@echo "Cleaning up staging directory..."
	@rm -rf "$(OUTPUT_DIR_PKG)/staging_intel"
	@echo ""
	@echo "------------------------------------"
	@echo "Intel macOS package created at: $(OUTPUT_DIR_PKG)/$(OUTPUT_PKG_NAME_INTEL)"
	@echo "Bundle Identifier: $(BUNDLE_ID_PKG)"
	@echo "Installation Location: $(INSTALL_LOCATION_PKG)"
	@echo "------------------------------------"
endif

b-arm: ## Build ARM arch debug profile using cargo
	cargo build --target $(TARGET_ARCH_ARM_PKG)

br-arm: ## Build ARM arch release profile using cargo
	cargo build --release --target $(TARGET_ARCH_ARM_PKG)

ifeq ($(shell uname),Linux)
pkg-arm: ## Create a .pkg installer for ARM Macs (aarch64)
	@echo "The 'pkg-arm' target is only available on macOS."
else
pkg-arm:
	@echo "--- Creating ARM macOS Package ---"
	@echo "Checking for Rust target $(TARGET_ARCH_ARM_PKG)..."
	@if ! rustup target list --installed | grep -q $(TARGET_ARCH_ARM_PKG); then \
		echo "Error: Rust target $(TARGET_ARCH_ARM_PKG) not installed."; \
		echo "Please run: rustup target add $(TARGET_ARCH_ARM_PKG)"; \
		exit 1; \
	fi
	@echo "Checking for pkgbuild command..."
	@if ! command -v pkgbuild >/dev/null 2>&1; then \
		echo "Error: pkgbuild command not found."; \
		echo "Please install Xcode Command Line Tools (run: xcode-select --install)."; \
		exit 1; \
	fi
	@echo "Building release binary for $(APP_NAME_PKG) version $(APP_VERSION_PKG) for $(TARGET_ARCH_ARM_PKG)......"
	@cargo build --release --target $(TARGET_ARCH_ARM_PKG)
	@echo "Creating package directory $(OUTPUT_DIR_PKG) ..."
	@mkdir -p $(OUTPUT_DIR_PKG)
	@echo "Creating ARM macOS package $(OUTPUT_PKG_NAME_ARM)"
	@echo "Preparing staging directory..."
	@rm -rf "$(OUTPUT_DIR_PKG)/staging_arm"
	@mkdir -p "$(OUTPUT_DIR_PKG)/staging_arm/$(INSTALL_LOCATION_PKG)"
	@cp "target/$(TARGET_ARCH_ARM_PKG)/release/$(APP_NAME_PKG)" "$(OUTPUT_DIR_PKG)/staging_arm/$(INSTALL_LOCATION_PKG)/"
	@echo "Building package..."
	@pkgbuild --root "$(OUTPUT_DIR_PKG)/staging_arm" \
		--identifier "$(BUNDLE_ID_PKG)" \
		--version "$(APP_VERSION_PKG)" \
		"$(OUTPUT_DIR_PKG)/$(OUTPUT_PKG_NAME_ARM)"
	@echo "Cleaning up staging directory..."
	@rm -rf "$(OUTPUT_DIR_PKG)/staging_arm"
	@echo ""
	@echo "------------------------------------"
	@echo "ARM macOS package created at: $(OUTPUT_DIR_PKG)/$(OUTPUT_PKG_NAME_ARM)"
	@echo "Bundle Identifier: $(BUNDLE_ID_PKG)"
	@echo "Installation Location: $(INSTALL_LOCATION_PKG)"
	@echo "------------------------------------"
endif

b-win: ## Build Windows (x86_64-pc-windows-gnu) debug profile using cargo
	cargo build --target $(TARGET_ARCH_INTEL_WIN)

br-win: ## Build Windows (x86_64-pc-windows-gnu) release profile using cargo
	cargo build --release --target $(TARGET_ARCH_INTEL_WIN)

ifeq ($(shell uname),Linux)
win-zip: ## Create a Windows executable for x86_64-pc-windows-gnu
	@echo "The 'win-zip' target is only available on macOS."
else
win-zip: ## Cross-compile release binary for Windows (x86_64-pc-windows-gnu) as a zip file
	@echo "--- Building Windows ($(TARGET_ARCH_INTEL_WIN)) Release Binary ---"
	@echo "Checking for Rust target $(TARGET_ARCH_INTEL_WIN)..."
	@if ! rustup target list --installed | grep -q $(TARGET_ARCH_INTEL_WIN); then \
		echo "Error: Rust target $(TARGET_ARCH_INTEL_WIN) not installed."; \
		echo "Please run: rustup target add $(TARGET_ARCH_INTEL_WIN)"; \
		exit 1; \
	fi
	@echo "Checking for MinGW cross-compiler (x86_64-w64-mingw32-gcc)..."
	@if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then \
		echo "Warning: MinGW-w64 cross-compiler not found. If build fails, install with: brew install mingw-w64"; \
	fi
	@cargo build --release --target $(TARGET_ARCH_INTEL_WIN)
	@echo "Packaging Windows executable..."
	@mkdir -p $(OUTPUT_DIR_PKG)
	@cd target/$(TARGET_ARCH_INTEL_WIN)/release && zip $(OUTPUT_PKG_NAME_INTEL_WIN) $(APP_NAME_PKG).exe
	@cp target/$(TARGET_ARCH_INTEL_WIN)/release/$(OUTPUT_PKG_NAME_INTEL_WIN) $(OUTPUT_DIR_PKG)
	@echo ""
	@echo "------------------------------------"
	@echo "Intel Win package created at: $(OUTPUT_DIR_PKG)/$(OUTPUT_PKG_NAME_INTEL_WIN)"
	@echo "------------------------------------"
endif

bump: ## Bump the version number in Cargo.toml
	@echo "Current version is APP_VERSION_PKG: $(APP_VERSION_PKG)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"

install-pkg: pkg ## Install the package to the system
	@echo "Installing $(APP_NAME_PKG) version $(APP_VERSION_PKG)..."
	@sudo installer -pkg "$(OUTPUT_DIR_PKG)/$(OUTPUT_PKG_NAME_INTEL)" -target /
	@echo "Installation complete."

uninstall-pkg: ## Uninstall the package from the system
	@echo "Uninstalling $(APP_NAME_PKG) version $(APP_VERSION_PKG)..."
	@sudo pkgutil --forget "$(BUNDLE_ID_PKG)"
	@echo "Uninstallation complete."

qinst: br ## Quick install: Build and install the package on the host architecture
	@echo "Quick install: Building and installing $(APP_NAME_PKG)..."
	sudo cp target/release/$(APP_NAME_PKG) $(INSTALL_LOCATION_PKG)

bump-build: ## Increment alpha/beta build number (e.g., X.Y.Z-alpha-000001 -> X.Y.Z-alpha-000002)
	@current_version=$(APP_VERSION_PKG) ; \
	echo "Current version in Cargo.toml: $$current_version"; \
    if echo "$$current_version" | grep -qE '^(.*)-(alpha|beta)([1-9]*)-([0-9]{4})$$'; then \
        base_part=$$(echo "$$current_version" | sed -E 's/^(.*)-(alpha[1-9]*|beta[1-9]*)-([0-9]{4})$$/\1/'); \
        prerelease_label=$$(echo "$$current_version" | sed -E 's/^(.*)-(alpha[1-9]*|beta[1-9]*)-([0-9]{4})$$/\2/'); \
        build_digits_str=$$(echo "$$current_version" | sed -E 's/^(.*)-(alpha[1-9]*|beta[1-9]*)-([0-9]{4})$$/\3/'); \
        \
        build_digits_int=$$((10#$$build_digits_str)); \
        new_build_digits_int=$$((build_digits_int + 1)); \
        new_build_digits_str_padded=$$(printf "%04d" $$new_build_digits_int); \
        \
        updated_version="$$base_part-$$prerelease_label-$$new_build_digits_str_padded"; \
        echo "Attempting to update version to: $$updated_version"; \
        \
		sed -i.bak -E "s/^version[[:blank:]]*=[[:blank:]]*\"$${current_version}\"/version = \"$${updated_version}\"/" Cargo.toml ;\
        if [ $$? -eq 0 ]; then \
            rm -f Cargo.toml.bak; \
            echo "Successfully updated Cargo.toml to version $$updated_version"; \
        else \
            echo "Error: Failed to update Cargo.toml. Check sed command and file permissions."; \
            if [ -f Cargo.toml.bak ]; then echo "Restoring Cargo.toml from backup."; mv Cargo.toml.bak Cargo.toml; fi; \
        fi; \
    else \
        echo "Version '$$current_version' is not in 'X.Y.Z-(alpha[1-9]*|beta[1-9]*)-NNNN' format."; \
        echo "No build number incremented. To start an alpha/beta series, manually edit Cargo.toml"; \
        echo "to a format like '1.0.0-alpha1-0001'."; \
    fi

mac-install: pkg ## Install the macOS package
	@echo "Installing $(APP_NAME_PKG) version $(APP_VERSION_PKG)..."
	@sudo installer -pkg "$(OUTPUT_DIR_PKG)/$(OUTPUT_PKG_NAME_INTEL)" -target /
	@echo "Installation complete."

mac-uninstall: ## Uninstall the macOS package
	@echo "Uninstalling $(APP_NAME_PKG) version $(APP_VERSION_PKG)..."
	@sudo pkgutil --forget "$(BUNDLE_ID_PKG)"
	@echo "Uninstallation complete."

b-linux: ## Build Linux (x86_64-unknown-linux-gnu) debug profile using cargo
	cargo build --target $(TARGET_ARCH_LINUX)

br-linux: ## Build Linux (x86_64-unknown-linux-gnu) release profile using cargo
	cargo build --release --target $(TARGET_ARCH_LINUX)

ifeq ($(shell uname),Darwin)
rpm: ## Create an RPM package for Fedora/RHEL Linux
	@echo "The RPM target is only available on Linux. Use 'make b-linux' to cross-compile the linux binary on OSX."
	@exit 1
else ifeq ($(RPM_RELEASE),"")
rpm:
	@echo "RPM_RELEASE variable is not set. Cannot build RPM package."
	@echo "This indicates that the RPM tool chain is not installed and configured correctly."
	@exit 1
else
rpm: 
	@echo "--- Building Fedora/RHEL RPM Package ---"
	@echo "Checking for Rust target $(TARGET_ARCH_LINUX)..."
	@if ! rustup target list --installed | grep -q $(TARGET_ARCH_LINUX); then \
		echo "Error: Rust target $(TARGET_ARCH_LINUX) not installed."; \
		echo "Please run: rustup target add $(TARGET_ARCH_LINUX)"; \
		exit 1; \
	fi
	@echo "Checking for rpmbuild command..."
	@if ! command -v rpmbuild >/dev/null 2>&1; then \
		echo "Error: rpmbuild command not found."; \
		echo "Please install rpm-build package: sudo dnf install rpm-build"; \
		exit 1; \
	fi
	@echo "Building release binary for $(APP_NAME_PKG) version $(APP_VERSION_PKG_RPM) for $(TARGET_ARCH_LINUX)..."
	@cargo build --release --target $(TARGET_ARCH_LINUX)
	@echo "Creating RPM build directories..."
	@mkdir -p $(OUTPUT_DIR_PKG)/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
	@mkdir -p $(OUTPUT_DIR_PKG)/rpmbuild/BUILDROOT/$(APP_NAME_PKG)-$(APP_VERSION_PKG_RPM)-$(RPM_RELEASE).x86_64/usr/local/bin
	@echo "Copying binary to buildroot..."
	@cp "target/$(TARGET_ARCH_LINUX)/release/$(APP_NAME_PKG)" "$(OUTPUT_DIR_PKG)/rpmbuild/BUILDROOT/$(APP_NAME_PKG)-$(APP_VERSION_PKG_RPM)-$(RPM_RELEASE).x86_64/usr/local/bin/"
	@echo "Creating RPM spec file..."
	@printf 'Name: %s\nVersion: %s\nRelease: %s\nSummary: Autonomous Lawn Mower (cutter) Simulation\nLicense: MIT OR Apache-2.0\nGroup: Applications/Engineering\nBuildArch: x86_64\nRequires: glibc\n\n%%description\nGridCover is an autonomous lawn mower simulation tool that models coverage patterns and optimization strategies for robotic lawn mowers.\n\n%%install\nmkdir -p %%{buildroot}/usr/local/bin\ncp %s %%{buildroot}/usr/local/bin/\n\n%%files\n/usr/local/bin/%s\n\n%%changelog\n* %s %s <%s> - %s-1\n- RPM package\n' \
		"$(APP_NAME_PKG)" "$(APP_VERSION_PKG_RPM)" "$(RPM_RELEASE)" \
		"$(PWD)/target/$(TARGET_ARCH_LINUX)/release/$(APP_NAME_PKG)" "$(APP_NAME_PKG)" \
		"$$(date +'%a %b %d %Y')" "$(RPM_USER)" "$(RPM_USER_EMAIL)" "$(APP_VERSION_PKG_RPM)" \
		> $(OUTPUT_DIR_PKG)/rpmbuild/SPECS/$(APP_NAME_PKG).spec
	@echo "Building RPM package..."
	@rpmbuild --define "_topdir $(PWD)/$(OUTPUT_DIR_PKG)/rpmbuild" \
		-bb $(OUTPUT_DIR_PKG)/rpmbuild/SPECS/$(APP_NAME_PKG).spec
	@echo "Copying RPM to output directory..."
	@cp $(OUTPUT_DIR_PKG)/rpmbuild/RPMS/x86_64/$(OUTPUT_RPM_NAME) $(OUTPUT_DIR_PKG)/
	@echo "Cleaning up build directories..."
	@rm	 -rf $(OUTPUT_DIR_PKG)/rpmbuild
	@echo ""
	@echo "------------------------------------"
	@echo "RPM package created at: $(OUTPUT_DIR_PKG)/$(OUTPUT_RPM_NAME)"
	@echo "Install with: sudo dnf install $(OUTPUT_DIR_PKG)/$(OUTPUT_RPM_NAME)"
	@echo "Or: sudo rpm -ivh $(OUTPUT_DIR_PKG)/$(OUTPUT_RPM_NAME)"
	@echo "------------------------------------"
endif

install-completions: ## Generate and install shell completions for bash and zsh
    # Generate completions
    # ./target/release/gridcover --generate-completion bash > completions/gridcover.bash
    # ./target/release/gridcover --generate-completion zsh > completions/_gridcover
    
    # Install bash completion
    @if [ -d /usr/local/etc/bash_completion.d ]; then \
        cp assets/completions/gridcover.bash /usr/local/etc/bash_completion.d/; \
    elif [ -d /etc/bash_completion.d ]; then \
        cp assets/completions/gridcover.bash /etc/bash_completion.d/; \
    else \
        echo "Copy assets/completions/gridcover.bash to your bash completion directory"; \
    fi
    
    # Install zsh completion
    @if [ -d /usr/local/share/zsh/site-functions ]; then \
        cp assets/completions/_gridcover /usr/local/share/zsh/site-functions/; \
    elif [ -d /usr/share/zsh/site-functions ]; then \
        cp assets/completions/_gridcover /usr/share/zsh/site-functions/; \
    else \
        echo "Copy completions/_gridcover to your zsh completion directory"; \
    fi
    
    @echo "Completions installed. Restart your shell or source your shell config."