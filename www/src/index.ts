import { corrupt } from "hamming-corruptor";
import { decode } from "hamming-decoder";
import { encode_8_4 } from "hamming-encoder";
import { downloadFile, FileName } from "./utils";

interface File {
  name: string;
  content: Uint8Array;
  encoded: Uint8Array;
  corrupted: Uint8Array;
  decodedFixed: Uint8Array;
  decodedUnfixed: Uint8Array;
  text: string;
}

let file: File | undefined;

const fileInput = document.getElementById("file") as HTMLInputElement;
const getEncoded = document.getElementById("get_encoded");
const getCorruptedEncoded = document.getElementById("get_corrupted_encoded");
const getCorrupted = document.getElementById("get_corrupted");
const getRecovered = document.getElementById("get_recovered");

if (!getEncoded || !getCorruptedEncoded || !getCorrupted || !getRecovered) {
  throw new Error("Script load failed, missing elements in the HTML tree");
}

fileInput.onchange = async () => {
  const inputFile = fileInput.files?.[0];
  if (inputFile) {
    const buffer = await inputFile.arrayBuffer();
    const content = new Uint8Array(buffer);
    const encoded = encode_8_4(content);
    const corrupted = corrupt(encoded);
    const decodedFixed = decode(corrupted, true);
    const decodedUnfixed = decode(corrupted, false);
    const text = new TextDecoder().decode(buffer);

    console.log(inputFile.name);

    file = {
      name: fileInput.value,
      content,
      encoded,
      corrupted,
      decodedFixed,
      decodedUnfixed,
      text,
    };
  }
};

getEncoded.onclick = () => {
  if (!file) return;

  const fileName = new FileName(file.name);
  fileName.appendName(`.${fileName.extension}`).setExtension("hamming");
  downloadFile(fileName.getFileName(), file.encoded);
};

getCorruptedEncoded.onclick = () => {
  if (!file) return;

  const fileName = new FileName(file.name);
  fileName
    .appendName(`_corrupted.${fileName.extension}`)
    .setExtension("hamming");
  downloadFile(fileName.getFileName(), file.corrupted);
};

getCorrupted.onclick = () => {
  if (!file) return;

  const fileName = new FileName(file.name).appendName("_corrupted");
  downloadFile(fileName.getFileName(), file.decodedUnfixed);
};

getRecovered.onclick = () => {
  if (!file) return;

  const fileName = new FileName(file.name).appendName("_recovered");
  downloadFile(fileName.getFileName(), file.decodedFixed);
};
