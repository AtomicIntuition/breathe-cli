#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const VERSION = '0.1.0';
const REPO = 'AtomicIntuition/breathe-cli';

function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;

  const platformMap = {
    'darwin-x64': 'breathe-darwin-x64',
    'darwin-arm64': 'breathe-darwin-arm64',
    'linux-x64': 'breathe-linux-x64',
    'win32-x64': 'breathe-windows-x64.exe',
  };

  const key = `${platform}-${arch}`;
  const binaryName = platformMap[key];

  if (!binaryName) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    console.error('Supported platforms: darwin-x64, darwin-arm64, linux-x64, win32-x64');
    console.error('\nYou can install from source with: cargo install breathe');
    process.exit(1);
  }

  return {
    binaryName,
    isWindows: platform === 'win32',
  };
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        file.close();
        fs.unlinkSync(dest);
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        file.close();
        fs.unlinkSync(dest);
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }

      response.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', (err) => {
      file.close();
      fs.unlinkSync(dest);
      reject(err);
    });
  });
}

async function install() {
  const { binaryName, isWindows } = getPlatformInfo();
  const binDir = path.join(__dirname, 'bin');
  const finalBinaryName = isWindows ? 'breathe.exe' : 'breathe';
  const binaryPath = path.join(binDir, finalBinaryName);
  const extension = isWindows ? '.zip' : '.tar.gz';

  // Create bin directory
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const downloadUrl = `https://github.com/${REPO}/releases/download/v${VERSION}/${binaryName}${extension}`;
  const archivePath = path.join(binDir, `breathe${extension}`);

  console.log(`Downloading breathe v${VERSION} for ${process.platform}-${process.arch}...`);

  try {
    await downloadFile(downloadUrl, archivePath);

    if (isWindows) {
      // Use PowerShell to extract zip on Windows
      execSync(`powershell -command "Expand-Archive -Force '${archivePath}' '${binDir}'"`, { stdio: 'inherit' });
      const extractedFile = path.join(binDir, binaryName);
      if (fs.existsSync(extractedFile)) {
        fs.renameSync(extractedFile, binaryPath);
      }
    } else {
      // Use tar on Unix
      execSync(`tar -xzf "${archivePath}" -C "${binDir}"`, { stdio: 'inherit' });
      const extractedFile = path.join(binDir, binaryName);
      if (fs.existsSync(extractedFile)) {
        fs.renameSync(extractedFile, binaryPath);
      }
      fs.chmodSync(binaryPath, 0o755);
    }

    // Clean up archive
    fs.unlinkSync(archivePath);

    console.log('breathe installed successfully!');
  } catch (error) {
    console.error(`\nFailed to install breathe: ${error.message}`);
    console.error('\nThe release binaries may not be available yet.');
    console.error('You can install from source with: cargo install breathe');
    process.exit(1);
  }
}

install();
