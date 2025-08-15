# Release Notes for v2.4.0 (since v2.3.0)

## Summary

This is mostly a cleanup release focusing on increasing internal maintainability and bumping
all external dependencies to latest versions. Also, some default argument values have been tweaked.

## ✨  User visible New Features 

**Feature 1**
Description


## 🚀 Improvements

**Sim loop refactoring**
Simplify the main sim loop by refactoring all logical parts into separate functions and modules


## 🐛 Bug Fixes
- xx

## 🛠  Build system
- Optimize the dev-container spec to avoid double inclusion of vs-code extensions already loaded by the
Rust feature. Include scc & xxd in the container from start.
- Tweaking the `gen_font_data.sh` script to increase maintainability.


## 📚 Documentation
- xx


**Full Changelog**: https://github.com/johan162/gridcover/compare/v2.3.0...v2.4.0