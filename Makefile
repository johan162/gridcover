## The cross compilation targets assume execution on macOS with the necessary tools installed.
## This Makefile is designed to work with Rust projects and provides commands for building, testing, packaging, and more.
## It includes targets for creating macOS .pkg installers for both Intel and ARM architectures, as well as a Windows executable.
## It also includes commands for linting, formatting, and generating coverage reports.
## Comments or bugs to: Johan Persson <johan162@gmail.com>
SHELL := /bin/bash
APP_NAME_PKG_PATH := $(shell cargo pkgid | cut -d'\#' -f1)
APP_NAME_PKG := $(shell basename $(APP_NAME_PKG_PATH))
APP_VERSION_PKG := $(shell cargo pkgid | cut -d'\#' -f2)
BUNDLE_ID_PKG := nu.aditus.oss.$(APP_NAME_PKG)
INSTALL_LOCATION_PKG := /usr/local/bin
TARGET_ARCH_INTEL_PKG := x86_64-apple-darwin
TARGET_ARCH_ARM_PKG := aarch64-apple-darwin
TARGET_ARCH_INTEL_WIN := x86_64-pc-windows-gnu
OUTPUT_DIR_PKG := target/pkg
OUTPUT_PKG_NAME_INTEL := $(APP_NAME_PKG)-$(APP_VERSION_PKG)-intel.pkg
OUTPUT_PKG_NAME_ARM := $(APP_NAME_PKG)-$(APP_VERSION_PKG)-arm.pkg
OUTPUT_PKG_NAME_INTEL_WIN := $(APP_NAME_PKG)-$(APP_VERSION_PKG)-windows.zip

## Setup PHONY targets for better readability and to avoid conflicts with file names
.PHONY: help, all, all-bin, clean, b, br, test, r, rr, lint, fmt, cov-html, cov, tst-pkg, pkg, pkg-intel, pkg-arm, win-exe, bump, install-pkg, uninstall-pkg
.DEFAULT_GOAL := help

help: ## Display this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

all-bin: ## Build the project for all supported architectures and create installers
	@echo "--- Building All Binaries and Packages ---"
	@echo "Building macOS Intel & ARM package..."
	@$(MAKE) pkg
	@echo "Building Windows package..."
	@$(MAKE) win-zip
	@echo "All binaries and packages created successfully."
	@echo ""

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

test: ## Test the project on host arch using cargo
	cargo test

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

tst-pkg-vars: ## Test the package creation process so that vars are set correctly
	@echo "--- Testing Package Variables ---"
	@echo "APP_NAME_PKG: $(APP_NAME_PKG)"
	@echo "APP_VERSION_PKG: $(APP_VERSION_PKG)"
	@echo "BUNDLE_ID_PKG: $(BUNDLE_ID_PKG)"
	@echo "TARGET_ARCH_INTEL_PKG: $(TARGET_ARCH_INTEL_PKG)"
	@echo "TARGET_ARCH_ARM_PKG: $(TARGET_ARCH_ARM_PKG)"
	@echo "OUTPUT_DIR_PKG: $(OUTPUT_DIR_PKG)"
	@echo "OUTPUT_PKG_NAME_INTEL: $(OUTPUT_PKG_NAME_INTEL)"
	@echo "OUTPUT_PKG_NAME_ARM: $(OUTPUT_PKG_NAME_ARM)"
	@echo "INSTALL_LOCATION_PKG: $(INSTALL_LOCATION_PKG)"
	@echo ""

pkg: pkg-intel pkg-arm ## Create a universal .pkg installer for both ARM and Intel Macs
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

b-intel: ## Build Intel arch debug profile using cargo
	cargo build --target $(TARGET_ARCH_INTEL_PKG)

br-intel: ## Build Intel arch release profile using cargo
	cargo build --release --target $(TARGET_ARCH_INTEL_PKG)

pkg-intel: ## Create a .pkg installer for Intel Macs (x86_64)
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

b-arm: ## Build ARM arch debug profile using cargo
	cargo build --target $(TARGET_ARCH_ARM_PKG)

br-arm: ## Build ARM arch release profile using cargo
	cargo build --release --target $(TARGET_ARCH_ARM_PKG)

pkg-arm: ## Create a .pkg installer for ARM Macs (aarch64)
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

b-win: ## Build Windows (x86_64-pc-windows-gnu) debug profile using cargo
	cargo build --target $(TARGET_ARCH_INTEL_WIN)

br-win: ## Build Windows (x86_64-pc-windows-gnu) release profile using cargo
	cargo build --release --target $(TARGET_ARCH_INTEL_WIN)

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

