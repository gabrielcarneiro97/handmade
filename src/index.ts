/* eslint-disable max-classes-per-file */

import PDFKit from 'pdfkit';
import * as fs from 'fs';
import * as path from 'path';
import sharp from 'sharp';

// const pdf = new PDFKit();
const lettersFolder = path.join(__dirname, 'assets', 'letters');

const colors : any = {
  black: {
    r: 0, g: 0, b: 0, alpha: 1,
  },
  blue: {
    r: 0, g: 0, b: 255, alpha: 1,
  },
};

const avaibleColors = Object.keys(colors);

const a4props = {
  width: 595,
  height: 842,
};

const pageProps = {
  margin: 10,
};

const lWidth = 7;

function avaibleLetters() {
  return fs.readdirSync(lettersFolder);
}

function getLetterWidh(letter : string) {
  return lWidth;
}

function randArrEl<T = any>(arr : Array<T>): T {
  return arr[Math.floor(Math.random() * arr.length)];
}

async function tintLetter(letterFileName : string, colorName : string) {
  if (!letterFileName.includes('black')) {
    throw new Error('Expected black letter to be tinted');
  }

  if (!avaibleColors.includes(colorName)) {
    throw new Error('Color not avaible');
  }

  const letterPath = path.join(lettersFolder, letterFileName);

  const newLetterFileName = letterFileName.replace('black', colorName);

  const newLetterPath = path.join(letterPath, newLetterFileName);

  await sharp(letterPath)
    .tint(colors[colorName])
    .toFile(newLetterPath);

  return newLetterFileName;
}

function getImagesFileNames(letter : string, colorName = 'black') {
  return avaibleLetters().filter((l) => l.startsWith(letter) && l.includes(colorName));
}

async function getImageFileName(letter : string, colorName : string) {
  const thisAvaible = avaibleColors.includes(colorName)
    ? getImagesFileNames(letter, colorName)
    : getImagesFileNames(letter);

  if (thisAvaible.length > 0) return randArrEl(thisAvaible);

  const thisBlack = getImagesFileNames(letter);

  const randBlack = randArrEl(thisBlack);

  return tintLetter(randBlack, colorName);
}

class Word {
  raw : string;
  colorName : string;
  width : number;
  imagesFileNames : string[];

  constructor(word : string, colorName : string) {
    this.raw = word;
    this.colorName = colorName;
    this.width = 0;
    this.imagesFileNames = [];
  }

  public static async processor(str : string, colorName : string) {
    const word = new Word(str, colorName);

    const letters = str.split('');

    word.imagesFileNames = await Promise.all(
      letters.map(async (letter) => {
        // const fileName = await getImageFileName(letter, colorName);
        word.width += getLetterWidh(letter);
        // return fileName;
        return letter;
      }),
    );

    return word;
  }
}

function getLineWidth(words : Word[]) {
  const spaces = words.length > 0 ? words.length - 1 : 0;
  return words.reduce((totalWidth, word) => totalWidth + word.width, 0) + (spaces * getLetterWidh(' '));
}

function addWord(line : Word[], word : Word) {
  const maxLineWidth = a4props.width - pageProps.margin * 2;

  const lineWidth = getLineWidth(line);
  if (lineWidth + word.width <= maxLineWidth) {
    line.push(word);
    return true;
  }

  return false;
}

async function defineLines(str : string, colorName : string = 'black') {
  const strWords = str.split(' ');
  const words = await Promise.all(
    strWords.map(async (word) => Word.processor(word, colorName)),
  );

  const lines : Array<Array<Word>> = [];

  let actualLineId = 0;
  lines[0] = [];

  const nextLine = () => {
    actualLineId += 1;
    lines[actualLineId] = [];
  };

  words.forEach((word) => {
    if (!addWord(lines[actualLineId], word)) {
      nextLine();
      addWord(lines[actualLineId], word);
    }
  });

  return lines;
}

async function textProcessor(str : string, colorName = 'black') {
  const lines = await defineLines(str, colorName);

  lines.forEach((line) => {
    console.log(line.map((word) => word.raw).join(' '));
  });
}

textProcessor('Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc vel quam semper, fermentum massa in, fringilla lacus. In hac habitasse platea dictumst. Morbi dolor ipsum, posuere nec dignissim eu, tempor laoreet sapien. Quisque ut lacinia tellus. Duis semper diam at urna consectetur condimentum. Mauris semper non libero sit amet dignissim. Sed vehicula urna sed quam pretium tristique. Morbi aliquam bibendum neque. In hac habitasse platea dictumst. Sed libero quam, sagittis ac nibh in, tincidunt venenatis neque.', 'black');

// const aTestPath = path.join(lettersFolder, 'a1.png');
// const aBlueTestPath = path.join(lettersFolder, 'a1b.png');

// pdf.image(aTestPath, 10, 20, { width: 10 });
// pdf.image(aTestPath, 20, 20, { width: 10 });

// sharp(aTestPath)
//   .tint(colors.black)
//   .toFile(aBlueTestPath);

// pdf.pipe(fs.createWriteStream('./teste.pdf'));

// pdf.end();
