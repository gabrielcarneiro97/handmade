import PDFKit from 'pdfkit';
import * as fs from 'fs';
import * as path from 'path';
import sharp from 'sharp';

const pdf = new PDFKit();

const colors = {
  black: {
    r: 0, g: 0, b: 0, alpha: 1,
  },
  blue: {
    r: 0, g: 0, b: 255, alpha: 1,
  },
};

const lettersFolder = path.join(__dirname, 'assets', 'letters');

const aTestPath = path.join(lettersFolder, 'a1.png');
const aBlueTestPath = path.join(lettersFolder, 'a1b.png');

// pdf.image(aTestPath, 10, 20, { width: 10 });
// pdf.image(aTestPath, 20, 20, { width: 10 });

sharp(aTestPath)
  .tint(colors.black)
  .toFile(aBlueTestPath);

// pdf.pipe(fs.createWriteStream('./teste.pdf'));

// pdf.end();
