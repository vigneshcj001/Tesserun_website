# Tesserun download website

A buildless, responsive landing page for the native Tesserun Windows app. HTML, CSS, and browser JavaScript live in `dist/`; no package installation is required.

## Preview

```powershell
cd D:\APp\Tesserun-website
node scripts/serve.mjs
```

Open http://127.0.0.1:4173. Run `node scripts/check.mjs` to check local links, accessibility references, and the download checksum.

## Publish on Vercel

Upload this website folder to its own GitHub repository and import that repository into Vercel. Choose **Other** as the framework, leave build and install commands empty, and set the output directory to **dist**. The included `vercel.json` declares these settings too. No environment variables are needed.

The Tesserun application repository is separate. Deploy this website repository to host the download page.

## Download

`dist/downloads/tesserun-0.3.0-windows-x64.exe` is copied from the successful local release build. It is a portable executable, not an installer. The page clearly identifies it as an unsigned early release with compatibility on other machines unverified. `SHA256SUMS.txt` identifies the distributed binary.

To replace the download, copy the new release into `dist/downloads/`, update the two download links and version/size text in `dist/index.html`, and regenerate the checksum file. Verify the binary and all packaged dependencies on a clean Windows machine before announcing broad compatibility.

The interactive workspace tour is illustrative. It never calls providers or publishes anything. The site uses no analytics, cookies, credentials, or backend. Google Fonts supplies optional typography; system fonts remain usable if unavailable. A reduced-motion preference disables animation.
