# Create GitHub Release - Instructions

## Option 1: Using GitHub CLI (after restarting terminal)

Once you restart your terminal (to refresh PATH), run these commands:

```bash
# Authenticate with GitHub (if not already done)
gh auth login

# Create the release
gh release create v0.1.0 \
  --title "GdSerial v0.1.0 - Initial Release" \
  --notes-file RELEASE_NOTES.md \
  --target main \
  release/addons \
  release/README.md \
  release/LICENSE

# Alternative: Create release with archive
tar -czf gdserial-v0.1.0.tar.gz -C release .
gh release create v0.1.0 \
  --title "GdSerial v0.1.0 - Initial Release" \
  --notes-file RELEASE_NOTES.md \
  gdserial-v0.1.0.tar.gz
```

## Option 2: Manual Web Interface

1. **Commit and push your changes**:
```bash
git add .
git commit -m "Release v0.1.0: Initial GdSerial release with plugin structure"
git push origin main
```

2. **Go to GitHub**: Navigate to your repository

3. **Create Release**: Click "Releases" → "Create a new release"

4. **Fill out form**:
   - **Tag**: `v0.1.0`
   - **Title**: `GdSerial v0.1.0 - Initial Release`
   - **Description**: Copy content from `RELEASE_NOTES.md`

5. **Upload files**:
   - Create ZIP file: `release/addons` folder
   - Upload as release asset

## Release Contents Prepared:

✅ Built release libraries in `addons/gdserial/bin/`
✅ Plugin structure in `addons/gdserial/`
✅ Release notes in `RELEASE_NOTES.md`
✅ Release package in `release/` folder

## For Godot Asset Library Submission:

After creating the GitHub release, you can submit to Godot Asset Library with:
- **Asset URL**: Your GitHub repository URL
- **Version String**: v0.1.0
- **Download URL**: Will be auto-detected from your releases