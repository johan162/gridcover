# Release Notes for v2.5.0 (since v2.4.0)

## Summary

This is a feature release which speeds up the creation of animation by offering a all-in-memory video creation. 


## ✨  User visible New Features 

**In memory animation**
The additional flag `in-memory-animation` was introduced. This will save all frames in-memory instead of disk to create the animation video.
As a (very) rough rule of thumb you need 1GB of RAM for eery 4-5min of animation. For large complex obstacle maps this could easily double.
When this flag is enabled the percentage memory used will be shown in the progress information as a guidline.


## 🚀 Improvements

** **



## 🐛 Bug Fixes
- xx

## 🛠  Build system & Internal changes
- Further herdening of the `gen_font_data.sh` script to increase maintainability and hardening.
- Refactored the main() function to ease maintenance


## 📚 Documentation
- xx


**Full Changelog**: https://github.com/johan162/gridcover/compare/v2.3.0...v2.4.0